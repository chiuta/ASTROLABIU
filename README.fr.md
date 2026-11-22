# ASTROLABIU — Moteur d'événements célestes : échelles de temps, éclipses, lever/coucher et calendriers — Rust pur, MIT.

[English](README.md) · [Română](README.ro.md) · **Français** · [Italiano](README.it.md) · [Español](README.es.md) · [Português](README.pt.md) · [Deutsch](README.de.md) · [Magyar](README.hu.md)

Version : 0.1.0-alpha.1 · Dernière mise à jour : 2026-11-22

🌐 https://astrolabiu.chiuta.ro

## État

Squelette précoce (pré-alpha) : le workspace compile et les tests passent en CI. Chaque fonctionnalité prévue est une tâche ouverte dans `docs/issues/`.

## Périmètre prévu

- Échelles de temps (UTC, TAI, TT) avec table des secondes intercalaires, ΔT pour −1999…3000 et conversions en jour julien, y compris le passage de 1582.
- Position du Soleil (longitude apparente et déclinaison) et de la Lune à partir de la série de Meeus tronquée.
- Phases de la Lune : prochaine nouvelle lune, pleine lune et quartiers.
- Lever, coucher, passage au méridien et crépuscule pour toute latitude et longitude, et circonstances des éclipses solaires à partir des éléments de Bessel.
- Calendriers (julien, hébraïque, islamique tabulaire, persan, semaine ISO), un trait `PlanetProvider` et une page de démonstration WebAssembly.

Rien de tout cela n’est encore implémenté : chaque point est une tâche ouverte dans `docs/issues/`.

## Compiler et tester

```bash
cargo test --workspace
cargo run -p astrolabiu-cli -- jd 2027 8 2.5
```

## Contribuer

Lisez `CONTRIBUTING.md`, choisissez une tâche étiquetée `good first issue` dans `docs/issues/` et ouvrez une pull request. Aucune contribution n'est trop petite : une ligne de table, un test ou une traduction comptent.

## Implémentation indépendante

Écrite à partir de spécifications publiques et de manuels ; aucun code source tiers n'a été copié (voir `CLEANROOM.md`).

## Licence

MIT — Copyright (c) 2026 Alexandru-Ionuț Chiuță (Alexio).

## Soutien

Trade-free : ni pistage ni publicité. Si cela vous est utile, vous pouvez soutenir l'auteur : https://buymeacoffee.com/echo.of.the.strings
