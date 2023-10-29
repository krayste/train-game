mod reverse_polish;
use std::env;

use reverse_polish::{create_base_equation, find_solutions, validate_num_string};

use crate::reverse_polish::print_equation_to_infix;

fn main() {
    let mut args = env::args();

    // find the number from the args: specified by
    // input=1234
    let input_num = args.find(|x| x.starts_with("input="));

    if let None = input_num {
        println!("No input number specified!");
        return;
    }

    let input_num = input_num.unwrap();

    let input_num = &input_num[6..];

    let validate_result = validate_num_string(input_num);

    if let Err(val) = validate_result {
        println!("{:?}", val);
        return;
    }

    let base_equation = create_base_equation(input_num);

    if let Err(val) = base_equation {
        println!("{:?}", val);
        return;
    }

    let base_equation = base_equation.unwrap();

    let results = find_solutions(&base_equation);

    if let Err(val) = results {
        println!("{:?}", val);
        return;
    }

    let results = results.unwrap();

    if results.len() == 0 {
        println!("No solutions found!");
    } else {
        println!("Found {} solutions for {}!", results.len(), input_num);
        for result in results {
            print_equation_to_infix(&result);
        }
    }
}
