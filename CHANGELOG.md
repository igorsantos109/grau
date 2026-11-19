# CHANGELOG — grau-web

## 2026-11-18 22:30 — Iteração 7 (P05-F6: marcadores visuais de checkpoints)
- feat: marcadores visuais 3D dos checkpoints (cones cilíndricos coloridos no circuito)
- add: shaders CONE_VERTEX_SHADER e CONE_FRAGMENT_SHADER (iluminação simples)
- add: função `create_cone_geometry()` para gerar geometria de cono (7 triângulos)
- add: função `render_checkpoints()` no Renderer para desenhar cones posicionados nos 5 checkpoints
- feat: sistema de volta (?autotest=lap) com checkpoints visualizados
- test: autotest_lap.sh PASS — 5/5 checkpoints cruzados, [GRAU][LAP] COMPLETED
- docs: README atualizado com circuito de checkpoints e autotest_lap.sh
- docs: HANDOFF.md atualizado com progresso P05-F6

## 2026-11-17 18:46 — Iteração 6 (docs + publish + CI/CD)
- docs: README reescrito em PT-BR com link do jogo online; HANDOFF.md criado
- ci: `.github/workflows/build-and-deploy.yml` builda WASM e deploy automático no GitHub Pages
- publish: push p/ github.com/igorsantos109/grau (main), merge preservando LICENSE AGPL
- pages: https://igorsantos109.github.io/grau/ no ar — autotest PASS remoto (16.8 km/h, 6.1 m)
- workflow run 44182610331: success

## 2026-11-17 17:21 — Iteração 5
- chore: build 100% limpo — 0 warnings no cargo check
- remove deps não usadas (console_log, serde_json, web-time)
- anota dead code com #[allow(dead_code)] (camera shake, áudio extra, input mouse, etc.)
- test: autotest.sh e smoke_test.sh seguem PASS após limpeza (sem regressão)
- commit: 87c63b3

## 2026-11-17 16:47 — Iteração 4
- feat: modo autotest (?autotest=1) simula W depois Space por 220 frames e loga resultado
- add: scripts/autotest.sh (Chrome headless tempo real) → PASS (speed 16.8 km/h, dist 6.1 m)
- add: Location ao web-sys p/ ler query string
- test: valida que física+input funcionam sem input humano real

## 2026-11-17 16:19 — Iteração 3
- feat: tela inicial (start overlay) com botão JOGAR que inicia AudioContext via gesto (evento grau-start)
- feat: pausa com ESC/P e overlay "PAUSADO"
- feat: recordes best angle/time persistem em localStorage (save/load no Bike)
- feat: reset suave automático ao tombar (fallen) salvando recordes antes
- feat: barras de input (throttle/freio/direção) e indicadores DRIFT/WHEELIE atualizam por frame
- fix: 0 warnings de autoplay — áudio não tenta iniciar antes do gesto
- add: Storage ao web-sys features; scripts/smoke_test.sh agora faz build+serve+headless
- test: smoke headless PASS, 0 erros fatais de JS
- commit: 2963f0e

## 2026-11-16 — Iteração 1
- Criados PROJECT_STATE/BACKLOG/DECISIONS/TEST_REPORT/NEXT_ACTION/CHANGELOG
- Build release sincronizado pkg→public; git init baseline (bc329f3) + gitignore (aa19c7b)
