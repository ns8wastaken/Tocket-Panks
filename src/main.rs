mod raylib_bindings;
mod terrain;

use raylib_bindings as raylib;

fn main() {
    raylib::init_window(800, 800, "Tocket Panks");

    // let mut terrain = terrain::generate_terrain(800, 200.0);
    let mut terrain: Vec<u8> = (0..800)
        .map(|x| {
            let fx = x as f32 / 800.0;
            (fx.sin() * 127.0 + 128.0) as u8  // example: sine wave height map
        })
        .collect();

    let terrain_img = raylib::Image::new(
        &mut terrain,
        800,
        1,
        1,
        raylib::PixelFormat::UncompressedGrayscale
    );
    let terrain_tex = raylib::load_texture_from_image(&terrain_img);

    let shader = raylib::Shader::load(
        None,
        Some("src/shaders/terrain.glsl")
    );

    shader.set_texture(
        shader.get_location("terrainTex"),
        &terrain_tex
    );
    let screen_height = 800.0f32;
    shader.set_value(
        shader.get_location("screenHeight"),
        &screen_height as *const f32 as *const _,
        raylib::ShaderUniformDataType::ShaderUniformFloat
    );

    raylib::begin_shader_mode(&shader);
    raylib::draw_rectangle_v(
        &raylib::Vector2::new(0.0, 0.0),
        &raylib::Vector2::new(800.0, 800.0),
        &raylib::Color::new(255, 255, 255, 255)
    );
    raylib::end_shader_mode();

    while !raylib::window_should_close() {
        raylib::begin_drawing();
        raylib::clear_background(&raylib::Color::new(100, 100, 100, 255));

        raylib::begin_shader_mode(&shader);
        raylib::draw_rectangle_v(
            &raylib::Vector2::new(0.0, 0.0),
            &raylib::Vector2::new(800.0, 800.0),
            &raylib::Color::new(255, 255, 255, 255)
        );
        raylib::end_shader_mode();

        raylib::end_drawing();
    }

    raylib::close_window();
}
