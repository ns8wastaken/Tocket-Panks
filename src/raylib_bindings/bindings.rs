use ::core::ffi::{c_int, c_char, c_uint, c_float, c_uchar, c_void};

#[link(name = "raylib", kind = "static")]
#[link(name = "opengl32")]
#[link(name = "gdc_int")]
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

    // Textures
    pub fn LoadTextureFromImage(image: Image) -> Texture;

    // Shaders
    pub fn BeginShaderMode(shader: Shader);
    pub fn EndShaderMode();

    pub fn LoadShader(vsFileName: *const c_char, fsFileName: *const c_char) -> Shader;
    pub fn SetShaderValueTexture(shader: Shader, locIndex: c_int, texture: Texture);
    pub fn GetShaderLocation(shader: Shader, uniformName: *const c_char) -> c_int;
}

#[repr(C)]
#[derive(Copy, Clone)]
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Image {
    pub data: *mut c_void,         // Image raw data
    pub width: c_int,              // Image base width
    pub height: c_int,             // Image base height
    pub mipmaps: c_int,            // Mipmap levels, 1 by default
    pub format: c_int,             // Data format (PixelFormat type)
}

impl Image {
    pub fn new(
        data: *mut c_void,
        width: c_int,
        height: c_int,
        mipmaps: c_int,
        format: c_int
    ) -> Self {
        Self { data, width, height, mipmaps, format }
    }
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

impl Texture {
    pub fn new(
        id: c_uint,
        width: c_int,
        height: c_int,
        mipmaps: c_int,
        format: c_int
    ) -> Self {
        Self { id, width, height, mipmaps, format }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Shader {
    id: c_uint,                // Shader program id
    locs: *mut c_int,          // Shader locations array (RL_MAX_SHADER_LOCATIONS)
}

impl Shader {
    pub fn new(id: c_uint, locs: *mut c_int) -> Self {
        Self { id, locs }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Vector2 {
    pub x: c_float,
    pub y: c_float,
}

impl Vector2 {
    pub fn new(x: c_float, y: c_float) -> Self {
        Self { x, y }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Color {
    pub r: c_uchar,
    pub g: c_uchar,
    pub b: c_uchar,
    pub a: c_uchar,
}

impl Color {
    pub fn new(r: c_uchar, g: c_uchar, b: c_uchar, a: c_uchar) -> Self {
        Self { r, g, b, a }
    }
}
