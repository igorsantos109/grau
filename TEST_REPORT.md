# TEST_REPORT — grau-web

## 2026-11-17 17:21 — Iteração 5 (build limpo)
- `cargo check --target wasm32-unknown-unknown`: PASS com **0 warnings** (antes: 23+3)
- Removidas deps não usadas (console_log, serde_json, web-time); dead code anotado com #[allow(dead_code)]
- `scripts/autotest.sh`: PASS GERAL (speed 16.8 km/h, dist 6.1 m, 0 erros fatais JS)
- `scripts/smoke_test.sh`: PASS (WASM inicia, sem erro fatal)

## 2026-11-17 16:47 — Iteração 4 (autotest interativo)
- `scripts/autotest.sh` (Chrome headless tempo real 20s, ?autotest=1):
  - `[GRAU][AUTOTEST] RESULT speed_kmh=16.8 dist_m=6.1 wheelie_deg=0.0 frames=220`
  - `[GRAU][AUTOTEST] PASS: moto acelerou e percorreu distância`
  - 0 erros fatais de JS → PASS GERAL
  - Valida: loop roda em tempo real, física aplica throttle, moto percorre distância.
- Nota: virtual-time-budget não serve para gameplay (só 2 ticks); autotest usa tempo real.

## 2026-11-17 16:19 — Iteração 3
- `cargo check --target wasm32-unknown-unknown`: PASS (0 erros)
- `wasm-pack build --release` via scripts/smoke_test.sh: PASS
- Smoke headless (Chrome 157 headless=new, 8s, virtual-time-budget):
  - GET index.html/js/wasm → 200
  - WASM inicia: logs [GRAU] main/loop/tick presentes
  - 0 warnings "AudioContext was not allowed" (corrigido: áudio só inicia no clique JOGAR)
  - 0 erros fatais de JS (Uncaught/TypeError/ReferenceError)
  - RESULTADO: PASS
- Limitação: headless com `--virtual-time-budget` roda poucos ticks reais (tick #0..#1);
  validação de gameplay (WASD, física, render visível) ainda requer navegador interativo.

## 2026-11-16 20:19 — Iteração 1
- cargo check PASS (22 warnings dead code), wasm-pack release PASS (wasm 145.742 B)
- Servidor estático: index/js/wasm → 200
- public/ sincronizado com pkg/

## Erros conhecidos
- Nenhum erro de build. Warnings: código morto (setters), 3 deps não usadas (console_log, serde_json, web-time).
- WebGL em headless usa SwiftShader (fallback software) — apenas aviso de performance, não erro.
