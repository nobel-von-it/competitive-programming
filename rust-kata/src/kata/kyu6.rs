use itertools::Itertools;
use std::collections::{BTreeSet, HashMap, HashSet};

pub fn mult_of_3_or_5(n: i32) -> i32 {
    (0..n).filter(|x| x % 3 == 0 || x % 5 == 0).sum()
}

pub fn spin_words(words: &str) -> String {
    words
        .split(' ')
        .map(|e| {
            if e.len() > 4 {
                e.chars().rev().join("")
            } else {
                e.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

pub fn find_odd_set(arr: &[i32]) -> i32 {
    let mut set = BTreeSet::new();
    for i in arr {
        if !set.insert(*i) {
            set.remove(i);
        }
    }
    set.pop_first().unwrap()
}
pub fn find_odd_xor(arr: &[i32]) -> i32 {
    //this is very fast solution
    arr.iter().fold(0i32, |a, v| {
        println!("{}^{} = {}", a, v, a ^ v);
        a ^ v
    })
}
pub fn find_odd_old(ns: &[i32]) -> i32 {
    let mut res = std::collections::HashMap::<i32, i32>::new();
    // [10]
    for x in ns {
        if let Some(n) = res.get_mut(x) {
            *n += 1;
        } else {
            res.insert(*x, 1);
        }
    }
    *res.iter().find(|(_, &x)| x % 2 != 0).unwrap().0
}

pub fn count_bits(n: i64) -> u32 {
    format!("{:b}", n).matches('1').count() as u32
}
pub fn count_bits_old(n: i64) -> u32 {
    let mut res = 0;
    let mut n = n;
    while n != 0 {
        if n % 2 == 1 {
            res += 1;
        }
        n /= 2;
    }
    res
}

pub fn array_diff<T: PartialEq>(a: Vec<T>, b: Vec<T>) -> Vec<T> {
    let mut res = vec![];
    for x in a {
        if !b.contains(&x) {
            res.push(x)
        }
    }
    res
}

pub fn digital_root(n: i64) -> i64 {
    let mut n = n;
    while n >= 10 {
        n = n
            .to_string()
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .sum::<u32>() as i64;
    }
    n
}

pub fn find_outlier(values: &[i32]) -> i32 {
    let odds = values.iter().filter(|&x| x % 2 == 0).collect::<Vec<&i32>>();
    let evens = values.iter().filter(|&x| x % 2 != 0).collect::<Vec<&i32>>();
    if odds.len() == 1 {
        *odds[0]
    } else if evens.len() == 1 {
        *evens[0]
    } else {
        0
    }
}

pub fn count_duplicates(text: &str) -> u32 {
    let mut set = HashSet::new();
    let mut res = 0;
    for i in text.to_lowercase().split("") {
        if !set.insert(i) {
            res += 1
        }
    }
    res
}

pub fn count_duplicates_old(text: &str) -> u32 {
    use std::collections::HashMap;
    let mut res = HashMap::new();
    for s in text.chars() {
        if let Some(n) = res.get_mut(&s) {
            *n = 2;
        } else {
            res.insert(s, 1);
        }
    }
    let mut num = 0;
    for (_, n) in res {
        if n == 2 {
            num += 1;
        }
    }
    num
}

pub fn count_duplicates_hashmap(text: &str) -> u32 {
    let mut map: HashMap<char, u16> = HashMap::new();
    let mut res: u32 = 0;
    for i in text.trim().to_lowercase().chars() {
        let v = map.entry(i).or_insert(0);
        *v += 1;
    }
    for (_, v) in map {
        if v > 1 {
            res += 1
        }
    }
    res
}
pub fn count_duplicates_one_line(text: &str) -> u32 {
    text.to_lowercase()
        .chars()
        .counts()
        .values()
        .filter(|&&i| i > 1)
        .count() as u32
}

fn duplicate_encode(text: &str) -> String {
    use std::collections::HashMap;
    let mut char_count: HashMap<char, u32> = HashMap::new();
    for c in text.to_lowercase().chars() {
        let e = char_count.entry(c).or_default();
        *e += 1;
    }
    text.to_lowercase()
        .chars()
        .map(|c| {
            if *char_count.get(&c).unwrap() > 1 {
                "(".to_string()
            } else {
                ")".to_string()
            }
        })
        .collect()
}

pub fn is_valid_walk(walk: &[char]) -> bool {
    if walk.len() != 10 {
        return false;
    };
    let mut x = 0;
    let mut y = 0;
    for i in walk {
        match i {
            'n' => y += 1,
            's' => y -= 1,
            'w' => x -= 1,
            'e' => x += 1,
            _ => {}
        }
    }
    x == 0 && y == 0
}
pub fn alphabet_position_old(text: &str) -> String {
    let alph = String::from("abcdefghijklmnopqrstuvwxyz");
    let mut result = vec![];
    for i in text.to_lowercase().chars() {
        if let Some(el) = alph.find(i) {
            result.push((el + 1).to_string())
        }
    }
    result.join(" ")
}

fn alphabet_position(text: &str) -> String {
    text.chars()
        .filter(|c| c.is_alphabetic())
        .map(|c| (c.to_digit(36).unwrap() - 9).to_string())
        .collect::<Vec<String>>()
        .join(" ")
}

pub fn persistence(num: u64) -> u64 {
    let mut num = num;
    let mut count = 0;
    while num > 9 {
        num = num
            .to_string()
            .chars()
            .map(|c| c.to_string().parse::<u64>().unwrap())
            .product();
        count += 1;
    }
    count
}

fn persistence_old(n: u64) -> u64 {
    let mut res = 0;
    let mut n = n;
    while n >= 10 {
        n = n
            .to_string()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u64)
            .product();
        res += 1;
    }
    res
}

pub fn is_pangram(s: &str) -> bool {
    s.to_lowercase()
        .chars()
        .filter(|c| c.is_alphabetic())
        .collect::<HashSet<char>>()
        .len()
        == 26
}

fn is_pangram_old(s: &str) -> bool {
    // use std::collections::HashMap;
    // let mut map = HashMap::new();
    // s.to_lowercase().chars().for_each(|c| {
    //     if c.is_alphabetic() {
    //         if let Some(cr) = map.get_mut(&c) {
    //             *cr += 1;
    //         } else {
    //             map.insert(c, 1);
    //         }
    //     }
    // });
    // map.len() == 26
    use std::collections::HashSet;
    s.to_lowercase()
        .chars()
        .filter(|c| c.is_alphabetic())
        .collect::<HashSet<_>>()
        .len()
        == 26
}

pub fn decode_morse(encoded: &str) -> String {
    let _morse_code: HashMap<&str, String> = HashMap::new();
    let mut result = String::new();
    for word in encoded.split("  ") {
        for ch in word.split_whitespace() {
            if let Some(c) = _morse_code.get(ch) {
                result.push_str(c)
            }
        }
        result.push(' ');
    }
    result.trim().to_string()
}

fn decode_morse_old(encoded: &str) -> String {
    let MORSE_CODE: HashMap<String, String> = HashMap::new();
    encoded
        .split("  ")
        .map(|w| {
            w.split(" ")
                .map(|c| MORSE_CODE.get(c).unwrap_or(&c.to_string()).clone())
                .collect::<Vec<String>>()
                .join("")
        })
        .collect::<Vec<String>>()
        .join("")
}

fn remove_parentheses(s: &str) -> String {
    // a (b) c
    let mut stack = Vec::new();
    let mut res = String::new();
    for c in s.chars() {
        if c == '(' {
            stack.push(c);
        } else if c == ')' {
            stack.pop();
        } else if stack.is_empty() {
            res.push(c);
        }
    }
    res
}

fn wave_sort(xs: &mut [i32]) {
    // For example array [1, 2, 34, 4, 5, 5, 5, 65, 6, 65, 5454, 4]
    // Sorted: [1, 2, 4, 4, 5, 5, 5, 6, 65, 65, 34, 5454]
    // Result: [2, 1, 4, 4, 5, 5, 6, 5, 65, 65, 5454, 34]

    if xs.len() < 2 {
        return;
    }
    xs.sort();
    for i in (0..xs.len() - 1).step_by(2) {
        xs.swap(i, i + 1);
    }
}

fn find_even_index_bruh(a: &[i32]) -> Option<usize> {
    for n in 0..a.len() {
        let left = a[..n].iter().sum::<i32>();
        let right = a[n + 1..].iter().sum::<i32>();
        println!("{left} {right}");
        if left == right {
            return Some(n);
        }
    }
    None
}

fn find_even_index(a: &[i32]) -> Option<usize> {
    let (mut r, mut l) = (a.iter().sum::<i32>(), 0);
    for (i, n) in a.iter().enumerate() {
        r -= n;
        if r == l {
            return Some(i);
        }
        l += n;
    }
    None
}

fn find_uniq(a: &[f64]) -> f64 {
    let first = *a.first().unwrap();
    let last = *a.last().unwrap();
    for n in a.iter().skip(1).take(a.len() - 1) {
        let n = *n;
        if n == first && n != last {
            return last;
        } else if n == last && n != first {
            return first;
        } else if n != first && n != last {
            return n;
        }
    }
    unreachable!()
}
fn sort_array(a: &[i32]) -> Vec<i32> {
    let mut a = a.to_vec();
    for i in 0..a.len() {
        for j in i + 1..a.len() {
            if a[i] > a[j] && a[j] % 2 != 0 && a[i] % 2 != 0 {
                a.swap(i, j);
            }
        }
    }
    a
}
fn high(s: &str) -> &str {
    s.split_ascii_whitespace()
        .rev()
        .max_by_key(|w| w.chars().map(|c| c as u16 - 96).sum::<u16>())
        .unwrap_or("")
}
fn break_camels(s: &str) -> String {
    s.chars()
        .map(|c| {
            if c.is_uppercase() {
                format!(" {}", c)
            } else {
                c.to_string()
            }
        })
        .collect()
}
fn valid_braces(s: &str) -> bool {
    let mut stack = Vec::new();
    for c in s.chars() {
        match c {
            '(' => stack.push(c),
            ')' => {
                if stack.pop() != Some('(') {
                    return false;
                }
            }
            '[' => stack.push(c),
            ']' => {
                if stack.pop() != Some('[') {
                    return false;
                }
            }
            '{' => stack.push(c),
            '}' => {
                if stack.pop() != Some('{') {
                    return false;
                }
            }
            _ => (),
        }
    }
    stack.is_empty()
}
fn is_prime_old(n: i64) -> bool {
    // best performance
    match n {
        1 | 0 => false,
        _ => {
            if n < 0 {
                false // negative numbers are not prime
            } else {
                let mut i = 2;
                while i * i <= n {
                    if n % i == 0 {
                        return false;
                    }
                    i += 1;
                }
                true
            }
        }
    }
}
fn expanded_from(n: u64) -> String {
    //     12 --> "10 + 2"
    //    45 --> "40 + 5"
    // 70304 --> "70000 + 300 + 4"

    let mut v = Vec::new();
    let mut n = n;
    let mut ln = 0;
    while n > 0 {
        let cur = (n % 10) * 10u64.pow(ln);
        if cur > 0 {
            v.push(cur);
        }
        ln += 1;
        n /= 10;
    }
    v.reverse();
    v.iter().map(|n| n.to_string()).join(" + ")
}

fn to_camel_case(text: &str) -> String {
    if text.is_empty() || text.len() == 1 {
        return text.to_string();
    }
    let first_is_upper = text[..1].to_ascii_uppercase() == text[..1];
    text.to_lowercase()
        .split(['-', '_'])
        .enumerate()
        .map(|(i, w)| match i {
            0 => {
                if first_is_upper {
                    w[..1].to_uppercase() + &w[1..]
                } else {
                    w.to_string()
                }
            }
            _ => w[..1].to_uppercase() + &w[1..],
        })
        .collect::<Vec<String>>()
        .join("")
}

fn tribonacci(signature: &[f64; 3], n: usize) -> Vec<f64> {
    match n {
        0 => vec![],
        1 => vec![signature[0]],
        2 => vec![signature[0], signature[1]],
        _ => {
            let mut v = signature.to_vec();
            let mut i = 0;
            while v.len() < n {
                let next = v[i] + v[i + 1] + v[i + 2];
                v.push(next);
                i += 1;
            }
            v
        }
    }
}

fn unique_in_order<T>(sequence: T) -> Vec<T::Item>
where
    T: std::iter::IntoIterator,
    T::Item: std::cmp::PartialEq + std::fmt::Debug,
{
    // let mut result = Vec::new();
    // let mut prev: Option<&T::Item> = None;
    //
    // for item in sequence {
    //     if prev.as_ref() = Some(&item) {
    //         result.push(item);
    //         prev = Some(&item);
    //     }
    // }
    //
    // result
    //
    // let mut result = Vec::new();
    // let mut iter = sequence.into_iter();
    //
    // if let Some(first) = iter.next() {
    //     result.push(first);
    //
    //     for item in iter {
    //         if item != *result.last().unwrap() {
    //             result.push(item);
    //         }
    //     }
    // }
    //
    // result

    let mut v = Vec::from_iter(sequence);
    v.dedup();
    v
}

pub fn split_strings(s: &str) -> Vec<String> {
    s.chars()
        .chunks(2)
        .into_iter()
        .map(|c| c.pad_using(2, |_| '_').collect())
        .collect()
}
pub fn find_number(from: u32, to: u32, res: &str) -> Vec<u32> {
    (from..=to)
        .filter(|x| res.contains(&x.to_string()))
        .collect_vec()
}
pub fn compute_depth(n: u16) -> u8 {
    use std::collections::HashSet;

    let original = n;
    let mut n;

    let mut depth = 0;
    let mut numbers = HashSet::new();
    while numbers.len() < 10 {
        depth += 1;
        n = original * depth;
        n.to_string()
            .chars()
            .map(|x| x.to_digit(10).unwrap())
            .for_each(|x| {
                numbers.insert(x);
            });
    }
    depth as u8 - 1
}

pub fn prime_reduction(a: u32, b: u32) -> usize {
    (a..b).filter(|x| is_prime(*x) && prime_end(*x)).count()
}

fn prime_end(n: u32) -> bool {
    let mut n = n;
    let mut seen = std::collections::HashSet::new();

    while n != 1 && !seen.contains(&n) {
        seen.insert(n);
        n = sum_squares(n);
    }

    n == 1
}

fn sum_squares(n: u32) -> u32 {
    let mut n = n;
    let mut res = 0;

    while n > 0 {
        let cur = n % 10;
        res += cur * cur;
        n /= 10;
    }
    res
}

fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }
    let sqrt = (n as f64).sqrt() as u32;
    for i in (3..=sqrt).step_by(2) {
        if n % i == 0 {
            return false;
        }
    }
    true
}
