# Homework 1 (Cloud)

## Create test file 
```
cargo run --release -- create test.txt --size 9G 
```

This creates a new `test.txt` with 9G or more of contents (random english words).

## Count words 
```
cargo run --release -- count test.txt --output test 
```

This creates a new `test.json` file with the results. 

## Results 
See `results.txt`, commits show different versions of the `count` function 
(sequential and parallel) tested with a 9GB file.