# TEST_REPORT — grau-web

## 2026-11-16 20:19 — Iteração 1
- `cargo check --target wasm32-unknown-unknown`: PASS (0 erros, 22 warnings dead code, 3 unused deps)
- `wasm-pack build --target web --out-dir pkg --release`: PASS (2.04s, wasm 145.742 bytes)
- Smoke test servidor estático (`python3 -m http.server 8899 --directory public`):
  - GET /index.html → 200
  - GET /grau_web.js → 200
  - GET /grau_web_bg.wasm → 200
- Artefatos public/ estavam desatualizados vs pkg/ → sincronizados nesta iteração
- NÃO EXECUTADO: validação em navegador (render WebGL, loop, input, console). Registrado como próximo P0.

## Erros conhecidos
- Nenhum erro de build. Warnings: setters não usados em physics.rs (12), deps não usadas (console_log, serde_json, web-time).
