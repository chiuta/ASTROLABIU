# ASTROLABIU — primele zece sarcini

[English](ISSUES.md) · **Română** · [Français](ISSUES.fr.md) · [Italiano](ISSUES.it.md) · [Español](ISSUES.es.md) · [Português](ISSUES.pt.md) · [Deutsch](ISSUES.de.md) · [Magyar](ISSUES.hu.md)

Creează-le pe GitHub cu `scripts/create-issues.sh OWNER/REPO`. Comentează la un issue ca să-l revendici.

1. **Zi iuliană ↔ calendar: ani negativi și trecerea din 1582, mai multe vectori de test** — `good first issue`, `tests`, `area:time`
   Extinde testele pentru `julian_day` / `calendar_date` cu exemplele rezolvate din Meeus și date î.Hr. Criteriu: ≥ 20 de vectori, toți trec; documentează regulile proleptice în `docs/`.

2. **Polinomul ΔT = TT − UT (Espenak–Meeus) pentru −1999…3000** — `help wanted`, `area:time`
   Implementează `delta_t(year_fraction)` din polinoamele pe porțiuni publicate, cu un tabel de valori de control și un interval de eroare documentat.

3. **Scări de timp: UTC, TAI, TT cu un tabel de secunde intercalare ca date** — `help wanted`, `data`, `area:time`
   Adaugă `time::scales` cu conversii și un fișier `data/leap_seconds.csv` încărcat la compilare (fără rețea). Include un test în jurul secundei intercalare din 2016-12-31.

4. **Soarele: longitudine aparentă și declinație (Meeus cap. 25, precizie joasă)** — `help wanted`, `area:sun`
   Implementează seria de precizie joasă cu acuratețea declarată (≈0,01°). Validează pe 5 valori publicate.

5. **Luna: longitudine/latitudine din seria trunchiată de termeni periodici (Meeus cap. 47)** — `help wanted`, `area:moon`
   Pune tabelele de termeni în `data/` ca text simplu și generează Rust la compilare. Validează pe exemplul rezolvat din carte.

6. **Fazele Lunii: următoarea lună nouă/plină/pătrar (Meeus cap. 49)** — `good first issue`, `area:moon`
   Implementează `next_phase(jd, Phase)`; test: lunile noi din 2027 față de un almanah publicat.

7. **Răsărit, apus, trecere la meridian și crepuscul pentru o latitudine/longitudine** — `help wanted`, `area:riseset`
   Soare și Lună; include constanta de refracție și cazurile de zi/noapte polară. Returnează ore `Option`, nu intra niciodată în panică.

8. **Circumstanțele eclipsei de Soare: elemente Bessel, magnitudine, durată** — `help wanted`, `area:eclipse`
   Calculează pentru o eclipsă dată; validează pe eclipsa totală din 2 august 2027 față de cifrele publicate de NASA/IMCCE (citează sursa în test).

9. **Calendare: un PR per calendar (iulian, ebraic, islamic tabular, persan, săptămână ISO)** — `good first issue`, `area:calendars`
   Creează `calendars/<name>.rs` cu `from_jd` / `to_jd` și ≥ 10 teste dus-întors fiecare. Alege un calendar și spune-o în issue.

10. **Trăsătura `PlanetProvider` și un build WebAssembly cu o pagină demo** — `help wanted`, `area:wasm`
   Definește trăsătura; oferă un furnizor de test fără efect; compilează pentru `wasm32-unknown-unknown`; adaugă o pagină statică în `docs/demo/` care apelează `julian_day`.
