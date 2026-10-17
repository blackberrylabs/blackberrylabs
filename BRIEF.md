# BRIEF — la tour de contrôle Blackberrylabs
<!-- Project brief (French). Code, comments, commits and README are in English. -->

Rev. 1 — 2026-10-16, valide le 2026-10-16 (« Je valide », T0 a T4 prises). Quatrieme logiciel
de la famille, le premier qui ne lit rien lui-meme.

## 1. Objet

**Un seul ecran qui dit a un developpeur ou un vibe codeur ce qui cloche sur sa machine cote
IA — le plus grave en premier — et d'ou il ouvre l'outil Blackberrylabs qui va bien.**

La tour ne calcule rien : elle lance les modules installes, lit leurs rapports, les assemble
en une liste de constats triee, et ouvre le module concerne au bon endroit. Elle est aussi
la vitrine de la famille : un module absent est montre avec ou le prendre.

Accroche : *Blackberrylabs — everything your AI agents do to your machine, on one screen.
Read-only, like the tools behind it.*

## 2. Pourquoi

- La recherche du 16/10 : les gens arrivent **apres** l'incident, par un moteur de recherche,
  avec une question (disque plein, agent qui supprime, outil qui agit sans demander). Ils
  n'ouvrent pas trois inspecteurs ; ils veulent une reponse en dix secondes.
- Alexandre, 16/10 : « Blackberrylabs, c'est comme un vrai petit projet avec plein d'outils
  MVP qu'on va ameliorer avec le temps » ; « a terme, la tour de controle d'un developpeur ou
  d'un vibe codeur sur son poste ». La tour est le produit ; les outils sont ses modules.
- Convention du 16/10 : des outils separes, une fondation commune, « la tour rassemble sans
  absorber ».

## 3. Posture

Decision T1 (16/10, « B version liste de constats ») : la tour est un **tableau de bord**,
pas un hall. Quatre chiffres en haut, une liste de constats tous modules confondus, triee
par gravite, chaque ligne ouvrant le module au bon endroit. Pas de croisement entre modules
en v1 : ils viendront quand cinq ou six modules montreront lesquels se repondent.

