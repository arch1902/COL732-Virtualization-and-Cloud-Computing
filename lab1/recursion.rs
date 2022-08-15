use std::env::args;
pub struct Recursion {}

impl Recursion {

    fn helper(i:usize, nums: &Vec<i32>, curr: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>){
        if i==nums.len() {
            ans.push(curr.to_vec());
        } else {
            Self::helper(i+1,nums,curr,ans);
            curr.push(nums[i]);
            Self::helper(i+1,nums,curr,ans);
            curr.pop();
        }
    }

    pub fn power_set(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans = Vec::new();
        let mut curr = Vec::new();

        Self::helper(0, &nums, &mut curr,&mut ans);
        ans
    }
}

fn main() {
    let mut args: Vec<String> = args().collect();
    let mut set: Vec<i32> = Vec::new();

    for i in &mut args[1..] {
        set.push(i.parse::<i32>().unwrap())
    }

    println!("{:?}", Recursion::power_set(set));
}