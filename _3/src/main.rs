use arun_euler_lib::primes;


fn main() {
    let n: usize = 600851475143;
    let maxprime = primes::gen_primes(n).filter(|x| n % x == 0).max().unwrap();
    println!("{}", maxprime);
}
