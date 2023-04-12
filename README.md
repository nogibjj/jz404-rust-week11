# Rust mini project Week 11 - mini calculator
This is a simple command-line calculator written in Rust. It supports basic arithmetic operations (addition, subtraction, multiplication, and division), as well as parentheses.

### Usage
To run the calculator, simply execute the following command:

```
$ cargo run
```
Once the calculator is running, you can enter expressions for it to evaluate. For example:

```
> 2 + 3 * 4 / 2
Result: 8
```
The calculator will print out the result of each expression you enter. If the expression is invalid (e.g. contains a syntax error), the calculator will print an error message instead.

To quit the calculator, simply enter the command quit:
```
> quit
Bye!
```

### Implementation
The calculator is implemented using Rust's standard library. It reads expressions from standard input, tokenizes them into numbers and operators, and then evaluates the expressions using a simple stack-based algorithm.