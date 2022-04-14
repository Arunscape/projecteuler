use arun_euler_lib::primes;

fn main() {
    let x: usize = primes::gen_primes(2_000_000).sum();

    println!("{}", x);
}
