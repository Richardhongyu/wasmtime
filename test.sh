#!/bin/bash

count=200
total_bt=0
total_1=0
total_2=0
total_3=0
total_4=0
total_5=0
total_6=0
total_7=0
total_8=0
total_9=0
total_10=0
total_11=0

for ((i=1;i<=count;i++))
do
    # ../../../target/debug/wasmtime --wasi-modules experimental-wasi-rdma rdma_test_server/target/wasm32-wasi/release/rdma_test.wasm > /dev/null 2>&1 &
    # sleep 1
    output2=$(./target/release/wasmtime examples/greeting.wat --wasi-modules=experimental-wasi-nn)
    # printf "%s" "$output2"
    # output2=$(../../../target/debug/wasmtime --wasi-modules experimental-wasi-rdma rdma_test/target/wasm32-wasi/release/rdma_test.wasm)
    bt=$(printf "%s" "$output2"| grep "the between time is" | awk '{print $5}')
    # echo $bt
    t1=$(printf "%s" "$output2"| grep "client: rdma_get_recv_comp:" | awk '{print $3}')
    # echo $b1
    t2=$(printf "%s" "$output2"| grep "client: rdma_get_send_comp:" | awk '{print $3}')
    # echo $b2
    t3=$(printf "%s" "$output2"| grep "client: rdma_post_send:" | awk '{print $3}')
    t4=$(printf "%s" "$output2"| grep "client: rdma_connect:" | awk '{print $3}')
    t5=$(printf "%s" "$output2"| grep "client: rdma_post_recv:" | awk '{print $3}')
    t6=$(printf "%s" "$output2"| grep "client: rdma_reg_msgs:" | awk 'NR==1{print $3}')
    # t7=$(printf "%s" "$output2"| grep "client: rdma_send_flags:" | awk '{print $3}')
    t8=$(printf "%s" "$output2"| grep "client: rdma_reg_msgs:" | awk 'NR==2{print $3}')
    t9=$(printf "%s" "$output2"| grep "client: rdma_init:" | awk '{print $3}')
    t10=$(echo "$output2" | grep "client: rdma_getaddrinfo:" | awk '{print $3}')
    t11=$(echo "$output2" | grep "client: rdma_create_ep:" | awk '{print $3}')
    total_bt=$(echo "scale=6; $total_bt + $bt" | bc)
    total_1=$(echo "scale=6; $total_1 + $t1" | bc)
    total_2=$(echo "scale=6; $total_2 + $t2" | bc)
    total_3=$(echo "scale=6; $total_3 + $t3" | bc)
    total_4=$(echo "scale=6; $total_4 + $t4" | bc)
    total_5=$(echo "scale=6; $total_5 + $t5" | bc)
    total_6=$(echo "scale=6; $total_6 + $t6" | bc)
    # total_7=$(echo "scale=6; $total_7 + $t7" | bc)
    total_8=$(echo "scale=6; $total_8 + $t8" | bc)
    total_9=$(echo "scale=6; $total_9 + $t9" | bc)
    total_10=$(echo "scale=6; $total_10 + $t10" | bc)
    total_11=$(echo "scale=6; $total_11 + $t11" | bc)
    echo "Iteration $i finished"
done

average_bt=$(echo "scale=6; $total_bt / $count" | bc)
average_1=$(echo "scale=6; $total_1 / $count" | bc)
average_2=$(echo "scale=6; $total_2 / $count" | bc)
average_3=$(echo "scale=6; $total_3 / $count" | bc)
average_4=$(echo "scale=6; $total_4 / $count" | bc)
average_5=$(echo "scale=6; $total_5 / $count" | bc)
average_6=$(echo "scale=6; $total_6 / $count" | bc)
# average_7=$(echo "scale=6; $total_7 / $count" | bc)
average_8=$(echo "scale=6; $total_8 / $count" | bc)
average_9=$(echo "scale=6; $total_9 / $count" | bc)
average_10=$(echo "scale=6; $total_10 / $count" | bc)
average_11=$(echo "scale=6; $total_11 / $count" | bc)

echo "the between time is $average_bt"
echo "client: rdma_create_ep: $average_11"
echo "client: rdma_getaddrinfo: $average_10"
echo "client: rdma_init: $average_9"
echo "client: rdma_reg_msgs: $average_8"
# echo "client: rdma_send_flags: $average_7"
echo "client: rdma_reg_msgs: $average_6"
echo "client: rdma_post_recv: $average_5"
echo "client: rdma_connect: $average_4"
echo "client: rdma_post_send: $average_3"
echo "client: rdma_get_send_comp: $average_2"
echo "client: rdma_get_recv_comp: $average_1"


