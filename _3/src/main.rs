use arun_euler_lib::{primes, sqrt::int_sqrt};

fn main() {
    let n: usize = 600851475143;
    let max = int_sqrt(n);
    let maxprime = primes::gen_primes(max)
        .filter(|x| n % x == 0)
        .max()
        .unwrap();
    println!("{}", maxprime);
}
