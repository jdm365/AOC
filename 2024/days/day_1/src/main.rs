fn get_sim_score(
    values_1_sorted: &Vec<isize>,
    values_2_sorted: &Vec<isize>,
) -> isize {
    let mut score: isize = 0;

    let mut left_idx:  usize = 0;
    let mut right_idx: usize = 0;

    while (left_idx < values_1_sorted.len()) && (right_idx < values_2_sorted.len()) {
        let mut right_count: isize = 0;
        let left_num = values_1_sorted[left_idx];

        while values_2_sorted[right_idx] < left_num {
            right_idx += 1;
        }

        while values_2_sorted[right_idx] == left_num {
            right_idx += 1;
            right_count += 1;

            if right_idx == values_2_sorted.len() {
                break;
            }
        }

        let score_add = left_num * right_count;

        while values_1_sorted[left_idx] == left_num {
            left_idx += 1;
            score += score_add;

            if left_idx == values_1_sorted.len() {
                break;
            }
        }
    }

    score
}


fn main() {
    let main_start_time = std::time::Instant::now();
    const FILENAME: &str = "input.txt";

    let (mut values_1, mut values_2): (Vec<isize>, Vec<isize>) = std::fs::read_to_string(FILENAME)
        .expect("Failed to read file!")
        .lines()
        .map(|line| {
            let mut iter = line.split_whitespace();
            (
                iter.next().unwrap().parse::<isize>().unwrap(),
                iter.next().unwrap().parse::<isize>().unwrap()
            )
        })
        .unzip();

    values_1.sort_unstable();
    values_2.sort_unstable();

    let mut sum: isize = 0;
    for i in 0..values_1.len() {
        sum += (values_1[i] - values_2[i]).abs();
    }

    println!("Sum: {}", sum);

    let start_time = std::time::Instant::now();
    let sim_score = get_sim_score(&values_1, &values_2);
    let elapsed_time = start_time.elapsed().as_micros();

    println!("Sim score: {}", sim_score);
    println!("Elapsed time: {} microseconds", elapsed_time);

    let main_elapsed_time = main_start_time.elapsed().as_micros();
    println!("Total elapsed time: {} microseconds", main_elapsed_time);
}
