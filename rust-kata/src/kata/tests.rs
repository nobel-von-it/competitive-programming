#[cfg(test)]
mod test {
    use crate::kata;
    use std::time::Instant;
    #[test]
    fn html_test() {
        let now = Instant::now();
        assert_eq!(
            kata::kyu8::html_special_chars("<h2>Hello World</h2>"),
            "&lt;h2&gt;Hello World&lt;/h2&gt;"
        );
        println!("{:?}", now.elapsed())
    }
    #[test]
    fn double_char_test() {
        let now = Instant::now();
        assert_eq!(kata::kyu8::double_char2("hello"), "hheelllloo");
        println!("{:?}", now.elapsed())
    }
    #[test]
    fn difference_test() {
        assert_eq!(
            kata::kyu8::find_difference(&[1, 2, 3], &[4, 5, 6]),
            (6i32 - 120i32).abs()
        )
    }

    #[test]
    fn phone_test() {
        let now = Instant::now();
        assert_eq!(
            kata::kyu8::create_phone_number1(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 0]),
            "(123) 456-7890"
        );
        println!("{:?}", now.elapsed())
    }
    #[test]
    fn avg_and_next_id_tests() {
        assert_eq!(kata::kyu8::get_average(&[2, 2, 2, 2]), 2);
        assert_eq!(kata::kyu8::get_average(&[1, 2, 3, 4, 5, 6, 7, 8, 9]), 5);
        assert_eq!(kata::kyu8::next_id(&[0, 1, 2, 4, 5]), 3);
        assert_eq!(kata::kyu8::next_id(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 11);
        assert_eq!(kata::kyu8::next_id(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 11]), 10);
        assert_eq!(
            kata::kyu8::next_id(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 9, 11]),
            10
        );
        assert_eq!(kata::kyu8::next_id(&[]), 0);
        assert_eq!(kata::kyu8::next_id(&[10]), 0);
    }
    #[test]
    fn two_sum_test() {
        assert_eq!(
            kata::leetcode::two_sum2(vec![1, 2, 3, 4, 55, 100], 155),
            vec![5, 4]
        )
    }
}
