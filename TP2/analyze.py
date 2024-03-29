#!/usr/bin/env python3

import argparse
from collections import defaultdict
from pathlib import Path
import re
import statistics
import subprocess
import matplotlib.pyplot as plt
import numpy as np
import pandas as pd
from scipy import stats
import seaborn as sns

DATA_INPUT_PATH = Path('data/generated')
ANALYSIS_OUTPUT_PATH = Path('analysis')
ALGORITHMS = {
    'Greedy': 'glouton',
    'BranchAndBound': 'branch_bound',
    'Tabu': 'tabou',
}
MAX_GRAPH_SIZES = {
    'Greedy': None,
    'BranchAndBound': 72,
    'Tabu': None,
}
THEORETICAL_COMPLEXITY_FUNCTIONS = {
    'Greedy': {
        'function': lambda x: np.power(x, 3),
        'string': '{}^3',
    },
    'BranchAndBound': {
        'function': lambda x: np.power(1.25, x),
        'string': '1.25^{}',
    },
    'Tabu': {
        'function': lambda x: np.power(x, 3),
        'string': '{}^3',
    },
}


def main():
    parser = argparse.ArgumentParser()
    subparsers = parser.add_subparsers(title='mode', dest='mode', required=True)
    subparsers.add_parser('measure', help='Measure and save the execution times and color count for the greedy, branch and bound, and tabu search algorithms').set_defaults(func=run_measure_subcommand)
    subparsers.add_parser('complexity', help='Generate graphs for the power, ratio and constants tests based on the execution time results').set_defaults(func=run_complexity_subcommand)
    args = parser.parse_args()

    sns.set_theme(style='ticks', palette='pastel')
    ANALYSIS_OUTPUT_PATH.mkdir(exist_ok=True)

    args.func()


