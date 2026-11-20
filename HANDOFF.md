# HANDOFF — Grau (raiz)

Última atualização: 2026-11-20 — G02-F3 (checkpoints de rota, HUD urbano, round trip proof implementados)

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
- G02-F2 Motor Godot para Windows, cena urbana procedural: **CONCLUÍDO (base jogável)**
- G02-F3 Checkpoints de rota, indicador de rota, HUD urbano: **CONCLUÍDO**
- G02-F4 Autotest `?autotest=route` e validação interactiva: PENDENTE
- G02-F5 Portal SP ↔ urbano e primeira entrada da moto: PENDENTE

## Resultados e verificações
- Proposta escrita em `docs/G02-F1-proposta-escopo.md`.
- Projeto Godot criado em `grau-godot/` (separado de `grau-web/`, preservando o Grau Web).
- Cena urbana: 2 quarteirões (60×30 m cada), Rua Central (N-S), Rua Transversal (E-O), Rua Base (E-O), cruzamento X em (0,0,0).
- Marcadores: SP=(-132,0.5,132) azul, PR1=(132,0.5,-132) verde, PR2=(132,0.5,132) vermelho, X=(0,0.5,0) amarelo.
- Player com câmera terceira pessoa, controles WASD + mouse, rodando no Godot 4.7.2.
- Projeto carrega e roda sem erros no Godot (Vulkan, sem erros de parse).
- Export Windows: NÃO VERIFICADO (templates de exportação não instalados; não baixados sem ordem específica).
- **G02-F3 implementado**: waypoints PN/PG/BC/C1 com detecção por raio, RouteManager com Rota A (PG→C1→PN) e Rota B (PN→C1→BC→C1→PG), HUD urbano com barra de progresso e label de rota, prova `ROUTE COMPLETED round_trip=true` ao completar round trip.
- Estado Git sincronizado com `origin/main` (HEAD `363291e`).
- Grau Web intacto: CI/CD GitHub Pages passa, autotests remotos PASS.

## Ficheiros alterados e referências
- `docs/G02-F1-proposta-escopo.md` (novo) — proposta completa da fase
- `grau-web/BACKLOG.md` — entrada da fase G02-F1
- `grau-web/HANDOFF.md` — registo da emenda de escopo
- `HANDOFF.md` (raiz) — este registo

## Operações em andamento
- Nenhuma. Push para `origin/main` concluído (G02-F1 documentação + G02-F2 base + G02-F3 waypoints/HUD/rotas).

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
- G02-F4: autotest roundtrip (script headless Godot equivalente a `?autotest=route`) e validação interactiva da prova.
- G02-F5: portal SP ↔ urbano + primeira entrada da moto (depois de 3 voltas de treino no Grau Web).

## Reversão
- `git revert <sha>` do commit de documentação, ou `git reset --hard 89f7f3b`.
