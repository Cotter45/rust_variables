use std::io;

fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);
    
    // addition
    let sum = 5 + 10;
    println!("The value of sum is: {}", sum);
    
    // subtraction
    let difference = 95.5 - 4.3;
    println!("The value of difference is: {}", difference);
    
    // multiplication
    let product = 4 * 30;
    println!("The value of product is: {}", product);
    
    // division
    let quotient = 56.7 / 32.2;
    let floored = 6/ 3; // Results in 0
    println!("The value of quotient is: {}, The value of floored is: {}", quotient, floored);
    
    // remainder
    let remainder = 43 % 5;
    println!("The value of remainder is: {}", remainder);
    
    // booleans
    let t = true;
    let f: bool = false; // with explicit type annotation
    println!("The value of t is: {}, The value of f is: {}", t, f);

    // characters, specified with single quotes
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("The value of c is: {}, The value of z is: {}, The value of heart_eyed_cat is: {}", c, z, heart_eyed_cat);

    // COMPOUND TYPES

    // tuple
    let tup = (500, 6.4, 1);

    // destructuring or 'pattern matching'
    let (_x, y, _z) = tup;

    println!("The value of y is: {}", y);

    let five_hundred = tup.0;

    let six_point_four = tup.1;

    let one = tup.2;

    println!("The value of five_hundred is: {}", five_hundred);
    println!("The value of six_point_four is: {}", six_point_four);
    println!("The value of one is: {}", one);

    // arrays - fixed length, similar types
    let months: [&str; 12] = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];

    println!("The value of the first month is: {}", months[0]);

    // user input an index and returns the value or err
    println!("Please enter an array index to view the month.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = months[index];

    println!(
        "The value of the element at index {} is: {}",
        index, element
    );

}