use arun_euler_lib::triangular::Triangular;

fn num_divisors(n: usize) -> usize {
    let mut ret = 1;

    let mut n = n;

    let mut factor = 2;
    while factor <= n {
        let mut tmp = 1;
        while n % factor == 0 {
            n /= factor;
            tmp += 1;
        }
        ret *= tmp;
        factor += 1;
    }

    ret
}

fn main() {
    let x = Triangular::new()
        .find(|&x| {
            let numdivisible = num_divisors(x);

            500 < numdivisible
        })
        .unwrap();

    println!("{}", x);
}
