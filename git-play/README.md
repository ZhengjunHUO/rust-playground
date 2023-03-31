### Set LD_LIBRARY_PATH before cargo run
```sh
# You are currently in the project root
$ export LD_LIBRARY_PATH=$PWD/libs/:$LD_LIBRARY_PATH
$ rustup override set nightly
$ cargo run -- /PATH/TO/GIT_REPO
```
