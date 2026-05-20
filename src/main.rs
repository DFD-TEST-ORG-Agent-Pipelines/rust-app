fn main() {
    println!("Rust Sample App");
    let result = calculate(10, 20);
    println!("10 + 20 = {}", result);
}
fn calculate(a: i32, b: i32) -> i32 { a + b }
