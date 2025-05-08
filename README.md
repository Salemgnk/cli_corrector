# CLI Corrector
CLI Corrector est une interface en ligne de commande (CLI) intelligente développée en Python pour aider les développeurs à corriger automatiquement les commandes mal tapées (ex. `gti` → `git`). Elle propose des suggestions rapides, enregistre un historique des corrections, et peut suggérer des alias pour les erreurs fréquentes. Ce projet est open source et conçu pour être simple, rapide et extensible.


## Fonctionnalités

- **Correction automatique :** Détecte les typos dans les commandes CLI en utilisant la distance de Levenshtein.
- **Suggestions intelligentes :** Propose des commandes similaires basées sur les outils installés ($PATH).
- **Historique :** Stocke les corrections dans un fichier JSON pour une personnalisation future.
- **Alias :** Recommande des alias shell pour les erreurs répétées (ex. alias gti='git').
- **Léger et rapide :** Optimisé pour une exécution en moins de 0,5 seconde.

## Prérequis

- Python 3.8 ou supérieur
- Systèmes supportés : Linux, macOS, Windows (WSL recommandé)
- Bibliothèques Python : prompt_toolkit, difflib (inclus dans Python)

## Installation

1. Clonez le dépôt :
```bash
git clone git@github.com:Salemgnk/cli_corrector.git
cd cli-corrector
```

2. Créez un environnement virtuel (optionnel, recommandé) :
```bash
python -m venv venv
source venv/bin/activate  # Linux/macOS
venv\Scripts\activate     # Windows
```

3. Installez les dépendances :
```bash
pip install -r requirements.txt
```

4. Lancez la CLI :
```bash
python cli_corrector.py
```


## Utilisation
Tapez une commande dans la CLI Corrector, et si elle est mal écrite, une suggestion sera affichée.
**Exemple :**
``` bash
$ gti status
Tu voulais dire 'git status' ?
```
Pour activer une suggestion, suivez les instructions affichées. L’historique des corrections est sauvegardé dans `cli_corrector_history.json`.

## Contribuer
Nous accueillons les contributions ! Voici comment commencer :

1. Forkez le dépôt.

2. Créez une branche pour votre fonctionnalité (`git checkout -b feature/nouvelle-fonction`).

3. Committez vos changements (`git commit -m "Ajout de nouvelle fonctionnalité"`).

4. Poussez votre branche (`git push origin feature/nouvelle-fonction`).
Ouvrez une Pull Request.

## Feuille de route

- [] Intégration d’un modèle IA léger pour des suggestions contextuelles.
- [] Support pour l’analyse des arguments de commande (ex. gti status → git status).
- [] Interface avec bash/zsh pour une correction en temps réel.
- [] Génération automatique d’alias dans .bashrc ou .zshrc.

## Licence
Ce projet est sous licence MIT. Vous êtes libre de l’utiliser, le modifier et le distribuer.

## Contact
Pour toute question ou suggestion, ouvrez une issue sur GitHub ou contactez `gnandisalem@gmail.com`.

⭐ Si vous aimez ce projet, donnez-lui une étoile sur GitHub !
