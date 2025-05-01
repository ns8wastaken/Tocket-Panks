mod raylib_bindings;
mod terrain;

use raylib_bindings as raylib;

fn main() {
    raylib::init_window(800, 800, "Tocket Panks");

    let terrain = terrain::generate_terrain(800, 200.0);

    // let terrain_img = raylib::Image::new(
    //     terrain,
    //     800,
    //     1,
    //     1,
    //     raylib::PixelFormat::UncompressedGrayAlpha.to_i32()
    // );
    let terrain_img = raylib::gen_image_color(
        800,
        1,
        &raylib::Color::new(0, 0, 0, 255)
    );
    let terrain_tex = raylib::load_texture_from_image(&terrain_img);

    let shader = raylib::load_shader(
        None,
        Some("src/shaders/terrain.glsl")
    );

    raylib::set_shader_value_texture(
        &shader,
        raylib::get_shader_location(&shader, "terrainTex"),
        &terrain_tex
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
