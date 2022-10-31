fn main() {
    let c = celsius_to_fahrenheit(25.0);
    let f = fahrenheit_to_celsius(18.0);
    println!("{}", c);
    println!("{}", f);
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    c * 1.8 + 32.0
}

fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) / 1.8
}