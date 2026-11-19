# HANDOFF — grau-web

Última atualização: 2026-11-18 22:30 -03 (P05-F6: marcadores visuais 3D dos checkpoints implementados e commitado)

## Objetivo
Protótipo jogável web (Rust→WASM/WebGL2) de jogo de moto com manobra "grau" (wheelie),
100% original (sem assets/nomes de terceiros). MVP: moto anda, wheelie/drift funcionam,
HUD completo, testes headless automatizados, código com 0 warnings.

## Critérios de conclusão do MVP atual
- [x] Build compila com 0 warnings (`cargo check --target wasm32-unknown-unknown`)
- [x] WASM carrega e roda o loop sem erro fatal de JS (smoke_test.sh PASS)
- [x] Física+input validados headless: autotest (?autotest=1) → moto acelera 16.8 km/h, 6.1 m (autotest.sh PASS)
- [x] Tela inicial JOGAR inicia áudio sem warning de autoplay
- [x] HUD: velocidade, RPM, marcha, grau, recordes, distância, FPS, barras de input, indicadores
- [x] Pausa ESC/P, reset R, reset automático ao tombar
- [x] Recordes persistem em localStorage
- [x] Documentação completa (README + 6 arquivos de estado + HANDOFF)
- [x] Publicado em https://github.com/igorsantos109/grau (branch main)
- [x] CI/CD: `.github/workflows/build-and-deploy.yml` builda WASM e deploy em GitHub Pages
- [x] Jogo online jogável em https://igorsantos109.github.io/grau/ (autotest PASS no site remoto)

## Restrições e configurações protegidas
- Nunca copiar assets/nomes/marcas de jogos comerciais (GTA, Projeto Grau original). Tudo procedural.
- `pkg/`, `target/`, `public/grau_web.js`, `public/grau_web_bg.wasm` são gitignored (gerados).
- Build sempre via `wasm-pack build --target web --out-dir pkg --release` e copiar para `public/`.
- Áudio só inicia após gesto do usuário (botão JOGAR) — política de autoplay.
- Aprovação humana: sessão opera com "ask approval"; mudanças são locais e reversíveis.

## Plano e progresso por etapa
- P01 Base reproduzível: CONCLUÍDO (build 0 warnings, git, smoke test)
- P02 Vertical slice: CONCLUÍDO (start screen, HUD, pausa, recordes, autotest)
- P03 Testes automatizados: CONCLUÍDO (smoke_test.sh + autotest.sh headless PASS)
- P04 Documentação + publish: CONCLUÍDO (README reescrito PT-BR, HANDOFF, push p/ GitHub)
- P05 Circuito com checkpoints/cones: CONCLUÍDO — regra de volta (5 checkpoints em anel) + `?autotest=lap` funcionando; marcadores visuais (cones/cilindros 3D) implementados
- P06 Dano visual no HUD, game feel (camera shake/FOV): PENDENTE
- P07 Gamepad, obstáculos, cenário com prédios: PENDENTE

## Resultados e verificações
- `cargo check`: 0 warnings/0 erros (2026-11-17 17:21)
- `scripts/autotest.sh`: PASS GERAL — `[GRAU][AUTOTEST] RESULT speed_kmh=16.8 dist_m=6.1 frames=220` (reverificado 2026-11-18 20:2x, 2/2 runs, sem regressão após P05-F3b)
- `scripts/autotest_lap.sh`: PASS GERAL — `[GRAU][LAP] COMPLETED lap=1 lap_time_ms≈47376–47399` (2026-11-18 20:2x, 2/2 runs, 5/5 checkpoints)
- `scripts/smoke_test.sh`: PASS — WASM inicia, sem Uncaught/TypeError
- GitHub: repo `igorsantos109/grau` main atualizado com todo o código
- GitHub Pages: https://igorsantos109.github.io/grau/ servindo index+js+wasm (200) — autotest PASS remoto
- Workflow `Build e Deploy (WASM → Pages)`: completed/success (run 44182610331)

## Arquivos alterados (referências)
- Código: `src/{lib,game,physics,bike,renderer,camera,input,audio,ui}.rs`, `Cargo.toml`
- Docs: `README.md`, `PROJECT_STATE.md`, `BACKLOG.md`, `DECISIONS.md`, `TEST_REPORT.md`, `NEXT_ACTION.md`, `CHANGELOG.md`, `HANDOFF.md`
- Testes: `scripts/smoke_test.sh`, `scripts/autotest.sh`
- Web: `public/index.html`

## Operações em andamento
- Nenhuma. Build e testes estão verdes.

## Decisões e justificativas
- Rust/WASM em vez de Unity: build reproduzível via terminal, sem editor no ambiente (D1)
- 100% original: só mecânicas genéricas, arte/áudio procedurais (D2)
- localStorage para save (natural em WASM/navegador) (D5)
- Autotest via query `?autotest=1` em vez de CDP: mais simples e reproduzível headless (D6)

## Bloqueios, autorizações e escopo
- Nenhum bloqueio ativo.
- Push para GitHub realizado com `gh`/token da conta igorsantos109 (escopo: repo grau).

## Próxima ação concreta
P06: implementar indicador visual de dano no HUD (bike.damage) e melhorias de game feel (camera shake/FOV). Commits: 6654650 (CHANGELOG), 758cfc1 (README), 8fb30e1 (cones), 58aa5f0 (circuito lap).

## Reversão
- Commits locais em `grau-web/`; `git revert <sha>` ou reset ao último commit verde.
- Remote main pode ser revertido via GitHub se necessário.

## Implementação P05-F6: Marcadores visuais 3D dos checkpoints
- Adicionados shaders CONE_VERTEX_SHADER e CONE_FRAGMENT_SHADER
- Função `create_cone_geometry()` para gerar geometria de cono (7 triângulos)
- Função `render_checkpoints()` no Renderer para desenhar cones posicionados nos 5 checkpoints
- Cones possuem cor laranja (RGB 255, 165, 0) com iluminação básica
- Teste autotest_lap.sh PASS confirmando funcionamento