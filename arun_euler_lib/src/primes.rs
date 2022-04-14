pub fn int_sqrt(n: usize) -> usize {
    // https://rosettacode.org/wiki/Isqrt_(integer_square_root)_of_X#Rust
    let mut q = 1;

    while q <= n {
        q <<= 2;
    }

    let mut z = n;
    let mut ans = 0;

    while 1 < q {
        q >>= 2;
        let t = z.checked_sub(ans).and_then(|diff| diff.checked_sub(q));
        ans >>= 1;

        if let Some(t) = t {
            z = t;
            ans += q;
        }
    }
    ans
}

pub fn gen_primes(n: usize) -> impl Iterator<Item = usize> {
    if n < 2 {
        Vec::new()
    } else {
        let max = int_sqrt(n);
        let mut is_prime = vec![true; max];

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
