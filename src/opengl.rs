use std::mem::size_of;
use std::ops::BitOr;

extern {
    fn compileShader(source: *const u8, len: u32, shader_type: u32) -> u32;
    fn linkShaderProgram(vertexShader: u32, fragmentShader: u32) -> u32;
    fn glUseProgram(program_id: u32);

    fn glClearColor(r: f32, g: f32, b: f32, a: f32);
    fn glEnable(x: u32);
    fn glDepthFunc(x: u32);
    fn glBlendFunc(x: u32, y: u32);
    fn glClear(x: u32);
    fn glGetAttribLocation(program_id: u32, name_ptr: *const u8, name_len: u32) -> i32;
    fn glGetUniformLocation(program_id: u32, name_ptr: *const u8,
                                name_len: u32) -> i32;
    fn glUniform4fv(location_id: i32, x: f32, y: f32, z: f32, w: f32);
    fn glUniform1i(location_id: i32, x: i32);
    fn glUniform1f(location_id: i32, x: f32);
    fn glCreateBuffer() -> u32;
    fn glBindBuffer(buffer_type: u32, buffer_id: u32);
    fn glBufferData(data_type: u32, data_ptr: *const f32, data_count: u32, draw_type: u32);
    fn glEnableVertexAttribArray(attribute_location: i32);
    fn glVertexAttribPointer(attribute_location: i32, attribute_size: u32,
                                 attribute_type: u32, normalize: u32, stride: u32, offset: u32);
    fn glDrawArrays(array_type: u32, offset: u32, count: u32);
    fn glCreateTexture() -> u32;
    fn glBindTexture(target: u32, texture_id: u32);
    fn glTexParameteri(target: u32, name: u32, value: u32);
    fn glActiveTexture(target: u32);
    fn glTexImage2D(target: u32, level: u32, internal_format: u32, width: u32, height: u32, border: u32, format: u32, image_type: u32, image_ptr: *const u8, image_len: u32);
}

#[derive(Clone, Copy)]
pub enum Capability { DepthTest = 2929, Blend = 3042 }
#[derive(Clone, Copy)]
pub enum ShaderType { Vertex = 35633, Fragment = 35632 }
#[derive(Clone, Copy)]
pub enum BufferType { Array = 34962 }
#[derive(Clone, Copy)]
pub enum ArrayType { Triangles = 4 }
#[derive(Clone, Copy)]
pub enum DrawType { Static = 35044 }
#[derive(Clone, Copy)]
pub enum DataType { Float = 5126, UnsignedByte = 5121 }
#[derive(Clone, Copy)]
pub enum Comparison { LessOrEqual = 515 }
#[derive(Clone, Copy)]
pub enum TextureType { Texture2D = 3553 }
#[derive(Clone, Copy)]
pub enum PixelFormat { RGBA = 6408 }
#[derive(Clone, Copy)]
pub enum TextureFilter { Mag = 10240, Min = 10241 }
#[derive(Clone, Copy)]
pub enum TextureFilterType { Nearest = 9728 }
#[derive(Clone, Copy)]
pub enum TextureUnit { Texture0 = 33984 }
#[derive(Clone, Copy)]
pub enum BlendFactor { SourceAlpha = 770, OneMinusSourceAlpha = 771, /*One = 1*/ }
#[derive(Clone, Copy)]
pub enum Bool { False = 0, /*True = 1*/ }

#[derive(Clone, Copy)]
pub struct Buffer(u32, BufferType);
#[derive(Clone, Copy)]
pub struct BoundBuffer<'a>(&'a Buffer);
#[derive(Clone, Copy)]
pub struct Texture(u32, TextureType);
#[derive(Clone, Copy)]
pub struct BoundTexture<'a>(&'a Texture);
#[derive(Clone, Copy)]
pub struct VertexShader(u32);
#[derive(Clone, Copy)]
pub struct FragmentShader(u32);
#[derive(Clone, Copy)]
pub struct ShaderProgram(u32);
#[derive(Clone, Copy)]
pub struct Attribute(i32);
#[derive(Clone, Copy)]
pub struct VertexArray(Attribute);
#[derive(Clone, Copy)]
pub struct Uniform(i32);

pub struct ClearBit(u32);
pub const COLOR_BUFFER: ClearBit = ClearBit(16384);
pub const DEPTH_BUFFER: ClearBit = ClearBit(256);

