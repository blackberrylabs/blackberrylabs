# CLAUDE.md — Blackberrylabs (la tour de controle)
<!-- Working rules for Claude in this repository (French). Code and docs are in English. -->

Blackberrylabs : un ecran qui dit ce qui cloche sur la machine cote IA, le plus grave en
premier, et ouvre le module Blackberrylabs qui va bien. Quatrieme logiciel de la famille,
apres Cleanator, MCProster et Permitt ; batie sur `bbl-report` (le contrat), `bbl-ui` (le
visage) ; ne depend pas de `hostrules` : la tour ne lit aucun fichier d'hote.

## Regles de travail

- BRIEF valide avant le code ; questions a Alexandre une par une ; PLAN et JOURNAL tenus ;
  `00_cadrage/` hors git.
- La tour ne calcule rien : elle lance les modules, lit leurs rapports, assemble, trie,
  ouvre. Pas de code specifique a un module hors sa fiche declarative (R5).
- Rien d'ecrit hors `~/.config/blackberrylabs/` et `~/.local/state/blackberrylabs/<module>/`.
  Aucune sortie reseau. Actions affichees, jamais executees.
- Commits locaux libres ; `git push`, suppression de fichiers, tags : accord explicite.
  Commandes completes, chemin absolu (`cd /home/nushoko/Bureau/Logiciels/Blackberrylabs && …`).
- Anglais : code, commentaires, commits, README, interface par defaut. Francais : pilotage.
- Seul nom cite : Alexandre Do.

## Identite git

Avant le **premier commit** : `git config user.name "Alexandre Do"` et
`git config user.email "277202463+Nimeria2026@users.noreply.github.com"` (config locale) ;
verifier `git log --format='%an <%ae>'` avant tout push.
