use proconio::input;
const LIMIT: i64 = 1_000_000;
fn main() {
    input! {l: i64, r: i64, d: i64, u: i64}
    let mut ans: i64 = 0;
    if l <= 0 && 0 <= r && d <= 0 && 0 <= u {
        ans += 1;
    }
    for radius in (2..=LIMIT).step_by(2) {
        // Horizontal edges (full width: [-radius, radius], half-open [-radius, radius + 1))
        // Top edge: y = radius
        if d <= radius && radius <= u {
            ans += clamp(radius + 1, l, r + 1) - clamp(-radius, l, r + 1);
        }
        // Bottom edge: y = -radius
        if d <= -radius && -radius <= u {
            ans += clamp(radius + 1, l, r + 1) - clamp(-radius, l, r + 1);
        }

        // Vertical edges (strictly interior: [-radius + 1, radius - 1], half-open [-radius + 1, radius))
        // Right edge: x = radius
        if l <= radius && radius <= r {
            ans += clamp(radius, d, u + 1) - clamp(-radius + 1, d, u + 1);
        }
        // Left edge: x = -radius
        if l <= -radius && -radius <= r {
            ans += clamp(radius, d, u + 1) - clamp(-radius + 1, d, u + 1);
        }
    }
    println!("{}", ans);
}

fn clamp(x: i64, minimum: i64, maximum: i64) -> i64 {
    if x < minimum {
        return minimum;
    }
    if x > maximum {
        return maximum;
    }
    x
}
