mod raylib_bindings;
use raylib_bindings::raylib;

use std::ffi::CString;

fn main() {
    unsafe {
        raylib::InitWindow(
            800,
            800,
            CString::new("Tocket Panks").unwrap().as_ptr()
        );

        while !raylib::WindowShouldClose() {
            raylib::BeginDrawing();

            raylib::ClearBackground(raylib::Color { r: 255, g: 255, b: 255, a: 255 });
            raylib::DrawText(
                CString::new("Congrats! You created your first window!").unwrap().as_ptr(),
                190,
                200,
                20,
                raylib::Color { r: 150, g: 150, b: 150, a: 255 }
            );

            raylib::EndDrawing();
        }

        raylib::CloseWindow();
    }
}
