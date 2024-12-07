#[derive(Debug)]
enum Object {
    Empty,
    Obstacle,
    Traversed,
}

enum GuardDirection {
    Up,
    Right,
    Down,
    Left,
}

#[inline]
fn in_bounds(
    guard_pos: (isize, isize),
    left_bound: isize,
    right_bound: isize,
    upper_bound: isize,
    lower_bound: isize,
    ) -> bool {
    let cond1 = guard_pos.0 > upper_bound;
    let cond2 = guard_pos.0 < lower_bound;
    let cond3 = guard_pos.1 < right_bound;
    let cond4 = guard_pos.1 > left_bound;

    cond1 && cond2 && cond3 && cond4
}

fn calc_num_squares_traveled(input: &mut Vec<Vec<Object>>, _guard_pos: (isize, isize)) -> usize {
    let mut num_squares_traveled: usize = 0;
    let mut guard_direction = GuardDirection::Up;

    let mut guard_pos = _guard_pos;

    let left_bound: isize = 0;
    let right_bound: isize = input[0].len() as isize - 1;
    let upper_bound: isize = 0;
    let lower_bound: isize = input.len() as isize - 1;

    loop {
        match guard_direction {
            GuardDirection::Up => {
                guard_pos.0 -= 1;
                if !in_bounds(
                    guard_pos,
                    left_bound,
                    right_bound,
                    upper_bound,
                    lower_bound,
                    ) {
                    return num_squares_traveled;
                }

                match input[guard_pos.0 as usize][guard_pos.1 as usize] {
                    Object::Empty => {
                        input[guard_pos.0 as usize][guard_pos.1 as usize] = Object::Traversed;
                        num_squares_traveled += 1;
                    },
                    Object::Obstacle => {
                        guard_direction = GuardDirection::Right;
                        guard_pos.0 += 1;
                    },
                    Object::Traversed => (),
                }
            },
            GuardDirection::Right => {
                guard_pos.1 += 1;
                if !in_bounds(
                    guard_pos,
                    left_bound,
                    right_bound,
                    upper_bound,
                    lower_bound,
                    ) {
                    return num_squares_traveled;
                }

                match input[guard_pos.0 as usize][guard_pos.1 as usize] {
                    Object::Empty => {
                        input[guard_pos.0 as usize][guard_pos.1 as usize] = Object::Traversed;
                        num_squares_traveled += 1;
                    },
                    Object::Obstacle => {
                        guard_direction = GuardDirection::Down;
                        guard_pos.1 -= 1;
                    },
                    Object::Traversed => (),
                }
            },
            GuardDirection::Down => {
                guard_pos.0 += 1;
                if !in_bounds(
                    guard_pos,
                    left_bound,
                    right_bound,
                    upper_bound,
                    lower_bound,
                    ) {
                    return num_squares_traveled;
                }

                match input[guard_pos.0 as usize][guard_pos.1 as usize] {
                    Object::Empty => {
                        input[guard_pos.0 as usize][guard_pos.1 as usize] = Object::Traversed;
                        num_squares_traveled += 1;
                    },
                    Object::Obstacle => {
                        guard_direction = GuardDirection::Left;
                        guard_pos.0 -= 1;
                    },
                    Object::Traversed => (),
                }
            },
            GuardDirection::Left => {
                guard_pos.1 -= 1;
                if !in_bounds(
                    guard_pos,
                    left_bound,
                    right_bound,
                    upper_bound,
                    lower_bound,
                    ) {
                    return num_squares_traveled;
                }

                match input[guard_pos.0 as usize][guard_pos.1 as usize] {
                    Object::Empty => {
                        input[guard_pos.0 as usize][guard_pos.1 as usize] = Object::Traversed;
                        num_squares_traveled += 1;
                    },
                    Object::Obstacle => {
                        guard_direction = GuardDirection::Up;
                        guard_pos.1 += 1;
                    },
                    Object::Traversed => (),
                }
            },
        }
    }
}

fn main() {
    let mut init_row = 0;
    let mut init_col = 0;

    let mut lines: Vec<Vec<Object>> = std::fs::read_to_string("input.txt")
        .expect("failed to read file")
        .lines()
        .enumerate()
        .map(|(row_idx, line)| {
            line.as_bytes().iter().enumerate().map(|(col_idx, x)| 
                match x {
                    35 => Object::Obstacle,
                    46 => Object::Empty,
                    94 => {
                        init_row = row_idx as isize;
                        init_col = col_idx as isize;

                        // Mark as empty.
                        Object::Empty
                    },
                    _ => panic!("Invalid puzzle input."),
                }).collect()
        })
        .collect();

    let num_squares_traveled = calc_num_squares_traveled(&mut lines, (init_row, init_col));
    println!("{}", num_squares_traveled);
}
