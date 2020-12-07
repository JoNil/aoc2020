use aoc2020::get_input;
use itertools::Itertools;
use std::collections::HashMap;

fn remove_suffix(input: &str) -> &str {
    input
        .trim()
        .trim_end_matches("bags")
        .trim_end_matches("bag")
        .trim()
}

fn parse_content(input: &str) -> (i32, &str) {
    let (count, name) = input
        .trim()
        .splitn(2, ' ')
        .tuples::<(_, _)>()
        .next()
        .unwrap();
    (count.trim().parse().unwrap(), remove_suffix(name))
}

fn parse_rule(input: &str) -> Option<(&str, Vec<(i32, &str)>)> {
    let input = input.trim();

    if input.is_empty() {
        return None;
    }

    let (name, rest) = input
        .splitn(2, "contain")
        .tuples::<(_, _)>()
        .next()
        .unwrap();

    let rest = rest.trim();

    let contents = if rest == "no other bags" {
        Vec::new()
    } else {
        rest.split(',').map(parse_content).collect()
    };

    Some((remove_suffix(name), contents))
}

fn count_bags_inside(graph: &HashMap<&str, Vec<(i32, &str)>>, bag: &str) -> i32 {
    let res = if let Some(contents) = graph.get(bag) {
        contents
            .iter()
            .map(|(count, bag)| count * count_bags_inside(graph, bag))
            .sum()
    } else {
        0
    };

    1 + res
}

fn count_bags(input: &str) -> i32 {
    let graph = input
        .split('.')
        .filter_map(|input| parse_rule(input))
        .collect::<HashMap<_, _>>();

    count_bags_inside(&graph, "shiny gold")
}

fn main() {
    let input = get_input();

    let entry_point_count = count_bags(&input) - 1;

    println!("{}", entry_point_count);
}

#[cfg(test)]
mod test {

    #[test]
    fn test_count_bags() {
        let input = "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";

        assert_eq!(super::count_bags(input) - 1, 126);
    }
}
