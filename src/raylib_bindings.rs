pub mod raylib {
    use ::std::os::raw::{c_int, c_char};

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct Vector2 {
        pub x: f32,
        pub y: f32,
    }

    impl Vector2 {
        pub fn new(x: f32, y: f32) -> Self {
            Self { x, y }
        }
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct Color {
        pub r: u8,
        pub g: u8,
        pub b: u8,
        pub a: u8,
    }

    impl Color {
        pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
            Self { r, g, b, a }
        }
    }

    #[link(name = "raylib", kind = "static")]
    #[link(name = "opengl32")]
    #[link(name = "gdi32")]
    #[link(name = "winmm")]
    #[link(name = "user32")]
    #[link(name = "shell32")]
    unsafe extern "C" {
        pub fn InitWindow(width: c_int, height: c_int, title: *const c_char);
        pub fn WindowShouldClose() -> bool;
        pub fn CloseWindow();

        pub fn BeginDrawing();
        pub fn EndDrawing();
        pub fn ClearBackground(color: Color);

        pub fn DrawText(text: *const c_char, posX: c_int, posY: c_int, fontSize: c_int, color: Color);
        pub fn DrawLineV(startPos: Vector2, endPos: Vector2, color: Color);
    }
}
