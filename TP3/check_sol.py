#!/usr/bin/env python3

# INF8775 - Analyse et conception d'algorithmes - TP3
#
#   AUTEUR :
#     BURLATS Auguste, 13 Novembre 2021
#
#
#   USAGE :
#     Ce script vérifie le stdout passé en pipe pour conformité avec les exigences du TP3 tel que rédigé à la session A20.
#     Ce même script sera utilisé par les correcteurs pour juger la qualité des programmes développés avec l'appel suivant :
#       $ (timeout 180s ./tp.sh -e FICHIER_EXEMPLAIRE -p; exit 0) | ./check_sol.py -e FICHIER_EXEMPLAIRE [-s FICHIER_SORTIE]
#     où :
#       * "timeout 180s" interrompt, après 3 minutes (180 sec), l'exécution de...
#       * "./tp.sh -e FICHIER_EXEMPLAIRE -c NB_CIRCONSCRIPTIONS -p" tel que...
#       * "FICHIER_EXEMPLAIRE" est l'adresse de l'exemplaire à résoudre et...
#       * "; exit 0" remplace le code de sortie de timeout pour permettre à...
#       * "|" de pipe la sortie de "tp.sh" après interruption par timeout à...
#       * "./check_sol.py" qui prend des paramètres *obligatoirement identiques* à ceux passés à "tp.sh" mais aussi...
#       * admet, de façon facultative, un paramètre "-s" avec le chemin/nom du fichier "FICHIER_SORTIE" où on désire sauvergarder le contenu en pipe.
#
#   EXEMPLES D'USAGE :
#     $ (timeout 180s ./tp.sh -e 66_99.0 -p; exit 0) | ./check_sol.py -e 66_99.0 -s sortie.out
#       cette commande exécute tp.sh pour un max de 180s et passe son affichage à ce présent script, appelé avec les mêmes paramètres pour vérifier le résultat.
#       L'affichage est par ailleurs sauvegardé dans un fichier texte "sortie.out" dans le même dossier.
#       Sauvegarder cet affichage de tp.sh permet de le revérifier ultérieurement, sans avoir à réexécuter le programme, en utilisant par exemple la commande ci-dessous.
#
#     $ cat sortie.out | ./check_sol.py -e 66_99.0
#       sortie.out contient l'affichage d'une exécution antérieure. L'utilisation de "cat" permet de vérifier la solution qu'il contient par ce présent script.
#       Prendre garde à utiliser les mêmes paramètres avec ce script que ceux qui ont été employés lors de la génération de "sortie.out".
#     
#     $ ./tp.sh -e 66_99.0 -p > sortie.out
#     $ cat sortie.out | ./check_sol.py -e 66_99.0
#       Alternativement, il est possible de lancer vos programmes dans un premier temps et d'enregistrer leur sortie avec ">"...
#       pour ensuite les vérifier plus tard avec la deuxième commande.
#
#   ATTENTION:
#     Pour que la commande :
#       $ (timeout 180s ./tp.sh -e FICHIER_EXEMPLAIRE -p; exit 0) | ./check_sol.py -e FICHIER_EXEMPLAIRE [-s FICHIER_SORTIE]
#     prenne en compte toutes vos solutions, il est INDISPENSABLE de flush votre stdout (ie, l'affichage standard de votre programme) À CHAQUE FOIS qu'une solution est trouvée.
#     Pour ce faire, après chaque affichage d'une solution améliorante:
#       * pour python 3: appelez sys.stdout.flush() ou spécifiez l'argument flush=True dans print().
#       * pour C : stdout est flushed automatiquement après un saut de ligne (ie impression de '\n' ou appel de println) ou appelez fflush(stdout).
#       * pour C++ : insérez (<<) à std::cout soit std::endl (qui flush automatiquement après un saut de ligne) soit std::flush.
#       * pour Java : System.out flush automatiquement à chaque saut de ligne ou appelez System.out.flush().
#
#     Il est nécessaire de rendre ce script exécutable en utilisant chmod +x
#     Python 3.5 ou ultérieur recommandé pour lancer ce script.


import sys
import re
import math
import argparse


def load_instance(instance_path):
    with open(instance_path,'r') as instance_stream:
        # Process first line which defines problem characteristics
        line_one = next(instance_stream)
        line_two = next(instance_stream)
        try:
            nb_ecolier = int(line_one)
            nb_paire = int(line_two)
        except:
            return 1

        liste_taille = []
        try: 
            for i in range(nb_ecolier):
                liste_taille.append(int(next(instance_stream)))
        except:
            return 1

        liste_paires = []
        for line in instance_stream:
            liste_paires.append([int(a) for a in line.split()])
            if len(liste_paires[-1]) != 2:
                return 1

        if len(liste_paires) != nb_paire:
            return 1

    return liste_taille, liste_paires


