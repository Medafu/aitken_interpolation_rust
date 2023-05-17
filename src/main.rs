use std::collections::HashMap;
use std::time::{Instant};

trait Rounding {
    fn round_to_num(self, num: i32) -> Self;
}

impl Rounding for f64 {
    fn round_to_num(self, num: i32) -> Self {
        let round = 10.0_f64.powi(num);
        (self * round).round() / round
    }
}
fn main() {
    let x: f64 = 2.6;
    let begin: u8 = 1;
    let end: u8 = 8;
    let t_start = Instant::now();
    let value = aitken_inter(x, begin, end);
    let duration = t_start.elapsed();
    match value {
        Some(i) => println!("The value of function at point {} is: {}", x, i),
        None => println!("The value was not found")
    }
    println!("Method execution time: {:?}", duration.as_secs_f64());
}

fn aitken_inter(x: f64, begin: u8, end: u8) -> Option<f64> {
    let mut dict: HashMap<u8, f64> = HashMap::new();
    for i in begin..=end {
        let x1 = calc_x(i as f64);
        dict.insert(i, x1); 
    }

    for i in begin+1..end {
        let first: f64 = calc_p(x, begin, i, &dict);
        let second: f64 = calc_p(x, begin, i + 1, &dict);

        println!("P {},{}: {}", begin - 1, i - 1, first);
        println!("P {},{}: {}\n", begin - 1, i, second);

        println!("{}", second - first);
        if second - first < 0.00001 {
            return Some(first); 
        }
    } 
    return None;
}

fn calc_x(x: f64) -> f64 {
    (17.0 / (8.0 * x.powi(2) + 42.0).powi(2)).round_to_num(5)
}

fn deter(x1: f64, x2: f64, x3: f64, x4: f64) -> f64 {
    ((x1 * x4) - (x2 * x3)).round_to_num(5)
}

fn calc_p(x: f64, x1: u8, x2: u8, dict: &HashMap<u8, f64>) -> f64 {
    if x2 - x1 == 1 {
        let mx1 = dict.get(&x1).expect("error unwrapping value from dict");
        let mx2 = dict.get(&x2).expect("error unwrapping value from dict");
        ((1.0 / (x2 - x1) as f64) * deter(*mx1, x1 as f64 - x, *mx2, x2 as f64 - x)).round_to_num(5)
    } else {
        let p1 = calc_p(x, x1, x2 - 1, dict);
        let p2 = calc_p(x, x1 + 1, x2, dict);
        (1.0 / (x2 - x1) as f64) * deter(p1, x1 as f64 - x, p2, x2 as f64 - x)
    } 
}