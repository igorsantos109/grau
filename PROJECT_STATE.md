# PROJECT_STATE — grau-web

Última atualização: 2026-11-16 20:25 (iteração 1)

## O que é este projeto
Protótipo jogável web (Rust → WASM/WebGL2) de jogo de moto com manobra "grau" (wheelie), reescrita 100% original a partir de engenharia reversa educacional do APK `projeto-grau-mod_1.6.5-an1.com.apk`. Não usa assets, nomes, marcas ou código do original.

## Stack
- Rust 1.99 + wasm-bindgen + web-sys (WebGL2, Web Audio, input teclado/toque)
- wasm-pack (target web, release com wasm-opt)
- Build compila limpo: `cargo check --target wasm32-unknown-unknown` OK (22 warnings de dead code, 0 erros)
- wasm-pack release gera `pkg/` (145 KB wasm) e artefatos copiados para `public/` para servir estático
- Servidor de teste: `python3 -m http.server` em `public/` — index/js/wasm retornam HTTP 200

## O que funciona
- Build release WASM compila e gera pkg sem erros
- public/index.html carrega `grau_web.js` + `grau_web_bg.wasm` (200 OK no smoke test)
- HUD em overlay HTML (velocidade, RPM, marcha, ângulo/tempo de grau, distância, FPS, câmera)
- 5 modos de câmera, física de wheelie/drift/zerinho com parâmetros documentados no README
- Input teclado + toque; áudio procedural (motor, vento, derrapagem)

## O que está quebrado / não validado
- NUNCA foi aberto num navegador nesta sessão: não há validação visual/runtime real (WebGL render, loop, erros de console)
- Sem testes automatizados (sem headless browser disponível ainda)
- `public/` tinha artefatos WASM desatualizados vs `pkg/` — corrigido nesta iteração (cópia + gitignore)
- 22 warnings de código morto (setters não usados) — não é erro, mas indica cobertura parcial

## Última build
- `wasm-pack build --target web --out-dir pkg --release` → EXIT 0 em 2.04s (2026-11-16 20:19)
- Commits: bc329f3 baseline, + gitignore

## Validação jogável
Pendente: precisa abrir http://localhost:8899 (ou similar) em navegador e confirmar: canvas renderiza, moto responde a WASD, HUD atualiza, sem erros de console.
