fn main() {
    let v = vec![1, 2, 3];
    println!("{:?}", v);

    // let v2 = v;
    let v2 = &v;
    println!("{:?}", v2);
    println!("{:?}", v);

    // v2[0] = 1;
}
