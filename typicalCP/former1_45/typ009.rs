use proconio::input;
use std::f64::consts::FRAC_1_PI;
use std::f64::consts::PI;
const RAD_TO_DEG: f64 = FRAC_1_PI * 180.0;

fn main() {
    input! {n: usize, xy: [(i64, i64); n]}
    let mut max_rad: f64 = 0.0;
    for center in 0..n {
        let mut angles: Vec<f64> = Vec::new();
        for i in 0..n {
            if i == center {
                continue;
            }
            angles.push(((xy[i].1 - xy[center].1) as f64).atan2((xy[i].0 - xy[center].0) as f64));
        }

        // duplicate the angles with + 2*pi
        for i in 0..n - 1 {
            angles.push(angles[i] + 2.0 * PI);
        }
        // for each angle
        for i in 0..n - 1 {
            let target = angles[i] + PI;
            let pos = angles.partition_point(|&ang| ang < target);
            // candidate 1
            if pos > i {
                let diff1: f64 = angles[pos - 1] - angles[i];
                max_rad = max_rad.max(diff1);
            }
            // candidate 2
            if pos < i + n - 1 {
                let diff2: f64 = angles[pos] - angles[i];
                max_rad = max_rad.max(diff2);
            }
        }
    }
    let max_deg = max_rad * RAD_TO_DEG;
    println!("{:.12}", max_deg);
}
