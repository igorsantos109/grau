# BACKLOG — grau-web (priorizado)

## Fase 0 — Estabilizar base (CONCLUÍDA em grande parte)
- [x] Validar runtime no navegador headless (render, loop, console sem erros) — smoke_test.sh PASS
- [x] Teste automatizado mínimo (scripts/smoke_test.sh com Chrome headless)
- [x] Tela inicial com gesto de áudio (0 warnings de autoplay)
- [ ] Validação interativa de input (WASD muda velocidade) — autotest via ?autotest=1 ou CDP
- [ ] Limpar warnings de dead code + deps não usadas
- [ ] Criar HANDOFF.md na raiz de grau-web

## Fase 1 — Vertical slice polido
- [x] Pausa (ESC/P) com overlay
- [x] Reset suave ao tombar + R manual
- [x] Barras de input no HUD (throttle/freio/direção/indicadores)
- [ ] Indicador visual de dano no HUD (bike.damage)
- [ ] Game feel: camera shake em impacto, FOV dinâmico com velocidade

## Fase 2 — Sistemas
- [x] Recordes persistidos em localStorage
- [ ] Placar/tempo de volta em circuito demarcado (cones/checkpoints)
- [ ] Obstáculos procedural (cones, rampas)
- [ ] Gamepad (Gamepad API)

## Fase 3 — Conteúdo/Polish
- [ ] Cenário com mais identidade (prédios low-poly procedurais, cercas)
- [ ] Trilha procedural original (Web Audio)
- [ ] Modo foto / replay curto
