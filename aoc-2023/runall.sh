#!/bin/zsh

for i in {1..5}
  echo $(cargo run -q -- $i)

exit 0;
