#!/bin/sh

#SBATCH --account=csmpi
#SBATCH --partition=csmpi_long
#SBATCH --nodelist=cn128
#SBATCH --mem=0
#SBATCH --cpus-per-task=16
#SBATCH --time=10:00:00
#SBATCH --output=sac_adapt_matmul.out

printf "size,threads,runtime,runtimesd,energy,energysd\n"

# Static approaches
for size in 500 1000 1500; do
    ../sac2c/build_r/sac2c_p -noprelude -t mt_pth_rt -mt_bind simple scripts_sac/matmul.sac -o matmul -DP=$size -DITER=200

    printf "$size,1,"
    ./matmul -mt 1
    printf "$size,8,"
    ./matmul -mt 8
    printf "$size,12,"
    ./matmul -mt 12
    printf "$size,16,"
    ./matmul -mt 16
done

rm *_matmul_*.csv

# Dynamic approach
for size in 500 1000 1500; do
    ../sac2c/build_r/sac2c_p -noprelude -t mt_pth_rt -mt_bind simple scripts_sac/matmul.sac -o matmul -DP=$size -DITER=200

    printf "$size,mt,"
    ./matmul -mt 16
done

rm matmul
rm matmul.c
rm matmul.i
rm matmul.o