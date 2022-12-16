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

## Performance

Time taken to complete each part of each day.
Very unreliable times calculated by averaging 10 runs of each part,
which were run on my PC while I was listening to music and had N Firefox tabs open :)

|Day|First Part|Second Part|
|:-:|:--------:|:---------:|
|01|45µs|67µs|
|02|75µs|72µs|
|03|251µs|300µs|
|04|319µs|297µs|
|05|139µs|198µs|
|06|222µs|930µs|
|07|1.09ms|1.10ms|
|08|238µs|383µs|
|09|820µs|1.21ms|
|10|43µs|54µs|
|11|132µs|28.17ms|
|12|510µs|29.69ms|
|13|||
|14|||
|15|||
|16|||
|17|||
|18|||
|19|||
|20|||
|21|||
|22|||
|23|||
|24|||
|25|||
