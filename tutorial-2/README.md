# Distributed System 
* [Tutorial Video](https://www.youtube.com/watch?v=gboGyccRVXI)

## About [Fly.IO Distributed System](http://fly.io/dist-sys/)
* A series of distributed systems challenges brought to you by Fly.io.


## About [Maelstrom](https://github.com/maelstrom-software/maelstrom)
* Maelstrom is a suite of tools for running tests in isolated micro-containers locally on your machine or distributed across arbitrarily large clusters. Maelstrom currently has test runners for Rust, Go, and Python, with more on the way. 


## Install Maelstrom on Your Mac
There are multiple way to install maelstrom, both cargo install, compile source codes and download the binary executable files are ok. But I recommend the guys who use Mac with M1 chips to directly download the binary executable file from [last released github page](https://github.com/jepsen-io/maelstrom/releases/tag/v0.2.3) directly, because for M1 chip users compile source codes may face some compiling errors. 

After download binary file to target path, do not forget declare the classpath and binary path in your `~/.bash_profile`, so that you can use command of `maelstrom` any path. 

## Run Maelstrom Test Echo Server 
* [](../maelstrom_test.sh)
```shell 
#!/bin/sh

source ~/.bash_profile

cargo build 

maelstrom test -w echo --bin ./target/debug/rustengan --node-count 1 --time-limit 10
```