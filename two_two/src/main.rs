use std::fs;
use std::io::Read;

fn two_two(s: &str) -> u32 {
    let mut res: u32 = 0;
    let a: Vec<u32> = s.chars().map(|c| c.to_digit(10).unwrap()).collect();

    // println!("av : {a:?}");
    let end = if a.len() > 241 { 241 } else { a.len() };

    for len in 1..=end {
        for win in a.windows(len) {
            let mut s: u64 = 0;
            if win[0] == 0 {
                // s = 0;
                continue;
            } else {
                // print!("s : ");
                for k in win {
                    s = s * 10 + *k as u64;
                    // print!("{s}, ");
                }
                // println!("window : {s}");
            }
            // let log_value = (s as f64).log(2.0);
            // if log_value.fract() == 0.0 && log_value <= 800.0 {
            // let p:u64 = s as u64;
            if (s & (s - 1)) == 0{
                println!("{s:b}  =  {s}");
                // println!("selected : {s:?}");
                res += 1;
            }
        }
    }
    res
}

fn main() -> std::io::Result<()> {
    let mut file = fs::File::open("./src/input")?;
    let mut input: String = String::new();
    file.read_to_string(&mut input)?;
    let mut lines = input.lines();

    // for y in (0..800) {
        // println!("{}")
    // }
    //
    //
    let e = "0".repeat(800);
    let e1  = "1".to_string() + &e;

    let n = 115792089237316195423570985008687907853269984665640564039457584007913129639936;

    println!("{}", e1);


    // let t = lines.next().unwrap().parse::<usize>().unwrap();
    // for _ in 0..t {
    //     let s = lines.next().unwrap().trim_end();
    //     // println!("a : {s}");
    //     println!("{} \n", two_two(s));
    // }
    // let num: usize = 2;
    // println!("{}", num.pow(800).to_string());

    Ok(())
}
