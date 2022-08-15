use std::env::args;
pub struct Typecast {}

impl Typecast {
    pub fn product(num1: String, num2: String) -> String {
        let n = num1.len();
        let m = num2.len();

        let mut tmp = vec![0; n+m];
        for (i, p) in num1.chars().rev().enumerate() {
            let mut carry = 0;
            
            for (j, q) in num2.chars().rev().enumerate() {
                let a = p.to_digit(10).unwrap();
                let b = q.to_digit(10).unwrap();
                
                let x = tmp[n+m-i-j-1] + a*b + carry;
                tmp[n+m-i-j-1] = x%10;
                carry = x/10;
            }
            tmp[n-i-1]+=carry;
        }
        
        let mut ans:Vec<char> = Vec::new();

        let mut flag:bool = false;

        for i in 0..(n+m){
            if tmp[i]==0 && !flag {
                continue;
            } else {
                flag = true;
                ans.push(char::from_digit(tmp[i], 10).unwrap());
            }
        }

        if ans.len()==0 {
            String::from("0")
        }else {
            ans.into_iter().collect()
        }
        
    }
}

fn main() {
    let s: String = args().nth(1).unwrap();
    let t: String = args().nth(2).unwrap();
    println!("{}", Typecast::product(s,t));
}