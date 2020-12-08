use std::{env, fs};

pub mod computer;

pub fn get_input() -> String {
    let program_name = env::current_exe().unwrap();
    let program_name = program_name.file_stem().unwrap().to_str().unwrap();
    let program_name = program_name.split('_').next().unwrap();

    fs::read_to_string(format!("input/{}", program_name)).unwrap()
}

pub fn get_input_i32() -> Vec<i32> {
    get_input()
        .lines()
        .filter_map(|s| str::parse::<i32>(s).ok())
        .collect::<Vec<_>>()
}
