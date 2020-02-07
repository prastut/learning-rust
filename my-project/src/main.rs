fn main() {
    let v = vec![1, 2, 3, 4, 5];

    let x = &v[0];

    v.push(8);

    println!("{}", x);
}
