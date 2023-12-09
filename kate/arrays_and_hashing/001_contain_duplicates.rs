fn main() {
    let res = contains_duplicate(vec![1,2,3,1]);
    println!("{}", res);
}

fn contains_duplicate(nums: Vec<i32>) -> bool {
     let mut mut_nums = nums.clone();
     mut_nums.sort();
     for (i, elem) in mut_nums.iter().enumerate() {
         if i != 0 && *elem == mut_nums[i-1] {
             return true;
         }
     }
     return false;
}
