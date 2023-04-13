use rayon::prelude::*;
use std::f64;

pub fn call(t: f64, s: f64, k: f64, r: f64, sigma: f64, q: f64, n: usize) -> f64 {
    let delta_t = t / (n as f64);
    let up = (sigma * delta_t.sqrt()).exp();
    let p0 = (up * (-q * delta_t).exp() - (-r * delta_t).exp()) / (up * up - 1.0);
    let p1 = (-r * delta_t).exp() - p0;

    let mut p: Vec<f64> = vec![0.0; n + 1];
    for i in 0..=n {
        let val = k - s * up.powf(2.0 * (i as f64) - (n as f64));
        p[i] = if val < 0.0 { 0.0 } else { val };
    }

    for j in (0..n).rev() {
        for i in 0..=j {
            let binomial_val = p0 * p[i+1] + p1 * p[i];
            let exercise_val = k - s * up.powf(2.0 * (i as f64) - (j as f64));
            p[i] = f64::max(binomial_val, exercise_val);
        }
    }

    p[0]
}

