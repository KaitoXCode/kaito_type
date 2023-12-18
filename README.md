# kaito_type

## backend

### dev watch commands

watch application level logs:

bash`cargo watch -q -c -w src/ -x run`

watch test level logs:

bash`cargo watch -q -c -w tests/ -x "test -q quick_dev -- --nocapture"`
