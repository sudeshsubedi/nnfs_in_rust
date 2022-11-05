use rand::Rng;

pub fn spiral(samples: usize, classes: u8) -> (Vec<(f64, f64)>, Vec<u8>) {
    let mut X: Vec<(f64, f64)> = vec![(0.0, 0.0); samples * classes as usize];
    let mut y: Vec<u8> = vec![0; samples * classes as usize];

    let mut rng = rand::thread_rng();

    for class_num in 0..classes {
        let start = class_num as usize * samples;
        let end = (class_num as usize + 1) * samples;

        for i in start..end {
            let r = (i - start) as f64 / samples as f64;
            let theta = class_num as f64 * 4.0 + r * (4.0 * (class_num as f64 + 1.0)) + rng.gen::<f64>() * 0.2;
            X[i] = (r * theta.sin(), r * theta.cos());
            y[i] = class_num; 
        }
    }

    (X, y)
}