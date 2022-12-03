pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;
pub mod day17;
pub mod day18;
pub mod day19;
pub mod day20;
pub mod day21;
pub mod day22;
pub mod day23;
pub mod day24;
pub mod day25;

// Mapping from day:part to actual solution code
pub const SOLUTION_MAPPING: [[fn(String) -> Result<String, String>; 2]; 25] = [
    [day01::solve_a, day01::solve_b],
    [day02::solve_a, day02::solve_b],
    [day03::solve_a, day03::solve_b],
    [day04::solve_a, day04::solve_b],
    [day05::solve_a, day05::solve_b],
    [day06::solve_a, day06::solve_b],
    [day07::solve_a, day07::solve_b],
    [day08::solve_a, day08::solve_b],
    [day09::solve_a, day09::solve_b],
    [day10::solve_a, day10::solve_b],
    [day11::solve_a, day11::solve_b],
    [day12::solve_a, day12::solve_b],
    [day13::solve_a, day13::solve_b],
    [day14::solve_a, day14::solve_b],
    [day15::solve_a, day15::solve_b],
    [day16::solve_a, day16::solve_b],
    [day17::solve_a, day17::solve_b],
    [day18::solve_a, day18::solve_b],
    [day19::solve_a, day19::solve_b],
    [day20::solve_a, day20::solve_b],
    [day21::solve_a, day21::solve_b],
    [day22::solve_a, day22::solve_b],
    [day23::solve_a, day23::solve_b],
    [day24::solve_a, day24::solve_b],
    [day25::solve_a, day25::solve_b],
];
