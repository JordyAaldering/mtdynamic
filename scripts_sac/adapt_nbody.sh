#!/bin/sh

#SBATCH --account=csmpi
#SBATCH --partition=csmpi_long
#SBATCH --nodelist=cn128
#SBATCH --mem=0
#SBATCH --cpus-per-task=16
#SBATCH --time=10:00:00
#SBATCH --output=sac_adapt_nbody.out

printf "size,runtime,runtimesd,energy,energysd\n"

# Static approaches
for size in 500 1000 1500; do
    ../sac2c/build_r/sac2c_p -noprelude -t mt_pth_rt -mt_bind simple scripts_sac/nbody.sac -o nbody -DP=$size -DITER=200

    printf "$size,8,"
    ./nbody -mt 8
    printf "$size,12,"
    ./nbody -mt 12
    printf "$size,16,"
    ./nbody -mt 16
done

rm *_timestep_*.csv

# Dynamic approach
for size in 500 1000 1500; do
    ../sac2c/build_r/sac2c_p -noprelude -t mt_pth_rt -mt_bind simple scripts_sac/nbody.sac -o nbody -DP=$size -DITER=200

    printf "$size,mt,"
    ./nbody -mt 16
done

rm nbody
rm nbody.c
rm nbody.i
rm nbody.o
