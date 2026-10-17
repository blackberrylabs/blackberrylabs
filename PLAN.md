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
- [x] Depot `blackberrylabs/blackberrylabs`, tag v0.1.0, release avec les trois paquets + SHA256SUMS ; verifiee depuis le cloud : somme OK, `bbl 0.1.0`, le `.deb` publie lit les trois `.deb` publies des modules — 2026-10-17
- [x] Les trois modules publies avant : Permitt 0.2.0, MCProster 0.2.1, Cleanator 0.2.0 — 2026-10-17
- [x] README d'organisation publie dans `blackberrylabs/.github` (`profile/README.md`), visible sur github.com/blackberrylabs — 2026-10-17

**Etape 3 close le 2026-10-17. Blackberrylabs 0.1.0 est publiee, avec ses trois modules.**

## Etape 4. Integration facile (decision T5, 2026-10-17)
- [x] `docs/new-module.md` : template, contrat, gravites et mappings existants, `--select`, fiche, avant la release — 2026-10-17
- [x] Gabarit `blackberrylabs/template` (`Logiciels/Template`) : core/cli/app, promesses testees, `report` conforme, `rename.sh` — 2026-10-17
- [x] Mise a jour de toute la doc de la famille : README des trois modules (« Part of Blackberrylabs », statut), CLAUDE.md et CONTRIBUTING (fondations, mapping), README des trois fondations, CONVENTIONS et PROJETS 00_moi — 2026-10-17
- [ ] Commits et pushes (docs seulement) — sur accord

## Apres 0.1.0 (idees, non engagees)
- Lire aussi des fiches deposees dans `~/.config/blackberrylabs/modules/` (modules tiers), en plus des fiches embarquees.
- La tour comme porte d'entree de la famille (idee d'Alexandre, 17/10) : « Get it » ouvre directement le bon paquet de la bonne release (format detecte via `/etc/os-release`), le navigateur telecharge ; R3 intact. README d'organisation qui presente la tour en premier.
- Verifier l'installation d'un module apres « Get it » (relecture du PATH a la demande).
