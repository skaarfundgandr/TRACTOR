use std::io::{self, Write};

fn triangle_side_len_calculation(a: f64, b: f64) -> f64 {
    return (a.powf(2.0) + b.powf(2.0)).sqrt();
}

fn area_of_triangle(a: f64, b: f64) -> f64 {
    return a * b / 2.0;
}

fn perimeter_of_triangle(a: f64, b: f64, c: f64) -> f64 {
    return a + b + c;
}

fn main() {
    let mut side_a: String = String::new();
    let mut side_b: String = String::new();

    print!("Enter side A: ");
    io::stdout()
        .flush()
        .expect("Error displaying prompt");

    io::stdin()
        .read_line(&mut side_a)
        .expect("Unable to read input");

    print!("Enter side B: ");
    io::stdout()
        .flush()
        .expect("Error displaying prompt");

    io::stdin()
        .read_line(&mut side_b)
        .expect("Unable to read input");

    let side_a: f64 = side_a.trim().parse().expect("Unable to convert to f64");
    let side_b: f64 = side_b.trim().parse().expect("Unable to convert to f64");

    let side_c: f64 = triangle_side_len_calculation(side_a, side_b);

    let area: f64 = area_of_triangle(side_a, side_b);
    let perimeter: f64 = perimeter_of_triangle(side_a, side_b, side_c);

    println!("Area of the triangle is: {area:.2}\nPerimeter of the triangle is: {perimeter:.2}\n")
}