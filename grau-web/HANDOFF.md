# HANDOFF — grau-web

Última atualização: 2026-11-19 14:30 -03 (G02-F1: proposta de emenda de escopo concluída)

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
- P06 Dano visual no HUD, game feel (camera shake/FOV): CONCLUÍDO — painel DAMAGE no HUD com cor/pulso, dano na física (impacto/tombamento/drift), camera shake em impacto, FOV dinâmico por velocidade; testes PASS
- P07 Gamepad, obstáculos, cenário com prédios: PENDENTE
- **G02-F1 Emenda de escopo (mundo aberto 2 quarteirões): CONCLUÍDA** — proposta textual, planta, provas de navegação, spawn da moto definidos
- **G02-F2 Implementação do layout urbano: EM ANDAMENTO** — renderer (chão/ruas/prédios), waypoints, autoteste roundtrip

## Resultados e verificações
- `cargo check --target wasm32-unknown-unknown`: 0 warnings/0 erros (reverificado 2026-11-18 22:40 após P06)
- `scripts/smoke_test.sh`: PASS — WASM inicia, sem Uncaught/TypeError (reverificado 2026-11-18 22:43 após P06)
- `scripts/autotest.sh`: PASS GERAL — `[GRAU][AUTOTEST] RESULT speed_kmh=16.8 dist_m=6.1 frames=220` PASS (reverificado 2026-11-18 22:44 após P06, sem regressão)
- `scripts/autotest_lap.sh`: PASS GERAL — `[GRAU][LAP] COMPLETED` (reverificado 2026-11-18 22:45 após P06, sem regressão)
- GitHub: repo `igorsantos109/grau` main atualizado com todo o código
- GitHub Pages: https://igorsantos109.github.io/grau/ servindo index+js+wasm (200) — autotest PASS remoto
- Workflow `Build e Deploy (WASM → Pages)`: completed/success (run 44182610331)

## Arquivos alterados (referências)
- Código: `src/{lib,game,physics,bike,renderer,camera,input,audio,ui}.rs`, `Cargo.toml`
- Docs: `README.md`, `PROJECT_STATE.md`, `BACKLOG.md`, `DECISIONS.md`, `TEST_REPORT.md`, `NEXT_ACTION.md`, `CHANGELOG.md`, `HANDOFF.md`
- Testes: `scripts/smoke_test.sh`, `scripts/autotest.sh`
- Web: `public/index.html`

## Operações em andamento
- **G02-F2**: Iniciando implementação do layout urbano (ruas, prédios procedurais, waypoints PN/PG/BC/C1, autoteste roundtrip)

## Decisões e justificativas
- Rust/WASM em vez de Unity: build reproduzível via terminal, sem editor no ambiente (D1)
- 100% original: só mecânicas genéricas, arte/áudio procedurais (D2)
- localStorage para save (natural em WASM/navegador) (D5)
- Autotest via query `?autotest=1` em vez de CDP: mais simples e reproduzível headless (D6)

## Bloqueios, autorizações e escopo
- Nenhum bloqueio ativo.
- Push para GitHub realizado com `gh`/token da conta igorsantos109 (escopo: repo grau).

## Próxima ação concreta
G02-F2-P01: Criar géometria procedural de ruas (plano XZ com textura de asfalto/faixas) + prédios low-poly (cubos escalonados) no renderer. Substituir chão infinito atual por malha urbana delimitada (2 quarteirões ~120x120m). Adicionar constantes de waypoints (PN, PG, BC, C1) em game.rs. Criar autoteste `?autotest=roundtrip`.

## Reversão
- Commits locais em `grau-web/`; `git revert <sha>` ou reset ao último commit verde.
- Remote main pode ser revertido via GitHub se necessário.

## Implementação P05-F6: Marcadores visuais 3D dos checkpoints
- Adicionados shaders CONE_VERTEX_SHADER e CONE_FRAGMENT_SHADER
- Função `create_cone_geometry()` para gerar geometria de cono (7 triângulos)
- Função `render_checkpoints()` no Renderer para desenhar cones posicionados nos 5 checkpoints
- Cones possuem cor laranja (RGB 255, 165, 0) com iluminação básica
- Teste autotest_lap.sh PASS confirmando funcionamento

## Implementação P06: Dano no HUD + game feel
- `ui.rs`: painel DAMAGE no HUD (porcentagem), visível só com dano>0; cor laranja→vermelha e pulso crítico ≥75%
- `index.html`: CSS `.damage-panel`, `.hud-panel.hidden`, `@keyframes damage-pulse`
- `physics.rs`: `apply_damage` calcula dano (impacto de queda, tombamento >60°, drift >20 m/s) e retorna shake
- `game.rs`: `physics.update` agora retorna `Option<f32>` de shake → `camera.add_shake` em impacto
- `camera.rs`: `add_shake` acumula intensidade; FOV dinâmico (+velocidade, −wheelie); decay por dt real
- `renderer.rs`: cor da moto varia com `bike.damage`
- Verificado: cargo check 0 warnings, smoke/autotest/lap todos PASS

## Plano G02-F2 (esta fase)
- **G02-F2-P01** (renderer): Chão urbano procedural (asfalto + faixas), prédios low-poly (cubos escalados), delimitação dos 2 quarteirões (~120x120m). Substituir `create_ground_geometry` + skybox estático por cena urbana.
- **G02-F2-P02** (game.rs): Constantes `URBAN_WAYPOINTS` (PN, PG, BC, C1) + modo `roundtrip_mode` + autoteste `?autotest=roundtrip` que executa rota A (PG→C1→PN) e volta rota B (PN→C1→BC→C1→PG) sem reset.
- **G02-F2-P03** (bike.rs): `reset()` posiciona em PG (0, 0.5, -60) heading norte (0 rad).
- **G02-F2-P04** (testes): `scripts/autotest_roundtrip.sh` + validação `cargo check` + smoke test.
- **G02-F2-P05** (docs): Atualizar README, BACKLOG, CHANGELOG, HANDOFF.