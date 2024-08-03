#!/bin/bash
cargo run --bin jit init mockgit;
cd mockgit;
echo "hello" > hello.txt;
mkdir foo;
touch foo/world.txt;
touch foo/zzz.txt;
cargo run --bin jit commit;
