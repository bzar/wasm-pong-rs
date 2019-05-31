mod data;

use std::cell::RefCell;
use std::rc::Rc;

use js_sys::WebAssembly;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{
    WebGlProgram, WebGlUniformLocation, WebGlRenderingContext, WebGlShader,
    WebGlTexture, WebGlBuffer, AudioContext, AudioBuffer, HtmlCanvasElement,
    KeyboardEvent
};

// Convenience alias for referring to OpenGL constants
type GL = WebGlRenderingContext;

const PADDLE_SPEED: f32 = 0.001;
const BALL_SPEED: f32 = 0.0012;

const AUDIO_BUFFER_SIZE: usize = 8192;
type WebGlVertexArray = i32;

struct RenderContext {
    gl: WebGlRenderingContext,
    program: WebGlProgram,
    position: WebGlVertexArray,
    texcoord: WebGlVertexArray,
    offset: WebGlUniformLocation,
    sampler: WebGlUniformLocation,
    opacity: WebGlUniformLocation,
}

#[derive(Clone)]
struct Vec2 {
    x: f32,
    y: f32
}

struct Model {
    vertex_buffer: WebGlBuffer,
    num_vertices: u32,
    texture: WebGlTexture,
    extent: Vec2
}

struct Ball {
    position: Vec2,
    velocity: Vec2
}

struct Paddle {
    position: Vec2,
    up: bool,
    down: bool,
}

struct Particle {
    position: Vec2,
    velocity: Vec2,
    acceleration: Vec2,
    life: i32,
    total_life: i32
}

struct ParticleSystem {
    max_particles: usize,
    particles: Vec<Particle>,   
}

struct Pong {
    ctx: RenderContext,
    audio_ctx: AudioContext,
    audio_buffer: AudioBuffer,

    timestamp: i32,

    ball_model: Model,
    ball_tail_model: Model,
    paddle_model: Model,
    spark_model: Model,
    field_model: Model,

    beep: Vec<f32>,
    boop: Vec<f32>,
    bloop: Vec<f32>,

    ball: Ball,
    left: Paddle,
    right: Paddle,

    ball_tail: ParticleSystem,
    sparks: ParticleSystem,

    left_score: u32,
    right_score: u32
}

