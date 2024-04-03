#!/bin/bash
cargo run init mockgit;
cd mockgit;
echo "hello" > hello.txt;
echo "world" > world.txt;
cargo run commit;
