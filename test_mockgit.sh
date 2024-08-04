#!/bin/bash
cargo run --bin jit init mockgit;
cd mockgit;
echo "hello" > hello.txt;
mkdir foo;
mkdir foo/bar;
touch foo/world.txt;
touch foo/bar/bax.txt;
touch foo/zzz.txt;
cargo run --bin jit commit;
