#!/bin/bash

EXAMPLES=$(ls examples/*.rs)

for line in $EXAMPLES; 
do
	filename=$(basename $line)
	example="${filename%.*}"
	if [[ $? = 0 ]]
	then
		cargo run --example $example
	else
		echo "[-] Problem with $example"
		exit 1
	fi
done
