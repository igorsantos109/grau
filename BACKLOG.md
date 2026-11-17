# BACKLOG — grau-web (priorizado)

## Fase 0 — Estabilizar base (AGORA)
- [P0] Validar runtime no navegador (headless ou manual): render, input, HUD, console sem erros
- [P0] Adicionar teste automatizado mínimo (ex.: Playwright/Puppeteer se disponível; senão script de fumaça via node + fetch do servidor + checagem de erros no JS de bootstrap)
- [P1] Limpar warnings de dead code (remover ou usar setters ociosos)
- [P1] Documentar Martina: criar HANDOFF.md na raiz de grau-web

## Fase 1 — Vertical slice polido
- [P1] Tela inicial com botão "Jogar" que inicia áudio (AudioContext exige gesto do usuário) e esconde loading com sucesso
- [P1] Pausa (ESC/P) e reinício (R) com feedback visual
- [P2] Indicador de dano/queda e reset suave quando a moto tomba
- [P2] Melhorar game feel: camera shake em impacto, FOV dinâmico com velocidade

## Fase 2 — Sistemas
- [P2] Recordes persistidos em localStorage (melhor ângulo/tempo de grau, distância)
- [P2] Placar/tempo de volta em circuito demarcado (cones/checkpoints)
- [P3] Tráfego simples de obstáculos (cones, rampas)生成 procedural
- [P3] Gamepad (Gamepad API)

## Fase 3 — Conteúdo/Polish
- [P3] Cenário com mais identidade (skybox melhorado, cercas, prédios low-poly procedurais)
- [P3] Trilha sonora procedural simples (Web Audio) original
- [P3] Modo foto / replay curto
