use crate::sqrt::int_sqrt;

pub fn gen_primes(n: usize) -> impl Iterator<Item = usize> {
    if n < 2 {
        Vec::new()
    } else {
        let max = int_sqrt(n);
        let mut is_prime = vec![true; n-1];

        for i in 2..max {
            let mut it = is_prime[(i - 2)..].iter_mut().step_by(i);

            if let Some(true) = it.next() {
                for x in it {
                    *x = false;
                }
            }
        }
        is_prime
    }
    .into_iter()
    .enumerate()
    .filter_map(|(n, b)| if b { Some(n + 2) } else { None })
}
