# PLAN — Blackberrylabs (la tour de controle)
<!-- Project plan (French). -->

## Etape 0. Le contrat
- [x] BRIEF rev. 1 ; T0 nom, T1 tableau de bord « liste de constats », T2 rafraichissement — 2026-10-16
- [x] Validation du BRIEF (T3 gravites, T4 crate a part) — 2026-10-16
- [x] Crate `bbl-report` 0.1.0 : types, echelle de gravite en mots, validation, test de conformite ; depot, tag v0.1.0 — 2026-10-16
- [x] `report` dans Permitt, MCProster, Cleanator (mapping des constats sur les quatre gravites dans chaque CHANGELOG) ; `--select` dans les trois apps ; un commit par module (Permitt 4, MCProster 14, Cleanator 21) — 2026-10-16
- [x] `.gitignore`, LICENSE MIT, README, CHANGELOG ecrits ; depot git local et commit 1 avec le coeur — 2026-10-17

## Etape 1. Coeur et CLI (`bbl`, `bbl modules`, `bbl refresh`, `--json`) ; fixtures de rapports ; tests R2
- [x] `bbl-core` : fiches `modules/*.toml`, decouverte sur PATH, lancement avec limite de temps, rapports enregistres, assemblage et tri — 2026-10-17
- [x] `bbl` : liste, `refresh`, `modules`, `--json`, `--min-severity`, `--module` — 2026-10-17
- [x] Tests sur faux modules (les trois vrais rapports en fixtures) : module absent, rapport enregistre avec son age, R2 (rien hors `state/`), schema 2 et champs en plus, module en echec — 2026-10-17
- [x] Passage reel dans le cloud avec les trois vrais binaires — 2026-10-17
- [ ] Commit 1
## Etape 2. App (maquette validee avant le code)
## Etape 3. Paquets, depot, release 0.1.0 — sur accord ; README d'organisation
