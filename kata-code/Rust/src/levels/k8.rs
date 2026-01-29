use itertools::Itertools;

fn summation(n: i32) -> i32 {
    (1..=n).sum()
}
fn square_sum(vec: Vec<i32>) -> i32 {
    vec.iter().fold(0, |ac, x| ac + x.pow(2))
}
fn abbrev_name(name: &str) -> String {
    name.split(" ").map(|s| s[..1].to_uppercase()).join(".")
}
fn digitize(n: u64) -> Vec<u8> {
    n.to_string()
        .chars()
        .rev()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect::<Vec<u8>>()
}
fn find_average(a: &[f64]) -> f64 {
    if a.is_empty() {
        0.0
    } else {
        a.iter().sum::<f64>() / a.len() as f64
    }
}
fn invert(arr: &[i32]) -> Vec<i32> {
    arr.iter().map(|&x| -x).collect()
}
fn get_average(a: &[i32]) -> i32 {
    a.iter().sum::<i32>() / a.len() as i32
}
