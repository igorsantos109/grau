# PROJECT_STATE — grau-web

Última atualização: 2026-11-17 16:47 (iteração 4)

## O que é este projeto
Protótipo jogável web (Rust → WASM/WebGL2) de jogo de moto com manobra "grau" (wheelie), 100% original.
Baseado em engenharia reversa educacional do APK na raiz, mas sem copiar assets/nomes/código.

## Stack
- Rust 1.99 + wasm-bindgen + web-sys (WebGL2, Web Audio, input teclado/toque, localStorage)
- wasm-pack (target web, release + wasm-opt) → pkg/ → copiado para public/
- Teste automatizado: scripts/smoke_test.sh (build + serve + Chrome headless capturando console)
- Git local em grau-web/ (commits: bc329f3 baseline, aa19c7b gitignore, 2963f0e feat iteração 2)

## O que funciona (validado em headless)
- Build release compila sem erros; WASM carrega e inicia o loop (logs [GRAU] tick)
- WebGL2 context + shaders compilam; sem erros fatais de JS
- Tela inicial com botão JOGAR (dispara evento que inicia AudioContext — 0 warnings de autoplay)
- HUD overlay: velocidade, RPM, marcha, ângulo/tempo de grau, best, distância, câmera, FPS
- Barras de input (throttle/freio/direção) e indicadores DRIFT/WHEELIE atualizam por frame
- Pausa com ESC/P (overlay "PAUSADO"), reset com R
- Recordes (best angle/time) persistem em localStorage e carregam ao iniciar
- Reset suave automático quando a moto tomba (fallen)
- 5 modos de câmera; física de wheelie/drift/zerinho

## O que falta / não validado
- Validação interativa real (teclado WASD movendo a moto, render visível) — headless não simula input
- Sem captura de screenshot real do frame renderizado
- Warnings de dead code pendentes de limpeza
- Sem gamepad, sem circuito/checkpoints, sem obstáculos

## Última build
- wasm-pack release via smoke_test.sh → PASS em 2026-11-17 16:19 (wasm ~145 KB)

## Como rodar
```
cd grau-web && ./scripts/smoke_test.sh 8899   # build + teste headless
# ou só servir:
cp pkg/grau_web* public/ 2>/dev/null; python3 -m http.server 8899 --directory public
# abrir http://127.0.0.1:8899 e clicar em JOGAR
```
