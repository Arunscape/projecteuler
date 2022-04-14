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
