Jour 1 — Cadran / Arithmétique Modulaire
Stratégie : Simulation simple avec rem_euclid(100). Pour p2, utilise une formule mathématique pour compter combien de fois on passe par 0 pendant un mouvement, sans itérer pas à pas.
Performance : ✅ O(n) — parfait, aucun problème.

Jour 2 — IDs de Produits Invalides
Stratégie : Force brute — itère sur chaque nombre de chaque plage et vérifie si c'est un "double" en convertissant en chaîne de caractères.
Performance : ⚠️ Problème majeur — si les plages contiennent des millions de nombres, on boucle sur chacun. L'allocation de n.to_string() dans une boucle chaude est aussi coûteuse. Une approche mathématique serait O(plages × 10) au lieu de O(total des nombres).

Jour 3 — Sous-séquence Maximale
Stratégie : Pile monotone gloutonne — parcourt les chiffres de gauche à droite, dépile quand un chiffre plus grand est trouvé, en gardant uniquement les k meilleurs chiffres.
Performance : ✅ O(n) par ligne — très efficace et élégant.

Jour 4 — Grille / Automate Cellulaire
Stratégie : Trouve répétitivement toutes les cellules @ avec moins de 4 voisins @ ("accessibles"), les supprime et les compte. Comme éplucher un oignon couche par couche.
Performance : ⚠️ Problème modéré — chaque itération rebalaye toute la grille. Une meilleure approche utiliserait une file d'attente avec seulement les candidats en bordure. Correct pour les petites grilles, lent pour les grandes.

Jour 5 — Appartenance à des Plages
Stratégie : p1 vérifie chaque ID contre toutes les plages avec iter().any(). p2 fusionne les plages qui se chevauchent puis somme leurs longueurs.
Performance : ⚠️ p1 est O(ids × plages) — correct si les deux sont petits, mais peut être lent sinon. p2 ✅ O(n log n) — fusion efficace.

Jour 6 — Problèmes Mathématiques en Colonnes
Stratégie : Découpe l'entrée en "blocs" séparés par des colonnes vides, lit les nombres et applique + ou *. p2 lit les nombres colonne par colonne (chiffres verticaux).
Performance : ✅ Aucun problème — parsing O(n) propre et efficace.

Jour 7 — Propagation de Faisceaux
Stratégie : Simule un faisceau qui descend dans une grille. p1 utilise un HashSet de colonnes actives. p2 utilise une HashMap<colonne, compteur> pour savoir combien de faisceaux passent par chaque colonne.
Performance : ✅ O(lignes × colonnes_actives) — utilisation intelligente des sets/maps pour éviter le travail redondant.

Jour 8 — Clustering de Points 3D (DSU)
Stratégie : Calcule toutes les distances entre paires de points, les trie, puis utilise une structure Union-Find (DSU) pour regrouper les points en clusters.
Performance : ⚠️ O(n²) paires — coûteux avec beaucoup de points. p1 a aussi une limite = 1000 codée en dur qui est une supposition, pas une garantie mathématique.

Jour 9 — Plus Grand Rectangle Propre
Stratégie : p1 est force brute O(n²) sur toutes les paires de points. p2 est très sophistiqué — utilise la compression de coordonnées pour construire une grille compressée, un flood fill pour marquer l'extérieur, puis une somme préfixe 2D pour vérifier si un rectangle est entièrement "rempli" en O(1).
Performance : p1 ⚠️ Force brute O(n²). p2 ✅ Très intelligent et efficace pour les grandes coordonnées.

Jour 10 — Optimisation de Pressions de Boutons
Stratégie : p1 teste toutes les 2^m combinaisons de boutons par XOR. p2 est un solveur exact par séparation et évaluation avec mémoïsation, élagage par borne inférieure, borne supérieure gloutonne et sélection de pivot — essentiellement un solveur sur mesure pour un problème de couverture.
Performance : p1 ⚠️ Exponentiel en nombre de boutons — fonctionne seulement si m est petit. p2 ✅ Très avancé — mais la complexité reste théoriquement exponentielle dans le pire cas ; l'élagage le rend rapide en pratique.

Jour 11 — Comptage de Chemins dans un DAG
Stratégie : DFS récursif avec mémoïsation (programmation dynamique descendante) sur un graphe orienté. p2 ajoute deux booléens (seen_dac, seen_fft) à la clé de mémo pour suivre les passages obligatoires.
Performance : ✅ O(nœuds × états) — efficace, DP classique sur un DAG.

Jour 12 — Empaquetage de Tetrominos
Stratégie : Recherche par backtracking / couverture exacte — génère toutes les rotations et réflexions de chaque forme, précalcule tous les placements valides sous forme de masques de bits, puis fait un DFS avec élagage. Utilise un cache d'états morts (HashSet) pour éviter de revisiter des configurations échouées.
Performance : ⚠️ Le pire cas reste exponentiel — problème NP-difficile (couverture exacte). Le cache et l'heuristique MRV (choisir la forme avec le moins de placements valides) aident beaucoup, mais les grandes grilles seront lentes.
