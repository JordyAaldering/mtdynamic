#!/bin/sh

#SBATCH --account=csmpi
#SBATCH --partition=csmpi_long
#SBATCH --nodelist cn128
#SBATCH --mem=0
#SBATCH --cpus-per-task=16
#SBATCH --time=10:00:00
#SBATCH --output=sac_savings.out

# With pinning
../sac2c/build_r/sac2c_p -noprelude -specmode akd -sigspec akd -maxlur 50 -mt_bind simple -t mt_pth    matmul_adapt.sac -o matmul
../sac2c/build_r/sac2c_p -noprelude -specmode akd -sigspec akd -maxlur 50 -mt_bind simple -t mt_pth_rt matmul_adapt.sac -o matmul_mt

# Warmup
stress --cpu 16 --timeout 30

printf "4,"
./matmul -mt 4
printf "8,"
./matmul -mt 8
printf "12,"
./matmul -mt 12
printf "16,"
./matmul -mt 16
printf "mt,"
./matmul_mt -mt 16

# Without pinning
../sac2c/build_r/sac2c_p -noprelude -specmode akd -sigspec akd -maxlur 50 -t mt_pth    matmul_adapt.sac -o matmul
../sac2c/build_r/sac2c_p -noprelude -specmode akd -sigspec akd -maxlur 50 -t mt_pth_rt matmul_adapt.sac -o matmul_mt

# Warmup
stress --cpu 16 --timeout 30

printf "4,"
./matmul -mt 4
printf "8,"
./matmul -mt 8
printf "12,"
./matmul -mt 12
printf "16,"
./matmul -mt 16
printf "mt,"
./matmul_mt -mt 16

# Cleanup
rm matmul
rm matmul.c
rm matmul.i
rm matmul.o

rm matmul_mt
rm matmul_mt.c
rm matmul_mt.i
rm matmul_mt.o
