#!/bin/sh

#SBATCH --account=csmpi
#SBATCH --partition=csmpi_long
#SBATCH --nodelist=cn128
#SBATCH --mem=0
#SBATCH --cpus-per-task=16
#SBATCH --time=10:00:00
#SBATCH --output=compare_matmul_rust.out

cargo build --release --example compare_matmul

# Warmup
stress --cpu 16 --timeout 30

printf "threads,size,pin,runtime,runtimesd,energy,energysd\n"

for pin in true false; do
    for size in 500 1000 1500; do
        ./target/release/examples/compare_matmul 100 $size $pin 1  s false
        ./target/release/examples/compare_matmul 100 $size $pin 8  s false
        ./target/release/examples/compare_matmul 100 $size $pin 12 s false
        ./target/release/examples/compare_matmul 100 $size $pin 16 s false
        ./target/release/examples/compare_matmul 200 $size $pin 16 e false
        ./target/release/examples/compare_matmul 200 $size $pin 16 r false
    done
done
