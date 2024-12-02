#[inline]
fn is_valid(record: &Vec<u32>) -> bool {
    let mut prev_val: u32 = 0;
    let mut increasing = false;
    let mut order_set = false;

    for (idx, val) in record.iter().enumerate() {
        if idx == 0 {
            prev_val = *val;
            continue;
        }

        if *val == prev_val {
            return false;
        }
        if (*val as i32 - prev_val as i32).abs() > 3 {
            return false;
        }

        if !order_set {
            if *val > prev_val {
                increasing = true;
                order_set = true;
                prev_val = *val;
            } else {
                order_set = true;
                prev_val = *val;
            }
        } else {
            if increasing {
                if *val < prev_val {
                    return false;
                }
            } else {
                if *val > prev_val {
                    return false;
                }
            }
            prev_val = *val;
        }
    }
    return true;
}

#[inline]
fn is_valid_part_2(record: &Vec<u32>) -> bool {
    let mut prev_val: u32 = 0;
    let mut increasing = false;
    let mut order_set = false;
    let mut bad_remaining = true;

    for (idx, val) in record.iter().enumerate() {
        if idx == 0 {
            prev_val = *val;
            continue;
        }

        if *val == prev_val {
            if bad_remaining {
                bad_remaining = false;
                continue;
            }
            return is_valid(&record[1..].to_vec());
        }
        if (*val as i32 - prev_val as i32).abs() > 3 {
            if bad_remaining {
                bad_remaining = false;
                continue;
            }
            return is_valid(&record[1..].to_vec());
        }

        if !order_set {
            if *val > prev_val {
                increasing = true;
                order_set = true;
                prev_val = *val;
            } else {
                order_set = true;
                prev_val = *val;
            }
        } else {
            if increasing {
                if *val < prev_val {
                    if bad_remaining {
                        bad_remaining = false;
                        continue;
                    }
                    return is_valid(&record[1..].to_vec());
                }
            } else {
                if *val > prev_val {
                    if bad_remaining {
                        bad_remaining = false;
                        continue;
                    }
                    return is_valid(&record[1..].to_vec());
                }
            }
            prev_val = *val;
        }
    }
    return true;
}

#[inline]
fn is_valid_part_2_brute_force(record: &Vec<u32>) -> bool {
    for idx in 0..record.len() {
        let mut new_vec = record.clone()[..idx].to_vec();
        new_vec.append(&mut record.clone()[idx+1..].to_vec());
        if is_valid(&new_vec) {
            return true;
        }
    }
    return false;
}

fn get_num_valid_records(records: &Vec<Vec<u32>>) -> u32 {
    let mut num_valid = 0;

    for record in records {
        let is_valid = is_valid_part_2_brute_force(record);
        if is_valid {
            num_valid += 1;
        }
    }
    num_valid
}

fn main() {
    const FILENAME: &str = "input.txt";

    let records: Vec<Vec<u32>> = std::fs::read_to_string(FILENAME)
        .expect("Failed to read file")
        .lines()
        .map(|line| {
            let iter = line.split_whitespace();
            iter.map(|x| x.parse::<u32>().unwrap()).collect()
        })
        .collect();

    let num_valid_records = get_num_valid_records(&records);
    println!("Number of valid records: {}", num_valid_records);
    println!("Total number of records: {}", records.len());
}
