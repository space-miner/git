Instructions to run the project as it is right now.

The test_mockgit.sh script runs all these commands and populates a dummy mockgit directory 
with files for testing the commit command. 

1) Build the project:
cargo build

2) Initialize a repo (jit init). 
cargo run --bin jit init  

OR specify a directory:
cargo run --bin jit init <directory>

3) Commit a file:
cargo run --bin jit commit

