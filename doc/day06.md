# Fuzzing

Adventures in fuzzing. I chose day06 because I wrote that code kind of carelessly and I'm pretty
sure it has a panic in it.

Starting out by installing `cargo-fuzz` and running it on this fuzz target:

```rust
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        let _ = aoc_2023::day06::part1(s).unwrap();
    }
});
```

Got an immediate panic:
```
called `Result::unwrap()` on an `Err` value: could not parse : Parsing Error: Error { input: "", code: Tag }
```

which...makes sense. Gonna have to make sure this is a valid input.

Changing this to
```rust
fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        let _ = aoc_2023::day06::part1(s);
    }
});
```

led to a much longer run. It's legitimately difficult to brute-force come up with a valid input; for reference the valid inputs look like this:
```
Time:      7  15   30
Distance:  9  40  200
```

the fuzzer did manage to make progress on reconstituting the input format:
```
Time:5Dist2
```

but it never really got past the parser itself.