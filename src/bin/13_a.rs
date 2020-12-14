use aoc2020::get_input;
use itertools::Itertools;

fn solve(input: &str) -> i32 {
    let (arrival, busses) = input.split('\n').tuples().next().unwrap();

    let arrival = arrival.parse::<i32>().unwrap();

    let mut min_minutes_to_wait = i32::MAX;
    let mut best_bus_id = 0;

    for bus in busses.split(',').filter_map(|b| b.parse::<i32>().ok()) {
        let modulo = arrival % bus;

        let minutes_to_next_bus = if modulo == 0 { 0 } else { bus - modulo };

        if minutes_to_next_bus < min_minutes_to_wait {
            best_bus_id = bus;
            min_minutes_to_wait = minutes_to_next_bus;
        }
    }

    best_bus_id * min_minutes_to_wait
}

fn main() {
    let input = get_input();

    let result = solve(&input);

    println!("{:?}", result);
}

#[cfg(test)]
mod test {

    #[test]
    fn test_solve() {
        assert_eq!(
            super::solve(
                "939
7,13,x,x,59,x,31,19"
            ),
            295
        );
    }
}
