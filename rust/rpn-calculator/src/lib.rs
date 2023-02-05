#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

struct Stack {
    stack: Vec<i32>,
}

impl Stack {
    fn new() -> Self {
        Self {
            stack: Vec::<i32>::new(),
        }
    }

    fn push(&mut self, value: i32) {
        self.stack.push(value);
    }

    fn pop(&mut self) -> Option<i32> {
        self.stack.pop()
    }

    fn len(&self) -> usize {
        self.stack.len()
    }

    fn op(&mut self, input: &CalculatorInput) -> Option<i32> {
        // Pop in reverse order because this is a stack
        let y = self.pop();
        let x = self.pop();

        if x.is_none() || y.is_none() {
            return None;
        }

        let x = x.unwrap();
        let y = y.unwrap();

        match input {
            CalculatorInput::Add => Some(x + y),
            CalculatorInput::Subtract => Some(x - y),
            CalculatorInput::Multiply => Some(x * y),
            CalculatorInput::Divide => {
                if y == 0 {
                    return None;
                }
                Some(x / y)
            }
            _ => None,
        }
    }
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    if inputs.len() == 0 {
        return None;
    }

    let mut stack = Stack::new();

    for input in inputs {
        match input {
            CalculatorInput::Value(v) => stack.push(*v),
            _ => {
                let res = stack.op(input);

                if res.is_none() {
                    return None;
                }

                stack.push(res.unwrap());
            }
        }
    }

    if stack.len() != 1 {
        return None;
    }

    stack.pop()
}
