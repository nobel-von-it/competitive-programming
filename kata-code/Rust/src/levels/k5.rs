fn is_prime(n: i64) -> bool {
    for i in 2..((n as f64).sqrt() as i64 - 1) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn prime_factors(n: i64) -> String {
    let dels = (2..((n as f64).sqrt() as i64 - 1))
        .filter(|x| n % x == 0)
        .filter(|&x| is_prime(x))
        .collect::<Vec<i64>>();
    // println!("{dels:#?}");
    String::new()
}

fn move_zeros(a: &[u8]) -> Vec<u8> {
    let mut res = a.to_owned();
    for i in (0..a.len()).rev() {
        if a[i] == 0 {
            res.remove(i);
            res.push(0);
        }
    }
    res
}

#[cfg(test)]
mod test {
    use super::{is_prime, move_zeros, prime_factors};

    #[test]
    fn prime_test() {
        prime_factors(7775460);
    }

    #[test]
    fn move_zeros_test() {
        let res = move_zeros(&[1, 2, 0, 1, 0, 1, 0, 3, 0, 1]);
        println!("{res:#?}");
    }

    // #[test]
    // fn is_prime_test() {
    //     for i in 0..100 {
    //         if is_prime(i) {
    //             println!("{i}")
    //         }
    //     }
    // }
}