static mut PONG: Option<Pong> = None;

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let canvas: HtmlCanvasElement = document.get_element_by_id("canvas").unwrap().dyn_into()?;
    let ctx_options = js_sys::Object::new();
    js_sys::Reflect::set(&ctx_options, &"alpha".into(), &false.into()).unwrap();
    let gl: WebGlRenderingContext = canvas
        .get_context_with_context_options("webgl", &ctx_options)?.unwrap().dyn_into()?;

    gl.clear_color(0.1, 0.1, 0.1, 1.0);
    gl.enable(GL::DEPTH_TEST);
    gl.enable(GL::BLEND);
    gl.depth_func(GL::LEQUAL);
    gl.blend_func(GL::SRC_ALPHA, GL::ONE_MINUS_SRC_ALPHA);
    gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);

    let vert_shader = compile_shader(&gl, GL::VERTEX_SHADER, data::VERTEX_SHADER)?;
    let frag_shader = compile_shader(&gl, GL::FRAGMENT_SHADER, data::FRAGMENT_SHADER)?;
    let program = link_program(&gl, &vert_shader, &frag_shader)?;
    gl.use_program(Some(&program));

    let ctx = RenderContext::new(gl, program);

    let ball_texture = ctx.load_texture(&data::BALL_TEXTURE, 4, 4);
    let ball_tail_texture = ctx.load_texture(&data::BALL_TAIL_TEXTURE, 4, 4);
    let spark_texture = ctx.load_texture(&data::SPARK_TEXTURE, 4, 4);
    let paddle_texture = ctx.load_texture(&data::PADDLE_TEXTURE, 8, 8);
    let field_texture = ctx.load_texture(&data::FIELD_TEXTURE, 8, 8);

    let ball_model = Model::new(&ctx, &data::BALL_VERTICES, ball_texture);
    let ball_tail_model = Model::new(&ctx, &data::BALL_TAIL_VERTICES, ball_tail_texture);
    let spark_model = Model::new(&ctx, &data::SPARK_VERTICES, spark_texture);
    let paddle_model = Model::new(&ctx, &data::PADDLE_VERTICES, paddle_texture);
    let field_model = Model::new(&ctx, &data::FIELD_VERTICES, field_texture);

    let mut beep: Vec<f32> = Vec::with_capacity(AUDIO_BUFFER_SIZE);
    let mut boop: Vec<f32> = Vec::with_capacity(AUDIO_BUFFER_SIZE);
    let mut bloop: Vec<f32> = Vec::with_capacity(AUDIO_BUFFER_SIZE);

    for i in 0..AUDIO_BUFFER_SIZE {
        let sq64 = if i/64 % 2 == 0 { 0.1 } else { -0.1 };
        let sq128 = if i/128 % 2 == 0 { 0.1 } else { -0.1 };
        beep.push(sq64);
        boop.push(sq128);
        bloop.push(sq64 + sq128);
    }

    let audio_ctx = AudioContext::new().unwrap();
    let audio_buffer = audio_ctx.create_buffer(
        1, (audio_ctx.sample_rate() * 2.0) as u32, audio_ctx.sample_rate()).unwrap();

    unsafe {
        PONG = Some(Pong {
            ctx, audio_ctx, audio_buffer,
            timestamp: 0,

            ball_model, ball_tail_model, paddle_model, spark_model, field_model,
            beep, boop, bloop,

            ball: Ball {
                position: Vec2::zero(),
                velocity: Vec2::new(1.0, 1.0),
            },
            left: Paddle {
                position: Vec2::new(-0.9, 0.0),
                up: false, down: false
            },
            right: Paddle {
                position: Vec2::new(0.9, 0.0),
                up: false, down: false
            },
            ball_tail: ParticleSystem::new(100),
            sparks: ParticleSystem::new(100),

            left_score: 0,
            right_score: 0
        });
    }

    // FIXME: Hack for requestAnimationFrame loop
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();
    *g.borrow_mut() = Some(Closure::wrap(Box::new(move |i| {
        on_animation_frame(i);
        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<FnMut(i32)>));
    request_animation_frame(g.borrow().as_ref().unwrap());

    // FIXME: Hacky key event handler binding
    let onkeydown_handler = Closure::wrap(Box::new(|e: KeyboardEvent| {
        on_key(e.key_code(), true);
    }) as Box<FnMut(KeyboardEvent)>);
    window.set_onkeydown(Some(onkeydown_handler.as_ref().unchecked_ref()));
    onkeydown_handler.forget();

    let onkeyup_handler = Closure::wrap(Box::new(|e: KeyboardEvent| {
        on_key(e.key_code(), false);
    }) as Box<FnMut(KeyboardEvent)>);
    window.set_onkeyup(Some(onkeyup_handler.as_ref().unchecked_ref()));
    onkeyup_handler.forget();

    Ok(())
}

fn request_animation_frame(f: &Closure<FnMut(i32)>) {
    web_sys::window().unwrap()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

pub fn on_animation_frame(timestamp: i32) {
    let pong = unsafe { PONG.as_mut().unwrap() };
    let delta = match pong.timestamp {
        0 => 1,
        x => timestamp - x
    } as f32;
    pong.timestamp = timestamp;

    let left = &mut pong.left;
    let right = &mut pong.right;

    let left_direction = if left.up == left.down { 0. } else if left.up { 1. } else { -1. };
    let right_direction = if right.up == right.down { 0. } else if right.up { 1. } else { -1. };
    left.position.y = clamp(left.position.y + left_direction * PADDLE_SPEED * delta, -0.8, 0.8);
    right.position.y = clamp(right.position.y + right_direction * PADDLE_SPEED * delta, -0.8, 0.8);

    let ball = &mut pong.ball;
    ball.position.x += ball.velocity.x * delta * BALL_SPEED;
    if collide(&ball.position, &pong.ball_model.extent,
               &left.position, &pong.paddle_model.extent) 
        || collide(&ball.position, &pong.ball_model.extent,
                   &right.position, &pong.paddle_model.extent) {
            ball.velocity.x = -ball.velocity.x;
            ball.position.x += ball.velocity.x * delta * BALL_SPEED;
            play_audio(&pong.beep);
            create_sparks(&mut pong.sparks,
                          (if ball.velocity.x > 0. { -1. } else { 1. })
                          * pong.ball_model.extent.x + ball.position.x,
                          ball.position.y, 2. * ball.velocity.x, 0.);
        } else if ball.position.x.abs() > 1.05 {
            if ball.position.x > 0.0 {
                pong.left_score += 1;
            } else {
                pong.right_score += 1;
            }

            ball.position.x = 0.0;
            ball.velocity.x = (1 - 2 * (timestamp % 2)) as f32;
            ball.velocity.y = (1 - 2 * ((timestamp/7) % 2)) as f32;
            play_audio(&pong.bloop);
            set_score(pong.left_score, pong.right_score);
        }

    ball.position.y += ball.velocity.y * delta as f32 * BALL_SPEED;
    if collide(&ball.position, &pong.ball_model.extent,
               &left.position, &pong.paddle_model.extent) 
        || collide(&ball.position, &pong.ball_model.extent,
                   &right.position, &pong.paddle_model.extent) {
            ball.velocity.y = -ball.velocity.y;
            ball.position.y += ball.velocity.y * delta as f32 * BALL_SPEED;
            play_audio(&pong.beep);
            create_sparks(&mut pong.sparks, ball.position.x,
                          (if ball.velocity.y > 0. { -1. } else { 1. })
                          * pong.ball_model.extent.y + ball.position.y,
                          0., 2. * ball.velocity.y);
        } else if ball.position.y > 0.95 {
            ball.velocity.y = -ball.velocity.y.abs();
            play_audio(&pong.boop);
            create_sparks(&mut pong.sparks, ball.position.x,
                          pong.ball_model.extent.y + ball.position.y,
                          0., 2. * ball.velocity.y);
        } else if ball.position.y < -0.95 {
            ball.velocity.y = ball.velocity.y.abs();
            play_audio(&pong.boop);
            create_sparks(&mut pong.sparks, ball.position.x,
                          -pong.ball_model.extent.y + ball.position.y,
                          0., 2. * ball.velocity.y);
        }

    pong.ball_tail.add(ball.position.clone(), Vec2::zero(), Vec2::zero(), 1000);
    pong.ball_tail.update(delta);
    pong.sparks.update(delta);

    pong.ctx.gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);
    pong.ctx.gl.use_program(Some(&pong.ctx.program));
    pong.ctx.gl.enable_vertex_attrib_array(pong.ctx.position as u32);
    pong.ctx.gl.enable_vertex_attrib_array(pong.ctx.texcoord as u32);

    pong.field_model.pre_render(&pong.ctx);
    pong.field_model.render(&Vec2 {x: 0.0, y: 0.0}, &pong.ctx);

    pong.ball_tail.render(&pong.ball_tail_model, &pong.ctx);
    pong.ball_model.pre_render(&pong.ctx);
    pong.ball_model.render(&ball.position, &pong.ctx);

    pong.paddle_model.pre_render(&pong.ctx);
    pong.paddle_model.render(&left.position, &pong.ctx);
    pong.paddle_model.render(&right.position, &pong.ctx);

    pong.sparks.render(&pong.spark_model, &pong.ctx);
}

pub fn on_key(key: u32, state: bool) {
    const KEY_UP: u32 = 38;
    const KEY_DOWN: u32 = 40;
    const KEY_A: u32 = 65;
    const KEY_Z: u32 = 90;

    let pong = unsafe { PONG.as_mut().unwrap() };

    match key {
        KEY_UP => pong.right.up = state,
        KEY_DOWN => pong.right.down = state,
        KEY_A => pong.left.up = state,
        KEY_Z => pong.left.down = state,
        _ => ()
    };
}

impl Vec2 {
    fn zero() -> Vec2 {
        Vec2 { x: 0.0, y: 0.0 }
    }
    fn new(x: f32, y: f32) -> Vec2 {
        Vec2 { x, y }
    }
}

impl RenderContext {
    fn new(gl: WebGlRenderingContext, program: WebGlProgram) -> RenderContext {
        let position = gl.get_attrib_location(&program, "a_position");
        let texcoord = gl.get_attrib_location(&program, "a_texcoord");
        let offset = gl.get_uniform_location(&program, "u_offset").unwrap();
        let sampler = gl.get_uniform_location(&program, "u_sampler").unwrap();
        let opacity = gl.get_uniform_location(&program, "u_opacity").unwrap();
        RenderContext {
            gl, program, position, texcoord,
            offset, sampler, opacity
        }
    }
    fn load_texture(&self, data: &[u8], width: i32, height: i32) -> WebGlTexture {
        let texture = self.gl.create_texture().unwrap();
        self.gl.active_texture(GL::TEXTURE0);
        self.gl.bind_texture(GL::TEXTURE_2D, Some(&texture));
        self.gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
            GL::TEXTURE_2D, 0, GL::RGBA as i32,
            width, height, 0, GL::RGBA, 
            GL::UNSIGNED_BYTE,
            Some(data)).unwrap();
        self.gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_MAG_FILTER, GL::NEAREST as i32);
        self.gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_MIN_FILTER, GL::NEAREST as i32);
        texture
    }
}

