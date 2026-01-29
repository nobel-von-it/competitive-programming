use core::f64;
use itertools::MinMaxResult::MinMax;
use itertools::{Either, Itertools};
use std::cmp::max;
use std::cmp::Ordering;

pub fn abbrev_name(name: &str) -> String {
    let v: Vec<&str> = name.split(' ').collect();

    let first = v[0].chars().next().unwrap();
    let second = v[1].chars().next().unwrap();

    format!(
        "{}.{}",
        first.to_ascii_uppercase(),
        second.to_ascii_uppercase()
    )
}

pub fn slice_plus_slice(x: &[i32], y: &[i32]) -> i32 {
    let xl = x.len();
    let yl = y.len();
    let max_len = max(xl, yl);
    let mut res = 0;

    for i in 0..max_len {
        if i < yl {
            res += y[i]
        }
        if i < xl {
            res += x[i]
        }
    }
    res
}

pub fn bin_to_decimal(i: &str) -> i32 {
    i32::from_str_radix(i, 2).unwrap()
}

pub fn bonus_time(s: u64, bonus: bool) -> String {
    let mut str = String::from("¥");
    if bonus {
        str.push_str(&(s * 10).to_string())
    } else {
        str.push_str(&s.to_string())
    }
    str
}

pub fn bonus_time2(s: u64, b: bool) -> String {
    format!("¥{}", if b { s * 10 } else { s })
}

pub fn contamination(text: &str, character: &str) -> String {
    let s = String::from(character);
    s.repeat(text.len())

    // character.repeat(text.len())
}

pub fn convert_to_i32(f: f32) -> i32 {
    f.to_bits() as i32
}

pub fn correct_tail(body: &str, tail: char) -> bool {
    body.chars().collect::<Vec<char>>()[body.len() - 1] == tail
}

pub fn correct_tail_simplify(b: &str, t: char) -> bool {
    b.ends_with(t)
}

pub fn create_phone_number1(numbers: &[u8]) -> String {
    let result = numbers
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    let fp = result[0..3].join("");
    let sp = result[3..6].join("");
    let tp = result[6..].join("");
    format!("({}) {}-{}", fp, sp, tp)
}
//or
pub fn create_phone_number2(numbers: &[u8]) -> String {
    let result: String = numbers.iter().map(|e| e.to_string()).collect();
    format!("({}) {}-{}", &result[..3], &result[3..6], &result[6..])
}
pub fn digital_root(n: i64) -> i64 {
    let mut res = n;
    while res > 9 {
        res = res
            .to_string()
            .chars()
            .fold(0, |a: i64, b| a + i64::from(b.to_digit(10).unwrap()));
    }
    res
}

// with smart math
pub fn digital_root_pp(n: i64) -> i64 {
    (n - 1) % 9 + 1
}

pub fn disemvowel(s: &str) -> String {
    s.chars()
        .filter(|e| "aeiouAEIOU".find(*e).is_none())
        .join("")
}

pub fn double_char(s: &str) -> String {
    let mut res = String::new();
    for i in s.chars() {
        res.push(i);
        res.push(i);
    }
    res
}

// or
pub fn double_char2(s: &str) -> String {
    s.chars().map(|c| c.to_string().repeat(2)).collect()
}

pub fn fake_bin1(s: &str) -> String {
    s.chars().map(|c| if c < '5' { '0' } else { '1' }).collect()
}

pub fn fake_bin2(s: &str) -> String {
    let mut r = String::new();
    for i in s.split("") {
        if !i.is_empty() {
            if i > "4" {
                r += "1"
            } else {
                r += "0"
            }
        }
    }
    r
}

pub fn find_average1(slice: &[f64]) -> f64 {
    if slice.is_empty() {
        return 0.0;
    }
    let sum: f64 = slice.iter().sum();
    sum / slice.len() as f64
}

pub fn find_average2(arr: &[f64]) -> f64 {
    match arr.len() {
        0 => 0.,
        n => arr.iter().sum::<f64>() / n as f64,
    }
}

pub fn find_difference(a: &[i32; 3], b: &[i32; 3]) -> i32 {
    //let va = a.iter().fold(1, |acc, i| acc * i);
    let va: i32 = a.iter().product();
    let vb: i32 = b.iter().product();
    (va - vb).abs()
}

pub fn find_multiples(n: u32, l: u32) -> Vec<u32> {
    let mut v: Vec<u32> = vec![];
    for i in 1..=l {
        if i % n == 0 {
            v.push(i)
        }
    }
    v
}

