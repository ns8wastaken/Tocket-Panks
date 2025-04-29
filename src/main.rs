mod utils;
mod raylib_bindings;
mod terrain;

use utils::StaticCString;
use raylib_bindings::raylib;

fn main() {
    unsafe {
        let title = StaticCString::new("Tocket Panks");
        raylib::InitWindow(
            800,
            800,
            title.ptr
        );

        let terrain = terrain::generate_terrain(800, 200.0);

        while !raylib::WindowShouldClose() {
            raylib::BeginDrawing();
            raylib::ClearBackground(raylib::Color::new(100, 100, 100, 255));

            for (x, y) in terrain.iter().enumerate() {
                raylib::DrawLineV(
                    raylib::Vector2::new(x as f32, 800.0),
                    raylib::Vector2::new(x as f32, 800.0 - *y as f32),
                    raylib::Color::new(0, 255, 0, 255)
                );
            }

            raylib::EndDrawing();
        }

        raylib::CloseWindow();
    }
}
