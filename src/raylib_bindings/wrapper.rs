#![allow(dead_code)]

use super::bindings;
use std::{
    ffi::{CString, c_int, c_uint, c_uchar, c_float, c_void},
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
    unsafe { bindings::ClearBackground(*color.raw()) }
}

pub fn draw_line_v(start_pos: &Vector2, end_pos: &Vector2, color: &Color) {
    unsafe { bindings::DrawLineV(*start_pos.raw(), *end_pos.raw(), *color.raw()) }
}

pub fn draw_rectangle_v(position: &Vector2, size: &Vector2, color: &Color) {
    unsafe { bindings::DrawRectangleV(*position.raw(), *size.raw(), *color.raw()) }
}

pub fn gen_image_color(width: i32, height: i32, color: &Color) -> Image {
    unsafe {
        Image(bindings::GenImageColor(
            width  as c_int,
            height as c_int,
            *color.raw()
        ))
    }
}

pub fn load_texture_from_image(image: &Image) -> Texture {
    unsafe {
        Texture(bindings::LoadTextureFromImage(*image.raw()))
    }
}

pub fn begin_shader_mode(shader: &Shader) {
    unsafe { bindings::BeginShaderMode(*shader.raw()) }
}

pub fn end_shader_mode() {
    unsafe { bindings::EndShaderMode() }
}

pub enum PixelFormat {
    UncompressedGrayscale = 1, // 8 bit per pixel (no alpha)
    UncompressedGrayAlpha,     // 8*2 bpp (2 channels)
    UncompressedR5G6B5,        // 16 bpp
    UncompressedR8G8B8,        // 24 bpp
    UncompressedR5G5B5A1,      // 16 bpp (1 bit alpha)
    UncompressedR4G4B4A4,      // 16 bpp (4 bit alpha)
    UncompressedR8G8B8A8,      // 32 bpp
    UncompressedR32,           // 32 bpp (1 channel - float)
    UncompressedR32G32B32,     // 32*3 bpp (3 channels - float)
    UncompressedR32G32B32A32,  // 32*4 bpp (4 channels - float)
    UncompressedR16,           // 16 bpp (1 channel - half float)
    UncompressedR16G16B16,     // 16*3 bpp (3 channels - half float)
    UncompressedR16G16B16A16,  // 16*4 bpp (4 channels - half float)
    CompressedDXT1RGB,         // 4 bpp (no alpha)
    CompressedDXT1RGBA,        // 4 bpp (1 bit alpha)
    CompressedDXT3RGBA,        // 8 bpp
    CompressedDXT5RGBA,        // 8 bpp
    CompressedETC1RGB,         // 4 bpp
    CompressedETC2RGB,         // 4 bpp
    CompressedETC2EACRGBA,     // 8 bpp
    CompressedPVRTRGB,         // 4 bpp
    CompressedPVRTRGBA,        // 4 bpp
    CompressedASTC4X4RGBA,     // 8 bpp
    CompressedASTC8X8RGBA,     // 2 bpp
}

impl PixelFormat {
    pub fn from_i32(value: i32) -> Option<Self> {
        match value {
            1  => Some(Self::UncompressedGrayscale),
            2  => Some(Self::UncompressedGrayAlpha),
            3  => Some(Self::UncompressedR5G6B5),
            4  => Some(Self::UncompressedR8G8B8),
            5  => Some(Self::UncompressedR5G5B5A1),
            6  => Some(Self::UncompressedR4G4B4A4),
            7  => Some(Self::UncompressedR8G8B8A8),
            8  => Some(Self::UncompressedR32),
            9  => Some(Self::UncompressedR32G32B32),
            10 => Some(Self::UncompressedR32G32B32A32),
            11 => Some(Self::UncompressedR16),
            12 => Some(Self::UncompressedR16G16B16),
            13 => Some(Self::UncompressedR16G16B16A16),
            14 => Some(Self::CompressedDXT1RGB),
            15 => Some(Self::CompressedDXT1RGBA),
            16 => Some(Self::CompressedDXT3RGBA),
            17 => Some(Self::CompressedDXT5RGBA),
            18 => Some(Self::CompressedETC1RGB),
            19 => Some(Self::CompressedETC2RGB),
            20 => Some(Self::CompressedETC2EACRGBA),
            21 => Some(Self::CompressedPVRTRGB),
            22 => Some(Self::CompressedPVRTRGBA),
            23 => Some(Self::CompressedASTC4X4RGBA),
            24 => Some(Self::CompressedASTC8X8RGBA),
            _  => None
        }
    }

    pub fn to_i32(self) -> i32 {
        self as i32
    }
}

