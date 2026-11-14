# ASTROLABIU — dix premières tâches

[English](ISSUES.md) · [Română](ISSUES.ro.md) · **Français** · [Italiano](ISSUES.it.md) · [Español](ISSUES.es.md) · [Português](ISSUES.pt.md) · [Deutsch](ISSUES.de.md) · [Magyar](ISSUES.hu.md)

Créez-les sur GitHub avec `scripts/create-issues.sh OWNER/REPO`. Commentez un ticket pour le réclamer.

1. **Jour julien ↔ calendrier : années négatives et passage de 1582, plus de vecteurs de test** — `good first issue`, `tests`, `area:time`
   Étendre les tests de `julian_day` / `calendar_date` avec les exemples traités de Meeus et des dates avant J.-C. Critère : ≥ 20 vecteurs, tous passent ; documenter les règles proleptiques dans `docs/`.

2. **Polynôme ΔT = TT − UT (Espenak–Meeus) pour −1999…3000** — `help wanted`, `area:time`
   Implémenter `delta_t(year_fraction)` à partir des polynômes par morceaux publiés, avec un tableau de valeurs de contrôle et une plage d’erreur documentée.

3. **Échelles de temps : UTC, TAI, TT avec une table de secondes intercalaires comme données** — `help wanted`, `data`, `area:time`
   Ajouter `time::scales` avec les conversions et un fichier `data/leap_seconds.csv` chargé à la compilation (pas de réseau). Inclure un test autour de la seconde intercalaire du 2016-12-31.

4. **Soleil : longitude apparente et déclinaison (Meeus chap. 25, basse précision)** — `help wanted`, `area:sun`
   Implémenter la série de basse précision avec sa précision annoncée (≈0,01°). Valider sur 5 valeurs publiées.

5. **Lune : longitude/latitude à partir de la série tronquée de termes périodiques (Meeus chap. 47)** — `help wanted`, `area:moon`
   Placer les tables de termes dans `data/` en texte brut et générer du Rust à la compilation. Valider sur l’exemple traité du livre.

6. **Phases de la Lune : prochaine nouvelle/pleine lune ou quartier (Meeus chap. 49)** — `good first issue`, `area:moon`
   Implémenter `next_phase(jd, Phase)` ; test : les nouvelles lunes de 2027 face à un almanach publié.

7. **Lever, coucher, passage au méridien et crépuscule pour une latitude/longitude** — `help wanted`, `area:riseset`
   Soleil et Lune ; inclure la constante de réfraction et les cas de jour/nuit polaire. Retourner des heures `Option`, ne jamais paniquer.

8. **Circonstances d’une éclipse solaire : éléments de Bessel, magnitude, durée** — `help wanted`, `area:eclipse`
   Calculer pour une éclipse donnée ; valider sur l’éclipse totale du 2 août 2027 avec les chiffres publiés par la NASA/l’IMCCE (citer la source dans le test).

9. **Calendriers : une PR par calendrier (julien, hébreu, islamique tabulaire, persan, semaine ISO)** — `good first issue`, `area:calendars`
   Créer `calendars/<name>.rs` avec `from_jd` / `to_jd` et ≥ 10 tests aller-retour chacun. Choisir un calendrier et le dire dans le ticket.

10. **Trait `PlanetProvider` et build WebAssembly avec une page de démo** — `help wanted`, `area:wasm`
   Définir le trait ; fournir un fournisseur de test sans effet ; compiler pour `wasm32-unknown-unknown` ; ajouter une page statique sous `docs/demo/` qui appelle `julian_day`.
