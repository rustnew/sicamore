(Due to technical issues, the search service is temporarily unavailable.)

### Description de l'Application

Cette application est une **liste de tâches (Todo List)** construite avec le framework **Sycamore** en Rust. Elle permet à l'utilisateur de :

1. **Ajouter des tâches** : L'utilisateur peut saisir une nouvelle tâche et l'ajouter à la liste en appuyant sur la touche `Entrée`.
2. **Marquer une tâche comme terminée** : En cliquant sur une tâche, l'utilisateur peut la marquer comme terminée (ou la réactiver).
3. **Modifier une tâche** : L'utilisateur peut cliquer sur le bouton "Edit Task" pour modifier le texte d'une tâche.
4. **Supprimer une tâche** : Chaque tâche peut être supprimée en cliquant sur le bouton "Remove".
5. **Persistance des données** : Les tâches sont sauvegardées dans le `localStorage` du navigateur, ce qui permet de conserver la liste même après un rafraîchissement de la page.

---

### Fonctionnalités Techniques

- **Framework** : Sycamore (framework frontend Rust pour créer des applications web réactives).
- **État réactif** : Utilisation de signaux (`Signal`) pour gérer l'état des tâches (texte, état de complétion, etc.).
- **Persistance** : Les tâches sont sauvegardées dans le `localStorage` du navigateur.
- **Optimisation des rendus** : Utilisation du composant `Keyed` pour optimiser les mises à jour de la liste des tâches.
- **Gestion des événements** : Gestion des clics et des événements clavier (comme la touche `Entrée`).

---

### Structure du Code

1. **`Todo`** : Une structure représentant une tâche avec :
   - `task` : Le texte de la tâche (géré par un signal).
   - `completed` : Un booléen indiquant si la tâche est terminée (géré par un signal).
   - `id` : Un identifiant unique pour chaque tâche.

2. **`TodoItem`** : Un composant qui affiche une tâche individuelle. Il permet de :
   - Marquer la tâche comme terminée.
   - Modifier le texte de la tâche.
   - Supprimer la tâche.

3. **`TodoList`** : Un composant qui affiche la liste des tâches en utilisant `Keyed` pour optimiser les rendus.

4. **`TodoInput`** : Un composant qui permet à l'utilisateur d'ajouter une nouvelle tâche.

5. **`App`** : Le composant principal qui gère l'état global de l'application, y compris :
   - La récupération des tâches depuis le `localStorage`.
   - La sauvegarde des tâches dans le `localStorage`.
   - La gestion des identifiants uniques pour les tâches.

---

### Comment Exécuter et Compiler l'Application

#### Prérequis

1. **Installer Rust** : Si vous n'avez pas Rust installé, suivez les instructions sur [rustup.rs](https://rustup.rs/).
2. **Installer Trunk** : Trunk est un outil pour compiler et servir des applications web en Rust. Installez-le avec :
   ```bash
   cargo install trunk
   ```

#### Étapes pour Exécuter l'Application

1. **Cloner le dépôt** :
   ```bash
   git clone https://github.com/votre-utilisateur/votre-depot.git
   cd votre-depot
   ```

2. **Compiler et servir l'application** :
   Utilisez Trunk pour compiler et servir l'application :
   ```bash
   trunk serve
   ```

3. **Ouvrir dans le navigateur** :
   Trunk démarre un serveur local. Ouvrez votre navigateur et accédez à :
   ```
   http://localhost:8080
   ```

---

### Exemple de `README.md` pour GitHub

Voici un exemple de contenu pour votre fichier `README.md` :

```markdown
# Todo List avec Sycamore

Une application de liste de tâches (Todo List) construite avec le framework Sycamore en Rust.

## Fonctionnalités

- Ajouter des tâches.
- Marquer une tâche comme terminée.
- Modifier une tâche.
- Supprimer une tâche.
- Persistance des données dans le `localStorage`.

## Comment Exécuter

### Prérequis

- [Rust](https://rustup.rs/)
- [Trunk](https://trunkrs.dev/)

### Étapes

1. Cloner le dépôt :
   ```bash
   git clone https://github.com/votre-utilisateur/votre-depot.git
   cd votre-depot
   ```

2. Compiler et servir l'application :
   ```bash
   trunk serve
   ```

3. Ouvrir dans le navigateur :
   ```
   http://localhost:8080
   ```

## Structure du Code

- **`Todo`** : Structure représentant une tâche.
- **`TodoItem`** : Composant pour afficher une tâche individuelle.
- **`TodoList`** : Composant pour afficher la liste des tâches.
- **`TodoInput`** : Composant pour ajouter une nouvelle tâche.
- **`App`** : Composant principal gérant l'état global.

## Contribuer

Les contributions sont les bienvenues ! Ouvrez une issue ou soumettez une pull request.

## Licence

Ce projet est sous licence MIT. Voir le fichier `LICENSE` pour plus de détails.
```

---

### Conclusion

Cette application est un excellent exemple de la manière dont Sycamore peut être utilisé pour créer des applications web réactives et performantes en Rust. Elle est facile à exécuter et à personnaliser, et elle démontre plusieurs concepts clés de Sycamore, tels que la gestion de l'état réactif, les composants réutilisables et la persistance des données.
