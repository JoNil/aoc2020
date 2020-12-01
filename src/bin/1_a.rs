use aoc2020::get_input_i32;

fn main() {
    let input = get_input_i32();

    for (i, a) in input.iter().enumerate() {
        for b in &input[i..] {
            if a + b == 2020 {
                println!("{}", a * b);
                return;
            }
        }
    }
}
