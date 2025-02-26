#!/bin/bash

total=0

# 循环200次
for i in {1..200}; do
    # 运行第一个程序，并将其输出存储到文件中
    ../../../target/debug/wasmtime --wasi-modules experimental-wasi-rdma rdma_test_server/target/wasm32-wasi/release/rdma_test.wasm > /dev/null 2>&1 &

    # 等待1秒钟
    sleep 1   

    # 运行第二个程序，并将其输出存储到变量中
    output2=$(../../../target/debug/wasmtime --wasi-modules experimental-wasi-rdma rdma_test/target/wasm32-wasi/release/rdma_test.wasm)

    # 在第二个程序的输出中查找以“the between time is”开头的行
    between_time=$(echo "$output2" | grep "^the between time is")

    # 如果找到了这样的行，则从中提取数字并将其添加到总和中
    if [ -n "$between_time" ]; then
        time_value=$(echo "$between_time" | awk '{print $5}')
        total=$((total + time_value))
    fi
done

# 计算平均值
average=$(echo "scale=2; $total/200" | bc)
echo "The average time value is: $average"

