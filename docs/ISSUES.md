# ASTROLABIU — first ten tasks

Create them on GitHub with `scripts/create-issues.sh OWNER/REPO`. Comment on an issue to claim it.

1. **Julian Day ↔ calendar: negative years and the 1582 switch, more test vectors** — `good first issue`, `tests`, `area:time`
   Extend the tests of `julian_day` / `calendar_date` with Meeus' worked examples and BCE dates. Acceptance: ≥ 20 vectors, all pass; document the proleptic rules in `docs/`.

2. **ΔT = TT − UT polynomial (Espenak–Meeus) for −1999…3000** — `help wanted`, `area:time`
   Implement `delta_t(year_fraction)` from the published piecewise polynomials, with a table of check values and a documented error range.

3. **Time scales: UTC, TAI, TT with a leap-second table as data** — `help wanted`, `data`, `area:time`
   Add `time::scales` with conversions and a `data/leap_seconds.csv` file loaded at compile time (no network). Include a test around the 2016-12-31 leap second.

4. **Sun: apparent longitude and declination (Meeus ch. 25, low precision)** — `help wanted`, `area:sun`
   Implement the low-precision series with its stated accuracy (≈0.01°). Validate against 5 published values.

5. **Moon: longitude/latitude from the truncated periodic-term series (Meeus ch. 47)** — `help wanted`, `area:moon`
   Put the term tables in `data/` as plain text and generate Rust at build time. Validate against the worked example in the book.

6. **Moon phases: next new/full/quarter moon (Meeus ch. 49)** — `good first issue`, `area:moon`
   Implement `next_phase(jd, Phase)`; test: new moons of 2027 vs a published almanac.

7. **Rise, set, transit and twilight for a latitude/longitude** — `help wanted`, `area:riseset`
   Sun and Moon; include refraction constant and the polar-day/night cases. Return `Option` times, never panic.

8. **Solar eclipse circumstances: Besselian elements, magnitude, duration** — `help wanted`, `area:eclipse`
   Compute for a given eclipse; validate on the total eclipse of 2 August 2027 against published NASA/IMCCE figures (cite the source in the test).

9. **Calendars: one PR per calendar (Julian, Hebrew, Islamic tabular, Persian, ISO week)** — `good first issue`, `area:calendars`
   Create `calendars/<name>.rs` with `from_jd` / `to_jd` and ≥ 10 round-trip tests each. Pick one calendar and say so in the issue.

10. **`PlanetProvider` trait and a WebAssembly build with a demo page** — `help wanted`, `area:wasm`
   Define the trait; provide a no-op test provider; build `wasm32-unknown-unknown`; add a static page under `docs/demo/` calling `julian_day`.
