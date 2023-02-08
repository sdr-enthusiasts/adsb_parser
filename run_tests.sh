#! /bin/sh

echo "Running all except ignored tests."
echo "Ignored tests are for showing parsing error messages."
echo "All other tests operate on a pass/fail with details if required."
echo "---"
cargo test --no-fail-fast
echo "---"
echo "Tests completed."