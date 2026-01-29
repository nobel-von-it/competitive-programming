use itertools::Itertools;

fn num_identical_pairs(ns: Vec<i32>) -> i32 {
    let mut count = 0;
    for i in 0..ns.len() {
        for j in i + 1..ns.len() {
            if ns[i] == ns[j] && i < j {
                count += 1;
            }
        }
    }
    count
}
fn get_sneaky_numbers(ns: Vec<i32>) -> Vec<i32> {
    let mut res = Vec::new();
    for i in 0..ns.len() {
        let mut count = 0;
        for j in i + 1..ns.len() {
            if ns[i] == ns[j] {
                count += 1;
            }
        }
        if count > 0 {
            res.push(ns[i]);
        }
    }
    res
}
fn minimum_operations(nums: Vec<i32>) -> i32 {
    nums.iter().filter(|&x| x % 3 != 0).count() as i32
}
fn difference_of_sums(n: i32, m: i32) -> i32 {
    let mut res = 0;
    for i in 0..n {
        match i % m == 0 {
            true => res -= i,
            false => res += i,
        }
    }
    res
}
fn find_words_containing(ws: Vec<String>, x: char) -> Vec<i32> {
    ws.iter()
        .enumerate()
        .filter_map(|(i, w)| w.contains(x).then_some(i as i32))
        .collect()
}
fn convert_date_to_binary(d: String) -> String {
    d.split("-")
        .map(|s| format!("{:b}", s.parse::<i32>().unwrap()))
        .collect::<Vec<String>>()
        .join("-")
}
fn maximum_wealth(a: Vec<Vec<i32>>) -> i32 {
    a.iter().map(|v| v.iter().sum()).max().unwrap()
}
fn count_max_or_subsets(nums: Vec<i32>) -> i32 {
    let max_or_val = nums.iter().fold(0, |a, &x| a | x);

    fn calc(nums: &[i32], idx: usize, cur: i32, tar: i32) -> i32 {
        if cur == tar {
            return 1 << (nums.len() - idx);
        }
        match nums.get(idx) {
            Some(&n) => calc(nums, idx + 1, cur | n, tar) + calc(nums, idx + 1, cur, tar),
            None => 0,
        }
    }

    calc(&nums, 0, 0, max_or_val)
}
fn smallest_even_multiple(n: i32) -> i32 {
    if n % 2 == 0 {
        n
    } else {
        n * 2
    }
}
fn find_permutation_difference(s: String, t: String) -> i32 {
    s.chars()
        .enumerate()
        .map(|(i, cs)| {
            (i as isize - t.chars().position(|c| c == cs).unwrap() as isize).abs() as i32
        })
        .sum()
}
fn subset_xor_sum(nums: Vec<i32>) -> i32 {
    nums.iter().fold(0, |a, &x| a | x) << (nums.len() - 1)
}
