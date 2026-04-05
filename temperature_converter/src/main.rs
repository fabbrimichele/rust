fn main() {
    let c = 20.0;
    let f = 68.0;

    println!("{c} C = {} F", c_to_f(c));
    println!("{f} F = {} C", f_to_c(f));
}

fn c_to_f(c: f32) -> f32 {
    (c * 9.0 / 5.0) + 32.0
}

fn f_to_c(f: f32) -> f32 {
    (f - 32.0) * 5.0 / 9.0
}
