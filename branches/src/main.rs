fn main() {
    test_for();
}

fn test_for() {
    let a = [10, 20, 30, 40, 50];
    
    for element in a {
        println!("the value is: {element}");
    }
    
    for number in (1..4).rev() {
        println!("{number}");
    }
    println!("LIFTOFF!!!");
}

fn test_while() {
    let mut number = 3;
    
    while number != 0 {
        println!("{number}!");
        
        number -= 1;
    }
    
    println!("LITOFF!!!");
}

fn test_loop() {
    let mut count = 0;
    'counting_up: loop {
        let mut remaining = 10;
        
        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up
            }
            remaining -= 1;
        }
        
        count += 1;
    }    
    println!("End count = {count}");
}

fn test_if() {
    let number = 6;
    
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else {
        println!("number is not divisible by 4 or 3");
    }
}

