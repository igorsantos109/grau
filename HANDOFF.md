<<<<<<< HEAD
# HANDOFF — Grau (raiz)

Última atualização: 2026-11-19 — G02-F1 (emenda de escopo do mundo aberto urbano)

## Objetivo
Fase G02-F1: emendar o escopo de "um quarteirão" para "dois quarteirões ligados por ruas, com
pelo menos um cruzamento e duas rotas entre pontos de referência", no projecto Godot para Windows
(jogo nativo), preservando o Grau Web (Rust/WASM) e o lançador Android.

Critérios de conclusão desta fase:
- [x] Planta em texto de dois quarteirões com um cruzamento e duas rotas entre PR
- [x] Definição de como se prova a escolha de caminho e o regresso ao início na mesma cena
- [x] Indicação de onde a moto entrará em fase posterior
- [x] Estado Git, referências lidas e proposta registados
- [x] Nenhuma edição de código, instalação ou publicação de artefactos de jogo

## Restrições e configurações protegidas
- **Grau Web preservado**: `grau-web/src/`, renderer WebGL2, HUD, áudio, testes headless e CI/CD
  de GitHub Pages não são alterados por esta fase.
- **Lançador Android preservado**: o APK de referência continua ignorado pelo git (`.gitignore`
  da raiz: `projeto-grau-mod_1.6.5-an1.com.apk`).
- **Sem cópia de GTA III**: mapa, personagens, marcas, arte e áudio originais; a referência é
  apenas o registo de exploração urbana em terceira pessoa.
- **Sem implementação nesta fase**: a planta e a prova são especificação.
- Motor do mundo aberto: **Godot para Windows**, jogo nativo (não apenas lançador).
- Registo do handoff: distinguir ação proposta, executada e verificada.

## Plano e progresso por etapa
- G02-F1 Emenda de escopo (dois quarteirões, rotas, prova): **CONCLUÍDO**
- G02-F2 Motor Godot para Windows, cena urbana procedural: PENDENTE
- G02-F3 Checkpoints de rota, indicador de rota, HUD urbano: PENDENTE
- G02-F4 Autotest `?autotest=route` e validação interactiva: PENDENTE
- G02-F5 Portal SP ↔ urbano e primeira entrada da moto: PENDENTE

## Resultados e verificações
- Proposta escrita em `docs/G02-F1-proposta-escopo.md`.
- Estado Git verificado antes da entrega: `main`, working tree limpa, HEAD `6f5f1d6`,
  sincronizado com `origin/main`.
- Nenhum build/teste executado: a fase não altera código executável.

## Ficheiros alterados e referências
- `docs/G02-F1-proposta-escopo.md` (novo) — proposta completa da fase
- `grau-web/BACKLOG.md` — entrada da fase G02-F1
- `grau-web/HANDOFF.md` — registo da emenda de escopo
- `HANDOFF.md` (raiz) — este registo

## Operações em andamento
- Nenhuma. Push para `origin/main` concluído (apenas documentação G02-F1).

## Decisões e justificativas
- A planta, o mecanismo de prova e a entrada da moto são especificação escrita, não código,
  por a restrição explícita da fase ("não edite ficheiros, não implemente" na fase de análise
  inicial; subsequentemente autorizada a registar a proposta no repositório).
- O mecanismo de prova reutiliza padrões já existentes no código (`LAP_CHECKPOINTS`,
  `checkpoints_crossed`, cones 3D em `render_checkpoints()`, autotests headless
  `?autotest=1` / `?autotest=lap`), reduzindo risco de introduzir padrões paralelos.

## Bloqueios, autorizações e seu âmbito
- Nenhum bloqueio activo.
- Autorização do utilizador: registar a proposta no repositório e enviar ao GitHub (`RUN`).

## Próxima ação concreta
- G02-F2: definir a cena urbana em Godot para Windows (dois quarteirões, Rua Central, Rua
  Transversal, Rua Base, cruzamento X, SP/PR1/PR2) sobre a planta de G02-F1.

## Reversão
- `git revert <sha>` do commit de documentação, ou `git reset --hard 6f5f1d6`.
=======
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
>>>>>>> origin/main
