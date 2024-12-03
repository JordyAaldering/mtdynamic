#!/bin/sh

#SBATCH --account=csmpi
#SBATCH --partition=csmpi_long
#SBATCH --nodelist=cn125
#SBATCH --mem=0
#SBATCH --cpus-per-task=16
#SBATCH --time=10:00:00
#SBATCH --output=find_best.out

ITER=50

cargo build --release --example matmul

# Warmup
stress --cpu 16 --timeout 30

printf "pin,size,threads,runtime,runtimesd,energy,energysd\n"

for pin in true false; do
    for size in `seq 500 100 1500`; do
        for threads in `seq 1 16`; do
            printf "$pin,$size,$threads,"
            ./target/release/examples/matmul $size $ITER $threads $pin "fixed"
        done
    done
done
