#!/bin/sh

#SBATCH --account=csmpi
#SBATCH --partition=csmpi_long
#SBATCH --nodelist=cn126
#SBATCH --mem=0
#SBATCH --cpus-per-task=16
#SBATCH --time=10:00:00
#SBATCH --output=sac_adapt_relax.out

# Warmup
stress --cpu 16 --timeout 30

printf "size,threads,runtime,runtimesd,energy,energysd\n"

# Static approaches
for size in 10000 25000 40000; do
    ../sac2c/build_r/sac2c_p -noprelude -maxwlur 9 -t mt_pth -mt_bind simple scripts_sac/relax.sac -o relax -DP=$size -DITER=100

    printf "$size,8,"
    ./relax -mt 8
    printf "$size,12,"
    ./relax -mt 12
    printf "$size,14,"
    ./relax -mt 14
    printf "$size,16,"
    ./relax -mt 16
done

# Energy-based approach
for size in 10000 25000 40000; do
    ../sac2c/build_r/sac2c_p -noprelude -maxwlur 9 -t mt_pth_rt -mt_bind simple scripts_sac/relax.sac -o relax -DP=$size

    printf "$size,mt,"
    ./relax -mt 16
done

# Runtime-based approach
for size in 10000 25000 40000; do
    ../sac2c/build_r/sac2c_p -noprelude -maxwlur 9 -t mt_pth_rt -domtdrt -mt_bind simple scripts_sac/relax.sac -o relax -DP=$size

    printf "$size,rt,"
    ./relax -mt 16
done

rm relax
rm relax.c
rm relax.i
rm relax.o
