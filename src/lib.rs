mod opengl;
mod data;

use opengl::*;

extern {
    //fn consoleLog(x: u32);
    fn playAudio(audio_ptr: *const f32, audio_len: u32);
    fn setScore(left: u32, right: u32);
}

const PADDLE_SPEED: f32 = 0.001;
const BALL_SPEED: f32 = 0.0012;

const AUDIO_BUFFER_SIZE: usize = 8192;

struct RenderContext {
    shader_program: ShaderProgram,
    position: VertexArray,
    texcoord: VertexArray,
    offset: Uniform,
    sampler: Uniform,
    opacity: Uniform,
}

impl RenderContext {
    unsafe fn new(program: ShaderProgram) -> RenderContext {
        RenderContext {
            shader_program: program,
            position: program.vertex_array("a_position"),
            texcoord: program.vertex_array("a_texcoord"),
            offset: program.uniform("u_offset"),
            sampler: program.uniform("u_sampler"),
            opacity:  program.uniform("u_opacity")
        }
    }
}

struct Pong {
    ctx: RenderContext,
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

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe fn onInit() {
    clear_color(0.1, 0.1, 0.1, 1.0);
    enable(Capability::DepthTest);
    enable(Capability::Blend);
    depth_func(Comparison::LessOrEqual);
    blend_func(BlendFactor::SourceAlpha, BlendFactor::One);
    clear(ColorBuffer | DepthBuffer);

    let vertex_shader = VertexShader::new(data::VERTEX_SHADER);
    let fragment_shader = FragmentShader::new(data::FRAGMENT_SHADER);
    let program = ShaderProgram::new(&vertex_shader, &fragment_shader);

    let ctx = RenderContext::new(program);

    let ball_texture = Texture::new(TextureType::Texture2D).with_data(&data::BALL_TEXTURE, 4, 4);
    let ball_tail_texture = Texture::new(TextureType::Texture2D).with_data(&data::BALL_TAIL_TEXTURE, 4, 4);
    let spark_texture = Texture::new(TextureType::Texture2D).with_data(&data::SPARK_TEXTURE, 4, 4);
    let paddle_texture = Texture::new(TextureType::Texture2D).with_data(&data::PADDLE_TEXTURE, 8, 8);
    let field_texture = Texture::new(TextureType::Texture2D).with_data(&data::FIELD_TEXTURE, 8, 8);

    let ball_model = Model::new(&data::BALL_VERTICES, ball_texture);
    let ball_tail_model = Model::new(&data::BALL_TAIL_VERTICES, ball_tail_texture);
    let spark_model = Model::new(&data::SPARK_VERTICES, spark_texture);
    let paddle_model = Model::new(&data::PADDLE_VERTICES, paddle_texture);
    let field_model = Model::new(&data::FIELD_VERTICES, field_texture);

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

    PONG = Some(Pong {
        ctx,
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

#[no_mangle]
#[allow(non_snake_case)]
pub fn onAnimationFrame(timestamp: i32) {
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
        } else if ball.position.x > 1.05 {
            ball.position.x = 0.0;
            pong.left_score += 1;
            ball.velocity.x = (1 - 2 * (timestamp % 2)) as f32;
            ball.velocity.y = (1 - 2 * ((timestamp/7) % 2)) as f32;
            play_audio(&pong.bloop);
            unsafe { setScore(0, pong.left_score) };
        } else if ball.position.x < -1.05 {
            ball.position.x = 0.0;
            pong.right_score += 1;
            ball.velocity.x = (1 - 2 * (timestamp % 2)) as f32;
            ball.velocity.y = (1 - 2 * ((timestamp/7) % 2)) as f32;
            play_audio(&pong.bloop);
            unsafe { setScore(1, pong.right_score) };
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

    clear(ColorBuffer | DepthBuffer);
    pong.ctx.shader_program.enable();
    pong.ctx.position.enable();
    pong.ctx.texcoord.enable();

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

#[no_mangle]
#[allow(non_snake_case)]
pub fn onKey(key: u32, state: bool) {
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

#[derive(Clone)]
struct Vec2 {
    x: f32,
    y: f32
}

impl Vec2 {
    fn zero() -> Vec2 {
        Vec2 { x: 0.0, y: 0.0 }
    }
    fn new(x: f32, y: f32) -> Vec2 {
        Vec2 { x, y }
    }
}

struct Model {
    vertex_buffer: Buffer,
    num_vertices: u32,
    texture: Texture,
    extent: Vec2
}

impl Model {
    fn new(vertices: &[f32], texture: Texture) -> Model {
        let vertex_buffer = Buffer::new(BufferType::Array);
        vertex_buffer.bind().data(vertices, DrawType::Static);

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
        self.vertex_buffer.bind();
        self.texture.bind(TextureUnit::Texture0);
        ctx.position.pointer(2, DataType::Float, Bool::False, 16, 0);
        ctx.texcoord.pointer(2, DataType::Float, Bool::False, 16, 8);
        ctx.sampler.int_1(0);
        ctx.opacity.float_1(1.0);
    }
    fn render(&self, pos: &Vec2, ctx: &RenderContext) {
        ctx.offset.float_4(pos.x, pos.y, 0.0, 0.0);
        draw_arrays(ArrayType::Triangles, 0, self.num_vertices / 4);
    }
    fn render_particle(&self, pos: &Vec2, opacity: f32, ctx: &RenderContext) {
        ctx.offset.float_4(pos.x, pos.y, 0.0, 0.0);
        ctx.opacity.float_1(opacity);
        draw_arrays(ArrayType::Triangles, 0, self.num_vertices / 4);
    }
}

struct Paddle {
    position: Vec2,
    up: bool,
    down: bool,
}

struct Ball {
    position: Vec2,
    velocity: Vec2
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
    alive: usize
}

impl ParticleSystem {
    fn new(max_particles: usize) -> ParticleSystem {
        ParticleSystem {
            max_particles,
            particles: Vec::with_capacity(max_particles),
            alive: 0
        }
    }
    fn render(&self, model: &Model, ctx: &RenderContext) {
        model.pre_render(ctx);
        for particle in self.particles.iter().take(self.alive) {
            model.render_particle(&particle.position, 
                                  particle.life as f32 / particle.total_life as f32,
                                  ctx);
        }
    }
    fn add(&mut self, position: Vec2, velocity: Vec2, acceleration: Vec2, life: i32) {
        self.particles.push(Particle { position, velocity, acceleration, life, total_life: life });
        self.alive += 1;
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
fn collide(p1: &Vec2, e1: &Vec2, p2: &Vec2, e2: &Vec2) -> bool {
    (if p1.x < p2.x { p2.x - p1.x } else { p1.x - p2.x }) < e1.x + e2.x &&
        (if p1.y < p2.y { p2.y - p1.y } else { p1.y - p2.y }) < e1.y + e2.y
}

fn clamp(x: f32, min: f32, max: f32) -> f32 {
    x.max(min).min(max)
}

fn play_audio(sample: &Vec<f32>) {
    unsafe {
        playAudio(sample.as_ptr(), sample.len() as u32);
    }
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
