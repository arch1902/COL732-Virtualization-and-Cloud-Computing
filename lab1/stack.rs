use std::env::args;
pub struct Stack {}

impl Stack {
    pub fn calculate(s: String) -> i32 {
        let mut stack: Vec<i32> = Vec::new();
        let mut n = 0;
        let mut f = 1;
        let mut ans = 0;
        stack.push(f);
        for c in s.chars() {
            if c.is_digit(10){
                n = n*10 + (c as i32 - '0' as i32);
            } else if c=='+'{
                ans += f*n;
                n = 0;
                f = *stack.last().unwrap();
            } else if c=='-'{
                ans += f*n;
                n=0;
                f = -*stack.last().unwrap();
            } else if c=='('{
                stack.push(f);
            } else if c==')'{
                stack.pop();
            } else {
                continue;
            }
        }
        ans += f*n;
        ans
    }
}

fn main() {
    let args: Vec<String> = args().collect();
    let expr: String = (&args[1..]).join(" ");

    println!("{:?}", Stack::calculate(expr));
}