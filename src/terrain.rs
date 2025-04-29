use noise::{NoiseFn, Perlin};
use rand::Rng;

pub fn generate_terrain(width: usize, height_min: f64) -> Vec<f64> {
    let mut rng = rand::rng();
    let perlin = Perlin::new(rng.random());
    let mut terrain = Vec::with_capacity(width);

    let num_octaves = 4;
    let persistence = 0.5;

    for i in 0..width {
        let t = i as f64 / (width - 1) as f64;
        // let mountain = (1.0 - (2.0 * t - 1.0).abs()).powf(2.0);
        // let mountain = 1.0 - (2.0 * t - 1.0).powf(2.0);
        let mountain = 1.0 - (3.0 - 4.0 * (t - 0.5).abs()) * (2.0 * (t - 0.5)).powi(2);

        let x = t * width as f64;

        let mut noise = 0.0;
        let mut freq = 0.005;
        let mut amp = 1.0;
        let mut max_amp = 0.0;

        for _ in 0..num_octaves {
            noise += perlin.get([x as f64 * freq]) * amp;
            max_amp += amp;
            freq *= 2.0;
            amp *= persistence;
        }

        noise /= max_amp;
        noise += mountain * 8.0;

        let noise = (noise + 1.0) / 2.0;
        let y = height_min + noise * 80.0;

        terrain.push(y);
    }

    terrain
}
