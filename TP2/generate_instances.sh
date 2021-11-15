#!/usr/bin/env bash

readonly SEED=8775
readonly OUTPUT_DIRECTORY=data/generated

mkdir -p ${OUTPUT_DIRECTORY}

for i in {8..64..8}; do
    for j in {0..4}; do
        filename=ex${i}_${j}
        ./data/generator/bin/generator "${OUTPUT_DIRECTORY}/${filename}" \
            <<< "0 ${SEED}${j} 1 $i 1 0.9" > /dev/null
    done
done
