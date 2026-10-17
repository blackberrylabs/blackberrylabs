# JOURNAL — Blackberrylabs (la tour de controle)
<!-- Project journal (French), newest first. -->

---

## 2026-10-16, naissance

- Idee du 07/10 (une app pour piloter tous les projets d'Alexandre), redefinie le 16/10 au
  matin (le point d'entree des outils Blackberrylabs), tranchee le 16/10 au soir : « ce
  n'est pas tous mes projets, c'est la tour de controle pour Blackberrylabs et tous ses
  modules » ; « a terme, la tour de controle d'un developpeur ou d'un vibe codeur sur son
  poste ». Blackberrylabs est le produit ; les outils sont ses modules ; la tour porte le nom.
- Ordre tenu : la fondation d'abord (`hostrules` le soir, `bbl-ui` la nuit), la tour ensuite.
  Alexandre : « bien faire des le depart pour pouvoir integrer d'autres modules plus tard ».
  Traduction dans le BRIEF : soigner le **contrat** tour ↔ module (rapport `bbl-report/1`,
  gravites communes, decouverte declarative, cache ecrit par la tour), pas l'ecran.

## 2026-10-16 — BRIEF valide

BRIEF rev. 1 valide d'un mot (« Je valide ») : T0 a T4 prises. Etape 0 ouverte : la crate
`bbl-report` d'abord (dossier `Logiciels/Bbl_report`, son propre BRIEF), puis `report --json`
dans Cleanator, MCProster et Permitt, et `--select` dans les trois apps. Fiches 00_moi
(bbl-ui, Blackberrylabs) ecrites dans PROJETS.md ; l'ancienne idee `Tour_de_controle`
requalifiee.

## 2026-10-16 — le contrat existe et trois modules le parlent

`bbl-report` 0.1.0 publie ; `permitt report`, `mcproster report`, `cleanator-cli report`
produisent chacun un document conforme, teste sur leurs maisons factices ; les trois apps
prennent `--select`. Ce que l'exercice a appris pour la tour :
- Le grain differe selon le module (un constat par constat chez Permitt et MCProster, un
  par groupe de regle chez Cleanator) ; la tour ne doit rien supposer sur le nombre de
  constats d'un module.
- `open.args` n'est pas toujours `--select x` : Cleanator passe d'abord le chemin scanne.
  La tour lance `open.app` avec `open.args` tels quels, sans les interpreter (R5).
- Les `summary` ont des tons : `good` pour une bonne nouvelle (safe a liberer), `amber`,
  `alert`. La tour affichera le ton du module, pas une regle a elle.
- Les constats `unknown` de Permitt et `disabled` de MCProster arrivent en information : la
  tour doit pouvoir les replier par defaut (filtre « gravite minimale »).
Prochaine etape : le coeur de la tour et `bbl` en ligne de commande, avec les trois vrais
rapports en fixtures.

## 2026-10-17 — le coeur et `bbl`

- `bbl-core` en quatre morceaux : `spec` (les fiches de modules, TOML declaratif embarque :
  id, nom, verbe, accroche, binaires, arguments du rapport, limite de temps — 900 s pour
  Cleanator qui scanne le disque, 60 s pour les autres —, schema, page, releases),
  `discover` (`which` sur PATH, rien d'autre), `run` (lance `<cli> report`, vide stdout et
  stderr sur deux fils pour qu'un module bavard ne bloque pas, tue au-dela de la limite,
  verifie la sortie avec `conformance::check`), `cache` (`~/.local/state/blackberrylabs/
  <module>/report.json`, ecriture a cote puis renommage, age par la date du fichier,
  `$BBL_STATE_DIR` pour les tests), `dashboard` (etats par module : frais, enregistre avec
  age, sans rapport, non installe, en echec ; lignes triees gravite → ordre de la famille →
  titre).
- `bbl` ne lance rien tout seul : il montre les rapports enregistres ; `bbl refresh` lance.
  Choix delibere pour la ligne de commande — previsible ; c'est l'app qui relancera en
  arriere-plan (T2).
- Passage reel dans le cloud : les trois vrais binaires sur un PATH temporaire. Le
  `cleanator-cli` 0.1.1 encore installe dans le cloud a repondu « unrecognized subcommand
  'report' » — exactement le cas d'un module plus vieux que le contrat, affiche comme
  echec avec la premiere ligne de stderr. Avec le binaire a jour : 23 constats Cleanator,
  2 MCProster, 45 Permitt, tries ensemble.
- Decouverte au passage : Cleanator citait parfois une phrase comme source ; corrige dans
  Cleanator (commit 22 : une source qui n'est pas une URL devient une indication).
- Un echec de rafraichissement n'est pas persiste : au prochain `bbl`, le module est « sans
  rapport » ou garde son ancien rapport. L'app montrera l'erreur en direct.
