use std::env::{args, Args};
fn main() {
    let mut args: Args = args();

    println!("{:?}", args);
    let first_arg: f32 = args.nth(1).unwrap().parse::<f32>().unwrap();

    println!("{:?}", args);
    let second_arg_probably_operator: char = args.nth(0).unwrap().chars().next().unwrap();
    println!("{} {:?}", second_arg_probably_operator, args);
    let third_arg: f32 = args.nth(0).unwrap().parse::<f32>().unwrap();

    let result: f32 = operate(first_arg, second_arg_probably_operator, third_arg);

    println!(
        "{} {} {} = {}",
        first_arg, second_arg_probably_operator, third_arg, result
    );
}
fn operate(first_number: f32, operator: char, second_number: f32) -> f32 {
    match operator {
        '+' => first_number + second_number,
        '-' => first_number - second_number,
        '/' => first_number / second_number,
        '*' | 'X' | 'x' => first_number * second_number,
        _ => panic!("Invalid operator used"),
    }
}
