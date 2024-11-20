#!/bin/sh

source ~/.bash_profile

# Everything looks good! ヽ(‘ー`)ノ
maelstrom test -w broadcast --bin ../../target/debug/broadcast --node-count 1 --time-limit 10 --rate 10