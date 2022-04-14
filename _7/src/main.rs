use arun_euler_lib::primes;

fn main() {
    let x = primes::gen_primes(1000000).nth(10000).unwrap();

    println!("{}", x);
}
