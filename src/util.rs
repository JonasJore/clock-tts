pub fn get_first_digit(digit: i32) -> i32 {
    let mut copy: i32 = digit;
    if copy < 10 {
        return copy;
    }
    loop {
        if copy < 9 {
            break;
        }
        copy /= 10;
    }
    copy
}

#[test]
fn test_get_first_digit() {
    assert_eq!(get_first_digit(100), 1);
    assert_eq!(get_first_digit(23), 2);
    assert_eq!(get_first_digit(45233), 4);
    assert_eq!(get_first_digit(5), 5);
}