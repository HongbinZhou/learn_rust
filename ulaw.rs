
// https://en.wikipedia.org/wiki/Μ-law_algorithm
fn ulaw(x: f32) -> f32 {
    assert!(x >= -1.0 && x <= 1.0);
    let mu = 255.0;
    let sign = if x > 0.0 { 1.0 } else { -1.0 };
    sign * (1.0 + mu * x * sign).ln() / (1.0 + mu).ln()
}

// https://en.wikipedia.org/wiki/A-law_algorithm
fn alaw(x: f32) -> f32 {
    const _A: f32 = 87.6;

    let sign = if x > 0.0 { 1.0 } else { -1.0 };
    let x = sign * x;

    if x < 1.0/_A {
       _A * x / (1.0 + _A.ln())
    } else if x >= 1.0/_A && x <= 1.0 {
        (1.0 + _A*x).ln() / (1.0 + _A.ln())
    } else {
        panic!("|x| must <=1 ")
    }
}

fn main() {
    let max = 10;
    for i in -max..max+1 {
        let x = i as f32 / (max as f32);
        println!("ulaw({:?}) = {:?}, alaw({}) = {}", x, ulaw(x), x, alaw(x));
    }
}
