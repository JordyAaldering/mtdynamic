#!/bin/sh

#SBATCH --account=csmpi
#SBATCH --partition=csmpi_fpga_long
#SBATCH --mem=0
#SBATCH --cpus-per-task=32
#SBATCH --time=10:00:00
#SBATCH --output=log/math.out

make release

printf "dynamic,busy,threads,energy,runtime,usertime\n"

LEN=10000000
ITER=500

for busy in `seq 0 4 32`; do
    for threads in `seq 1 32`; do
        printf "false,$busy,$threads,"
        ./target/release/busywork_f 1 $busy ./target/release/examples/math $LEN $ITER $threads true
    done
done

for busy in `seq 0 4 32`; do
    printf "true,$busy,$threads,"
    ./target/release/busywork_f 1 $busy ./target/release/examples/math $LEN $ITER 32 false
done
