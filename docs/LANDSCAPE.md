# Related projects and where ASTROLABIU fits

*Checked with a limited web search on 13 November 2026. Facts change quickly: re-check before announcing or depending on anything below.*

- **Xalen Ephemeris** (Rust, Apache-2.0) — an existing pure-Rust Swiss Ephemeris replacement for planet positions, ayanamsas and stars: <https://github.com/vedika-io/xalen-ephemeris>. Its README said, when checked on 13 Nov 2026, that registry publication was still pending; a dev.to post says the crates are live. **Check before depending on it.**
- **Swiss Ephemeris** (C, AGPL or paid licence) and Rust FFI wrappers such as `swiss-eph` (AGPL-3.0) — the reason a permissive alternative matters.
- **Why ASTROLABIU still makes sense:** it does not compete on planet positions. It targets the *event layer* (eclipses with Besselian elements, rise/set, phases, calendars, time scales) as a small, `std`-only, WASM-ready crate that can sit on top of any position provider. We found no Rust crate covering this layer; that was a limited search, so please re-check.
