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
- [x] Commit 1 — 2026-10-17
## Etape 2. App (maquette validee avant le code)
- [x] Maquette HTML (en marche + premier lancement), validee d'un mot — 2026-10-17
- [x] `bbl-app` (iced, bbl-ui) : phrase, quatre gravites, cartes de modules, liste, detail, ouvrir dans le module ; rafraichissement en arriere-plan (T2) ; EN/FR ; clair/sombre — 2026-10-17
- [x] Captures docs/screenshots (clair, sombre) — 2026-10-17
- [x] Commit 2 (f4f780a) — 2026-10-17
## Etape 3. Paquets, depot, release 0.1.0 — sur accord ; README d'organisation
- [x] `packaging/build.sh`, `.desktop`, icone (radar + quatre gravites), metadonnees deb/rpm — 2026-10-17
- [x] Passage reel sur la machine d'Alexandre (binaires reconstruits dans ~/.local/bin) : Cleanator 22 constats en 5,1 s, MCProster 1, Permitt 12 ; 0 a agir, 17 a regarder, 7 bon a savoir, 11 informations — 2026-10-17
- [ ] Depot `blackberrylabs/blackberrylabs`, release 0.1.0 — accord du 2026-10-17 ; les trois modules publies avant (Permitt 0.2.0, MCProster 0.2.1, Cleanator 0.2.0, verifies depuis le cloud) : commit 3, build.sh, push, tag, release
- [ ] README d'organisation (profil GitHub blackberrylabs)

## Apres 0.1.0 (idees, non engagees)
- La tour comme porte d'entree de la famille (idee d'Alexandre, 17/10) : « Get it » ouvre directement le bon paquet de la bonne release (format detecte via `/etc/os-release`), le navigateur telecharge ; R3 intact. README d'organisation qui presente la tour en premier.
- Verifier l'installation d'un module apres « Get it » (relecture du PATH a la demande).
