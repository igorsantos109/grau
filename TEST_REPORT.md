# TEST_REPORT — grau-web

## 2026-11-17 16:19 — Iteração 3 (atual)
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
