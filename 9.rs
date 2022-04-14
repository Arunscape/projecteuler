fn main() {
    let (a, b, c) = (1..1000)
        .enumerate()
        .flat_map(|(i, x)| {
            (1..1000)
                .enumerate()
                .skip(i + 1)
                .flat_map(move |(j, y)| (1..1000).skip(j + 1).map(move |z| (x, y, z)))
        })
        .filter(|(a, b, c)| a + b + c == 1000)
        .find(|(a, b, c)| a * a + b * b == c * c)
        .unwrap();

    dbg!(&a, &b, &c);

    println!("{}", a * b * c);
}
