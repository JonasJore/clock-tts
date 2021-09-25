pub fn hours_bigger_than_twelve_formatter(hour: i32) -> String {
  let hours_key: i32 = hour - 12;
  if hours_key < 10 {
      if hours_key < 0 {
          return "0".to_string();
      }
      return format!("0{}", hours_key).to_string();
  }

  return format!("{}", &hours_key).to_string();
}

#[test]
fn test_hour_formatter() {
    assert_eq!(hours_bigger_than_twelve_formatter(13), "01");
    assert_eq!(hours_bigger_than_twelve_formatter(23), "11");
    assert_eq!(hours_bigger_than_twelve_formatter(0), "0");
}