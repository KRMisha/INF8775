#!/usr/bin/env python3

# INF8775 - Analyse et conception d'algorithmes
#   TP1 - Multiplication de Matrices
#
#   AUTEUR :
#     NAHON, Remi - 10 septembre 2021
#
#   RESUME DES CHANGEMENTS :
#     10/09/2021 - Disponibilite initiale.
#
#   USAGE :
#     Ce script genere les exemplaires requis pour le TP1 portant sur le probleme de la multiplication de matrices.
#
#     $ ./inst_gen.py [-h] -S TAILLE_MIN [-t NB_TAILLES] [-n NB_EXEMPLAIRES] [-r RANDOM_SEED]
#
#     où :
#       * TAILLE_MIN est la premiere taille des matrices, qui seront de taille 2^TAILLE_MIN*2^TAILLE_MIN
#       * NB_TAILLES est le nombre de tailles consécutives à TAILLE_MIN (incluse) que l'on souhaite construire (par defaut)
#       * NB_EXEMPLAIRES est le nombre d'exemplaires differents construits par taille de matrice (par defaut 5).
#
#     Il est necessaire de rendre ce script executable en utilisant chmod +x
#     Python 3.5 ou ulterieur recommande pour lancer ce script.

import random
import argparse

if __name__ == "__main__":
    # Parse arguments
    parser = argparse.ArgumentParser()
    parser.add_argument("-S", "--taille_min", required=True, type=int,
                        help="Represente la premiere taille des matrices, qui seront de taille 2^S*2^S")
    parser.add_argument("-t", "--nb_tailles", required=False, default=5, type=int,
                        help="Represente le nombre de tailles consécutives des matrices à construire : \
                        pour t, on aura des matrices de tailles allant de 2^S*2^S a 2^(S+t-1)*2^(S+t-1)")
    parser.add_argument("-n", "--nb_exemplaires",required=False, default=5, type=int,
                        help="Represente le nombre d'exemplaires d'une meme taille a generer")
    parser.add_argument("-r", "--random_seed", required=False, default=1, type=int,
                        help="Represente le germe du generateur de nombres aléatoires")
    args = parser.parse_args()


    # Parameters
    min_v = 0
    max_v = 5
    a = random.seed(args.random_seed)
    taille_min = args.taille_min
    nb_tailles = args.nb_tailles
    nb_exemplaires = args.nb_exemplaires

    # Generate
    for i in range(nb_exemplaires):
        for j in range(taille_min, taille_min+nb_tailles):
            with open('ex' + str(j) + '_' + str(i), 'w') as inst:
                inst.write("%d\n" % j)
                for k in range(2**j):
                    for l in range((2**j)-1):
                        v = random.randint(min_v, max_v)
                        inst.write("%d " % v)
                    v = random.randint(min_v, max_v)
                    inst.write("%d\n" % v)

