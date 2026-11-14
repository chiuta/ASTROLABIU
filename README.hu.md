# ASTROLABIU — Égi események motorja: időskálák, fogyatkozások, kelés/nyugvás és naptárak — tiszta Rust, MIT.

[English](README.md) · [Română](README.ro.md) · [Français](README.fr.md) · [Italiano](README.it.md) · [Español](README.es.md) · [Português](README.pt.md) · [Deutsch](README.de.md) · **Magyar**

Verzió: 0.1.0-alpha.1 · Utolsó frissítés: 2026-11-14

🌐 https://astrolabiu.chiuta.ro

## Állapot

Korai váz (pre-alfa): a workspace lefordul, a tesztek a CI-ben lefutnak. Minden tervezett funkció nyitott feladat a `docs/issues/` mappában.

## Tervezett terjedelem

- Időskálák (UTC, TAI, TT) szökőmásodperc-táblával, ΔT a −1999…3000 tartományra és Julián-napi átváltás, az 1582-es váltással együtt.
- A Nap (látszó hosszúság és deklináció) és a Hold helyzete a csonkolt Meeus-sorozatból.
- Holdfázisok: a következő újhold, telihold és negyedek.
- Kelte, nyugta, delelés és szürkület tetszőleges szélességre és hosszúságra, valamint napfogyatkozás-körülmények Bessel-elemekből.
- Naptárak (Julián, héber, iszlám táblázatos, perzsa, ISO-hét), egy `PlanetProvider` trait és egy WebAssembly demóoldal.

Ezek közül még semmi sincs megvalósítva: minden pont nyitott feladat a `docs/issues/` mappában.

## Fordítás és tesztelés

```bash
cargo test --workspace
cargo run -p astrolabiu-cli -- jd 2027 8 2.5
```

## Közreműködés

Olvasd el a `CONTRIBUTING.md` fájlt, válassz egy `good first issue` címkéjű feladatot a `docs/issues/` mappából, és nyiss pull requestet. Nincs túl kicsi hozzájárulás: egy táblázatsor, egy teszt vagy egy fordítás is számít.

## Független megvalósítás

Nyilvános specifikációk és tankönyvek alapján íródott; harmadik fél forráskódját nem másoltuk (lásd `CLEANROOM.md`).

## Licenc

MIT — Copyright (c) 2026 Alexandru-Ionuț Chiuță (Alexio).

## Támogatás

Trade-free: nincs követés, nincs reklám. Ha hasznos számodra, támogathatod a szerzőt: https://buymeacoffee.com/echo.of.the.strings
