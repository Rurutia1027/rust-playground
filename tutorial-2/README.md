# Distributed System

- [Tutorial Video 1](https://www.youtube.com/watch?v=gboGyccRVXI)
- [Tutorial Video 2](https://www.youtube.com/watch?v=BI_bHCGRgMY&t=17s)

## About [Fly.IO Distributed System](http://fly.io/dist-sys/)

- A series of distributed systems challenges brought to you by Fly.io.

## About [Maelstrom](https://github.com/maelstrom-software/maelstrom)

- Maelstrom is a suite of tools for running tests in isolated micro-containers locally on your machine or distributed across arbitrarily large clusters. Maelstrom currently has test runners for Rust, Go, and Python, with more on the way.

## Install Maelstrom on Your Mac

There are multiple ways to install maelstrom, like cargo install, compile source codes and download the binary executable file. But I recommend the guys who use Mac with M1 chips to directly download the binary executable file from [last released github page](https://github.com/jepsen-io/maelstrom/releases/tag/v0.2.3), because for M1 chip users compile source codes may face some compiling errors.

After download binary file to target path, do not forget declare the classpath and binary path in your `~/.bash_profile`, so that you can use command of `maelstrom` under any path.

- Do not forget install `gnuplot` by `brew install gnuplot` otherwise you'll get error msg during test

```
WARN [2024-11-19 17:29:19,498] clojure-agent-send-off-pool-1 - jepsen.checker Error while checking history:
java.lang.IllegalStateException: Error rendering plot, verify gnuplot is installed and reachable
        at jepsen.checker.perf$plot_BANG_.invokeStatic(perf.clj:489)
...
Errors occurred during analysis, but no anomalies found. ಠ~ಠ
```

---

## Build the Complete System Before Executing Maelstrom Test Commands

Due to the structure of the project, we need to change path to `rust-playground`, then execute the `cargo build` command.
All compiled files are located under the `rust-playground/target` instead of current `tutorial-2/target` folder.

## Run Maelstrom Test Echo Server

- [](../maelstrom_test.sh)

```shell
#!/bin/sh

source ~/.bash_profile

cargo build

maelstrom test -w echo --bin ./target/debug/rustengan --node-count 1 --time-limit 10
```

---

## Invoke Dist's Broadcast Service

- broadcast

```shell
#!/bin/sh

source ~/.bash_profile

# Everything looks good! ヽ(‘ー`)ノ
maelstrom test -w broadcast --bin ../../target/debug/broadcast --node-count 1 --time-limit 10 --rate 10
```

- multibroadcast

```shell
#!/bin/sh

source ~/.bash_profile

# Everything looks good! ヽ(‘ー`)ノ
maelstrom test -w broadcast --bin ../../target/debug/multibroadcast --node-count 8 --time-limit 10 --rate 10
```

---

## Invoke Dist's Unique Id Service

```shell
#!/bin/sh

source ~/.bash_profile

# Everything looks good! ヽ(‘ー`)ノ
maelstrom test -w unique-ids --bin ../../target/debug/unique_id --time-limit 30 --rate 1000 --node-count 3 --availability total --nemesis partition
```

---

## Run Serde Topic Codes

```shell
cargo run --bin serdewhatnow
```
