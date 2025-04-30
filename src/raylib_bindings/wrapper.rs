use super::bindings;
use std::{
    ffi::{CString, c_char},
    ptr
};

pub fn init_window(width: i32, height: i32, title: &str) {
    let c_title = CString::new(title).unwrap();
    unsafe {
        bindings::InitWindow(width, height, c_title.as_ptr())
    }
}

pub fn window_should_close() -> bool {
    unsafe { bindings::WindowShouldClose() }
}

pub fn close_window() {
    unsafe { bindings::CloseWindow() }
}

pub fn begin_drawing() {
    unsafe { bindings::BeginDrawing() }
}

pub fn end_drawing() {
    unsafe { bindings::EndDrawing() }
}

pub fn clear_background(color: &Color) {
    unsafe { bindings::ClearBackground(color.raw()) }
}

pub fn draw_line_v(start_pos: &Vector2, end_pos: &Vector2, color: &Color) {
    unsafe { bindings::DrawLineV(start_pos.raw(), end_pos.raw(), color.raw()) }
}

pub fn draw_rectangle_v(position: &Vector2, size: &Vector2, color: &Color) {
    unsafe { bindings::DrawRectangleV(position.raw(), size.raw(), color.raw()) }
}

pub fn load_texture_from_image(image: &Image) -> Texture {
    unsafe {
        Texture(bindings::LoadTextureFromImage(image.raw()))
    }
}

pub fn begin_shader_mode(shader: &Shader) {
    unsafe { bindings::BeginShaderMode(shader.raw()) }
}

pub fn end_shader_mode() {
    unsafe { bindings::EndShaderMode() }
}

pub fn load_shader(vs_file_name: Option<&str>, fs_file_name: Option<&str>) -> Shader {
    let c_vs_file_name: *const c_char = match vs_file_name {
        Some(name) => CString::new(name).unwrap().as_ptr(),
        None => ptr::null()
    };

    let c_fs_file_name: *const c_char = match fs_file_name {
        Some(name) => CString::new(name).unwrap().as_ptr(),
        None => ptr::null()
    };

    unsafe {
        Shader(bindings::LoadShader(c_vs_file_name, c_fs_file_name))
    }
}

pub fn set_shader_value_texture(shader: &Shader, loc_index: i32, texture: &Texture) {
    unsafe {
        bindings::SetShaderValueTexture(shader.raw(), loc_index, texture.raw())
    }
}

pub fn get_shader_location(shader: Shader, uniform_name: &str) -> i32 {
    let c_uniform_name = CString::new(uniform_name).unwrap();
    unsafe {
        bindings::GetShaderLocation(shader.raw(), c_uniform_name.as_ptr())
    }
}

pub struct Image(bindings::Image);

impl Image {
    pub fn raw(&self) -> bindings::Image {
        self.0
    }
}

pub struct Texture(bindings::Texture);

impl Texture {
    pub fn raw(&self) -> bindings::Texture {
        self.0
    }
}

pub struct Shader(bindings::Shader);

impl Shader {
    pub fn raw(&self) -> bindings::Shader {
        self.0
    }
}

pub struct Vector2(bindings::Vector2);

impl Vector2 {
    pub fn raw(&self) -> bindings::Vector2 {
        self.0
    }
}

pub struct Color(bindings::Color);

impl Color {
    pub fn raw(&self) -> bindings::Color {
        self.0
    }
}
