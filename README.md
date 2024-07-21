<h3>Instructions to run the project as it is right now.</h3>

The test_mockgit.sh script runs all these commands and populates a dummy mockgit directory 
with files for testing the commit command. 

<h3>1) Build the project:</h3>
cargo build

<h3>2) Initialize a repo (jit init).</h3> 
cargo run --bin jit init  

OR specify a directory:
cargo run --bin jit init <directory>

<h3>3) Commit a file:</h3>
cargo run --bin jit commit

<h3>Extra</h3>

There is also a simple version of cat-file, which is its own standalone executable and 
is different from git's cat-file. It inflates object contents and allows the user to 
pass the object path on the command line, rather than requiring the object ID as an 
argument. For development/debugging purposes.

Since this project interoperates with git, the normal git cat-file can be used with no 
problem.

Example usage:

cargo run --bin cat-file .git/objects/AB/CDE......F 

And this can be further piped through \<hexdump -C> to inspect the bytes.  
