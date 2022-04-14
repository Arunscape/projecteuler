fn main() {
    let x = (20..).find(|x| (1..=20).all(|y| x % y == 0)).unwrap();
    println!("{}", x);
}
