fn fib(n: i32) -> i32 {
    if n < 2 {
        n
    } else {
        fib(n - 1) + fib(n - 2)
    }
}

fn main() {
    for i in 0..11 {
        println!("fib({:2}) = {:3}", i, fib(i));
    }
}
