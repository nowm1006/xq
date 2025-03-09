#! /bin/bash
xq $1 $2 |
awk -F, '$1~/[0-9]+/ {print $0}'|
head -c -1|
jq -R -s -c -f mapping.jq