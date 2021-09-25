use std::collections::HashMap;
use crate::util;
use crate::formatters;

struct Clock<'a> {
    hours: &'a str,
    minutes: &'a str,
}

pub fn clock_time_algorithm(input_clock_time: &str) -> String {
    let hours_map: HashMap<&str, &str> = [
        ("01", "one"),
        ("02", "two"),
        ("03", "three"),
        ("04", "four"),
        ("05", "five"),
        ("06", "six"),
        ("07", "seven"),
        ("08", "eight"),
        ("09", "nine"),
        ("10", "ten"),
        ("11", "eleven"),
        ("12", "twelve"),
        ("00", "twelve"),
    ]
    .iter()
    .cloned()
    .collect();

    let minutes_map: HashMap<&str, &str> = [
        ("00", "zero"),
        ("01", "one"),
        ("02", "two"),
        ("03", "three"),
        ("04", "four"),
        ("05", "five"),
        ("06", "six"),
        ("07", "seven"),
        ("08", "eight"),
        ("09", "nine"),
        ("10", "ten"),
        ("11", "eleven"),
        ("12", "twelve"),
        ("13", "thirteen"),
        ("14", "fourteen"),
        ("15", "fifteen"),
        ("16", "sixteen"),
        ("17", "seventeen"),
        ("18", "eighteen"),
        ("19", "nineteen"),
    ]
    .iter()
    .cloned()
    .collect();

    let minute_tens_map: HashMap<&str, &str> = [
        ("1", "ten"),
        ("2", "twenty"),
        ("3", "thirty"),
        ("4", "forty"),
        ("5", "fifty"),
    ]
    .iter()
    .cloned()
    .collect();

    let input_split: Vec<&str> = input_clock_time.split(':').collect();

    let clock = Clock {
        hours: &input_split[0],
        minutes: &input_split[1],
    };

    let hours_as_int = clock.hours.parse::<i32>().unwrap();

    let parsed_hours = match hours_as_int {
        hour if hour >= 12 => Some(hours_map[&formatters::hours_bigger_than_twelve_formatter(hour).as_str()]),
        hour if hour == 0 => Some(hours_map[(hour + 12).to_string().as_str()]),
        hour if hour < 10 => Some(hours_map[&format!("0{}", hour).to_string().as_str()]),
        hour => Some(hours_map[&format!("{}", hour).to_string().as_str()]),
    };

    let minutes_as_int = clock.minutes.parse::<i32>().unwrap();
    let last_digit = minutes_as_int % 10;
    let first_digit = util::get_first_digit(minutes_as_int);
    let mut parsed_minute_tens = "";

    if first_digit > 1 && minutes_as_int > 9 {
        parsed_minute_tens = minute_tens_map[format!("{}", first_digit).as_str()];
    }

    let mut parsed_minute_;
    parsed_minute_ = minutes_map[format!("0{}", last_digit).as_str()];

    let am_pm = if hours_as_int < 12 { "AM" } else { "PM" };

    // driver-code
    let mut time_string = String::new();

    if minutes_as_int == 0 || parsed_minute_ == "zero" {
        if parsed_minute_tens != "" {
            time_string = format!(
                "It's {} {} {}",
                parsed_hours.unwrap(),
                parsed_minute_tens,
                am_pm
            );
        } else {
            println!("parsed hours: {:?}", parsed_hours);
            time_string = format!("It's {} {}", parsed_hours.unwrap(), am_pm);
        }
    } else if parsed_minute_tens != "" && minutes_as_int > 9 {
        time_string = format!(
            "It's {} {} {} {}",
            parsed_hours.unwrap(),
            parsed_minute_tens,
            parsed_minute_,
            am_pm
        );
    } else if minutes_as_int < 10 {
        time_string = format!(
            "It's {} oh {} {}",
            parsed_hours.unwrap(),
            parsed_minute_,
            am_pm
        );
    }

    match time_string.is_empty() {
        true => panic!("time string never got its value."),
        false => println!("{}", time_string),
    };
    
    return time_string;
}

#[test]
fn test_10_am() {
    clock_time_algorithm("10:00");
}
