use aoc;
use std::collections::HashMap;

type OpPrecedence = HashMap<char, usize>;
type RPN = Vec<char>;

// https://en.wikipedia.org/wiki/Shunting-yard_algorithm
fn parse2(expr : &String, opm: &OpPrecedence) -> RPN {
    let mut rpn : Vec<char> = vec!();
    let mut op : Vec<char> = vec!();

    for t in expr.chars().filter(|c| *c != ' ') {
        match t {
            '(' => op.push(t),
            ')' => {
                while !op.is_empty() && *op.last().unwrap() != '(' {
                    rpn.push(op.pop().unwrap());
                }
                if *op.last().unwrap() == '(' {
                    op.pop(); //discard
                }
            },
            '+' | '*' => {

                while !op.is_empty()  {
                    let peek = *op.last().unwrap();
                    if peek == '(' {
                        break;
                    }
                    // Check if new op has higher precedence than top of stack
                    if opm.get(&peek).unwrap() < opm.get(&t).unwrap() {
                        break;
                    }
                    rpn.push(op.pop().unwrap());
                }

                op.push(t)
            },
            _ => rpn.push(t)
        }
    }

    while !op.is_empty() {
        rpn.push(op.pop().unwrap());
    }

    rpn
}

fn eval(expr : RPN) -> u64 {

    let mut out : Vec<u64> = vec!();

    for token in expr {
        match token {
            '+' => {
                let op1 = out.pop().unwrap();
                let op2 = out.pop().unwrap();
                out.push(op1 + op2);
            },
            '*' => {
                let op1 = out.pop().unwrap();
                let op2 = out.pop().unwrap();
                out.push(op1 * op2);
            },
            _ => {
                out.push(token.to_digit(10).unwrap() as u64)
            }
        }
    }

    *out.last().unwrap()
}

fn main() {
    let lines = aoc::lines_from_file("input.txt");

    let mut op_prec = HashMap::new();
    op_prec.insert('+', 0);
    op_prec.insert('*', 0);

    let sum1 : u64 = lines.iter().map(|l| eval(parse2(l, &op_prec))).sum();
    dbg!(sum1);

    // elevate + above *
    op_prec.insert('+', 1);
    let sum2 : u64 = lines.iter().map(|l| eval(parse2(l, &op_prec))).sum();
    dbg!(sum2);
}
