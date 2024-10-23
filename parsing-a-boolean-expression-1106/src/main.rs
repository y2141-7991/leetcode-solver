struct Solution;

impl Solution {
    pub fn parse_bool_expr(expression: String) -> bool {
        let mut stack: Vec<u8> = Vec::new();
        for a in expression.bytes() {
            if a == b',' {
                continue;
            } else if a == b')' {
                let mut values = vec![];
                while let Some(top) = stack.pop() {
                    if top == b'(' {
                        break;
                    }
                    values.push(top);
                }
                match stack.pop() {
                    Some(operator) => {
                        let res = Self::evaluate(operator, values);
                        if res {
                            stack.push(b't');
                        } else {
                            stack.push(b'f');
                        }
                    }
                    None => {}
                }
            } else {
                stack.push(a);
            }
        }
        match stack.pop() {
            Some(top) => top == b't',
            None => false,
        }
    }

    fn evaluate(operator: u8, values: Vec<u8>) -> bool {
        match operator {
            b'!' => values[0] == b'f',
            b'&' => !values.into_iter().any(|v| v == b'f'),
            b'|' => values.into_iter().any(|v| v == b't'),
            _ => false,
        }
    }
}

fn main() {
    let a = String::from("&(|(f))");

    println!("{:?}", Solution::parse_bool_expr(a));
}
