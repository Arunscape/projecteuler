struct Fibonacci {
    curr: u128,
    next: u128,
}

impl Fibonacci {
    pub fn new() -> Self {
        Self { curr: 0, next: 1 }
    }
}

impl Iterator for Fibonacci {
    type Item = u128;
    fn next(&mut self) -> Option<u128> {
        let next = self.curr + self.next;
        self.curr = self.next;
        self.next = next;
        Some(self.curr)
    }
}

fn main() {
    let ans: u128 = Fibonacci::new()
        .take_while(|&n| n <= 4000000)
        .filter(|&n| n % 2 == 0)
        .sum();
    println!("{}", ans);
}
