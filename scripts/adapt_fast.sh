#!/bin/sh

#SBATCH --account=csmpi
#SBATCH --partition=csmpi_long
#SBATCH --nodelist=cn128
#SBATCH --mem=0
#SBATCH --cpus-per-task=16
#SBATCH --time=10:00:00
#SBATCH --output=adapt_fast.out

cargo build --release --example adapt_fast

# Warmup
stress --cpu 16 --timeout 30

./target/release/examples/adapt_fast 8  false false
./target/release/examples/adapt_fast 12 false false
./target/release/examples/adapt_fast 16 false false
./target/release/examples/adapt_fast 16 true  false
