use std::io;

fn main() {
    println!("Welcome to the Rust Calculator!");

    loop {
        // read expression
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        // remove the trailing newline
        let input = input.trim();

        // if input is "quit", exit
        if input == "quit" {
            println!("Bye!");
            break;
        }

        // if input is empty, ask for expression again
        if input.is_empty() {
            println!("Please enter an expression.");
            continue;
        }

        // calculate expression
        match eval(input) {
            Ok(result) => println!("Result: {}", result),
            Err(error) => println!("Error: {}", error),
        }
    }
}

// calculate expression
fn eval(expr: &str) -> Result<f64, String> {
    // parse expression
    let mut nums: Vec<f64> = Vec::new();
    let mut ops: Vec<char> = Vec::new();
    let mut num_buf = String::new();
    for c in expr.chars() {
        if c.is_digit(10) || c == '.' {
            num_buf.push(c);
        } else if !num_buf.is_empty() {
            nums.push(num_buf.parse().unwrap());
            num_buf.clear();
        }
        if "+-*/()".contains(c) {
            ops.push(c);
        }
    }
    if !num_buf.is_empty() {
        nums.push(num_buf.parse().unwrap());
    }

    // calculate
    let mut stack: Vec<f64> = Vec::new();
    let mut op_stack: Vec<char> = Vec::new();
    for i in 0..nums.len() {
        stack.push(nums[i]);
        if i < ops.len() {
            while !op_stack.is_empty() && precedence(&op_stack[op_stack.len() - 1]) >= precedence(&ops[i]) {
                eval_op(&mut stack, op_stack.pop().unwrap());
            }
            op_stack.push(ops[i]);
        }
    }
    while !op_stack.is_empty() {
        eval_op(&mut stack, op_stack.pop().unwrap());
    }
    if stack.len() == 1 {
        Ok(stack[0])
    } else {
        Err("Invalid expression".to_owned())
    }
}

// evaluate operator
fn eval_op(stack: &mut Vec<f64>, op: char) {
    let rhs = stack.pop().unwrap();
    let lhs = stack.pop().unwrap();
    match op {
        '+' => stack.push(lhs + rhs),
        '-' => stack.push(lhs - rhs),
        '*' => stack.push(lhs * rhs),
        '/' => stack.push(lhs / rhs),
        _ => (),
    }
}

// return precedence of operator
fn precedence(op: &char) -> i32 {
    match op {
        '+' | '-' => 1,
        '*' | '/' => 2,
        _ => 0,
    }
}
