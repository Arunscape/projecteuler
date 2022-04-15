pub struct Triangular {
    n: usize,
    sum: usize,
}

impl Triangular {
    pub fn new() -> Self {
        Self { n: 0, sum: 0 }
    }
}

impl Iterator for Triangular {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.n += 1;
        self.sum += self.n;
        Some(self.sum)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let mut t = Triangular::new();

        let x = t.next().unwrap();
        assert_eq!(x, 1);
        assert_eq!(1, t.n);
        assert_eq!(1, t.sum);

        let x = t.next().unwrap();
        assert_eq!(x, 3);
        assert_eq!(2, t.n);
        assert_eq!(3, t.sum);

        let x = t.next().unwrap();
        assert_eq!(x, 6);
        assert_eq!(3, t.n);
        assert_eq!(6, t.sum);

        let x = t.next().unwrap();
        assert_eq!(x, 10);
        assert_eq!(4, t.n);
        assert_eq!(10, t.sum);

        let x = t.next().unwrap();
        assert_eq!(x, 15);
        assert_eq!(5, t.n);
        assert_eq!(15, t.sum);

        let x = t.next().unwrap();
        assert_eq!(x, 21);
        assert_eq!(6, t.n);
        assert_eq!(21, t.sum);

        let x = t.next().unwrap();
        assert_eq!(x, 28);
        assert_eq!(7, t.n);
        assert_eq!(28, t.sum);
    }
}
