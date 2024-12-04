#!/bin/sh

#SBATCH --account=csmpi
#SBATCH --partition=csmpi_long
#SBATCH --nodelist=cn125
#SBATCH --mem=0
#SBATCH --cpus-per-task=16
#SBATCH --time=10:00:00
#SBATCH --output=threads_rust.out

cargo build --release --example matmul

printf "pin,size,threads,runtime,runtimesd,energy,energysd\n"

for pin in true false; do
    for size in `seq 500 100 1500`; do
        for threads in `seq 1 16`; do
            printf "$pin,$size,$threads,"
            ./target/release/examples/matmul $size 50 $threads $pin "fixed"
        done
    done
done
