use arun_euler_lib::triangular::Triangular;
fn main() {
    let x = Triangular::new()
        .find(|&x| {
            let numdivisible = (1..=x).filter(|y| x % y == 0).count();

            500 < numdivisible
        })
        .unwrap();

    println!("{}", x);
}