impl BitOr for ClearBit {
    type Output = ClearBit;
    fn bitor(self, rhs: Self) -> Self {
        ClearBit(self.0 | rhs.0)
    }
}
impl Buffer {
    pub fn new(buffer_type: BufferType) -> Buffer {
        Buffer(unsafe { glCreateBuffer() }, buffer_type)
    }
    pub fn bind(&self) -> BoundBuffer {
        unsafe { glBindBuffer(self.1 as u32, self.0) }
        BoundBuffer(self)
    }
}
impl<'a> BoundBuffer<'a> {
    pub fn data(&mut self, data: &[f32], draw_type: DrawType) {
        unsafe { glBufferData((self.0).1 as u32, data.as_ptr(), data.len() as u32, draw_type as u32) }
    }
}
impl Texture {
    pub fn new(texture_type: TextureType) -> Texture {
        unsafe { Texture(glCreateTexture(), texture_type) }
    }
    pub fn bind(&self, texture_unit: TextureUnit) -> BoundTexture {
        unsafe {
            glActiveTexture(texture_unit as u32);
            glBindTexture(self.1 as u32, self.0);
        }
        BoundTexture(self)
    }
    pub fn with_data(self, data: &[u8], width: u32, height: u32) -> Texture {
        self.bind(TextureUnit::Texture0).data(data, width, height);
        self
    }
}
impl<'a> BoundTexture<'a> {
    pub fn data(&mut self, data: &[u8], width: u32, height: u32) {
        unsafe {
            glTexImage2D((self.0).1 as u32, 0, PixelFormat::RGBA as u32,
                         width, height, 0, PixelFormat::RGBA as u32, DataType::UnsignedByte as u32,
                         data.as_ptr(), width * height * (size_of::<f32>() as u32));
            glTexParameteri((self.0).1 as u32, TextureFilter::Mag as u32,
                            TextureFilterType::Nearest as u32);
            glTexParameteri((self.0).1 as u32, TextureFilter::Min as u32,
                            TextureFilterType::Nearest as u32);
        }
    }
}
impl VertexShader {
    pub fn new(shader: &str) -> VertexShader {
        let shader_id = unsafe {
            compileShader(shader.as_ptr(), shader.len() as u32, ShaderType::Vertex as u32)
        };
        VertexShader(shader_id)
    }
}
impl FragmentShader {
    pub fn new(shader: &str) -> FragmentShader {
        let shader_id = unsafe {
            compileShader(shader.as_ptr(), shader.len() as u32, ShaderType::Fragment as u32)
        };
        FragmentShader(shader_id)
    }
}
impl ShaderProgram {
    pub fn new(vertex_shader: &VertexShader, fragment_shader: &FragmentShader) -> ShaderProgram {
        unsafe { ShaderProgram(linkShaderProgram(vertex_shader.0, fragment_shader.0)) }
    }
    pub fn attribute(&self, name: &str) -> Attribute {
        unsafe { Attribute(glGetAttribLocation(self.0, name.as_ptr(), name.len() as u32)) }
    }
    pub fn vertex_array(&self, name: &str) -> VertexArray {
        VertexArray(self.attribute(name))
    }
    pub fn uniform(&self, name: &str) -> Uniform {
        unsafe { Uniform(glGetUniformLocation(self.0, name.as_ptr(), name.len() as u32)) }
    }
    pub fn enable(&self) {
        unsafe { glUseProgram(self.0) }
    }
}
impl VertexArray {
    pub fn enable(&self) -> &Self {
        unsafe { glEnableVertexAttribArray((self.0).0) }
        self
    }
    pub fn pointer(&self, attribute_size: u32, attribute_type: DataType,
               normalize: Bool, stride: u32, offset: u32) {
        unsafe {
            glVertexAttribPointer((self.0).0, attribute_size, attribute_type as u32, 
                                  normalize as u32, stride, offset)
        }
    }
}
impl Uniform {
    pub fn float_1(&self, x: f32) {
        unsafe { glUniform1f(self.0, x) }
    }
    pub fn float_4(&self, x: f32, y: f32, z: f32, w: f32) {
        unsafe { glUniform4fv(self.0, x, y, z, w) }
    }
    pub fn int_1(&self, x: i32) {
        unsafe { glUniform1i(self.0, x) }
    }
}
pub fn enable(capability: Capability) {
    unsafe { glEnable(capability as u32) }
}
pub fn depth_func(comparison: Comparison) {
    unsafe { glDepthFunc(comparison as u32) }
}
pub fn clear_color(r: f32, g: f32, b: f32, a: f32) {
    unsafe { glClearColor(r, g, b, a) }
}
pub fn draw_arrays(array_type: ArrayType, offset: u32, count: u32) {
    unsafe { glDrawArrays(array_type as u32, offset, count) }
}
pub fn blend_func(src: BlendFactor, dst: BlendFactor) {
    unsafe { glBlendFunc(src as u32, dst as u32) }
}
pub fn clear(clear_bit: ClearBit) {
    unsafe { glClear(clear_bit.0) }
}