impl Model {
    fn new(ctx: &RenderContext, vertices: &[f32], texture: WebGlTexture) -> Model {
        let vertex_buffer = ctx.gl.create_buffer().unwrap();
        ctx.gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&vertex_buffer));

        // FIXME: Hack to fill a vertex buffer
        let memory_buffer = wasm_bindgen::memory()
            .dyn_into::<WebAssembly::Memory>().unwrap()
            .buffer();
        let vertices_location = vertices.as_ptr() as u32 / 4;
        let vertex_array = js_sys::Float32Array::new(&memory_buffer)
            .subarray(vertices_location, vertices_location + vertices.len() as u32);

        ctx.gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&vertex_buffer));
        ctx.gl.buffer_data_with_array_buffer_view(WebGlRenderingContext::ARRAY_BUFFER,
                                                  &vertex_array,
                                                  WebGlRenderingContext::STATIC_DRAW);

        let mut x: f32 = 0.0;
        let mut y: f32 = 0.0;

        for xs in vertices.chunks(4) {
            x = x.max(xs.get(0).unwrap().abs());
            y = y.max(xs.get(1).unwrap().abs());
        }

        Model {
            vertex_buffer,
            num_vertices: vertices.len() as u32,
            texture,
            extent: Vec2 { x: x * 0.9, y: y * 0.9 }
        }
    }
    fn pre_render(&self, ctx: &RenderContext) {
        ctx.gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.vertex_buffer));
        ctx.gl.active_texture(GL::TEXTURE0);
        ctx.gl.bind_texture(GL::TEXTURE_2D, Some(&self.texture));
        ctx.gl.vertex_attrib_pointer_with_i32(ctx.position as u32, 2, GL::FLOAT, false, 16, 0);
        ctx.gl.vertex_attrib_pointer_with_i32(ctx.texcoord as u32, 2, GL::FLOAT, false, 16, 8);
        ctx.gl.uniform1i(Some(&ctx.sampler), 0);
        ctx.gl.uniform1f(Some(&ctx.opacity), 1.0);
    }
    fn render(&self, pos: &Vec2, ctx: &RenderContext) {
        ctx.gl.uniform4f(Some(&ctx.offset), pos.x, pos.y, 0.0, 0.0);
        ctx.gl.draw_arrays(GL::TRIANGLES, 0, self.num_vertices as i32 / 4);
    }
    fn render_particle(&self, pos: &Vec2, opacity: f32, ctx: &RenderContext) {
        ctx.gl.uniform4f(Some(&ctx.offset), pos.x, pos.y, 0.0, 0.0);
        ctx.gl.uniform1f(Some(&ctx.opacity), opacity);
        ctx.gl.draw_arrays(GL::TRIANGLES, 0, self.num_vertices as i32 / 4);
    }
}



