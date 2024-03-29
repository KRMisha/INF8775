#!/usr/bin/env python3

import argparse
import itertools
import math
from pathlib import Path
import re
import statistics
import subprocess
import matplotlib.pyplot as plt
import numpy as np
import pandas as pd
from scipy import stats
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
THEORETICAL_COMPLEXITY_POWERS = {
    'Conventional': 3,
    'Strassen': math.log2(7),
    'StrassenThreshold': math.log2(7),
}


def main():
    parser = argparse.ArgumentParser()
    subparsers = parser.add_subparsers(title='mode', dest='mode', required=True)
    subparsers.add_parser('threshold', help='Compare the execution time of different thresholds for the Strassen with threshold algorithm').set_defaults(func=run_threshold_subcommand)
    subparsers.add_parser('measure', help='Measure and save the execution times for the conventional, Strassen, and Strassen with threshold algorithms').set_defaults(func=run_measure_subcommand)
    subparsers.add_parser('complexity', help='Generate graphs for the power, ratio and constants tests based on the execution time results').set_defaults(func=run_complexity_subcommand)
    args = parser.parse_args()

    sns.set_theme(style='ticks', palette='pastel')
    ANALYSIS_OUTPUT_PATH.mkdir(exist_ok=True)

    args.func()


def run_threshold_subcommand():
    MAX_N_SIZES['StrassenThreshold'] = 9

    df = compare_strassen_thresholds()
    print('Execution times of the StrassenThreshold algorithm with different thresholds')
    print(df)

    with open(ANALYSIS_OUTPUT_PATH / 'strassen_thresholds.md', 'w') as file:
        file.write(df.to_markdown() + '\n')

    plt.figure()
    ax = sns.lineplot(data=df[-3:]) # Only show results for the three biggest matrices
    ax.set(
        title='Temps d\'exécution pour différents seuils avec l\'algorithme de Strassen',
        xlabel=r'$N\quad(\mathrm{taille\ de\ la\ matrice} = 2^N$)',
        ylabel='Temps d\'exécution (ms)',
    )
    ax.get_xaxis().set_major_locator(plt.MaxNLocator(integer=True))
    plt.savefig(ANALYSIS_OUTPUT_PATH / 'strassen_thresholds.png', bbox_inches='tight')


def run_measure_subcommand():
    df = measure_execution_times(ALGORITHMS)
    print('Execution times of the three different algorithms')
    print(df)

    with open(ANALYSIS_OUTPUT_PATH / 'execution_times.md', 'w') as file:
        file.write(df.to_markdown() + '\n')

    df.to_csv(ANALYSIS_OUTPUT_PATH / 'execution_times.csv')

    plt.figure()
    ax = sns.lineplot(data=df)
    ax.set(
        title='Temps d\'exécution pour chaque algorithme',
        xlabel=r'$N\quad(\mathrm{taille\ de\ la\ matrice} = 2^N$)',
        ylabel='Temps d\'exécution (ms)',
    )
    ax.get_xaxis().set_major_locator(plt.MaxNLocator(integer=True))
    plt.savefig(ANALYSIS_OUTPUT_PATH / 'execution_times.png', bbox_inches='tight')


