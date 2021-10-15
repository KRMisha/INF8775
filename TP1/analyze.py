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
ANALYSIS_OUTPUT_PATH = Path('analysis')
ALGORITHMS = {
    'Conventional': 'conv',
    'Strassen': 'strassen',
    'StrassenThreshold': 'strassenSeuil',
}
MAX_N_SIZES = {
    'Conventional': None,
    'Strassen': 9,
    'StrassenThreshold': None,
}


def measure_execution_times(algorithms, trial_count=1, extra_args=[]):
    matrix_filenames = [x for x in DATA_PATH.iterdir() if x.is_file()]
    matrix_n_sizes = sorted(set(int(re.search(r'ex(\d*?)_', filename.name).group(1)) for filename in matrix_filenames))

    results = {}

    for algorithm_name, algorithm_arg in algorithms.items():
        print('Measuring execution time for', algorithm_name)

        results[algorithm_name] = {}

        for n in matrix_n_sizes:
            if MAX_N_SIZES[algorithm_name] is not None and n >= MAX_N_SIZES[algorithm_name]:
                break

            matrix_n_size_filenames = sorted(DATA_PATH.glob(f'ex{n}_*'))
            matrix_filename_pairs = list(itertools.combinations(matrix_n_size_filenames, 2))

            execution_times_ms = []
            for matrix_1_filename, matrix_2_filename in matrix_filename_pairs * trial_count:
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

    for threshold in [2 ** i for i in range(2, 9)]:
        print(f'Threshold: {threshold}')
        df = measure_execution_times({'StrassenThreshold': 'strassenSeuil'}, trial_count=1, extra_args=['--threshold', str(threshold)])
        results[threshold] = df['StrassenThreshold']

    df = pd.DataFrame(results)
    return df


def main():
    sns.set_theme(style='ticks', palette='pastel')

    ANALYSIS_OUTPUT_PATH.mkdir(exist_ok=True)

    # Strassen threshold evaluation
    df = compare_strassen_thresholds()
    print('Execution times of the StrassenThreshold algorithm with different thresholds')
    print(df)

    with open(ANALYSIS_OUTPUT_PATH / 'strassen_thresholds.md', 'w') as file:
        file.write(df.to_markdown())

    plt.figure()
    ax = sns.lineplot(data=df[-3:]) # Only show results for the three biggest matrices
    ax.set_title('Execution time for various Strassen thresholds')
    ax.set(xlabel='N', ylabel='Execution time (ms)')
    ax.get_xaxis().set_major_locator(plt.MaxNLocator(integer=True))
    plt.savefig(ANALYSIS_OUTPUT_PATH / 'strassen_thresholds.png', bbox_inches='tight')

    print('\n--------------------\n')

    # Algorithm comparison
    df = measure_execution_times(ALGORITHMS)
    print('Execution times of the three different algorithms')
    print(df)

    with open(ANALYSIS_OUTPUT_PATH / 'execution_times.md', 'w') as file:
        file.write(df.to_markdown())

    plt.figure()
    ax = sns.lineplot(data=df)
    ax.set_title('Execution time for each algorithm')
    ax.set(xlabel='N', ylabel='Execution time (ms)')
    ax.get_xaxis().set_major_locator(plt.MaxNLocator(integer=True))
    plt.savefig(ANALYSIS_OUTPUT_PATH / 'execution_times.png', bbox_inches='tight')


if __name__ == '__main__':
    main()