impl ParticleSystem {
    fn new(max_particles: usize) -> ParticleSystem {
        ParticleSystem {
            max_particles,
            particles: Vec::with_capacity(max_particles)
        }
    }
    fn render(&self, model: &Model, ctx: &RenderContext) {
        model.pre_render(ctx);
        for particle in self.particles.iter() {
            model.render_particle(&particle.position, 
                                  particle.life as f32 / particle.total_life as f32,
                                  ctx);
        }
    }
    fn add(&mut self, position: Vec2, velocity: Vec2, acceleration: Vec2, life: i32) {
        if self.particles.len() < self.max_particles {
            self.particles.push(Particle { position, velocity, acceleration, life, total_life: life });
        }
    }
    fn update(&mut self, delta: f32) {
        for p in self.particles.iter_mut() {
            p.life -= delta as i32;
            p.velocity.x += p.acceleration.x * delta / 1000.0;
            p.velocity.y += p.acceleration.y * delta / 1000.0;
            p.position.x += p.velocity.x * delta / 1000.0;
            p.position.y += p.velocity.y * delta / 1000.0;
        }
        self.particles.retain(|p| p.life > 0);
    }
}

fn compile_shader(ctx: &WebGlRenderingContext, shader_type: u32, source: &str) -> Result<WebGlShader, String> {
    let shader = ctx.create_shader(shader_type)
        .ok_or_else(|| String::from("Unable to create shader object"))?;
    ctx.shader_source(&shader, source);
    ctx.compile_shader(&shader);

    let ok = ctx.get_shader_parameter(&shader, GL::COMPILE_STATUS)
            .as_bool()
            .unwrap_or(false);
    if ok {
        Ok(shader)
    } else {
        Err(ctx.get_shader_info_log(&shader)
            .unwrap_or_else(|| String::from("Unknown error creating shader")))
    }
}

