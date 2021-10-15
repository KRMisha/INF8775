#!/usr/bin/env python3

import itertools
from pathlib import Path
import re
import statistics
import subprocess
import pandas as pd

DATA_PATH = Path('data')
ALGORITHMS = {
    'Conventional': 'conv',
    'Strassen': 'strassen',
    'StrassenThreshold': 'strassenSeuil',
}


def main():
    matrix_filenames = [x for x in DATA_PATH.iterdir() if x.is_file()]
    matrix_n_sizes = sorted(set(int(re.search(r'ex(\d*?)_', filename.name).group(1)) for filename in matrix_filenames))
    
    results = {}

    for algorithm_name, algorithm_arg in ALGORITHMS.items():
        print('Measuring execution time for', algorithm_name)
        results[algorithm_name] = {}
        
        for n in matrix_n_sizes:
            matrix_n_size_filenames = sorted(DATA_PATH.glob(f'ex{n}_*'))
            matrix_filename_pairs = itertools.combinations(matrix_n_size_filenames, 2)

            execution_times_ms = []
            for matrix_1_filename, matrix_2_filename in matrix_filename_pairs:
                result = subprocess.run(
                    ['./tp1.sh', '-e1', matrix_1_filename, '-e2', matrix_2_filename, '-a', algorithm_arg, '-t'],
                    stdout=subprocess.PIPE,
                )
                execution_times_ms.append(float(result.stdout.decode('utf-8')))

            average_execution_time_ms = statistics.mean(execution_times_ms)
            print(f'\tN: {n} - Average execution time: {average_execution_time_ms}')

            results[algorithm_name][n] = average_execution_time_ms
        
        print()

    df = pd.DataFrame(results)
    print(df)


if __name__ == '__main__':
    main()
