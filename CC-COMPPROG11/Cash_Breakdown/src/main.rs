use std::io;

fn main() {
    let mut amount = String::new();

    println!("Enter the amount of money: ");

    io::stdin()
        .read_line(&mut amount)
        .expect("Failed to read input");

    let mut amount: i64 = amount.trim().parse().expect("Unable to convert to i64");

    let mut total: i64 = amount / 1000;
    println!("1000 PHP = {total}");

    amount = amount - (total * 1000);

    total = amount / 500;
    println!("500 PHP = {total}");

    amount = amount - (total * 500);
    
    total = amount / 200;
    println!("200 PHP = {total}");

    amount = amount - (total * 200);
    
    total = amount / 100;
    println!("100 PHP = {total}");

    amount = amount - (total * 100);

    total = amount / 50;
    println!("50 PHP = {total}");

    amount = amount - (total * 50);

    total = amount / 20;
    println!("20 PHP = {total}");

    amount = amount - (total * 20);

    total = amount / 10;
    println!("10 PHP = {total}");

    amount = amount - (total * 10);

    total = amount / 5;
    println!("5 PHP = {total}");

    amount = amount - (total * 5);

    println!("1 PHP = {amount}");
}
