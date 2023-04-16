pub fn call(t: f64, s: f64, k: f64, r: f64, sigma: f64, q: f64, n: usize) -> f64 {
    let delta_t = t / (n as f64);
    let up = (sigma * delta_t.sqrt()).exp();
    let down = (-sigma * delta_t.sqrt()).exp();
    let prob = (((r - q) * delta_t).exp() - down) / (up - down);

    let mut p: Vec<f64> = vec![0.0; n + 1];
    for i in 0..=n {
        // we use i to represent number of DOWNTICKS (so i=0 means top = highest price)
        let val = s * up.powf((n - i) as f64) * down.powf(i as f64) - k;
        p[i] = if val < 0.0 { 0.0 } else { val };
    }

    for j in (0..n).rev() {
        // in slice j
        for i in 0..=j {
            // every node in slice j consists of j ticks
            // top node has i=0 and is purely upticks,
            // so node i of slice j has i downticks, and j - i upticks (total ticks = i + j - i = j)
            // we use i to represent number of DOWNTICKS (so i=0 means top = highest price)
            let binomial_val = (-r * delta_t).exp() * (prob * p[i] + (1.0 - prob) * p[i + 1]);
            let exercise_val = k - s * up.powf((j - i) as f64) * down.powf(i as f64);
            p[i] = f64::max(binomial_val, exercise_val);
        }
    }

    p[0]
}

pub fn binomial_call_price(
    S: f64,
    K: f64,
    ty: f64,
    r: f64,
    q: f64,
    sigma: f64,
    n: usize,
) -> Option<f64> {
    let dt = ty / (n as f64);
    if dt >= f64::powf(sigma / (r - q), 2.0) {
        eprintln!("time steps are too large, increase n");
        return None;
    }

    if ty == 0.0 {
        return Some(f64::max((S - K) as f64, 0.0));
    }

    // CRR model
    let up = f64::exp(sigma * f64::sqrt(dt));
    let down = f64::exp(-sigma * f64::sqrt(dt)); // 1/up;
    let prob = (f64::exp((r - q) * dt) - down) / (up - down);

    let mut s_prices_right: Vec<f64> = vec![0.0; n + 1];
    // maybe actually only store this, s_prices is not really needed
    let mut opt_prices_right: Vec<f64> = vec![0.0; s_prices_right.len()];
    for i in 0..s_prices_right.len() {
        s_prices_right[i] = S * up.powf((n - i) as f64) * down.powf(i as f64);
        opt_prices_right[i] = f64::max(s_prices_right[i] - K, 0.0);
    }

    let mut s_prices_left: Vec<f64> = vec![0.0; s_prices_right.len() - 1];
    // maybe actually only store this, s_prices is not really needed
    let mut opt_prices_left: Vec<f64> = vec![0.0; s_prices_right.len() - 1];
    for j in (0..n).rev() {
        for k in 0..=j {
            s_prices_left[k] = S * up.powf((j - k) as f64) * down.powf(k as f64);
            // s_prices_left[k] = f64::exp(-r * dt) * (prob * s_prices_right[k] + (1.0 - prob) * s_prices_right[k + 1]);
            opt_prices_left[k] = f64::exp(-r * dt)
                * (prob * opt_prices_right[k] + (1.0 - prob) * opt_prices_right[k + 1]);
            let exercise_val = f64::max(K - s_prices_left[k], 0.0);
            let opt_price = f64::max(exercise_val, opt_prices_left[k]); // check exercise
            opt_prices_left[k] = opt_price;
        }
        std::mem::swap(&mut s_prices_left, &mut s_prices_right);
        std::mem::swap(&mut opt_prices_left, &mut opt_prices_right);
    }

    Some(opt_prices_right[0])
}
