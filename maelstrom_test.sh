#!/bin/sh

source ~/.bash_profile

maelstrom test -w echo --bin ./target/debug/rustengan --node-count 1 --time-limit 10