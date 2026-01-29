
pub fn score_of_string(s: String) -> i32 {
    let nums: Vec<u8> = s.chars().map(|c| c as u8).collect();
    let mut res = 0;
    for i in 0..nums.len()-1 {
        res += i32::abs(nums[i] as i32 - nums[i+1] as i32);
    }
    res
}
