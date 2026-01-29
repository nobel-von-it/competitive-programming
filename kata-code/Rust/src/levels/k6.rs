use std::collections::HashMap;

use itertools::Itertools;

pub fn mult_of_3_or_5(n: i32) -> i32 {
    (0..n).filter(|x| x % 3 == 0 || x % 5 == 0).sum()
}
pub fn spin_words(s: &str) -> String {
    s.split_whitespace()
        .map(|w| {
            if w.len() > 4 {
                w.chars().rev().collect()
            } else {
                w.to_string()
            }
        })
        .collect::<Vec<String>>()
        .join(" ")
}
pub fn count_bits(n: i64) -> u32 {
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
pub fn find_odd(ns: &[i32]) -> i32 {
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

fn persistence(n: u64) -> u64 {
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

fn alphabet_position(text: &str) -> String {
    text.chars()
        .filter(|c| c.is_alphabetic())
        .map(|c| (c.to_digit(36).unwrap() - 9).to_string())
        .collect::<Vec<String>>()
        .join(" ")
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

fn is_pangram(s: &str) -> bool {
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
fn decode_morse(encoded: &str) -> String {
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
fn is_prime(n: i64) -> bool {
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

#[cfg(test)]
mod test {

    use crate::levels::k6::high;

    use super::{to_camel_case, unique_in_order, wave_sort};

    fn is_wave_sorted(xs: &[i32]) -> bool {
        if xs.len() < 2 {
            return true;
        }
        for i in (1..xs.len() - 1).step_by(2) {
            if xs[i - 1] < xs[i] || xs[i] > xs[i + 1] {
                return false;
            }
        }
        true
    }

    #[test]
    fn high_test() {
        let t1 = high("man i need a taxi up to ubud");
        println!("{t1}");
        let t2 = high("aa b");
        println!("{t2}");
    }

    #[test]
    fn wave_sort_test() {
        let mut v = vec![1, 2, 34, 4, 5, 5, 5, 65, 6, 65, 5454, 4];
        wave_sort(v.as_mut());
        assert!(is_wave_sorted(&v))
    }

    #[test]
    fn to_camel_case_test() {
        let t1 = to_camel_case("The_Stealth-Warrior");
        println!("{t1}");
        let t2 = to_camel_case("thE-steAlTh-warRioR");
        println!("{t2}");
        let t3 = to_camel_case("");
        println!("{t3}");
        let t4 = to_camel_case("A-B_C");
        println!("{t4}");
    }

    #[test]
    fn unique_in_order_test() {
        let t1 = unique_in_order("AAAABBBBCCC".chars());
        println!("{t1:?}");
        let t2 = unique_in_order(vec![1, 2, 2, 3, 4]);
        println!("{t2:?}");
    }
}
