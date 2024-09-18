#!/bin/sh

#SBATCH --account=csmpi
#SBATCH --partition=csmpi_long
#SBATCH --nodelist cn127
#SBATCH --mem=0
#SBATCH --cpus-per-task=16
#SBATCH --time=10:00:00
#SBATCH --output=log/cn_math.out

make release

printf "dynamic,busy,threads,energy,runtime,usertime\n"

REPEAT=5000
ITER=20

for busy in `seq 1 15`; do
    for threads in `seq 1 16`; do
        printf "false,$busy,$threads,"
        numactl --interleave all ./target/release/examples/math2 $REPEAT $ITER $threads $busy
    done
done
