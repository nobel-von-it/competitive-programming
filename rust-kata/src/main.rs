pub mod kata;

// in the kata module:
// ---
// kata is a ready-made code, divided by complexity,
// there are also some tests,
// but I will not write tests for all codes;
// ---
// maybe_kata is where I will write any code before structuring and optimizing.
// ---
fn main() {
    // need switch to --lib
    println!("{:?}", kata::kyu5::solution(0, u64::MAX));
}
