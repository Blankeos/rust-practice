// TODO: Fix the compiler error without changing the function signature.
fn current_favorite_color() -> String {
    "blue".to_string()
}

fn main() {
    let answer = current_favorite_color();
    println!("My current favorite color is {answer}");
}

// So what I learned:
// String vs &str
// - &str is "" - it's a 'slice slice' in a 'static' lifetime.
// - String can be created in multiple ways:
//   - String::from().
//   - "".to_string()
//   - "".to_owned()
//   - "".into()
//   - format!()
