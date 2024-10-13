use text_io::read;

fn main() {
    println!("Howeth-many digit of the fib would you like? :");
    let input: i32 = read!();
    let output: u128 = iterative_fibonacci(input);
    println!("{output}")
}

fn iterative_fibonacci(n:i32)-> u128{
    let mut x:u128= 1;
    let mut x1: u128 = x;
    for _i in 1..n {
        (x,x1) = (x1, x1+x);
    }
    x1
}

