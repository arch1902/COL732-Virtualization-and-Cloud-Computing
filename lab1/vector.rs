use std::env;

pub struct Vector {}

#[derive(Debug, PartialEq, Eq)]
pub struct Interval {
    pub start: i32,
    pub end: i32,
}

impl Interval {
    #[inline]
    pub fn new(start: i32, end: i32) -> Self {
        Interval { start, end }
    }
}

impl Vector {
    pub fn overlap(intervals: &mut Vec<Interval>) -> Vec<Interval> {
        intervals.sort_unstable_by(|x, y| x.start.cmp(&y.start));

        let mut ans: Vec<Interval> = Vec::new(); 
        let n = intervals.len();
        if n==0 {
            return ans;
        }
        if n==1 {
            ans.push(Interval::new(intervals[0].start,intervals[0].end));
            return ans;
        }
        
        ans.push(Interval::new(intervals[0].start,intervals[0].end));

        let mut start_ = intervals[0].start;
        let mut end_ = intervals[0].end;

        for i in 1..n {
            if intervals[i].start <= end_ {
                ans.pop();
                end_ = std::cmp::max(end_, intervals[i].end);
                ans.push(Interval::new(start_, end_));
            }else {
                start_ = intervals[i].start;
                end_ = intervals[i].end;
                ans.push(Interval::new(start_, end_));
            }
        }

        ans
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let intervals = &args[1..];

    let mut input: Vec<Interval> = Vec::new();
    let mut temp: i32 = 0;

    for (pos, e) in intervals.iter().enumerate() {
        if let 1=pos%2{
            input.push(Interval::new(temp, e.parse::<i32>().unwrap()))
        }
        else {
            temp = e.parse::<i32>().unwrap();
        }
    }

    println!("{:?}", Vector::overlap(&mut input));
}