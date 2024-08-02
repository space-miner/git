#!/bin/bash
cargo run --bin jit init mockgit;
cd mockgit;
echo "hello" > hello.txt;
cargo run --bin jit commit;
