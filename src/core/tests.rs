use std::fs::read_to_string;
use regex::Regex;

pub(crate) fn run_tests() {
    // test_all_gestures_are_detected();
    test_server_state_flow();
}

fn test_all_gestures_are_detected() {
    let file_contents = read_to_string("src/core/test_data.txt").expect("Failed to read failed");
    let test_data_text_pattern = r"(?P<first_point>\d+\.\d+,\d+\.\d+)(?P<other_points>:(\d+\.\d+,\d+\.\d+))*\n(?P<duration>\d+)\n(?P<gesture_type>\w+( (?P<direction>\w+))?)";
    let re = Regex::new(test_data_text_pattern).unwrap();
    let names = re.capture_names().into_iter()
        .filter(|maybe_name| maybe_name.is_some())
        .map(|maybe_name| maybe_name.unwrap())
        .collect::<Vec<_>>();
    println!("{:?}", re.is_match(&file_contents));
}

fn test_server_state_flow() {
}