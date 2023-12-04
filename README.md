# aoc-2023

Generate a puzzle for a given day:

```
$ ./generate.sh roar 01
```

Arguments are:

1. The directory to put the puzzle in, e.g. `roar` will be created under `src/roar` and put inputs under `input/roar`
2. The day of the puzzle - must be 2 digits, e.g. `01` for day 1 or `23` for day 23

### Testing

We use the following convention for test names: <programmer>_q<day>_p<1|2>_<main|sample>

This allows granularity to run tests as follows:

- `cargo test` - Run all tests
- `cargo test roar` - Run all roar's tests
- `cargo test gee_q02` - Run all gee's tests for question 2
- `cargo test roar_q03_p1` - Run the sample and main tests for roar, question 3, part 1
- `cargo test gee_q01_p2_main` - Run only gee's question 1, part 2, main test
- `cargo test q03` - Run both gee's and roar's tests for question 3


