use super::{is_greater_than_100, say_your_name, sum};

#[test]
fn sum_4_with_6() {
    assert_eq!(sum(4, 6), 10);
}

#[test]
fn sum_1_with_2_wrong_example() {
    let result = sum(1, 3);
    assert_eq!(result, 3, "Result is {}, expect 3", result);
}

#[test]
fn say_john() {
    let result = say_your_name("John");
    assert!(result.contains("My name is John"));
}

#[test]
fn message_error_example() {
    let result = say_your_name("John");
    assert!(
        result.contains("Maria"),
        "result not contain name, value = {}",
        result
    );
}

#[test]
#[should_panic(expected = "Value is invalid")]
fn greater_than_100() {
    is_greater_than_100(200);
}