pub fn first_non_consecutive(arr: &Vec<i32>) -> Option<i32> {
    let mut prev = arr[0];
    for i in arr {
        let check = i - prev;
        if check > 1 {
            return Some(*i);
        }
        prev = *i;
    }
    None
}

pub fn flick_switch(list: &[&str]) -> Vec<bool> {
    let mut v: Vec<bool> = vec![];
    let mut flag = true;
    for &i in list {
        if i == "flick" {
            flag = !flag
        }
        v.push(flag)
    }
    v
}

pub fn flick_switch2(list: &[&str]) -> Vec<bool> {
    let mut f = true;
    list.iter()
        .map(|&e| {
            if e != "flick" {
                f
            } else {
                f = !f;
                f
            }
        })
        .collect()
}

pub fn html_special_chars(html: &str) -> String {
    html.to_string()
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

pub fn merge_arrays1(arr1: &[i32], arr2: &[i32]) -> Vec<i32> {
    let mut v: Vec<i32> = vec![];
    for i in arr1 {
        if !v.contains(i) {
            v.push(*i)
        }
    }
    for i in arr2 {
        if !v.contains(i) {
            v.push(*i)
        }
    }
    v.sort();
    v
}

pub fn merge_arrays2(a1: &[i32], a2: &[i32]) -> Vec<i32> {
    let mut v = [a1, a2].concat();
    v.sort();
    v.dedup();
    v
}

pub fn nearest_sq1(n: u32) -> u32 {
    let mut nq = f64::sqrt(n as f64);
    nq = nq.round();
    (nq * nq) as u32
}

// or best practices

pub fn nearest_sq2(n: u32) -> u32 {
    ((n as f64).sqrt().round() as u32).pow(2)
}

pub fn is_palindrome(x: i32) -> bool {
    if x < 0 {
        return false;
    }
    let mut x = x;
    let mut len = ("1".to_owned() + &*"0".repeat(x.to_string().len() - 1))
        .parse::<i32>()
        .unwrap();
    while len > 1 {
        let first = x % 10;
        let end = x / len % 10;
        if first != end {
            return false;
        }
        x /= 10;
        len /= 100;
    }
    true
}

pub fn rps2(p1: &str, p2: &str) -> &'static str {
    if p1 == p2 {
        return "Draw!";
    }
    match (p1, p2) {
        ("scissors", "paper") | ("paper", "rock") | ("rock", "scissors") => "Player 1 won!",
        _ => "Player 2 won!",
    }
}

pub fn sum_mix(a: &[Either<i32, String>]) -> i32 {
    let mut r = 0;
    for i in a {
        match i {
            Either::Left(x) => r += x,
            Either::Right(x) => r += x.parse::<i32>().unwrap(),
        }
    }
    r
}

pub fn sum_of_differences1(arr: &[i8]) -> Option<i8> {
    if arr.len() < 2 {
        return None;
    }
    let mut array = arr.to_vec();
    let mut res: i8 = 0;

    array.sort();
    array.reverse();

    for i in 0..arr.len() - 1 {
        res += array[i] - array[i + 1]
    }
    Some(res)
}
//or best practices

pub fn sum_of_differences2(arr: &[i8]) -> Option<i8> {
    match arr.iter().minmax() {
        MinMax(x, y) => Some(y - x),
        _ => None,
    }
}

pub fn mt2(n: u64) -> String {
    (1..=10)
        .map(|a| format!("{} * {} = {}", a, n, a * n))
        .join("\n")
}

pub fn points(g: &[String]) -> u32 {
    g.iter()
        .map(|e| {
            let (l, r) = e.split_once(':').unwrap();
            match l.cmp(r) {
                Ordering::Less => 0,
                Ordering::Equal => 1,
                Ordering::Greater => 3,
            }
        })
        .sum()
}

pub fn two_sort(arr: &[&str]) -> String {
    let mut res = arr.iter().min().expect("is_empty").to_string();
    for i in (1..res.len()).rev() {
        res.insert_str(i, "***")
    }
    res
}

pub fn get_average(marks: &[i32]) -> i32 {
    // marks.iter().fold(0, |ac, el| ac + el) / (marks.len() as i32)
    marks.iter().sum::<i32>() / marks.len() as i32
}
pub fn next_id(ids: &[usize]) -> usize {
    let mut ids = ids.to_vec();
    ids.sort();
    for i in 0..ids.len() {
        if !ids.contains(&i) {
            return i;
        }
    }
    ids.len()
}
pub fn square_sum(vec: Vec<i32>) -> i32 {
    vec.iter().map(|x| x.pow(2)).sum()
}
pub fn maps(values: &[i32]) -> Vec<i32> {
    values.iter().map(|x| x * 2).collect()
}