fn link_program(ctx: &WebGlRenderingContext, vert_shader: &WebGlShader, frag_shader: &WebGlShader) -> Result<WebGlProgram, String> {
    let program = ctx.create_program().ok_or_else(|| String::from("Unable to create shader object"))?;

    ctx.attach_shader(&program, vert_shader);
    ctx.attach_shader(&program, frag_shader);
    ctx.link_program(&program);

    let ok = ctx.get_program_parameter(&program, GL::LINK_STATUS)
        .as_bool()
        .unwrap_or(false);
    if ok {
        Ok(program)
    } else {
        Err(ctx.get_program_info_log(&program)
            .unwrap_or_else(|| String::from("Unknown error creating program object")))
    }
}
fn collide(p1: &Vec2, e1: &Vec2, p2: &Vec2, e2: &Vec2) -> bool {
    (if p1.x < p2.x { p2.x - p1.x } else { p1.x - p2.x }) < e1.x + e2.x &&
        (if p1.y < p2.y { p2.y - p1.y } else { p1.y - p2.y }) < e1.y + e2.y
}

fn clamp(x: f32, min: f32, max: f32) -> f32 {
    x.max(min).min(max)
}

fn play_audio(sample: &[f32]) {
    let pong = unsafe { PONG.as_mut().unwrap() };
    let ctx = &pong.audio_ctx;
    let buffer = &pong.audio_buffer;

    let source = ctx.create_buffer_source().unwrap();

    // FIXME: copy_to_channel requires a mutable reference for some reason
    let mut_sample = unsafe {(sample as *const [f32] as *mut [f32]).as_mut().unwrap()};

    buffer.copy_to_channel(mut_sample, 0).unwrap();
    source.set_buffer(Some(&buffer));
    source.connect_with_audio_node(&ctx.destination()).unwrap();
    ctx.resume().unwrap();
    source.start().unwrap();
}

fn set_score(left: u32, right: u32) {
    let document = web_sys::window().unwrap().document().unwrap();
    document.get_element_by_id("score_left").unwrap()
        .set_text_content(Some(&left.to_string()));
    document.get_element_by_id("score_right").unwrap()
        .set_text_content(Some(&right.to_string()));
}

fn create_sparks(ps: &mut ParticleSystem, x: f32, y: f32, dx: f32, dy: f32) {
    for i in 0..4 {
        let i = i as f32;
        let ddx = (i + 1.0) * dx / 10.0;
        let ddy = (i + 1.0) * dy / 10.0;
        ps.add(Vec2::new(x, y), Vec2::new(dy + ddx, -dx + ddy), Vec2::zero(), 100);
        ps.add(Vec2::new(x, y), Vec2::new(-dy + ddx, dx + ddy), Vec2::zero(), 100);
    }
}


