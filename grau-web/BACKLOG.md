# BACKLOG — grau-web (priorizado)

## Fase 0 — Estabilizar base (CONCLUÍDA)
- [x] Validar runtime headless — smoke_test.sh PASS
- [x] Teste automatizado mínimo (smoke_test.sh)
- [x] Tela inicial com gesto de áudio (0 warnings autoplay)
- [x] Validação interativa de input — autotest.sh PASS (?autotest=1, moto acelera)
- [x] Limpar warnings de dead code + deps não usadas (0 warnings)
- [ ] Criar HANDOFF.md na raiz de grau-web
- [ ] Circuito com checkpoints/cones e placar de volta ← PRÓXIMO

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

## Fase G02 — Mundo aberto urbano (Godot para Windows, jogo nativo)

> Âmbito phaseado e fora do runtime Web actual: o Grau Web e o lançador Android **preservam-se**
> intactos. Proposta de escopo: `../docs/G02-F1-proposta-escopo.md`.

- [x] **G02-F1** — Emenda de escopo: "dois quarteirões ligados por ruas, com pelo menos um
      cruzamento e duas rotas entre pontos de referência"; planta em texto, critério de prova de
      escolha de caminho/ regresso ao ponto inicial na mesma cena, e ponto de entrada da moto em
      fase posterior (SP → portal urbano). **CONCLUÍDO (proposta; sem implementação)**
- [ ] G02-F2 — Motor Godot para Windows: cena urbana com QT A / QT B, Rua Central, Rua Transversal,
      Rua Base, cruzamento X e marcadores SP / PR1 / PR2
- [ ] G02-F3 — Checkpoints de rota, indicador de rota ativa no HUD e marcadores direcionais
- [ ] G02-F4 — Autotest de rota (`?autotest=route`) e validação interactiva da prova
- [ ] G02-F5 — Portal SP ↔ urbano e primeira entrada da moto (depois de 3 voltas de treino)
