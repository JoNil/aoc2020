use aoc2020::get_input_i32;

fn main() {
    let input = get_input_i32();

    for a in &input {
        for b in &input {
            for c in &input {
                if a + b + c == 2020 {
                    println!("{}", a * b * c);
                    return;
                }
            }
        }
    }
}
