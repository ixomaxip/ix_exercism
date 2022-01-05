#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

fn calc_result(stack: &mut Vec<Option<i32>>, operator: &CalculatorInput) -> Option<i32> {
    match (stack.pop().flatten(), stack.pop().flatten()) {
        (Some(b), Some(a)) => {
            return match operator {
                    CalculatorInput::Multiply => Some(a * b),
                    CalculatorInput::Divide => Some(a / b),
                    CalculatorInput::Add => Some(a + b),
                    CalculatorInput::Subtract => Some(a - b),
                    _ => unreachable!()
                };
            },
        _ => return None
    };
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut stack: Vec<Option<i32>> = vec![];

    for item in inputs {
        let result = match item {
            CalculatorInput::Value(n) => Some(*n),
            _ => calc_result(&mut stack, item)
        };
        stack.push(result);
    }

    match stack.len() {
        1 => stack.pop().flatten(),
        _ => None
    }    
}