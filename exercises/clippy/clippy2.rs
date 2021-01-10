// clippy2.rs
// Make me compile! Execute `rustlings hint clippy2` for hints :)

// I AM NOT DONE
// 不清楚这样写是不是符合题目本意

fn main() {
    let mut res = 42;
    let option = Some(12);
    if let Some(12) = option {
        res += 12;
    }
    println!("{}", res);
}
