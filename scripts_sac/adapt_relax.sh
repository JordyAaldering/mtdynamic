#!/bin/sh

#SBATCH --account=csmpi
#SBATCH --partition=csmpi_long
#SBATCH --nodelist=cn128
#SBATCH --mem=0
#SBATCH --cpus-per-task=16
#SBATCH --time=10:00:00
#SBATCH --output=sac_adapt_relax.out

printf "size,runtime,runtimesd,energy,energysd\n"

for size in 10000 25000 40000; do
    ../sac2c/build_r/sac2c_p -noprelude -maxwlur 9 -t mt_pth_rt -mt_bind simple scripts_sac/relax.sac -o relax -DP=$size

    printf "$size,"
    ./relax -mt 16
done

rm relax
rm relax.c
rm relax.i
rm relax.o
