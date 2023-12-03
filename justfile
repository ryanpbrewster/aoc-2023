profile-bench:
  cargo flamegraph --bench day02_benchmark --root --output /tmp/flamegraph.svg --open -- --bench --profile-time 5
test:
  RUST_TEST_TIME_UNIT=50,500 cargo +nightly test --release -- -Z unstable-options --report-time
