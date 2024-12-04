#!/bin/sh

#SBATCH --account=csmpi
#SBATCH --partition=csmpi_long
#SBATCH --nodelist=cn126
#SBATCH --mem=0
#SBATCH --cpus-per-task=16
#SBATCH --time=10:00:00
#SBATCH --output=adapt_stencil.out

printf "size,threads,runtime,runtimesd,energy,energysd\n"

# Static approaches
for size in 10000 25000 40000; do
    ../sac2c/build_r/sac2c_p -noprelude -maxwlur 9 -t mt_pth -mt_bind simple scripts_sac/stencil.sac -o stencil -DP=$size

    printf "$size,8,"
    ./stencil -mt 8
    printf "$size,12,"
    ./stencil -mt 12
    printf "$size,14,"
    ./stencil -mt 14
    printf "$size,16,"
    ./stencil -mt 16
done

# Energy-based approach
for size in 10000 25000 40000; do
    ../sac2c/build_r/sac2c_p -noprelude -maxwlur 9 -t mt_pth_rt -mt_bind simple scripts_sac/stencil.sac -o stencil -DP=$size

    printf "$size,mt,"
    ./stencil -mt 16
done

# Runtime-based approach
for size in 10000 25000 40000; do
    ../sac2c/build_r/sac2c_p -noprelude -maxwlur 9 -t mt_pth_rt -domtdrt -mt_bind simple scripts_sac/stencil.sac -o stencil -DP=$size

    printf "$size,rt,"
    ./stencil -mt 16
done

rm stencil
rm stencil.c
rm stencil.i
rm stencil.o
