use std::io;

fn fibb(n: i64) -> i64 {
    if n == 0 || n == 1 {
        return n;
    }
    return fibb(n - 1) + fibb(n - 2);
}

fn main() {
    let mut num = String::new();

    println!("Enter the number of terms: ");

    io::stdin()
        .read_line(&mut num)
        .expect("Error reading input");

    let num: i64 = num.trim().parse().expect("Error converting input to i64");

    println!("{}", fibb(num));
}
