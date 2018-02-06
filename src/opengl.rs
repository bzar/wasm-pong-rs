use std::mem::size_of;

extern {
    pub fn compileShader(source: *const u8, len: u32, shader_type: u32) -> u32;
    pub fn linkShaderProgram(vertexShaderId: u32, fragmentShaderId: u32) -> u32;

    pub fn glClearColor(r: f32, g: f32, b: f32, a: f32);
    pub fn glEnable(x: u32);
    pub fn glDepthFunc(x: u32);
    pub fn glBlendFunc(x: u32, y: u32);
    pub fn glClear(x: u32);
    pub fn glGetAttribLocation(program_id: u32, name_ptr: *const u8, name_len: u32) -> i32;
    pub fn glGetUniformLocation(program_id: u32, name_ptr: *const u8, name_len: u32) -> i32;
    pub fn glUniform4fv(location_id: i32, x: f32, y: f32, z: f32, w: f32);
    pub fn glUniform1i(location_id: i32, x: i32);
    pub fn glUniform1f(location_id: i32, x: f32);
    pub fn glCreateBuffer() -> u32;
    pub fn glBindBuffer(buffer_type: u32, buffer_id: u32);
    pub fn glBufferData(data_type: u32, data_ptr: *const f32, data_count: u32, draw_type: u32);
    pub fn glUseProgram(program_id: u32);
    pub fn glEnableVertexAttribArray(attribute_location: i32);
    pub fn glVertexAttribPointer(attribute_location: i32, attribute_size: u32, attribute_type: u32, normalize: u32, stride: u32, offset: u32);
    pub fn glDrawArrays(array_type: u32, offset: u32, count: u32);
    pub fn glCreateTexture() -> u32;
    pub fn glBindTexture(target: u32, texture_id: u32);
    pub fn glTexImage2D(target: u32, level: u32, internal_format: u32, width: u32, height: u32, border: u32, format: u32, image_type: u32, image_ptr: *const u8, image_len: u32);
    pub fn glTexParameteri(target: u32, name: u32, value: u32);
    pub fn glActiveTexture(target: u32);
}

pub const GL_VERTEX_SHADER: u32 = 35633;
pub const GL_FRAGMENT_SHADER: u32 = 35632;
pub const GL_ARRAY_BUFFER: u32 = 34962;
pub const GL_TRIANGLES: u32 = 4;
pub const GL_STATIC_DRAW: u32 = 35044;
pub const GL_FLOAT: u32 = 5126;
pub const GL_DEPTH_TEST: u32 = 2929;
pub const GL_LEQUAL: u32 = 515;
pub const GL_COLOR_BUFFER_BIT: u32 = 16384;
pub const GL_DEPTH_BUFFER_BIT: u32 = 256;
pub const GL_TEXTURE_2D: u32 = 3553;
pub const GL_RGBA: u32 = 6408;
pub const GL_UNSIGNED_BYTE: u32 = 5121;
pub const GL_TEXTURE_MAG_FILTER: u32 = 10240;
pub const GL_TEXTURE_MIN_FILTER: u32 = 10241;
pub const GL_NEAREST: u32 = 9728;
pub const GL_TEXTURE0: u32 = 33984;
pub const GL_BLEND: u32 = 3042;
pub const GL_SRC_ALPHA: u32 = 770;
pub const GL_ONE_MINUS_SRC_ALPHA: u32 = 771;
pub const GL_ONE: u32 = 1;

pub fn gl_compile_shader(shader: &str, shader_type: u32) -> u32 {
    unsafe {
        compileShader(shader.as_ptr(), shader.len() as u32, shader_type)
    }
}
pub fn gl_init_texture(data: &[u8], width: u32, height: u32) -> u32 {
    unsafe {
        let id = glCreateTexture();
        glBindTexture(GL_TEXTURE_2D, id);
        glTexImage2D(GL_TEXTURE_2D, 0, GL_RGBA,
                     width, height, 0, GL_RGBA, GL_UNSIGNED_BYTE,
                     data.as_ptr(), width * height * (size_of::<f32>() as u32));
        glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_NEAREST);
        glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_NEAREST);

        id
    }
}

pub fn gl_get_attrib_location(program_id: u32, name: &str) -> i32 {
    unsafe {
        glGetAttribLocation(program_id, name.as_ptr(), name.len() as u32)
    }
}

pub fn gl_get_uniform_location(program_id: u32, name: &str) -> i32 {
    unsafe {
        glGetUniformLocation(program_id, name.as_ptr(), name.len() as u32)
    }
}

pub fn gl_buffer_data(data_type: u32, data: &[f32], draw_type: u32) {
    unsafe {
        glBufferData(data_type, data.as_ptr(), data.len() as u32, draw_type)
    }
}
pub fn gl_tex_image_2d(target: u32, level: u32, internal_format: u32, width: u32, height: u32, border: u32, format: u32, image_type: u32, image: &[u8]) {
    unsafe {
        glTexImage2D(target, level, internal_format, width, height, border, format, image_type, image.as_ptr(), (image.len() * size_of::<u8>()) as u32)
    }
}
