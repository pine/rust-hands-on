fn fib(n: i32) -> i32 {
    if n == 0 {
        0
    } else if n <= 2 {
        1
    } else {
        fib(n - 1) + fib(n - 2)
    }
}

fn main() {
    for i in 0..10 {
        println!("fib({}) = {:3}", i, fib(i));
    }
}