pub enum ShaderUniformDataType {
    ShaderUniformFloat = 0,       // Shader uniform type: float
    ShaderUniformVec2,            // Shader uniform type: vec2 (2 float)
    ShaderUniformVec3,            // Shader uniform type: vec3 (3 float)
    ShaderUniformVec4,            // Shader uniform type: vec4 (4 float)
    ShaderUniformInt,             // Shader uniform type: int
    ShaderUniformIvec2,           // Shader uniform type: ivec2 (2 int)
    ShaderUniformIvec3,           // Shader uniform type: ivec3 (3 int)
    ShaderUniformIvec4,           // Shader uniform type: ivec4 (4 int)
    ShaderUniformSampler2d,       // Shader uniform type: sampler2d
}

impl ShaderUniformDataType {
    pub fn from_i32(value: i32) -> Option<Self> {
        match value {
            0  => Some(Self::ShaderUniformFloat),
            1  => Some(Self::ShaderUniformVec2),
            2  => Some(Self::ShaderUniformVec3),
            3  => Some(Self::ShaderUniformVec4),
            4  => Some(Self::ShaderUniformInt),
            5  => Some(Self::ShaderUniformIvec2),
            6  => Some(Self::ShaderUniformIvec3),
            7  => Some(Self::ShaderUniformIvec4),
            8  => Some(Self::ShaderUniformSampler2d),
            _  => None
        }
    }

    pub fn to_i32(self) -> i32 {
        self as i32
    }
}

pub struct Image(bindings::Image);

impl Image {
    pub fn new<T>(
        data: &mut Vec<T>,
        width: i32,
        height: i32,
        mipmaps: i32,
        format: PixelFormat
    ) -> Self {
        Self {
            0: bindings::Image {
                data:    data.as_mut_ptr() as *mut c_void,
                width:   width   as c_int,
                height:  height  as c_int,
                mipmaps: mipmaps as c_int,
                format:  format  as c_int
            }
        }
    }

    pub fn raw(&self) -> &bindings::Image {
        &self.0
    }
}

pub struct Texture(bindings::Texture);

impl Texture {
    pub fn new(
        id: u32,
        width: i32,
        height: i32,
        mipmaps: i32,
        format: i32
    ) -> Self {
        Self {
            0: bindings::Texture {
                id:      id      as c_uint,
                width:   width   as c_int,
                height:  height  as c_int,
                mipmaps: mipmaps as c_int,
                format:  format  as c_int
            }
        }
    }

    pub fn raw(&self) -> &bindings::Texture {
        &self.0
    }
}

pub struct Shader(bindings::Shader);

impl Shader {
    pub fn load(vs_file_name: Option<&str>, fs_file_name: Option<&str>) -> Self {
        let c_vs_file_name = vs_file_name
            .map(|name| CString::new(name).unwrap());

        let c_fs_file_name = fs_file_name
            .map(|name| CString::new(name).unwrap());

        unsafe {
            Shader(bindings::LoadShader(
                c_vs_file_name
                    .as_ref()
                    .map_or(ptr::null(), |s| s.as_ptr()),

                c_fs_file_name
                    .as_ref()
                    .map_or(ptr::null(), |s| s.as_ptr())
            ))
        }
    }

    pub fn raw(&self) -> &bindings::Shader {
        &self.0
    }

    pub fn get_location(&self, uniform_name: &str) -> i32 {
        let c_uniform_name = CString::new(uniform_name).unwrap();
        unsafe {
            bindings::GetShaderLocation(
                self.0,
                c_uniform_name.as_ptr()
            )
        }
    }

    pub fn set_value(
        &self,
        loc_index: i32,
        value: *const c_void,
        uniform_type: ShaderUniformDataType
    ) {
        unsafe {
            bindings::SetShaderValue(
                self.0,
                loc_index as c_int,
                value,
                uniform_type.to_i32()
            )
        }
    }

    pub fn set_texture(&self, loc_index: i32, texture: &Texture) {
        unsafe {
            bindings::SetShaderValueTexture(
                self.0,
                loc_index as c_int,
                *texture.raw()
            )
        }
    }
}


pub struct Vector2(bindings::Vector2);

impl Vector2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            0: bindings::Vector2 {
                x: x as c_float,
                y: y as c_float
            }
        }
    }

    pub fn raw(&self) -> &bindings::Vector2 {
        &self.0
    }
}

pub struct Color(bindings::Color);

impl Color {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self {
            0: bindings::Color {
                r: r as c_uchar,
                g: g as c_uchar,
                b: b as c_uchar,
                a: a as c_uchar
            }
        }
    }

    pub fn raw(&self) -> &bindings::Color {
        &self.0
    }
}
