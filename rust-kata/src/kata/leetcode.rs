use std::{
    collections::{HashMap, HashSet},
    u8,
};

pub mod palindrome_linked_list;
pub mod score;

pub fn two_sum1(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for i in 0..nums.len() {
        for j in i + 1..nums.len() {
            if nums[i] + nums[j] == target {
                return vec![i as i32, j as i32];
            }
        }
    }
    todo!()
}

pub fn two_sum2(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut hash = HashMap::new();
    for (i, num) in nums.iter().enumerate() {
        match hash.get(&(target - *num)) {
            Some(&j) => return vec![i as i32, j],
            None => hash.insert(*num, i as i32),
        };
    }
    todo!()
}

pub fn is_palindrome_number(x: i32) -> bool {
    let str = x.to_string();
    let data: Vec<char> = str.trim().chars().collect();

    let (mut left, mut right) = (0, data.len() - 1);

    while left < right {
        if data[left] != data[right] {
            return false;
        }
        left += 1;
        right -= 1;
    }

    true
}
pub fn is_palindrome_number_fastest(x: i32) -> bool {
    let str = x.to_string();
    let mut cv: Vec<char> = str.chars().collect();
    cv.reverse();
    let cstr: String = cv.iter().collect();
    cstr == str
}
pub fn divide(x1: i32, x2: i32) -> i32 {
    match (x1, x2) {
        (i32::MIN, -1) => i32::max_value(),
        (_, _) => x1 / x2,
    }
}
pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut res = Vec::new();
    pub fn gen_per(
        res: &mut Vec<Vec<i32>>,
        cur: &mut Vec<i32>,
        nums: &[i32],
        used: &mut HashSet<usize>,
    ) {
        if nums.len() == cur.len() {
            res.push(cur.clone());
            return;
        }
        for i in 0..nums.len() {
            if !used.contains(&i) {
                used.insert(i);
                cur.push(nums[i]);
                gen_per(res, cur, nums, used);
                cur.pop();
                used.remove(&i);
            }
        }
    }
    gen_per(&mut res, &mut Vec::new(), &nums, &mut HashSet::new());
    res
}

pub fn multiply(s1: String, s2: String) -> String {
    let mut res = vec![0; s1.len() + s2.len()];

    for (i, c1) in s1.chars().rev().enumerate() {
        for (j, c2) in s2.chars().rev().enumerate() {
            let mut sum = (c1 as u8 - b'0') * (c2 as u8 - b'0');
            let idx = i + j;

            sum += res[idx] as u8;
            res[idx] = sum % 10;
            res[idx + 1] = sum / 10;
        }
    }
    let mut st = res.len() - 1;
    while st > 0 && res[st] == 0 {
        st -= 1;
    }

    let mut resstr = String::new();
    for i in (0..st).rev() {
        resstr.push(char::from(res[i] + b'0'));
    }

    resstr
}

#[test]
fn test() {
    assert_eq!(
        multiply("123".to_string(), "321".to_string()),
        "39483".to_string()
    );
}
