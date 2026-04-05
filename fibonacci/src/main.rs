fn main() {
    let n = 6;
    println!("Fibonacci({n}) = {}", fibonacci(n)); 
}

/*
    F(0) = 0
    F(1) = 1
    F(2) = (0 + 1) = 1
    F(3) = (1 + 1) = 2
    F(4) = (1 + 2) = 3
    F(5) = (2 + 3) = 5
*/
fn fibonacci(n: u32) -> u32 {
    if n == 0 {
        return 0;
    }

    let mut fib_n_1 = 0;    // F(n - 1)
    let mut fib_n = 1;      // F(n)      
    
    for _i in 1..n {        
        let new = fib_n + fib_n_1; // F(n + 1)
        fib_n_1 = fib_n;
        fib_n = new;
    }
    
    fib_n
}
