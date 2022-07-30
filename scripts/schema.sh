#!/bin/bash

declare -a contracts=("packs-dao" "packs-dao-factory")

for i in "${contracts[@]}"
do
    echo "$i"
    cd contracts/$i
    cargo schema
    cd ../../
done

declare -a packages=("packs")

for i in "${packages[@]}"
do
    echo "$i"
    cd packages/$i
    cargo schema
    cd ../../
done
