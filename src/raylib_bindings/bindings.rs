use ::core::ffi::{c_int, c_char, c_uint, c_float, c_uchar, c_void};

#[link(name = "raylib", kind = "static")]
#[link(name = "opengl32")]
#[link(name = "gdi32")]
#[link(name = "winmm")]
#[link(name = "user32")]
#[link(name = "shell32")]
unsafe extern "C" {
    // Main stuff
    pub fn InitWindow(width: c_int, height: c_int, title: *const c_char);
    pub fn WindowShouldClose() -> bool;
    pub fn CloseWindow();

    pub fn BeginDrawing();
    pub fn EndDrawing();
    pub fn ClearBackground(color: Color);

    // Drawing
    pub fn DrawLineV(startPos: Vector2, endPos: Vector2, color: Color);
    pub fn DrawRectangleV(position: Vector2, size: Vector2, color: Color);

    // Images
    pub fn GenImageColor(width: c_int, height: c_int, color: Color) -> Image;

    // Textures
    pub fn LoadTextureFromImage(image: Image) -> Texture;

    // Shaders
    pub fn BeginShaderMode(shader: Shader);
    pub fn EndShaderMode();

    pub fn LoadShader(vsFileName: *const c_char, fsFileName: *const c_char) -> Shader;
    pub fn SetShaderValue(shader: Shader, locIndex: c_int, value: *const c_void, uniformType: c_int);
    pub fn SetShaderValueTexture(shader: Shader, locIndex: c_int, texture: Texture);
    pub fn GetShaderLocation(shader: Shader, uniformName: *const c_char) -> c_int;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Image {
    pub data: *mut c_void,         // Image raw data
    pub width: c_int,              // Image base width
    pub height: c_int,             // Image base height
    pub mipmaps: c_int,            // Mipmap levels, 1 by default
    pub format: c_int,             // Data format (PixelFormat type)
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Texture {
    pub id: c_uint,                // OpenGL texture id
    pub width: c_int,              // Texture base width
    pub height: c_int,             // Texture base height
    pub mipmaps: c_int,            // Mipmap levels, 1 by default
    pub format: c_int,             // Data format (PixelFormat type)
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Shader {
    pub id: c_uint,                // Shader program id
    pub locs: *mut c_int,          // Shader locations array (RL_MAX_SHADER_LOCATIONS)
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Vector2 {
    pub x: c_float,
    pub y: c_float,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Color {
    pub r: c_uchar,
    pub g: c_uchar,
    pub b: c_uchar,
    pub a: c_uchar,
}
