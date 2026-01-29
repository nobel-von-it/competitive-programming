use std::panic::resume_unwind;

use itertools::Itertools;
use regex::Regex;

pub fn get_count(s: &str) -> usize {
    s.chars().filter(|c| "aeiou".contains(*c)).count()
}
pub fn disemvowel(s: &str) -> String {
    s.chars().filter(|&c| !"aeiouAEIOU".contains(c)).collect()
}
fn square_digits(n: u64) -> u64 {
    n.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap().pow(2).to_string())
        .collect::<String>()
        .parse::<u64>()
        .unwrap()
}
fn fix_string_case(s: &str) -> String {
    let lowers = s.chars().filter(|c| c.is_lowercase()).count();
    if lowers >= s.len() / 2 {
        s.to_lowercase()
    } else {
        s.to_uppercase()
    }
}
fn get_middle(s: &str) -> &str {
    let lm = s.len() / 2;
    match s.len() % 2 {
        0 => &s[lm - 1..=lm],
        _ => &s[lm..=lm],
    }
}

fn descending_order(x: u64) -> u64 {
    let mut res = x.to_string().chars().collect::<Vec<char>>();
    res.sort_by(|a, b| b.cmp(a));
    String::from_iter(res).parse().unwrap()
}

fn accum(s: &str) -> String {
    use std::fmt::Write;
    s.to_lowercase()
        .chars()
        .enumerate()
        .fold(String::new(), |mut output, (i, c)| {
            write!(
                output,
                "{}{}",
                c.to_ascii_uppercase(),
                c.to_string().repeat(i)
            )
            .expect("write error");
            if s.len() - 1 != i {
                output.push('-');
            }
            output
        })
}

fn sum_two_smallest_numbers(numbers: &[u32]) -> u32 {
    let mut ns = numbers.to_vec();
    ns.sort();
    ns[0] + ns[1]
}

fn is_square(n: i64) -> bool {
    (n as f64).sqrt().round() == (n as f64).sqrt()
}

fn reverse_bits(n: u32) -> u32 {
    let mut n = n;
    let mut res = 0;
    while n > 0 {
        res <<= 1;
        res |= n & 1;
        n >>= 1;
    }
    res
}

fn fire_fight(s: &str) -> String {
    s.replace("Fire", "~~")
}

fn two_are_positive(a: i32, b: i32, c: i32) -> bool {
    !(a <= 0 && b <= 0 && c <= 0)
}

// Every consecutive sequence of ones is immediately followed by an equal-length consecutive sequence of zeroes, and the number of ones is equal to the number of zeroes.
//
// A leading zero always results in a False outcome.
// dotest("11101010010010", false);
// dotest("1001", false);
fn same_length(s: &str) -> bool {
    if s.starts_with("0") {
        return false;
    }
    let mut chs = s.chars().peekable();
    while let Some(&c) = chs.peek() {
        if c == '1' {
            let mut ones = 0;
            while let Some('1') = chs.peek() {
                ones += 1;
                chs.next();
            }
            let mut zeros = 0;
            while let Some('0') = chs.peek() {
                zeros += 1;
                chs.next();
            }
            if ones != zeros {
                return false;
            }
        } else {
            return false;
        }
    }
    true
}

fn same_length_regex(str: &str) -> bool {
    Regex::new(r"1*0*").unwrap().find_iter(str).all(|s| {
        0 == s
            .as_str()
            .chars()
            .fold(0i32, |a, c| a + if c == '1' { 1 } else { -1 })
    })
}

fn change(s: &str) -> String {
    let mut res = vec!['0'; 26];
    s.to_lowercase()
        .chars()
        .filter(|c| c.is_ascii_alphabetic())
        .for_each(|c| res[c as usize - 'a' as usize] = '1');
    String::from_iter(res)
}

fn recursion101(a: usize, b: usize) -> (usize, usize) {
    if a == 0 || b == 0 {
        (a, b)
    } else if a >= 2 * b {
        recursion101(a - 2 * b, b)
    } else if b >= 2 * a {
        recursion101(a, b - 2 * a)
    } else {
        (a, b)
    }
}

fn consecutive_letters(s: &str) -> bool {
    use itertools::Itertools;
    use std::collections::HashSet;
    let mut lets = HashSet::new();
    for c in s.chars() {
        if !lets.insert((c as usize) - ('a' as usize)) {
            return false;
        }
    }
    lets.iter()
        .sorted()
        .tuple_windows()
        .all(|(a, b)| b - a == 1)
}

fn consecutive_letters_easy(s: &str) -> bool {
    use itertools::Itertools;
    "abcdefghijklmnopqrstuvwxyz".contains(&s.chars().sorted().join(""))
}

fn expected_value(r: i64, c: i64) -> i64 {
    if r == c {
        return 0;
    }
    let r = r as i128;
    let c = c as i128;

    let rc = r * (r + 1) / 2 * (c + 1);
    let cc = c * (c + 1) / 2 * (r + 1);
    let n = rc - cc;

    let den = (r + 1) * (c + 1);

    (n / den) as i64
}

fn divisors(n: u32) -> Result<Vec<u32>, String> {
    let mut res = Vec::new();
    for i in 2..(n as f64).sqrt() as u32 + 1 {
        if n % i == 0 {
            res.push(i);
            if n / i != i {
                res.push(n / i);
            }
        }
    }
    if res.is_empty() {
        Err(format!("{} is prime", n))
    } else {
        res.sort();
        Ok(res)
    }
}

fn stanton_measure(arr: &[u32]) -> u32 {
    let n = arr.iter().filter(|&x| *x == 1).count();
    arr.iter().filter(|&x| *x == n as u32).count() as u32
}

fn dobleton(n: u32) -> u32 {
    // if it contains exactly two distinct digits
    for i in n + 1.. {
        if i.to_string().chars().unique().count() == 2 {
            return i;
        }
    }
    0
}

fn fit_in(a: u32, b: u32, m: u32, n: u32) -> bool {
    a.max(b) <= m.min(n) && a + b <= m.max(n)
}
fn add_binary(a: u64, b: u64) -> String {
    format!("{:b}", a + b)
}
fn row_sum_odd_numbers(n: u32) -> u32 {
    n.pow(3)
}
fn reverse_words(s: &str) -> String {
    s.split_whitespace()
        .map(|w| w.chars().rev().collect::<String>())
        .join(" ")
}
fn find_short(s: &str) -> u32 {
    s.split_ascii_whitespace()
        .min_by_key(|w| w.len())
        .unwrap()
        .len() as u32
}

#[cfg(test)]
mod test {
    use crate::levels::k7::accum;

    use super::{expected_value, get_middle};

    #[test]
    fn get_middle_test() {
        let t1 = get_middle("test");
        let t2 = get_middle("a");
    }
    #[test]
    fn repeat_test() {
        println!("H{}", "h".to_string().repeat(0))
    }
    #[test]
    fn accum_test() {
        let t1 = String::from("ZpglnRxqenU");
        println!("{} -> {}", &t1, accum(&t1))
    }

    #[test]
    fn expected_value_test() {
        let t1 = expected_value(0, 0);
        assert_eq!(t1, 0);
        let t2 = expected_value(11, 5);
        assert_eq!(t2, 3);
        let t3 = expected_value(10, 10);
        assert_eq!(t3, 0);

        let t4 = expected_value(i64::MAX, i64::MAX);
        assert_eq!(t4, 0);
    }
}
