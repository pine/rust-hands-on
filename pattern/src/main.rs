fn main() {
    for i in 0..5 {
        let x = match i {
            1 => "one",
            2 => "two",
            _ => "etc"
        };
        println!("{}: {}", i, x);
    }
}
