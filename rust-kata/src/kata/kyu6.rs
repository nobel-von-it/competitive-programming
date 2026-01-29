use itertools::Itertools;
use std::collections::{BTreeSet, HashMap, HashSet};

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
pub fn count_bits(n: i64) -> u32 {
    format!("{:b}", n).matches('1').count() as u32
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
pub fn alphabet_position(text: &str) -> String {
    let alph = String::from("abcdefghijklmnopqrstuvwxyz");
    let mut result = vec![];
    for i in text.to_lowercase().chars() {
        if let Some(el) = alph.find(i) {
            result.push((el + 1).to_string())
        }
    }
    result.join(" ")
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

pub fn is_pangram(s: &str) -> bool {
    s.to_lowercase()
        .chars()
        .filter(|c| c.is_alphabetic())
        .collect::<HashSet<char>>()
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
