fn is_palindrome_str(s: &str) -> bool {
    if s.len() < 2 {
        return true;
    }

    if s.chars().next() != s.chars().next_back() {
        return false;
    }

    let chop = &s[1..s.len() - 1];
    is_palindrome_str(chop)
}

fn is_palindrome_num(n: usize) -> bool {
    let s = format!("{}", n);
    is_palindrome_str(&s)
}

fn main() {
    let x: usize = (100..=999)
        .flat_map(|x| (100..=999).map(move |y| (x, y)))
        .filter_map(|(x, y)| {
            if is_palindrome_num(x * y) {
                Some(x * y)
            } else {
                None
            }
        })
        .max().unwrap();

    println!("{}", x);
}