def is_solution_format_valid(raw_solution):
    target_pattern = r'^\s*(?:\d+\s+)*\d+\s*$'
    return bool(re.match(target_pattern, raw_solution))


def parse_solution(raw_solution):
    solution_data = []
    single_solution = []

    for line in raw_solution.splitlines():
        line_contents = line.split()

        if not line_contents:
            continue
        else :
            solution_data.append([int(elem) for elem in line_contents])
    return solution_data


def check_consistency(solutions, liste_taille, liste_paires):

    for (solution, solution_index) in zip(solutions, range(len(solutions))):
        # Check whether number of student is correct
        if len(solution) != len(liste_taille):
            return (1, solution_index)

        #Check if a student is duplicated or missing
        for i in range(len(solution)):
            if solution.count(i+1) !=1:
                return(2, solution_index, i+1)

        #Check if two student that would fight are sitting next to each other
        for student_index in range(len(solution)-1):
            if [solution[student_index], solution[student_index+1]] not in liste_paires and [solution[student_index+1], solution[student_index]] not in liste_paires:
                return(3, solution_index, solution[student_index], solution[student_index+1])
    return 0


def compute_objective(solution, liste_tailles):
    objective = 0
    for i in range(len(solution)-1, 0,-1):
        for j in range(i):
            if liste_tailles[solution[i]-1] < liste_tailles[solution[j]-1]:
                objective += 1
                break

    return objective


if __name__ == '__main__':
    # Parse arguments
    parser = argparse.ArgumentParser()
    parser.add_argument("-e", "--exemplaire", \
                        help="Représente l'exemplaire correspondant à la solution étudiée", \
                        action='store', required=True, metavar='FICHIER_EXEMPLAIRE')
    parser.add_argument("-s", "--sortie", \
                        help="Si indiqué, écrire stdin (ie la sortie de votre programme tel que piped) dans un fichier du nom indiqué", \
                        action='store', required=False, metavar='FICHIER_SORTIE')

    args = parser.parse_args()
    # Store pipe
    piped_content = sys.stdin.read()

    # Save pipe if requested
    try:
        if args.sortie:
            with open(args.sortie,'w') as f_output:
                f_output.write(piped_content)
            print("Info : affichage en pipe sauvegardé dans " + args.sortie + ".")
    except:
        print("Attention : impossible de sauvegarder l'affichage en pipe. Vérifiez les permissions d'écriture. Sauvegarde ignorée.")
    
    # Check whether format is as expected, ie an even number of space-separated integers on every line 
    if not is_solution_format_valid(piped_content):
        print("Erreur : les solutions fournies en pipe à stdin ont un format non valide. Revoyez la convention discutée dans l'énoncé.", file=sys.stderr)
        print("A reçu :", file=sys.stderr)
        print(piped_content, file=sys.stderr)
        sys.exit(1)


    # Load instance corresponding to solution
    instance_data = None
    instance_data = load_instance(args.exemplaire)
    try:
        instance_data = load_instance(args.exemplaire)
    except:
        print("Erreur : impossible d'ouvrir le fichier de l'exemplaire.", file=sys.stderr)
        sys.exit(1)
    
    if instance_data == 1 and instance_data is not None:
        print("Erreur : l'exemplaire fourni en argument à ce script de vérification a un format non valide. "\
            "Vérifiez le chemin et/ou le contenu de l'exemplaire.", file=sys.stderr)
        sys.exit(1)

    # Structure piped solution in memory
    resolution_data = parse_solution(piped_content)
    
    # Check solutions' consistency
    consistency_result = check_consistency(resolution_data, instance_data[0], instance_data[1])
    if consistency_result != 0:
        print("Erreur : une ou plusieurs des solutions fournies en pipe à stdin présentent un problème de cohérence.", file=sys.stderr)

        if consistency_result[0] == 1:
            print("Raison : la solution " + str(consistency_result[1]) \
                  + " (0-indexé) contient un nombre inadéquat d'écoliers.", file=sys.stderr)
        elif consistency_result[0] == 2:
            print("Raison : l'écolier " + str(consistency_result[2]) + " de la solution " + str(consistency_result[1]) \
                  + " (0-indexés) est dupliqué ou absent.", file=sys.stderr)
        elif consistency_result[0] == 3:
            print("Raison : les écoliers " + str(consistency_result[3]) + " et "  \
                  + str(consistency_result[2]) + " de la solution " + str(consistency_result[1]) \
                  + " vont se chamailler.", file=sys.stderr)
        
        sys.exit(1)

    # Satisfied by the solutions' presentation, compute best objective
    objective = compute_objective(resolution_data[-1], instance_data[0])
    print("OK : la valeur de l'objectif de la dernière (ie, meilleure) solution fournie est de " + str(objective) + ".\n")
    print("Nombre de solutions reçues : " + str(len(resolution_data)))

