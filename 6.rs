fn main() {
    let sum_of_squares: usize = (1..=100).map(|x| x * x).sum();
    let square_of_sum: usize = (1..=100).sum();
    let square_of_sum = square_of_sum * square_of_sum;

    dbg!(&sum_of_squares);
    dbg!(&square_of_sum);

    println!("{}", square_of_sum - sum_of_squares);
}
