#!/bin/bash

total=0

for i in {1..200}; do
    # 执行命令并将输出存储到变量中
    output=$(./target/debug/wasmtime examples/greeting.wat --wasi-modules=experimental-wasi-nn)
    # echo $output
    # printf "%s" "$output"
    # 在输出中查找以“the between time is”开头的行
    between_time=$(printf "%s" "$output" | grep "the between time is")
    # echo $between_time

    # 如果找到了这样的行，则从中提取数字并将其添加到总和中
    if [ -n "$between_time" ]; then
        time_value=$(echo "$between_time" | awk '{print $5}')
	echo $time_value
        total=$((total + time_value))
    fi
done

average=$(echo "scale=2; $total/200" | bc)
echo "The average time value is: $average"

