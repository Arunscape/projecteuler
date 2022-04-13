struct Fibonacci {
    curr: usize,
    next: usize,
}

impl Fibonacci {
    pub fn new() -> Self {
        Self { curr: 0, next: 1 }
    }
}

impl Iterator for Fibonacci {
    type Item = usize;
    fn next(&mut self) -> Option<usize> {
        let next = self.curr + self.next;
        self.curr = self.next;
        self.next = next;
        Some(self.curr)
    }
}

fn main() {
    let ans: usize = Fibonacci::new()
        .take_while(|&n| n <= 4000000)
        .filter(|&n| n % 2 == 0)
        .sum();
    println!("{}", ans);
}
