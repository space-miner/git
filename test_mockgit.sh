#!/bin/bash
cargo run --bin jit init mockgit;
cd mockgit;
echo "hello" > hello.txt;
mkdir a;
mkdir a/b;
mkdir a/c;
touch a/b/d.txt;
touch a/c/e.txt;
cargo run --bin jit commit;