def run_measure_subcommand():
    df_color_counts, df_execution_times = measure_execution_times(ALGORITHMS)
    print('Color counts and execution times for the three different algorithms')
    print(df_color_counts)
    print(df_execution_times)

    with open(ANALYSIS_OUTPUT_PATH / 'color_counts.md', 'w') as file:
        file.write(df_color_counts.to_markdown() + '\n')
    with open(ANALYSIS_OUTPUT_PATH / 'execution_times.md', 'w') as file:
        file.write(df_execution_times.to_markdown() + '\n')

    df_execution_times.to_csv(ANALYSIS_OUTPUT_PATH / 'execution_times.csv')

    plt.figure()
    ax = sns.lineplot(data=df_execution_times)
    ax.set(
        title='Temps d\'exécution pour chaque algorithme',
        xlabel='Nombre de sommets du graphe ($n$)',
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
    long_df = pd.melt(wide_df.reset_index(), id_vars=['GraphSize'], var_name='Algorithm', value_name='ExecutionTime')
    long_df['log2(GraphSize)'] = np.log2(long_df['GraphSize'])
    long_df['log2(ExecutionTime)'] = np.log2(long_df['ExecutionTime'])

    # Power test
    plt.figure()
    ax = sns.lmplot(x='log2(GraphSize)', y='log2(ExecutionTime)', hue='Algorithm', data=long_df) # Log-log plot
    legend_labels = plt.legend().get_texts()
    for i, algorithm_name in enumerate(ALGORITHMS): # Calculate linear regression equation
        long_df_filtered = long_df[long_df['Algorithm'] == algorithm_name].dropna()
        slope, intercept, _, _, _ = stats.linregress(x=long_df_filtered['log2(GraphSize)'], y=long_df_filtered['log2(ExecutionTime)'])
        legend_labels[i].set_text(fr'$\log_2(y) = {slope:.4f}\log_2(x){"+" if intercept > 0 else ""}{intercept:.2f}$')
    ax.set(
        title='Test de puissance',
        xlabel=r'$\log_2(n)$',
        ylabel=r'$\log_2(\mathrm{temps\ d\'exécution})$',
    )
    plt.savefig(ANALYSIS_OUTPUT_PATH / 'power_test.png', bbox_inches='tight')

    # Ratio test
    for algorithm_name in ALGORITHMS:
        long_df_filtered = long_df[long_df['Algorithm'] == algorithm_name].dropna()
        growth_function = THEORETICAL_COMPLEXITY_FUNCTIONS[algorithm_name]
        long_df_filtered['y/h(x)'] = long_df_filtered['ExecutionTime'] / growth_function['function'](long_df_filtered['GraphSize'])

        with sns.axes_style('whitegrid'):
            plt.figure()
            ax = sns.lineplot(x='GraphSize', y='y/h(x)', data=long_df_filtered, marker='o')
            ax.set(
                title=f'Test du rapport pour {algorithm_name}',
                xlabel='Nombre de sommets du graphe ($n$)',
                ylabel=fr'$\mathrm{{temps\ d\'exécution}}\ /\ {{{growth_function["string"].format("n")}}}$',
            )
            plt.savefig(ANALYSIS_OUTPUT_PATH / f'ratio_test_{algorithm_name.lower()}.png', bbox_inches='tight')

    # Constants test
    for algorithm_name in ALGORITHMS:
        long_df_filtered = long_df[long_df['Algorithm'] == algorithm_name].dropna()
        growth_function = THEORETICAL_COMPLEXITY_FUNCTIONS[algorithm_name]
        long_df_filtered['h(x)'] = growth_function['function'](long_df_filtered['GraphSize'])

        slope, intercept, _, _, _ = stats.linregress(x=long_df_filtered['h(x)'], y=long_df_filtered['ExecutionTime'])

        plt.figure()
        ax = sns.lmplot(
            x='h(x)', y='ExecutionTime', data=long_df_filtered,
            line_kws={'label': fr'$y = {slope:.4} \cdot {{{growth_function["string"].format("x")}}}{"+" if intercept > 0 else ""}{intercept:.4f}$'}
        )
        plt.legend()
        ax.set(
            title=f'Test des constantes pour {algorithm_name}',
            xlabel=fr'${{{growth_function["string"].format("n")}}}$',
            ylabel='temps d\'exécution',
        )
        plt.savefig(ANALYSIS_OUTPUT_PATH / f'constants_test_{algorithm_name.lower()}.png', bbox_inches='tight')


def measure_execution_times(algorithms):
    filenames = [x for x in DATA_INPUT_PATH.iterdir() if x.is_file()]
    filenames_by_graph_size = defaultdict(list)
    for filename in filenames:
        size = int(re.search(r'ex(\d*?)_', filename.name).group(1))
        filenames_by_graph_size[size].append(filename)
    filenames_by_graph_size = dict(sorted(filenames_by_graph_size.items()))

    color_count_results = {}
    execution_time_results = {}

    for algorithm_name, algorithm_arg in algorithms.items():
        print('Measuring execution time for', algorithm_name)

        color_count_results[algorithm_name] = {}
        execution_time_results[algorithm_name] = {}

        for graph_size, filenames in filenames_by_graph_size.items():
            if MAX_GRAPH_SIZES[algorithm_name] is not None and graph_size > MAX_GRAPH_SIZES[algorithm_name]:
                break

            color_counts = []
            execution_times_ms = []
            for filename in filenames:
                result = subprocess.run(
                    ['./tp.sh', '-a', algorithm_arg, '-e', filename, '-p', '-t'],
                    stdout=subprocess.PIPE,
                )
                output_lines = result.stdout.decode('utf-8').splitlines()
                color_counts.append(int(output_lines[0]))
                execution_times_ms.append(float(output_lines[2]))

            average_color_count = statistics.mean(color_counts)
            average_execution_time_ms = statistics.mean(execution_times_ms)
            print(
                f'\tGraph node count: {graph_size} - '
                f'Average color count: {average_color_count} - Average execution time: {average_execution_time_ms}'
            )

            color_count_results[algorithm_name][graph_size] = average_color_count
            execution_time_results[algorithm_name][graph_size] = average_execution_time_ms

        print()

    df_color_counts = pd.DataFrame(color_count_results).rename_axis('GraphSize')
    df_execution_times = pd.DataFrame(execution_time_results).rename_axis('GraphSize')
    return df_color_counts, df_execution_times


if __name__ == '__main__':
    main()
