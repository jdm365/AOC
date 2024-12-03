use once_cell::sync::Lazy;
use regex::Regex;


static REGEX_MUL: &str = r"mul\((-?\d+),(-?\d+)\)";
static REGEX_CONDITIONAL: &str = r"mul\((-?\d+),(-?\d+)\)|don't()|do()";

static MATCH: Lazy<Regex> = Lazy::new(|| Regex::new(REGEX_MUL).unwrap());
static MATCH_CONDITIONAL: Lazy<Regex> = Lazy::new(|| Regex::new(REGEX_CONDITIONAL).unwrap());

fn get_sum(text: &str) -> usize {
    let mut sum = 0;

    for cap in MATCH.captures_iter(text) {
        let x: usize = cap[1].parse().unwrap();
        let y: usize = cap[2].parse().unwrap();
        sum += x * y;
    }

    sum
}

fn get_sum_conditional(text: &str) -> usize {
    let mut sum = 0;
    let mut do_mul = true;

    for cap in MATCH_CONDITIONAL.captures_iter(text) {
        println!("{:?}", &cap);
        match &cap[0] {
            "do" => do_mul = true,
            "don't" => do_mul = false,
            _ => {
                if do_mul {
                    let x: usize = cap[1].parse().unwrap();
                    let y: usize = cap[2].parse().unwrap();
                    sum += x * y;
                }
            }
        }
    }

    sum
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let sum = get_sum(&input);
    let sum_conditional = get_sum_conditional(&input);
    println!("Sum: {}", sum);
    println!("Sum Conditional: {}", sum_conditional);
}
