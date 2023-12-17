# Fuzzing

After attempting to fuzz day06, I moved on to this problem for fuzzing.

I started off with
```rust
fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        let _ = aoc_2023::day08::part1(s);
    }
});
```

and within a few minutes it had found an infinite loop. The UX around infinte loops is not great.

Luckily `cargo-fuzz` keeps a corpus and I ran
```
ls -lrt
```
which came back with this input:
```
   R     AAA     =

(x, R)

=       R 
(x, R)           
```
which is poorly formatted, but technically passes the parser and definitely triggers a timeout.

This led me to discover the very useful `-timeout` option on cargo fuzz. Re-running with
```bash
cargo +nightly fuzz run fuzz_target_1 -- -only-ascii -timeout=5s
```
provides a much nicer experience.


## Fixing the timeout
In part 2 I added handling specifically to detect unintended cyling, but that seems like overkill for part 1.

There's a straightforward upper bound: There are only `directions.len() * graph.len()` number of distinct states. Consider the code in part 2
```rust
vis.entry(cur).or_default().insert(pos, i);
```

so we can safely cut off part 1 if we ever hit that many steps.