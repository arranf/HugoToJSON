# Fuzzing with AFL.rs

[Tutorial](https://fuzz.rs/book/afl.html)

```bash
cargo install afl
# Should work
cargo afl build
# If not working try
# export RUSTFLAGS="-Clink-arg=-fuse-ld=gold" cargo afl build
cargo afl fuzz -i ../example_input -o out target/debug/hugo_to_json-fuzz-target
```