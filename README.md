# Advent of Code 2022

Here are my solution for Advent of Code 2022 - in Rust!

## Usage

```
cargo run <DAY> <PART>
```

where we want to solve PART (`a` for first part, `b` for second) of the problem of the DAY.

Example:

```
cargo run 1 a
```

This "program" allow to run code to solve a part of a given day and includes:
- Reading input from file in `input` folder (expected name is `{DAY}.txt`)
- Calling the respective method for a given day and part.
  Solutions should be placed in `days` folder with `day{DAY:0>2}.rs` name.
  The method to solve the first part should be called `solve_a` for the first part and `solve_b` for the second.
  The method should receive one argument with the input as a String and return a Result with a String indication
  the problem solution (if the problem was solved) or with an error string to be output explaining what error happened.
- Outputing the solution and the time taken by the solving method (if it returned an Ok result)

## Progress

- [x] Day 01
- [x] Day 02
- [x] Day 03
- [x] Day 04
- [x] Day 05
- [ ] Day 06
- [ ] Day 07
- [ ] Day 08
- [ ] Day 09
- [ ] Day 10
- [ ] Day 11
- [ ] Day 12
- [ ] Day 13
- [ ] Day 14
- [ ] Day 15
- [ ] Day 16
- [ ] Day 17
- [ ] Day 18
- [ ] Day 19
- [ ] Day 20
- [ ] Day 21
- [ ] Day 22
- [ ] Day 23
- [ ] Day 24
- [ ] Day 25