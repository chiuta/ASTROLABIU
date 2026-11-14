# ASTROLABIU — primeiras dez tarefas

[English](ISSUES.md) · [Română](ISSUES.ro.md) · [Français](ISSUES.fr.md) · [Italiano](ISSUES.it.md) · [Español](ISSUES.es.md) · **Português** · [Deutsch](ISSUES.de.md) · [Magyar](ISSUES.hu.md)

Crie-as no GitHub com `scripts/create-issues.sh OWNER/REPO`. Comente numa issue para a reivindicar.

1. **Dia juliano ↔ calendário: anos negativos e a mudança de 1582, mais vetores de teste** — `good first issue`, `tests`, `area:time`
   Amplie os testes de `julian_day` / `calendar_date` com os exemplos resolvidos de Meeus e datas a. C. Critério: ≥ 20 vetores, todos passam; documente as regras proléticas em `docs/`.

2. **Polinômio ΔT = TT − UT (Espenak–Meeus) para −1999…3000** — `help wanted`, `area:time`
   Implemente `delta_t(year_fraction)` a partir dos polinômios por partes publicados, com uma tabela de valores de verificação e um intervalo de erro documentado.

3. **Escalas de tempo: UTC, TAI, TT com uma tabela de segundos intercalares como dados** — `help wanted`, `data`, `area:time`
   Adicione `time::scales` com conversões e um arquivo `data/leap_seconds.csv` carregado na compilação (sem rede). Inclua um teste em torno do segundo intercalar de 2016-12-31.

4. **Sol: longitude aparente e declinação (Meeus cap. 25, baixa precisão)** — `help wanted`, `area:sun`
   Implemente a série de baixa precisão com a exatidão declarada (≈0,01°). Valide com 5 valores publicados.

5. **Lua: longitude/latitude a partir da série truncada de termos periódicos (Meeus cap. 47)** — `help wanted`, `area:moon`
   Ponha as tabelas de termos em `data/` como texto simples e gere Rust na compilação. Valide com o exemplo resolvido do livro.

6. **Fases da Lua: próxima lua nova/cheia/quarto (Meeus cap. 49)** — `good first issue`, `area:moon`
   Implemente `next_phase(jd, Phase)`; teste: as luas novas de 2027 contra um almanaque publicado.

7. **Nascer, pôr, trânsito e crepúsculo para uma latitude/longitude** — `help wanted`, `area:riseset`
   Sol e Lua; inclua a constante de refração e os casos de dia/noite polar. Devolva horas `Option`, nunca entre em pânico.

8. **Circunstâncias do eclipse solar: elementos de Bessel, magnitude, duração** — `help wanted`, `area:eclipse`
   Calcule para um dado eclipse; valide com o eclipse total de 2 de agosto de 2027 contra os números publicados pela NASA/IMCCE (cite a fonte no teste).

9. **Calendários: um PR por calendário (juliano, hebraico, islâmico tabular, persa, semana ISO)** — `good first issue`, `area:calendars`
   Crie `calendars/<name>.rs` com `from_jd` / `to_jd` e ≥ 10 testes de ida e volta cada um. Escolha um calendário e diga-o na issue.

10. **Trait `PlanetProvider` e build WebAssembly com uma página de demonstração** — `help wanted`, `area:wasm`
   Defina o trait; forneça um provedor de teste sem efeito; compile para `wasm32-unknown-unknown`; adicione uma página estática em `docs/demo/` que chame `julian_day`.
