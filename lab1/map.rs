use std::env::args;
pub struct Map {}

impl Map {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        if s.len()!=t.len() {
            return false;
        }
        let mut mp = std::collections::HashMap::new();

        let mut svec: Vec<char> = Vec::new();
        let mut tvec: Vec<char> = Vec::new();

        for c in s.chars(){
            svec.push(c);
        }

        for c in t.chars(){
            tvec.push(c);
        }

        let n = s.len();

        for i in 0..n {
            if !mp.contains_key(&svec[i]){
                for (_, val) in mp.iter() {
                    if tvec[i]==*val {
                        return false;
                    }
                 }
                mp.insert(svec[i],tvec[i]);
            } else {
                if tvec[i]!=mp[&svec[i]] {
                    println!("{}", mp[&svec[i]]);
                    return false;
                }
            }
        }

        true
    }
}

fn main() {
    let s: String = args().nth(1).unwrap();
    let t: String = args().nth(2).unwrap();
    println!("{}", Map::is_isomorphic(s,t));
}