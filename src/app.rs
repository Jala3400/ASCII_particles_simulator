pub struct App {
    pub noise_intensity: f32,
    pub min_brightness: f32,
    pub max_brightness: f32,
}

impl App {
    pub fn new() -> Self {
        let mut p_particles = [[0.0; 20]; 10];

        for j in 0..20 {
            p_particles[9][j] = 1.0;
        }

        App {
            noise_intensity: 1.0,
            min_brightness: -1.0,
            max_brightness: 1.0,
        }
    }
}
