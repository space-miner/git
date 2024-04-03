#!/bin/bash
cargo run init mockgit;
cd mockgit;
echo "hello" > hello.txt;
cargo run commit;
