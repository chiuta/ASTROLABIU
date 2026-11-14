# ASTROLABIU — primeras diez tareas

[English](ISSUES.md) · [Română](ISSUES.ro.md) · [Français](ISSUES.fr.md) · [Italiano](ISSUES.it.md) · **Español** · [Português](ISSUES.pt.md) · [Deutsch](ISSUES.de.md) · [Magyar](ISSUES.hu.md)

Créalas en GitHub con `scripts/create-issues.sh OWNER/REPO`. Comenta en una issue para reclamarla.

1. **Día juliano ↔ calendario: años negativos y el cambio de 1582, más vectores de prueba** — `good first issue`, `tests`, `area:time`
   Amplía las pruebas de `julian_day` / `calendar_date` con los ejemplos resueltos de Meeus y fechas a. C. Criterio: ≥ 20 vectores, todos pasan; documenta las reglas del calendario proléptico en `docs/`.

2. **Polinomio ΔT = TT − UT (Espenak–Meeus) para −1999…3000** — `help wanted`, `area:time`
   Implementa `delta_t(year_fraction)` a partir de los polinomios por tramos publicados, con una tabla de valores de control y un rango de error documentado.

3. **Escalas de tiempo: UTC, TAI, TT con una tabla de segundos intercalares como datos** — `help wanted`, `data`, `area:time`
   Añade `time::scales` con conversiones y un archivo `data/leap_seconds.csv` cargado en compilación (sin red). Incluye una prueba alrededor del segundo intercalar del 2016-12-31.

4. **Sol: longitud aparente y declinación (Meeus cap. 25, baja precisión)** — `help wanted`, `area:sun`
   Implementa la serie de baja precisión con su exactitud declarada (≈0,01°). Valida con 5 valores publicados.

5. **Luna: longitud/latitud a partir de la serie truncada de términos periódicos (Meeus cap. 47)** — `help wanted`, `area:moon`
   Pon las tablas de términos en `data/` como texto plano y genera Rust en compilación. Valida con el ejemplo resuelto del libro.

6. **Fases lunares: próxima luna nueva/llena/cuarto (Meeus cap. 49)** — `good first issue`, `area:moon`
   Implementa `next_phase(jd, Phase)`; prueba: las lunas nuevas de 2027 frente a un almanaque publicado.

7. **Salida, puesta, tránsito y crepúsculo para una latitud/longitud** — `help wanted`, `area:riseset`
   Sol y Luna; incluye la constante de refracción y los casos de día/noche polar. Devuelve horas `Option`, nunca entres en pánico.

8. **Circunstancias de un eclipse solar: elementos de Bessel, magnitud, duración** — `help wanted`, `area:eclipse`
   Calcula para un eclipse dado; valida con el eclipse total del 2 de agosto de 2027 frente a las cifras publicadas por NASA/IMCCE (cita la fuente en la prueba).

9. **Calendarios: un PR por calendario (juliano, hebreo, islámico tabular, persa, semana ISO)** — `good first issue`, `area:calendars`
   Crea `calendars/<name>.rs` con `from_jd` / `to_jd` y ≥ 10 pruebas de ida y vuelta cada uno. Elige un calendario y dilo en la issue.

10. **Trait `PlanetProvider` y compilación WebAssembly con una página de demostración** — `help wanted`, `area:wasm`
   Define el trait; aporta un proveedor de prueba sin efecto; compila para `wasm32-unknown-unknown`; añade una página estática en `docs/demo/` que llame a `julian_day`.
