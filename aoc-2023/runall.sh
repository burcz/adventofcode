#!/bin/zsh

for i in {1..10}
  echo $(cargo run -q -- $i)

exit 0;
