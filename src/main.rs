use std::io::{stdin, stdout, Write};

fn read(input: &mut String) {
	stdout().flush().expect("Failed to flush");
	stdin().read_line(input).expect("Failed to read");
}

fn main() {
	println!("welcome to the calcuator");
	loop {
		let mut num1 = String::new();
		let mut num2 = String::new();
		let mut operator = String::new();

		print!("what is the first number?: ");
		read(&mut num1);

		print!("what is the second number?: ");
		read(&mut num2);

		print!("select operator +-*/ ");
		read(&mut operator);

		let num1: f32 = num1.trim().parse().unwrap();
		let num2: f32 = num2.trim().parse().unwrap();
		let operator: char = operator.trim().chars().next().unwrap();

		let operators = String::from("+-*/");
		if !operators.contains(operator) {
			println!("unknown operator");
			continue;
		}

		let result = match operator {
			'+' => num1 + num2,
			'-' => num1 - num2,
			'*' => num1 * num2,
			'/' => num1 / num2,
			_ => panic!("error in opertor"),
		};

		println!("the result of {} {} {}  = {}", num1, operator, num2, result);
	}
}
