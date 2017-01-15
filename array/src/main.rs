fn main() {
    let a = [1, 2, 3];
    // let a: [i32; 3] = [1, 2, 3];

    let b = &a[0..2];

    println!("a len = {}", a.len());
    println!("b len = {}", b.len());

    for x in a.iter() {
        print!("{} ", x);
    }
    println!("");

    for x in b.iter() {
        print!("{} ", x);
    }
    println!("");
}
