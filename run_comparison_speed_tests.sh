# /bin/sh

echo "Running Ignored tests without output capturing."
echo "The ignored tests are verbose tests to show exact parsing error messages."
echo "These tests will only fail if file loading is unsuccessful."
echo "---"
cargo test test_speed_large_queue -- --nocapture --ignored --test-threads 1
echo "---"
echo "Tests completed."