Ce qu'elle n'est pas : un quatrieme inspecteur (elle ne lit aucun fichier d'hote, ne scanne
aucun disque), un gestionnaire (elle n'agit jamais), un pilote des projets d'Alexandre
(tranche le 16/10 : Blackberrylabs seulement).

## 4. Le contrat tour ↔ module — le coeur de ce BRIEF

« Bien faire des le depart » (Alexandre, 16/10) = soigner le contrat, pas l'ecran. Il doit
tenir quand le dixieme module arrivera.

### 4.1 Un module = un binaire qui produit un rapport

`<module> report --json` ecrit sur stdout un document `bbl-report/1`. Les trois modules
existants l'ajoutent (etape 0) : `cleanator-cli report`, `mcproster report`, `permitt report`.
Leurs `--json` actuels restent ; `report` est la vue normalisee.

### 4.2 Le rapport (`bbl-report/1`)

```jsonc
{
  "schema": "bbl-report/1",
  "module": { "id": "permitt", "name": "Permitt", "version": "0.1.0",
              "verb": "inspect",                       // clean | inventory | inspect | …
              "tagline": "What your AI agents may do without asking",
              "app": "permitt-app", "cli": "permitt",
              "home": "https://github.com/blackberrylabs/permitt" },
  "generated_at": "2026-10-16T21:03:00Z",
  "duration_ms": 120,
  "summary": [                                         // 2 a 4 chiffres, ceux des cartes KPI
    { "key": "hosts", "label": "Hosts installed", "value": "4", "tone": "neutral" },
    { "key": "auto",  "label": "Run without asking", "value": "6", "tone": "alert" }
  ],
  "findings": [
    { "id": "runs-without-asking/opencode/commands",   // stable dans le module : la tour s'en sert pour « deja vu »
      "severity": 2,                                   // 1 agir maintenant · 2 a regarder · 3 bon a savoir · 4 information
      "kind": "runs-without-asking",                   // le slug du module, tel quel
      "title": "OpenCode runs shell commands without asking",
      "subject": { "kind": "host", "label": "OpenCode" },   // host | server | path | file | setting
      "detail": "by default, nothing changed it — « Most permissions default to \"allow\" »",
      "evidence": [ { "label": "~/.config/opencode/opencode.json", "path": "/home/…/opencode.json", "line": null } ],
      "actions": [ { "kind": "hint", "label": "Add a permission block in ~/.config/opencode/opencode.json" },
                   { "kind": "open-file", "label": "Open file", "target": "/home/…/opencode.json" } ],
      "open": { "app": "permitt-app", "args": ["--select", "opencode:commands"] }   // le module au bon endroit
    }
  ],
  "notes": [ "3 shell files read", "Cursor's Run Mode is not readable" ],   // phrases courtes, facultatif
  "errors": []                                         // ce qui n'a pas pu etre lu
}
```

Regles du rapport :
- **Gravite commune, quatre niveaux**, definis en mots dans la crate : 1 = agir maintenant
  (perte de donnees possible, secret en clair, contournement dans un shell) ; 2 = a regarder
  (agit sans demander, acces large, gros volume recuperable) ; 3 = bon a savoir (doublon,
  version qui differe, liste de refus seule) ; 4 = information (desactive, pas en cache).
  Chaque module **mappe ses propres constats** sur cette echelle ; la tour ne reinterprete pas.
- **Rien d'inedit dans le rapport** : il ne contient que ce que le module affiche deja. Pas de
  secret (les modules masquent avant), pas de chemin que le module ne montrerait pas.
- **Les actions sont affichees, jamais executees** par la tour — meme regle que partout.
- **Versionne** : `bbl-report/1`. Un champ ajoute = meme version (les lecteurs ignorent
  l'inconnu). Un champ retire ou change de sens = `bbl-report/2`, et la tour lit les deux.

### 4.3 La crate `bbl-report`

Troisieme fondation, avec `hostrules` (le coeur) et `bbl-ui` (le visage) : les types Rust du
rapport, la serialisation, la validation, l'echelle de gravite en mots, et un test de
conformite que chaque module execute sur sa propre sortie. Depot `blackberrylabs/bbl-report`,
MIT, dependance git par tag. Nom verifie libre sur crates.io.

### 4.4 La decouverte des modules

Une liste declarative dans la tour, meme esprit que les fiches d'hotes : id, nom, verbe,
binaires attendus (`cli`, `app`), commande de rapport, version minimale de schema, page
d'accueil, paquets. Un module est **installe** si son binaire est sur le `PATH` (ou a un
chemin declare). Absent : la tour le montre quand meme, avec sa description et ou le
prendre — la vitrine. Ajouter un module = une fiche + un binaire conforme.

### 4.5 Le rafraichissement (decision T2, 16/10)

A l'ouverture, la tour **affiche le dernier rapport enregistre, avec son age**, puis
**relance chaque module en arriere-plan** et remplace quand c'est pret. Relance manuelle
toujours possible, par module ou globale. Les rapports enregistres vivent dans
`~/.local/state/blackberrylabs/<module>/report.json` et sont **ecrits par la tour**, pas
par les modules : leurs promesses (rien d'ecrit hors leur propre dossier de reglages)
restent intactes. Un module lent (Cleanator scanne le disque) montre un etat « en cours »
sans bloquer les autres.

### 4.6 Ouvrir au bon endroit

`open.app` + `open.args` : chaque app accepte un argument de selection (`--select …`), a
ajouter a l'etape 0 (Permitt a deja l'equivalent par variable d'environnement). Si l'app n'est
pas installee, la tour affiche la commande.

## 5. L'ecran (a ne pas surconcevoir)

Meme visage que les trois modules (`bbl-ui`) : cartes KPI en haut (les `summary` de chaque
module, agrégés simplement : un chiffre par module ou les quatre plus parlants), la liste de
constats au centre (gravite en pastille, module en etiquette, titre, sujet ; clic → detail a
droite : `detail`, preuves, actions, bouton « Open in <module> »), les modules en bas ou en
colonne (installes, version, age du rapport, « rafraichir » ; absents avec « ou le prendre »).
Filtres : par module, par gravite minimale, texte. Pied : statut, EN·FR, theme.

Ce qui n'a **pas** besoin d'etre tranche maintenant : la disposition exacte, l'agregation des
KPI, un historique (« deja vu » grace aux `id`), des notifications. Maquette validee avant le
code de l'app, comme pour les autres.

## 6. Regles

- **R1 La tour ne lit aucun fichier d'hote et ne scanne rien** : elle lance, lit, assemble,
  trie, ouvre.
- **R2 Rien d'ecrit** hors `~/.config/blackberrylabs/` (reglages d'interface) et
  `~/.local/state/blackberrylabs/<module>/report.json` (rapports enregistres).
- **R3 Aucune sortie reseau.** Les liens « ou le prendre » sont affiches, pas suivis.
- **R4 Les actions sont affichees, jamais executees.**
- **R5 Un module se branche par le contrat seulement** : pas de code specifique a un module
  dans la tour, hors sa fiche declarative.
- **R6 Anglais partout, francais en option**, visage `bbl-ui`.

## 7. Etapes

0. **Le contrat** : crate `bbl-report` (types, gravite, validation, test de conformite) ;
   `report --json` dans Cleanator, MCProster, Permitt (avec le mapping de leurs constats sur
   les quatre gravites, ecrit dans chaque CHANGELOG) ; `--select` dans les trois apps. Un
   commit par module. C'est l'etape la plus longue et la plus importante.
1. **La tour, coeur et CLI** : fiches de modules, lancement, lecture, cache, tri ; `bbl`
   (ou le nom choisi) en ligne de commande : `bbl` (la liste), `bbl modules`, `bbl refresh`,
   `--json`. Faux rapports en fixtures ; tests : « rien n'est ecrit hors R2 », « un rapport
   inconnu (schema 2, champ en plus) est lu sans casser », « un module absent est montre ».
2. **L'app** (maquette validee avant le code).
3. Paquets, depot, release 0.1.0 — sur accord. README d'organisation Blackberrylabs qui
   presente la famille et renvoie a chaque module.

## 8. Hors perimetre (v1)

Croisements entre modules ; historique et tendances ; notifications ; execution d'actions ;
modules non Blackberrylabs ; pilotage des projets d'Alexandre (autre chose, autre nom si un
jour).

