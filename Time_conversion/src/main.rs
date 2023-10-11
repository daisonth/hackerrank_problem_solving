// https://www.hackerrank.com/challenges/time-conversion/problem?isFullScreen=true

fn time_conversion(s: &str) -> String {
    let mut _ans: String = String::new();

    match s.chars().nth_back(1).unwrap() {
        'A' => {
            if s.split(":").nth(0).unwrap() == "12" {
                _ans = s.get(0..8).unwrap().replacen("12", "00", 1);
            } else {
                _ans = s.get(0..8).unwrap().to_string();
            }
        }
        'P' => {
            let mut num = s.split(":").nth(0).unwrap().parse::<i32>().unwrap();
            if num != 12 {
                num += 12
            }
            _ans = num.to_string() + s.get(2..8).unwrap();
        }
        _ => {}
    }
    _ans
}

fn main() {
    println!("{}", time_conversion("12:45:54PM"));
}
