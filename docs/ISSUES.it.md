# ASTROLABIU — primi dieci compiti

[English](ISSUES.md) · [Română](ISSUES.ro.md) · [Français](ISSUES.fr.md) · **Italiano** · [Español](ISSUES.es.md) · [Português](ISSUES.pt.md) · [Deutsch](ISSUES.de.md) · [Magyar](ISSUES.hu.md)

Creale su GitHub con `scripts/create-issues.sh OWNER/REPO`. Commenta una issue per prenderla in carico.

1. **Giorno giuliano ↔ calendario: anni negativi e passaggio del 1582, più vettori di test** — `good first issue`, `tests`, `area:time`
   Estendi i test di `julian_day` / `calendar_date` con gli esempi svolti di Meeus e date a.C. Criterio: ≥ 20 vettori, tutti passano; documenta le regole prolettiche in `docs/`.

2. **Polinomio ΔT = TT − UT (Espenak–Meeus) per −1999…3000** — `help wanted`, `area:time`
   Implementa `delta_t(year_fraction)` dai polinomi a tratti pubblicati, con una tabella di valori di controllo e un intervallo d’errore documentato.

3. **Scale di tempo: UTC, TAI, TT con una tabella di secondi intercalari come dati** — `help wanted`, `data`, `area:time`
   Aggiungi `time::scales` con le conversioni e un file `data/leap_seconds.csv` caricato in compilazione (niente rete). Includi un test attorno al secondo intercalare del 2016-12-31.

4. **Sole: longitudine apparente e declinazione (Meeus cap. 25, bassa precisione)** — `help wanted`, `area:sun`
   Implementa la serie a bassa precisione con l’accuratezza dichiarata (≈0,01°). Valida su 5 valori pubblicati.

5. **Luna: longitudine/latitudine dalla serie troncata di termini periodici (Meeus cap. 47)** — `help wanted`, `area:moon`
   Metti le tabelle dei termini in `data/` come testo semplice e genera Rust in compilazione. Valida sull’esempio svolto nel libro.

6. **Fasi lunari: prossima luna nuova/piena/quarto (Meeus cap. 49)** — `good first issue`, `area:moon`
   Implementa `next_phase(jd, Phase)`; test: le lune nuove del 2027 contro un almanacco pubblicato.

7. **Alba, tramonto, transito e crepuscolo per una latitudine/longitudine** — `help wanted`, `area:riseset`
   Sole e Luna; includi la costante di rifrazione e i casi di giorno/notte polare. Restituisci orari `Option`, non andare mai in panic.

8. **Circostanze dell’eclissi solare: elementi di Bessel, magnitudine, durata** — `help wanted`, `area:eclipse`
   Calcola per una data eclissi; valida sull’eclissi totale del 2 agosto 2027 con le cifre pubblicate da NASA/IMCCE (cita la fonte nel test).

9. **Calendari: una PR per calendario (giuliano, ebraico, islamico tabulare, persiano, settimana ISO)** — `good first issue`, `area:calendars`
   Crea `calendars/<name>.rs` con `from_jd` / `to_jd` e ≥ 10 test di andata e ritorno ciascuno. Scegli un calendario e dillo nella issue.

10. **Trait `PlanetProvider` e build WebAssembly con una pagina demo** — `help wanted`, `area:wasm`
   Definisci il trait; fornisci un provider di test senza effetto; compila per `wasm32-unknown-unknown`; aggiungi una pagina statica in `docs/demo/` che chiama `julian_day`.
