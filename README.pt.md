# ASTROLABIU — Motor de eventos celestes: escalas de tempo, eclipses, nascer/pôr e calendários — Rust puro, MIT.

[English](README.md) · [Română](README.ro.md) · [Français](README.fr.md) · [Italiano](README.it.md) · [Español](README.es.md) · **Português** · [Deutsch](README.de.md) · [Magyar](README.hu.md)

Versão: 0.1.0-alpha.1 · Última atualização: 2026-11-22

🌐 https://astrolabiu.chiuta.ro

## Estado

Esqueleto inicial (pré-alfa): o workspace compila e os testes passam no CI. Cada funcionalidade prevista é uma tarefa aberta em `docs/issues/`.

## Âmbito previsto

- Escalas de tempo (UTC, TAI, TT) com tabela de segundos intercalares, ΔT para −1999…3000 e conversões para dia juliano, incluindo a mudança de 1582.
- Posição do Sol (longitude aparente e declinação) e da Lua a partir da série de Meeus truncada.
- Fases da Lua: próxima lua nova, cheia e quartos.
- Nascer, pôr, passagem meridiana e crepúsculo para qualquer latitude e longitude, e circunstâncias dos eclipses solares a partir dos elementos de Bessel.
- Calendários (juliano, hebraico, islâmico tabular, persa, semana ISO), um trait `PlanetProvider` e uma página de demonstração WebAssembly.

Nada disto está ainda implementado: cada ponto é uma tarefa aberta em `docs/issues/`.

## Compilar e testar

```bash
cargo test --workspace
cargo run -p astrolabiu-cli -- jd 2027 8 2.5
```

## Contribuir

Leia `CONTRIBUTING.md`, escolha uma tarefa com a etiqueta `good first issue` em `docs/issues/` e abra um pull request. Nenhuma contribuição é demasiado pequena: uma linha de tabela, um teste ou uma tradução contam.

## Implementação independente

Escrita a partir de especificações públicas e manuais; não foi copiado código-fonte de terceiros (ver `CLEANROOM.md`).

## Licença

MIT — Copyright (c) 2026 Alexandru-Ionuț Chiuță (Alexio).

## Apoio

Trade-free: sem rastreamento nem publicidade. Se lhe for útil, pode apoiar o autor: https://buymeacoffee.com/echo.of.the.strings
