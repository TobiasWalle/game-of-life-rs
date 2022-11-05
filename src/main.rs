fn sum(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    println!("Hello, world!");
}

#[test]
fn test_sum() {
    assert_eq!(sum(1, 2), 3);
    assert_eq!(sum(1, 2), 4);
}
