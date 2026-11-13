# ASTROLABIU — architecture

`astrolabiu-core` is `std`-only and compiles to WebAssembly. Modules planned (one file each): `time` (Julian Day, ΔT, UTC/TAI/TT),
`sun`, `moon`, `phases`, `riseset`, `eclipse` (Besselian elements), `calendars/*` (one file per calendar).
Planet positions are **not** reimplemented: a `PlanetProvider` trait lets callers plug in an ephemeris crate
(see LANDSCAPE.md). Core scope = *events* computed on top of positions.
