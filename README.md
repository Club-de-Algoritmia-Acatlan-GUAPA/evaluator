# Overview of evaluator

# Running tests

Tests ( for now ) should be running using:
```
   cargo test -- --test-threads=1 --show-output 
```

to avoid having conflicts overwriting files

# Todos

- Protect compilation time ( currently compilation is being executed without a jailed command )
- Store temporary files created by compilation in `./playground/{user_id}`
- Set a limit for output in compilation and runtime
- Currently if user send script like `print("ls /resources/")` it's possible to see all test cases, this must be solved before being contest ready.
- Add a real cache module, including the download of test cases
- Parallelize downloading testcases with compiling user code
- Avoid compiling checker again if it's already compiled
- Remove hardcoded strings in `problem_executor.rs`

