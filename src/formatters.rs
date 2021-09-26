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

//todo: this function should take in a struct instead
pub fn oh_clock_formatter(
    parsed_hours: Option<&str>,
    parsed_minute_map_lookup: &str,
    am_pm: &str,
) -> String {
    return format!(
        "It's {} oh {} {}",
        parsed_hours.unwrap(),
        parsed_minute_map_lookup,
        am_pm
    );
}

#[test]
fn test_hour_formatter() {
    assert_eq!(hours_bigger_than_twelve_formatter(13), "01");
    assert_eq!(hours_bigger_than_twelve_formatter(23), "11");
    assert_eq!(hours_bigger_than_twelve_formatter(0), "0");
}

#[test]
fn oh_clock_formatter_test() {
    let parsed_hours = Some("four");
    let parsed_minutes = "five";
    let under_test = oh_clock_formatter(parsed_hours, parsed_minutes, "PM");
    assert_eq!(under_test, "It's four oh five PM");
}
