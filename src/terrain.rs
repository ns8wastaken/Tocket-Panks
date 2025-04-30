use rand::Rng;

fn displace(
    terrain: &mut [f64],
    left: usize,
    right: usize,
    displacement: f64,
    roughness: f64,
    rng: &mut impl rand::Rng,
) {
    if right - left <= 1 {
        return;
    }

    let mid = (left + right) / 2;
    let avg = (terrain[left] + terrain[right]) / 2.0;
    let offset = rng.random_range(-0.5..0.5) * displacement;

    terrain[mid] = avg + offset;

    displace(terrain, left, mid, displacement * roughness, roughness, rng);
    displace(terrain, mid, right, displacement * roughness, roughness, rng);
}

// Midpoint displacement
pub fn generate_terrain(width: usize, height_min: f64) -> Vec<f64> {
    let roughness = 0.5;
    let initial_displacement = width as f64 / 4.0;

    let mut terrain = vec![0.0; width];

    let mut rng = rand::rng();
    terrain[0] = rng.random_range(0.0..initial_displacement);
    terrain[width - 1] = rng.random_range(0.0..initial_displacement);

    displace(
        &mut terrain,
        0,
        width - 1,
        initial_displacement,
        roughness,
        &mut rng,
    );

    terrain
        .iter()
        .map(|x| x + height_min)
        .collect()
}
