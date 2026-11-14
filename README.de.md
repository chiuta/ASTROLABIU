# ASTROLABIU — Himmelsereignis-Engine: Zeitskalen, Finsternisse, Auf-/Untergang und Kalender — reines Rust, MIT.

[English](README.md) · [Română](README.ro.md) · [Français](README.fr.md) · [Italiano](README.it.md) · [Español](README.es.md) · [Português](README.pt.md) · **Deutsch** · [Magyar](README.hu.md)

Version: 0.1.0-alpha.1 · Letzte Aktualisierung: 2026-11-14

🌐 https://astrolabiu.chiuta.ro

## Status

Frühes Grundgerüst (Pre-Alpha): Das Workspace baut, und die Tests laufen in der CI durch. Jede geplante Funktion ist eine offene Aufgabe in `docs/issues/`.

## Geplanter Umfang

- Zeitskalen (UTC, TAI, TT) mit Schaltsekundentabelle, ΔT für −1999…3000 und Umrechnung in Julianische Tage, einschließlich der Umstellung 1582.
- Position der Sonne (scheinbare Länge und Deklination) und des Mondes aus der gekürzten Meeus-Reihe.
- Mondphasen: nächster Neumond, Vollmond und Viertel.
- Auf-, Untergang, Kulmination und Dämmerung für beliebige Breite und Länge sowie Sonnenfinsternis-Umstände aus Besselschen Elementen.
- Kalender (julianisch, hebräisch, islamisch tabellarisch, persisch, ISO-Woche), ein `PlanetProvider`-Trait und eine WebAssembly-Demoseite.

Nichts davon ist bisher umgesetzt: Jeder Punkt ist eine offene Aufgabe in `docs/issues/`.

## Bauen und testen

```bash
cargo test --workspace
cargo run -p astrolabiu-cli -- jd 2027 8 2.5
```

## Mitmachen

Lies `CONTRIBUTING.md`, wähle in `docs/issues/` eine Aufgabe mit dem Label `good first issue` und öffne einen Pull Request. Kein Beitrag ist zu klein: eine Tabellenzeile, ein Test oder eine Übersetzung zählt.

## Unabhängige Implementierung

Geschrieben nach öffentlichen Spezifikationen und Lehrbüchern; es wurde kein fremder Quellcode kopiert (siehe `CLEANROOM.md`).

## Lizenz

MIT — Copyright (c) 2026 Alexandru-Ionuț Chiuță (Alexio).

## Unterstützung

Trade-free: kein Tracking, keine Werbung. Wenn es dir nützt, kannst du den Autor unterstützen: https://buymeacoffee.com/echo.of.the.strings
