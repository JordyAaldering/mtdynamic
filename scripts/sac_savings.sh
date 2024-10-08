#!/bin/sh

#SBATCH --account=csmpi
#SBATCH --partition=csmpi_long
#SBATCH --nodelist cn128
#SBATCH --mem=0
#SBATCH --cpus-per-task=16
#SBATCH --time=10:00:00
#SBATCH --output=sac_savings.out

../sac2c/build_r/sac2c_p -noprelude -t mt_pth -mt_bind simple matmul.sac -o matmul -DP=1000 -DITER=50
../sac2c/build_r/sac2c_p -noprelude -t mt_pth_rt -mt_bind simple matmul.sac -o matmul_mt -DP=1000 -DITER=50

# Warmup
./matmul -mt 16

printf "\n"
printf "type,,energy,runtime,usertime\n"

printf "8,"
./matmul -mt 8
printf "12,"
./matmul -mt 12
printf "16,"
./matmul -mt 16
printf "mt,"
./matmul_mt -mt 16

printf "8,"
./matmul -mt 8
printf "12,"
./matmul -mt 12
printf "16,"
./matmul -mt 16
printf "mt,"
./matmul_mt -mt 16

rm matmul
rm matmul.c
rm matmul.i
rm matmul.o

rm matmul_mt
rm matmul_mt.c
rm matmul_mt.i
rm matmul_mt.o
