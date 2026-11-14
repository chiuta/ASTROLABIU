# ASTROLABIU — Motor de eventos celestes: escalas de tiempo, eclipses, salida/puesta y calendarios — Rust puro, MIT.

[English](README.md) · [Română](README.ro.md) · [Français](README.fr.md) · [Italiano](README.it.md) · **Español** · [Português](README.pt.md) · [Deutsch](README.de.md) · [Magyar](README.hu.md)

Versión: 0.1.0-alpha.1 · Última actualización: 2026-11-14

🌐 https://astrolabiu.chiuta.ro

## Estado

Esqueleto inicial (pre-alfa): el workspace compila y las pruebas pasan en CI. Cada función prevista es una tarea abierta en `docs/issues/`.

## Alcance previsto

- Escalas de tiempo (UTC, TAI, TT) con tabla de segundos intercalares, ΔT para −1999…3000 y conversiones a día juliano, incluido el cambio de 1582.
- Posición del Sol (longitud aparente y declinación) y de la Luna a partir de la serie de Meeus truncada.
- Fases de la Luna: próxima luna nueva, llena y cuartos.
- Salida, puesta, tránsito y crepúsculo para cualquier latitud y longitud, y circunstancias de los eclipses solares a partir de los elementos de Bessel.
- Calendarios (juliano, hebreo, islámico tabular, persa, semana ISO), un trait `PlanetProvider` y una página de demostración WebAssembly.

Nada de esto está implementado todavía: cada punto es una tarea abierta en `docs/issues/`.

## Compilar y probar

```bash
cargo test --workspace
cargo run -p astrolabiu-cli -- jd 2027 8 2.5
```

## Contribuir

Lee `CONTRIBUTING.md`, elige una tarea con la etiqueta `good first issue` en `docs/issues/` y abre un pull request. Ninguna contribución es demasiado pequeña: una fila de tabla, una prueba o una traducción cuentan.

## Implementación independiente

Escrita a partir de especificaciones públicas y manuales; no se ha copiado código fuente de terceros (véase `CLEANROOM.md`).

## Licencia

MIT — Copyright (c) 2026 Alexandru-Ionuț Chiuță (Alexio).

## Apoyo

Trade-free: sin rastreo ni publicidad. Si te resulta útil, puedes apoyar al autor: https://buymeacoffee.com/echo.of.the.strings
