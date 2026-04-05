fn main() {
    another_function(5);
    
    let x = five();
    let y = plus_one(x);
    print_labelled_measurement(y, 'h');
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labelled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
