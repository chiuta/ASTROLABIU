# ASTROLABIU — az első tíz feladat

[English](ISSUES.md) · [Română](ISSUES.ro.md) · [Français](ISSUES.fr.md) · [Italiano](ISSUES.it.md) · [Español](ISSUES.es.md) · [Português](ISSUES.pt.md) · [Deutsch](ISSUES.de.md) · **Magyar**

Hozd létre őket a GitHubon a `scripts/create-issues.sh OWNER/REPO` paranccsal. Szólj hozzá egy issue-hoz, hogy lefoglald.

1. **Julián-nap ↔ naptár: negatív évek és az 1582-es váltás, több tesztvektor** — `good first issue`, `tests`, `area:time`
   Bővítsd a `julian_day` / `calendar_date` teszteket Meeus kidolgozott példáival és i. e. dátumokkal. Elfogadási feltétel: ≥ 20 vektor, mind átmegy; dokumentáld a prolepticus szabályokat a `docs/`-ban.

2. **ΔT = TT − UT polinom (Espenak–Meeus) −1999…3000 tartományra** — `help wanted`, `area:time`
   Implementáld a `delta_t(year_fraction)`-t a közzétett szakaszonkénti polinomokból, ellenőrző értékek táblázatával és dokumentált hibatartománnyal.

3. **Időskálák: UTC, TAI, TT, a szökőmásodperc-táblázat adatként** — `help wanted`, `data`, `area:time`
   Adj hozzá `time::scales`-t átváltásokkal és fordításkor betöltött `data/leap_seconds.csv` fájllal (nincs hálózat). Vegyél fel egy tesztet a 2016-12-31-i szökőmásodperc körül.

4. **Nap: látszólagos hosszúság és deklináció (Meeus 25. fej., alacsony pontosság)** — `help wanted`, `area:sun`
   Implementáld az alacsony pontosságú sort a megadott pontossággal (≈0,01°). Validáld 5 közzétett értéken.

5. **Hold: hosszúság/szélesség a csonkolt periodikus tagok sorából (Meeus 47. fej.)** — `help wanted`, `area:moon`
   A tagtáblázatokat egyszerű szövegként tedd a `data/`-ba, és fordításkor generálj Rustot. Validáld a könyv kidolgozott példáján.

6. **Holdfázisok: a következő újhold/telihold/negyed (Meeus 49. fej.)** — `good first issue`, `area:moon`
   Implementáld a `next_phase(jd, Phase)`-t; teszt: 2027 újholdjai egy közzétett almanachhoz képest.

7. **Kelte, nyugta, delelés és szürkület egy szélességre/hosszúságra** — `help wanted`, `area:riseset`
   Nap és Hold; tartalmazza a refrakciós állandót és a sarki nappal/éjszaka eseteit. `Option` időket adj vissza, soha ne paníkolj.

8. **Napfogyatkozás körülményei: Bessel-elemek, magnitúdó, időtartam** — `help wanted`, `area:eclipse`
   Számítsd ki egy adott fogyatkozásra; validáld a 2027. augusztus 2-i teljes napfogyatkozáson a NASA/IMCCE közzétett adataival (a teszt hivatkozzon a forrásra).

9. **Naptárak: naptáranként egy PR (Julián, héber, táblázatos iszlám, perzsa, ISO-hét)** — `good first issue`, `area:calendars`
   Hozd létre a `calendars/<name>.rs`-t `from_jd` / `to_jd`-vel és naptáranként ≥ 10 oda-vissza teszttel. Válassz egy naptárat, és mondd meg az issue-ban.

10. **`PlanetProvider` trait és WebAssembly build demóoldallal** — `help wanted`, `area:wasm`
   Definiáld a traitet; adj egy hatástalan tesztszolgáltatót; fordíts `wasm32-unknown-unknown`-ra; adj hozzá egy statikus oldalt a `docs/demo/` alatt, amely a `julian_day`-t hívja.