## 9. Decisions

- **T0 2026-10-16** — Le nom : l'application s'appelle **Blackberrylabs** (binaire `bbl`, app
  `bbl-app`) : la tour porte le nom de la famille parce qu'elle *est* le produit ; les
  modules gardent leurs noms. (`bbl`, `bblabs`, `blackberrylabs`, `bbl-report` libres sur
  crates.io.) Depot `blackberrylabs/blackberrylabs`, dossier local `Logiciels/Blackberrylabs`.
- **T1 2026-10-16** — Tableau de bord, version « une liste de constats », pas un hall ; pas de
  croisement en v1.
- **T2 2026-10-16** — Dernier rapport affiche avec son age, relance en arriere-plan,
  rapports ecrits par la tour dans `~/.local/state/blackberrylabs/`.
- **T3 2026-10-16** — Quatre niveaux de gravite communs, mappes par chaque module.
- **T4 2026-10-16** — `bbl-report` est une crate a part (ni dans `hostrules`, ni dans
  `bbl-ui`) : ce n'est ni lecture d'hotes ni interface, c'est le contrat.
- **T5 2026-10-17** — « S'integre facilement » s'ecrit : un gabarit de depot
  (`blackberrylabs/template`, qui parle deja le contrat et garde deja les promesses) et une
  page `docs/new-module.md` dans la tour (les trois obligations d'un module : `report`,
  mapping des gravites ecrit dans le CHANGELOG, `--select` ; puis la fiche). Decide avec
  Alexandre apres la 0.1.0 (« Oui, pour ta recommandation »).