def run_complexity_subcommand():
    # Load execution time results
    execution_time_results_filename = ANALYSIS_OUTPUT_PATH / 'execution_times.csv'
    try:
        wide_df = pd.read_csv(ANALYSIS_OUTPUT_PATH / 'execution_times.csv', index_col=0)
    except FileNotFoundError:
        print(
            f'Execution time results could not be read (\'{execution_time_results_filename}\'). '
            f'Please run the script with the \'measure\' mode and try again.',
        )

    # Convert dataframe from wide form to long form for plotting with seaborn's lmplot
    long_df = pd.melt(wide_df.reset_index(), id_vars=['N'], var_name='Algorithm', value_name='ExecutionTime')
    long_df['2^N'] = 2 ** long_df['N']
    long_df['log2(2^N)'] = np.log2(long_df['2^N'])
    long_df['log2(ExecutionTime)'] = np.log2(long_df['ExecutionTime'])

    # Power test
    plt.figure()
    ax = sns.lmplot(x='log2(2^N)', y='log2(ExecutionTime)', hue='Algorithm', data=long_df) # Log-log plot
    legend_labels = plt.legend().get_texts()
    for i, algorithm_name in enumerate(ALGORITHMS): # Calculate linear regression equation
        long_df_filtered = long_df[long_df['Algorithm'] == algorithm_name].dropna()
        slope, intercept, _, _, _ = stats.linregress(x=long_df_filtered['log2(2^N)'], y=long_df_filtered['log2(ExecutionTime)'])
        legend_labels[i].set_text(fr'$\log_2(y) = {slope:.4f}\log_2(x){"+" if intercept > 0 else ""}{intercept:.2f}$')
    ax.set(
        title='Test de puissance',
        xlabel=r'$\log_2(\mathrm{taille\ de\ la\ matrice}) = \log_2(2^N) = N$',
        ylabel=r'$\log_2(\mathrm{temps\ d\'exécution})$',
    )
    plt.savefig(ANALYSIS_OUTPUT_PATH / 'power_test.png', bbox_inches='tight')

    # Ratio test
    for algorithm_name in ALGORITHMS:
        long_df_filtered = long_df[long_df['Algorithm'] == algorithm_name].dropna()
        power = THEORETICAL_COMPLEXITY_POWERS[algorithm_name]
        long_df_filtered['y/h(x)'] = long_df_filtered['ExecutionTime'] / long_df_filtered['2^N'] ** power

        with sns.axes_style('whitegrid'):
            plt.figure()
            ax = sns.lineplot(x='2^N', y='y/h(x)', data=long_df_filtered, marker='o')
            ax.set(
                title=f'Test du rapport pour {algorithm_name}',
                xlabel=r'$\mathrm{taille\ de\ la\ matrice} = 2^N$',
                ylabel=fr'$\mathrm{{temps\ d\'exécution}}\ /\ N^{{{round(power, 4)}}}$',
            )
            plt.savefig(ANALYSIS_OUTPUT_PATH / f'ratio_test_{algorithm_name.lower()}.png', bbox_inches='tight')

    # Constants test
    for algorithm_name in ALGORITHMS:
        long_df_filtered = long_df[long_df['Algorithm'] == algorithm_name].dropna()
        power = THEORETICAL_COMPLEXITY_POWERS[algorithm_name]
        long_df_filtered['h(x)'] = long_df_filtered['2^N'] ** power

        slope, intercept, _, _, _ = stats.linregress(x=long_df_filtered['h(x)'], y=long_df_filtered['ExecutionTime'])

        plt.figure()
        ax = sns.lmplot(
            x='h(x)', y='ExecutionTime', data=long_df_filtered,
            line_kws={'label': fr'$y = {slope:.4} \cdot x^{{{round(power, 4)}}}{"+" if intercept > 0 else ""}{intercept:.4f}$'}
        )
        plt.legend()
        ax.set(
            title=f'Test des constantes pour {algorithm_name}',
            xlabel=fr'$\mathrm{{taille\ de\ la\ matrice}}^{{{round(power, 4)}}}$',
            ylabel='temps d\'exécution',
        )
        plt.savefig(ANALYSIS_OUTPUT_PATH / f'constants_test_{algorithm_name.lower()}.png', bbox_inches='tight')


def measure_execution_times(algorithms, trial_count=1, extra_args=[]):
    matrix_filenames = [x for x in DATA_PATH.iterdir() if x.is_file()]
    matrix_n_sizes = sorted(set(int(re.search(r'ex(\d*?)_', filename.name).group(1)) for filename in matrix_filenames))

    results = {}

    for algorithm_name, algorithm_arg in algorithms.items():
        print('Measuring execution time for', algorithm_name)

        results[algorithm_name] = {}

        for n in matrix_n_sizes:
            if MAX_N_SIZES[algorithm_name] is not None and n > MAX_N_SIZES[algorithm_name]:
                break

            matrix_n_size_filenames = sorted(DATA_PATH.glob(f'ex{n}_*'))
            matrix_filename_pairs = list(itertools.combinations(matrix_n_size_filenames, 2))

            execution_times_ms = []
            for matrix_1_filename, matrix_2_filename in matrix_filename_pairs * trial_count:
                result = subprocess.run(
                    ['./tp.sh', '-a', algorithm_arg, '-e1', matrix_1_filename, '-e2', matrix_2_filename, '-t', *extra_args],
                    stdout=subprocess.PIPE,
                )
                execution_times_ms.append(float(result.stdout.decode('utf-8')))

            average_execution_time_ms = statistics.mean(execution_times_ms)
            print(f'\tN: {n} - Average execution time: {average_execution_time_ms}')

            results[algorithm_name][n] = average_execution_time_ms

        print()

    df = pd.DataFrame(results).rename_axis('N')
    return df


def compare_strassen_thresholds():
    results = {}

    for threshold in [2 ** i for i in range(2, 9)]:
        print(f'Threshold: {threshold}')
        df = measure_execution_times({'StrassenThreshold': 'strassenSeuil'}, trial_count=3, extra_args=['--threshold', str(threshold)])
        results[threshold] = df['StrassenThreshold']

    df = pd.DataFrame(results).rename_axis('N')
    return df


if __name__ == '__main__':
    main()
