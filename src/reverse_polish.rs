// We need to know how many symbols there can possibly be. When updating by
// adding new symbols, you MUST update this value.
const NUM_SYMBOLS: u32 = 4;
#[derive(Debug, Clone)]
pub enum Symbol {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug, Clone)]
pub enum Token {
    Operator(Symbol),
    Operand(f32),
}

const ERR_MSG_LT_TWO: &str = "Stack length is < 2";
const ERR_MSG_NE_ONE: &str = "Stack length != 1";
const ERR_MSG_NE_FOUR: &str = "Expected number to be of length 4";
const ERR_MSG_COULD_NOT_PARSE: &str = "Could not parse number";
const ERR_MSG_UNKNOWN_OPERATOR: &str = "Unknown operator";
const RADIX: u32 = 10;
const NUM_OPERANDS: u32 = 4;
const NUM_OPERATORS: u32 = NUM_OPERANDS - 1;
const NUM_COMBINATIONS: u32 = NUM_SYMBOLS.pow(NUM_OPERATORS as u32);
const GOAL_NUM: f32 = 10.0;

pub fn solve_reverse_polish(equation: &Vec<Token>) -> Result<f32, &str> {
    let mut stack: Vec<f32> = Vec::new();

    for token in equation {
        match token {
            Token::Operand(value) => stack.push(*value),

            Token::Operator(value) => {
                if stack.len() < 2 {
                    return Err(ERR_MSG_LT_TWO);
                }
                let top_value = stack.pop().unwrap();
                let second_top_value = stack.pop().unwrap();

                match value {
                    Symbol::Add => stack.push(second_top_value + top_value),
                    Symbol::Subtract => stack.push(second_top_value - top_value),
                    Symbol::Multiply => stack.push(second_top_value * top_value),
                    Symbol::Divide => stack.push(second_top_value / top_value),
                }
            }
        }
    }

    if stack.len() != 1 {
        return Err(ERR_MSG_NE_ONE);
    }
    let final_value = stack.pop().unwrap();

    Ok(final_value)
}

pub fn validate_num_string(number: &str) -> Result<bool, &str> {
    if number.chars().count() as u32 != NUM_OPERANDS {
        return Err(ERR_MSG_NE_FOUR);
    }

    if let Err(_) = number.parse::<u16>() {
        return Err(ERR_MSG_COULD_NOT_PARSE);
    }

    Ok(true)
}

pub fn create_base_equation(number: &str) -> Result<Vec<Token>, &str> {
    if let Err(val) = validate_num_string(number) {
        return Err(val);
    }

    let mut chars = number.chars();
    let mut equation: Vec<Token> = Vec::new();

    for _ in 0..NUM_OPERANDS {
        let char = chars.next().unwrap();
        let digit = char.to_digit(RADIX).unwrap();
        let operand = Token::Operand(digit as f32);
        equation.push(operand);
    }

    Ok(equation)
}

pub fn find_solutions(base_equation: &Vec<Token>) -> Result<Vec<Vec<Token>>, &str> {
    // Iterate over all possible combinations of operators, doing this by
    // encoding a number and using modulo to extract the
    // operators
    let mut solutions: Vec<Vec<Token>> = Vec::new();

    for i in 0..NUM_COMBINATIONS {
        let mut equation = base_equation.clone();
        let mut i = i;
        for _ in 0..NUM_OPERATORS {
            let operator = match i % NUM_SYMBOLS {
                0 => Token::Operator(Symbol::Add),
                1 => Token::Operator(Symbol::Subtract),
                2 => Token::Operator(Symbol::Multiply),
                3 => Token::Operator(Symbol::Divide),
                _ => return Err(ERR_MSG_UNKNOWN_OPERATOR),
            };

            equation.push(operator);

            i /= NUM_SYMBOLS;
        }

        if let Ok(result) = solve_reverse_polish(&equation) {
            //
            if float_eq(result, GOAL_NUM) {
                solutions.push(equation);
            }
        }
    }

    Ok(solutions)
}

pub fn float_eq(a: f32, b: f32) -> bool {
    (a - b).abs() < std::f32::EPSILON
}

pub fn print_equation_to_infix(equation: &Vec<Token>) {
    let mut stack: Vec<String> = Vec::new();

    for token in equation {
        match token {
            Token::Operand(value) => stack.push(value.to_string()),

            Token::Operator(value) => {
                let top_value = stack.pop().unwrap();
                let second_top_value = stack.pop().unwrap();

                let string = match value {
                    Symbol::Add => format!("({} + {})", second_top_value, top_value),
                    Symbol::Subtract => format!("({} - {})", second_top_value, top_value),
                    Symbol::Multiply => format!("({} * {})", second_top_value, top_value),
                    Symbol::Divide => format!("({} / {})", second_top_value, top_value),
                };

                stack.push(string.clone());
            }
        }
    }

    let equation_str = stack.pop().unwrap();

    println!("{} = {}", equation_str, GOAL_NUM);
}

#[test]
fn solve_empty() {
    assert_eq!(Err(ERR_MSG_NE_ONE), solve_reverse_polish(&Vec::new()));
}
