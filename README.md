# ASTROLABIU — Sky-events engine: time scales, eclipses, rise/set and calendars — pure Rust, MIT.

**English** · [Română](README.ro.md) · [Français](README.fr.md) · [Italiano](README.it.md) · [Español](README.es.md) · [Português](README.pt.md) · [Deutsch](README.de.md) · [Magyar](README.hu.md)

Version: 0.1.0-alpha.1 · Last updated: 2026-11-22

🌐 https://astrolabiu.chiuta.ro

## Status

Early skeleton (pre-alpha): the workspace builds and the tests pass in CI. Every planned feature is an open task in `docs/issues/`.

## Planned scope

- Time scales (UTC, TAI, TT) with a leap-second table, ΔT for −1999…3000 and Julian Day conversions including the 1582 switch.
- Sun position (apparent longitude and declination) and Moon position from the truncated Meeus series.
- Moon phases: next new, full and quarter moon.
- Rise, set, transit and twilight for any latitude and longitude, and solar-eclipse circumstances from Besselian elements.
- Calendars (Julian, Hebrew, Islamic tabular, Persian, ISO week), a `PlanetProvider` trait and a WebAssembly demo page.

None of this is implemented yet: each item is an open task in `docs/issues/`.

## Build and test

```bash
cargo test --workspace
cargo run -p astrolabiu-cli -- jd 2027 8 2.5
```

## Contribute

Read `CONTRIBUTING.md`, pick a task labelled `good first issue` from `docs/issues/` and open a pull request. No contribution is too small: a table row, a test or a translation counts.

## Independent implementation

Written from public specifications and textbooks; no third-party source code is copied (see `CLEANROOM.md`).

## License

MIT — Copyright (c) 2026 Alexandru-Ionuț Chiuță (Alexio).

## Support

Trade-free: no tracking, no ads. If it is useful, you can support the author: https://buymeacoffee.com/echo.of.the.strings
