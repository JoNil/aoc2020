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

fn can_find_shiny_gold(graph: &HashMap<&str, Vec<(i32, &str)>>, node: &str) -> bool {
    if node == "shiny gold" {
        true
    } else if let Some(candidates) = graph.get(node) {
        candidates
            .iter()
            .any(|new_node| can_find_shiny_gold(graph, new_node.1))
    } else {
        false
    }
}

fn count_paths(input: &str) -> i32 {
    let graph = input
        .split('.')
        .filter_map(|input| parse_rule(input))
        .collect::<HashMap<_, _>>();

    graph
        .keys()
        .filter(|from| **from != "shiny gold")
        .filter(|from| can_find_shiny_gold(&graph, from))
        .count() as i32
}

fn main() {
    let input = get_input();

    let entry_point_count = count_paths(&input);

    println!("{}", entry_point_count);
}

#[cfg(test)]
mod test {

    #[test]
    fn test_count_paths() {
        let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

        assert_eq!(super::count_paths(input), 4);
    }
}
