use std::io;
fn main() {
    println!("Calculator!");
    let mut nb1 = String::new();
    let mut nb2 = String::new();
    let mut operator = String::new();
    println!("Enter the first number: ");
    io::stdin().read_line(&mut nb1).expect("Failed to read line");
    println!("Enter the second number: ");
    io::stdin().read_line(&mut nb2).expect("Failed to read line");
    println!("Enter the operator: ");
    io::stdin().read_line(&mut operator).expect("Failed to read line");
    let nb1: i32 = nb1.trim().parse().expect("Please type a number!");
    let nb2: i32 = nb2.trim().parse().expect("Please type a number!");
    let operator: char = operator.trim().parse().expect("Please type a number!");
    let result = match operator {
        '+' => nb1 + nb2,
        '-' => nb1 - nb2,
        '*' => nb1 * nb2,
        '/' => nb1 / nb2,
        _ => panic!("Unknown operator"),
    };
    println!("{} {} {} = {}", nb1, operator, nb2, result);
}
