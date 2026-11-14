# ASTROLABIU — Motor pentru evenimente cerești: scări de timp, eclipse, răsărit/apus și calendare — Rust pur, MIT.

[English](README.md) · **Română** · [Français](README.fr.md) · [Italiano](README.it.md) · [Español](README.es.md) · [Português](README.pt.md) · [Deutsch](README.de.md) · [Magyar](README.hu.md)

Versiune: 0.1.0-alpha.1 · Ultima actualizare: 2026-11-14

🌐 https://astrolabiu.chiuta.ro

## Stare

Schelet timpuriu (pre-alfa): workspace-ul se compilează, iar testele trec în CI. Fiecare funcție planificată este o sarcină deschisă în `docs/issues/`.

## Domeniu planificat

- Scări de timp (UTC, TAI, TT) cu tabel de secunde intercalare, ΔT pentru −1999…3000 și conversii Zi Iuliană, inclusiv trecerea din 1582.
- Poziția Soarelui (longitudine aparentă și declinație) și a Lunii din seria Meeus trunchiată.
- Fazele Lunii: următoarea lună nouă, plină și sferturile.
- Răsărit, apus, trecere la meridian și crepuscul pentru orice latitudine și longitudine, și circumstanțele eclipselor de Soare din elementele Bessel.
- Calendare (iulian, ebraic, islamic tabelar, persan, săptămână ISO), un trait `PlanetProvider` și o pagină demo WebAssembly.

Nimic din asta nu este implementat încă: fiecare punct este o sarcină deschisă în `docs/issues/`.

## Compilare și testare

```bash
cargo test --workspace
cargo run -p astrolabiu-cli -- jd 2027 8 2.5
```

## Contribuie

Citește `CONTRIBUTING.md`, alege o sarcină etichetată `good first issue` din `docs/issues/` și deschide un pull request. Nicio contribuție nu e prea mică: un rând de tabel, un test sau o traducere contează.

## Implementare independentă

Scrisă pornind de la specificații publice și manuale; nu s-a copiat cod sursă de la terți (vezi `CLEANROOM.md`).

## Licență

MIT — Copyright (c) 2026 Alexandru-Ionuț Chiuță (Alexio).

## Sprijin

Trade-free: fără urmărire, fără reclame. Dacă ți-e de folos, poți sprijini autorul: https://buymeacoffee.com/echo.of.the.strings
