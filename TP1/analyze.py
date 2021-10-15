#!/usr/bin/env python3

import itertools
from pathlib import Path
import re
import statistics
import subprocess
import matplotlib.pyplot as plt
import pandas as pd
import seaborn as sns

DATA_PATH = Path('data')
GRAPH_OUTPUT_PATH = Path('analysis')
ALGORITHMS = {
    'Conventional': 'conv',
    'Strassen': 'strassen',
    'StrassenThreshold': 'strassenSeuil',
}


def measure_execution_times(algorithms, extra_args=[]):
    matrix_filenames = [x for x in DATA_PATH.iterdir() if x.is_file()]
    matrix_n_sizes = sorted(set(int(re.search(r'ex(\d*?)_', filename.name).group(1)) for filename in matrix_filenames))

    results = {}

    for algorithm_name, algorithm_arg in algorithms.items():
        print('Measuring execution time for', algorithm_name)
        results[algorithm_name] = {}

        for n in matrix_n_sizes:
            matrix_n_size_filenames = sorted(DATA_PATH.glob(f'ex{n}_*'))
            matrix_filename_pairs = itertools.combinations(matrix_n_size_filenames, 2)

            execution_times_ms = []
            for matrix_1_filename, matrix_2_filename in matrix_filename_pairs:
                result = subprocess.run(
                    ['./tp1.sh', '-a', algorithm_arg, '-e1', matrix_1_filename, '-e2', matrix_2_filename, '-t', *extra_args],
                    stdout=subprocess.PIPE,
                )
                execution_times_ms.append(float(result.stdout.decode('utf-8')))

            average_execution_time_ms = statistics.mean(execution_times_ms)
            print(f'\tN: {n} - Average execution time: {average_execution_time_ms}')

            results[algorithm_name][n] = average_execution_time_ms

        print()

    df = pd.DataFrame(results)
    return df


def compare_strassen_thresholds():
    results = {}

    for threshold in [2 ** i for i in range(5, 9)]:
        print(f'Threshold: {threshold}')
        df = measure_execution_times({'StrassenThreshold': 'strassenSeuil'}, ['--threshold', str(threshold)])
        results[threshold] = df['StrassenThreshold']

    df = pd.DataFrame(results)
    return df


def main():
    GRAPH_OUTPUT_PATH.mkdir(exist_ok=True)

    df = compare_strassen_thresholds()
    print('Execution times of the StrassenThreshold algorithm with different thresholds')
    print(df)
    print('\n--------------------\n')

    df = measure_execution_times(ALGORITHMS)
    print('Execution times of the three different algorithms')
    print(df)

    sns.lineplot(data=df)
    plt.savefig(GRAPH_OUTPUT_PATH / 'execution_times.png')


if __name__ == '__main__':
    main()
