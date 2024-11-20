#!/bin/sh

source ~/.bash_profile

# Everything looks good! ヽ(‘ー`)ノ  
maelstrom test -w unique-ids --bin ../../target/debug/unique_id --time-limit 30 --rate 1000 --node-count 3 --availability total --nemesis partition