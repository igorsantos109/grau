# NEXT_ACTION — grau-web

## Contexto
Projeto publicado e no ar: https://igorsantos109.github.io/grau/ (CI builda+deploy a cada push na main).

## Próxima ação (menor, testável)
**CONCLUÍDO** (2026-11-18 22:45): P06 — dano no HUD + game feel implementados e verificados.

- ✔️ Painel DAMAGE no HUD (%, cor dinâmica, pulso crítico ≥75%)
- ✔️ Dano na física: impacto de queda, tombamento extremo, desgaste em drift
- ✔️ Camera shake proporcional ao impacto
- ✔️ FOV dinâmico por velocidade (−4° no grau)
- ✔️ Moto muda de cor no 3D com o dano
- ✔️ smoke_test.sh / autotest.sh / autotest_lap.sh PASS (sem regressão)
- ✔️ cargo check 0 warnings

## Próxima ação pendente (P07)
- Gamepad API
- Obstáculos no circuito
- Cenário com prédios procedurais