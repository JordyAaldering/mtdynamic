#!/bin/sh

#SBATCH --account=csmpi
#SBATCH --partition=csmpi_fpga_long
#SBATCH --mem=0
#SBATCH --cpus-per-task=16
#SBATCH --time=10:00:00
#SBATCH --output=log/fpga_sleepy.out

cargo build --release --bin busywork_f
cargo build --release --example sleepy

printf "dynamic,busy,threads,energy,runtime,usertime\n"

REPEAT=10000
ITER=50

for busy in `seq 0 2 16`; do
    for threads in `seq 1 16`; do
        printf "false,$busy,$threads,"
        ./target/release/busywork_f 1 $busy numactl --interleave all ./target/release/examples/sleepy $REPEAT $ITER $threads true
    done
done

for busy in `seq 0 2 16`; do
    printf "true,$busy,$threads,"
    ./target/release/busywork_f 1 $busy numactl --interleave all ./target/release/examples/sleepy $REPEAT $ITER 16 false
done