# ASTROLABIU — die ersten zehn Aufgaben

[English](ISSUES.md) · [Română](ISSUES.ro.md) · [Français](ISSUES.fr.md) · [Italiano](ISSUES.it.md) · [Español](ISSUES.es.md) · [Português](ISSUES.pt.md) · **Deutsch** · [Magyar](ISSUES.hu.md)

Lege sie auf GitHub mit `scripts/create-issues.sh OWNER/REPO` an. Kommentiere ein Issue, um es zu übernehmen.

1. **Julianischer Tag ↔ Kalender: negative Jahre und die Umstellung von 1582, mehr Testvektoren** — `good first issue`, `tests`, `area:time`
   Die Tests für `julian_day` / `calendar_date` um die durchgerechneten Beispiele von Meeus und v.-Chr.-Daten erweitern. Kriterium: ≥ 20 Vektoren, alle bestehen; die proleptischen Regeln in `docs/` dokumentieren.

2. **ΔT = TT − UT-Polynom (Espenak–Meeus) für −1999…3000** — `help wanted`, `area:time`
   `delta_t(year_fraction)` aus den veröffentlichten stückweisen Polynomen implementieren, mit einer Tabelle von Kontrollwerten und einem dokumentierten Fehlerbereich.

3. **Zeitskalen: UTC, TAI, TT mit einer Schaltsekundentabelle als Daten** — `help wanted`, `data`, `area:time`
   `time::scales` mit Umrechnungen und einer zur Kompilierzeit geladenen Datei `data/leap_seconds.csv` hinzufügen (kein Netzwerk). Einen Test rund um die Schaltsekunde vom 2016-12-31 aufnehmen.

4. **Sonne: scheinbare Länge und Deklination (Meeus Kap. 25, geringe Genauigkeit)** — `help wanted`, `area:sun`
   Die Reihe geringer Genauigkeit mit ihrer angegebenen Genauigkeit (≈0,01°) implementieren. An 5 veröffentlichten Werten validieren.

5. **Mond: Länge/Breite aus der gekürzten Reihe periodischer Terme (Meeus Kap. 47)** — `help wanted`, `area:moon`
   Die Termtabellen als Klartext in `data/` ablegen und Rust zur Kompilierzeit erzeugen. Am durchgerechneten Beispiel des Buchs validieren.

6. **Mondphasen: nächster Neu-/Voll-/Viertelmond (Meeus Kap. 49)** — `good first issue`, `area:moon`
   `next_phase(jd, Phase)` implementieren; Test: die Neumonde 2027 gegen einen veröffentlichten Almanach.

7. **Aufgang, Untergang, Kulmination und Dämmerung für eine Breite/Länge** — `help wanted`, `area:riseset`
   Sonne und Mond; Refraktionskonstante und Polartag-/Polarnachtfälle einschließen. `Option`-Zeiten zurückgeben, nie in Panik geraten.

8. **Umstände einer Sonnenfinsternis: Besselsche Elemente, Größe, Dauer** — `help wanted`, `area:eclipse`
   Für eine gegebene Finsternis berechnen; an der totalen Sonnenfinsternis vom 2. August 2027 gegen veröffentlichte NASA-/IMCCE-Werte validieren (Quelle im Test nennen).

9. **Kalender: ein PR pro Kalender (Julianisch, Hebräisch, tabellarisch Islamisch, Persisch, ISO-Woche)** — `good first issue`, `area:calendars`
   `calendars/<name>.rs` mit `from_jd` / `to_jd` und je ≥ 10 Rundlauf-Tests anlegen. Einen Kalender wählen und das im Issue sagen.

10. **Trait `PlanetProvider` und ein WebAssembly-Build mit Demoseite** — `help wanted`, `area:wasm`
   Den Trait definieren; einen wirkungslosen Testprovider bereitstellen; für `wasm32-unknown-unknown` bauen; eine statische Seite unter `docs/demo/` hinzufügen, die `julian_day` aufruft.
