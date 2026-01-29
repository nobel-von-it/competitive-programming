use itertools::Itertools;
use std::collections::HashMap;
use std::str::FromStr;

pub fn get_count(s: &str) -> usize {
    s.chars().filter(|c| "aeiou".contains(*c)).count()
}
pub fn disemvowel(s: &str) -> String {
    s.chars().filter(|&c| !"aeiouAEIOU".contains(c)).collect()
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
    regex::Regex::new(r"1*0*").unwrap().find_iter(str).all(|s| {
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
fn reverse_words(s: &str) -> String {
    s.split_whitespace()
        .map(|w| w.chars().rev().collect::<String>())
        .join(" ")
}

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

pub fn square_digits_old(n: u64) -> u64 {
    u64::from_str(
        &n.to_string()
            .chars()
            .map(|e| u32::pow(e.to_digit(10).unwrap(), 2))
            .join(""),
    )
    .unwrap()
}
fn square_digits(n: u64) -> u64 {
    n.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap().pow(2).to_string())
        .collect::<String>()
        .parse::<u64>()
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
pub fn find_short_old(s: &str) -> u32 {
    let mut min: u32 = 10000;
    for word in s.split_whitespace() {
        min = min.min(word.len() as u32)
    }
    min
}

fn find_short(s: &str) -> u32 {
    s.split_ascii_whitespace()
        .min_by_key(|w| w.len())
        .unwrap()
        .len() as u32
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
pub fn descending_order_old(x: u64) -> u64 {
    let mut ch: Vec<String> = x.to_string().chars().map(|c| c.to_string()).collect();
    ch.sort();
    ch.reverse();
    ch.join("").parse::<u64>().unwrap()
}

fn descending_order(x: u64) -> u64 {
    let mut res = x.to_string().chars().collect::<Vec<char>>();
    res.sort_by(|a, b| b.cmp(a));
    String::from_iter(res).parse().unwrap()
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
