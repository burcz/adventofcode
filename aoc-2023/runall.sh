#!/bin/zsh

for i in {1..9}
  echo $(cargo run -q -- $i)

exit 0;
