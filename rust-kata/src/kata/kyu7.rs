use itertools::Itertools;
use std::collections::HashMap;
use std::str::FromStr;

pub fn neutralise(s1: &str, s2: &str) -> String {
    assert_eq!(s1.len(), s2.len());
    let v1: Vec<&str> = s1.split("").collect();
    let v2: Vec<&str> = s2.split("").collect();
    let mut str = String::new();
    for i in 0..s1.len() + 1 {
        if v1[i] != v2[i] {
            str.push('0')
        } else {
            str.push_str(v1[i])
        }
    }
    str
}

pub fn square_digits(n: u64) -> u64 {
    u64::from_str(
        &n.to_string()
            .chars()
            .map(|e| u32::pow(e.to_digit(10).unwrap(), 2))
            .join(""),
    )
    .unwrap()
}
pub fn high_and_low(nums: &str) -> String {
    let mut min = 10000;
    let mut max = -10000;
    for n in nums.split(' ') {
        match n.parse::<i32>() {
            Ok(v) => {
                if min > v {
                    min = v
                }
                if max < v {
                    max = v
                }
            }
            Err(_) => panic!("parse error"),
        }
    }
    format!("{} {}", max, min)
}
pub fn find_short(s: &str) -> u32 {
    let mut min: u32 = 10000;
    for word in s.split_whitespace() {
        min = min.min(word.len() as u32)
    }
    min
}
pub fn row_sum_odd_numbers(n: i64) -> i64 {
    n.pow(3)
}
pub fn open_or_senior(data: Vec<(i32, i32)>) -> Vec<String> {
    let mut result = vec![];
    for i in data {
        if i.0 >= 55 && i.1 > 7 {
            result.push("Senior".to_string())
        } else {
            result.push("Open".to_string())
        }
    }
    result
}
pub fn word_pattern(word: &str) -> String {
    let mut res = String::new();
    let mut cryptomap = HashMap::new();
    let mut counter = 0;

    for c in word.to_lowercase().chars() {
        match cryptomap.insert(c, counter) {
            None => {
                counter += 1;
            }
            Some(v) => {
                cryptomap.insert(c, v).unwrap();
            }
        };
        match cryptomap.get(&c) {
            None => {}
            Some(v) => res.push_str(&format!("{v}.")),
        }
    }
    res.pop();

    res
}

pub fn wall_paper(l: f64, w: f64, h: f64) -> String {
    const NUMBERS: [&str; 21] = [
        "zero",
        "one",
        "two",
        "three",
        "four",
        "five",
        "six",
        "seven",
        "eight",
        "nine",
        "ten",
        "eleven",
        "twelve",
        "thirteen",
        "fourteen",
        "fifteen",
        "sixteen",
        "seventeen",
        "eighteen",
        "nineteen",
        "twenty",
    ];
    if l == 0.0 || w == 0.0 || h == 0.0 {
        return NUMBERS[0].to_string();
    }
    let p = 2.0 * l * h + 2.0 * w * h;
    let pp = p + p / 100.0 * 15.0;

    NUMBERS[(pp / 5.2).ceil() as usize].to_string()
}
pub fn get_sum(a: i64, b: i64) -> i64 {
    let min = a.min(b);
    let max = a.max(b);
    let mut result: i64 = 0;
    for i in min..=max {
        result += i
    }
    result
}
pub fn validate_pin(pin: &str) -> bool {
    match pin.len() {
        4 => pin.matches(char::is_numeric).count() == 4,
        6 => pin.matches(char::is_numeric).count() == 6,
        _ => false,
    }
}
pub fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}
pub fn descending_order(x: u64) -> u64 {
    let mut ch: Vec<String> = x.to_string().chars().map(|c| c.to_string()).collect();
    ch.sort();
    ch.reverse();
    ch.join("").parse::<u64>().unwrap()
}
pub fn reverse_letters(s: &str) -> String {
    s.chars().rev().filter(|c| c.is_alphabetic()).collect()
}
pub fn min_max(lst: &[i32]) -> (i32, i32) {
    (
        lst.iter().min().unwrap().to_owned(),
        lst.iter().max().unwrap().to_owned(),
    )
}
pub fn bingo<S: AsRef<str>>(ticket: &[(S, u8)], win: usize) -> &'static str {
    if ticket
        .iter()
        .filter(|(s, n)| !s.as_ref().as_bytes().contains(n))
        .count()
        >= win
    {
        "Winner!"
    } else {
        "Loser!"
    }
}
pub fn max_rot(n: u64) -> u64 {
    let mut rots = vec![];
    for i in 1..=3 {
        let mut sn = n.to_string().chars().skip(i).join("");
        sn.push_str(&n.to_string().chars().take(i).join(""));
        println!("{}", &sn);
        rots.push(sn.parse::<u64>().unwrap())
    }
    rots.iter().max().unwrap().to_owned()
}
pub fn my_languages(res: HashMap<&str, i32>) -> Vec<&str> {
    res.iter()
        .filter(|(_, &s)| s >= 60)
        .sorted_by_key(|a| -a.1)
        .map(|(&l, _)| l)
        .collect()
}
pub fn is_triangle(a: i64, b: i64, c: i64) -> bool {
    if a <= 0 || b <= 0 || c <= 0 {
        return false;
    }
    a + b > c && a + c > b && b + c > a
}
#[cfg(test)]
mod test {
    use super::max_rot;
    #[test]
    fn test_max_rot() {
        let _n = max_rot(38458215);
    }
}
