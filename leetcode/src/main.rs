use std::collections::HashMap;


use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut seen = HashMap::new();
        for (index, integer) in nums.iter().enumerate() {
        let diff = target - integer;
        println!("{:?}", seen);
            if seen.contains_key(&diff) {
                return vec![index as i32, *seen.get(&diff).unwrap() as i32];
            }
            else {
                seen.insert(integer , index);
            }
    }
    vec![]
    
    }
}
        
}


fn main() {


    println!("Hello, world!");
}
