use primes::{Sieve, PrimeSet};


fn main() {
    let n: u64 = 600851475143;
    let mut pset = Sieve::new();
    let maxprime: u64 = pset.iter().take_while(move |x| x * x < n).filter(|x| n % x == 0).max().unwrap();
    println!("{}", maxprime);
}
