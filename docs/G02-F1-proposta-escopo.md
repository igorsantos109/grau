# G02-F1 — Emenda de escopo: mundo aberto urbano (dois quarteirões)

Status: **PROPOSTA (G02-F1)** — fase de planeamento, sem implementação.
Data: 2026-11-19
Referência de engine para esta fase: **Godot para Windows** (jogo nativo, não apenas lançador).
Referência de exploração: terceira pessoa em cidade (inspiração no nível de liberdade de exploração
urbana de jogos clássicos de mundo aberto), **sem copiar mapa, personagens, marcas, arte ou áudio**.

---

## 1. Objetivo da fase

Substituir o escopo "um quarteirão" por:

> **dois quarteirões ligados por ruas, com pelo menos um cruzamento e duas rotas entre pontos
> de referência.**

Esta fase apenas **esboça a planta em texto** e **define como se prova** que o jogador pode
escolher o caminho e regressar ao ponto inicial **na mesma cena**.

## 2. Restrições e artefactos protegidos

| Restrição | Aplicação |
|---|---|
| Preservar o **Grau Web** (Rust → WASM/WebGL2) | Nenhuma alteração ao código de `grau-web/src/`, ao renderer WebGL2, ao HUD, ao áudio, aos testes headless ou ao CI/CD de GitHub Pages |
| Preservar o **lançador Android** | Nenhuma alteração ao artefacto de lançamento Android; o APK de referência continua ignorado pelo git (`.gitignore` da raiz) |
| Não copiar GTA III | Mapa, personagens, marcas, arte e áudio são **originais**; apenas o género/registo de exploração urbana é referência |
| Não implementar nesta fase | A planta e o mecanismo de prova são **specificação**; nenhum código de mundo urbano é escrito |
| Motor alvo do mundo aberto | **Godot para Windows**, jogo nativo |
| Arte/áudio do mundo urbano | Procedurais e originais, coerentes com a decisão D2 já existente (`grau-web/DECISIONS.md`) |

## 3. Esboço da planta em texto

Escala aproximada: 10 m por quadro da grade. N/S = norte/sul; L/O = leste/oeste.

```
                    Rua Transversal (Leste–Oeste)
                    60 m
        ┌───────────────────────┬───────────────────────┐
        │      QT A (Norte)     │      QT B (Norte)     │   ← PR1  Mirante
        │   60 m      ×   30 m  │   60 m      ×   30 m  │   (canto sup. dir.)
        └───────────┬───────────┴───────────┬───────────┘
                    │                       │
                    │   Rua Central (N–S)   │
                    │       60 m            │
    SP ─────────────┼───────────────────────┼───────────── Rua Base (L–O)
    (Posto de      │                       │   60 m
     partida)      └───────────┬───────────┴───────────┐
                    │                       │
        ┌───────────┴───────────┬───────────┴───────────┐
        │      QT A (Sul)       │      QT B (Sul)       │
        │   60 m      ×   30 m  │   60 m      ×   30 m  │
        └───────────────────────┴───────────────────────┘
                    │                       │
                  PR2 ──────────────────────┘
               (Garagem/Posto de retorno)  60 m
```

### 3.1 Elementos

- **Dois quarteirões**: QT A (oeste) e QT B (este), cada um com 60 m × 30 m, lado a lado.
- **Rua Central (N–S)**: separa QT A de QT B; é a via que liga diretamente os dois quarteirões.
- **Rua Transversal (E–O)**: passa a norte de ambos os quarteirões, paralela à Rua Base.
- **Rua Base (E–O)**: passa a sul de ambos os quarteirões.
- **Cruzamento X**: interseção em "+" entre a Rua Central e a Rua Transversal. É o **cruzamento
  obrigatório** e o ponto nodal da planta: concentra as duas rotas e permite mudar de caminho.
- **Contornos**: Rua Oeste (a poeste de QT A) e Rua Leste (a nascente de QT B) fecham o anel viário.
- **Pontos de referência**:
  - **SP — Posto de partida**: canto inferior esquerdo.
  - **PR1 — Mirante**: canto superior direito.
  - **PR2 — Garagem de retorno**: canto inferior direito.

### 3.2 Duas rotas entre pontos de referência

| Rota | Caminho SP → PR1 | Extensão aprox. |
|---|---|---|
| **Rota 1 (Norte)** | SP sobe pela Rua Oeste → vira na Rua Transversal → passa por cima de QT A e QT B → PR1 | ~150 m |
| **Rota 2 (Sul)** | SP segue pela Rua Base → passa por baixo de QT A e QT B → sobe pela Rua Leste → PR1 | ~150 m |
| **Retorno PR1 → SP** | PR1 desce pela Rua Leste → vira na Rua Base → passa por baixo de QT B e QT A → SP | ~150 m |

As duas rotas SP↔PR1 **convergem nos extremos** (SP, PR1) e **cruzam-se em X**:
a Rota 1 passa por cima de X; a Rota 2 passa por baixo de X. Ao chegar a X, o jogador pode
continuar na Transversal (Rota 1) ou descer a Rua Central para a Base (Rota 2) — a escolha é
geométrica e não é imposta pelo sistema.

### 3.3 Duas rotas alternativas até PR2

- **Via Este**: SP → Rua Central (norte) → Rua Transversal → Rua Leste → PR2.
- **Via Sul**: SP → Rua Base (este) → PR2.

PR2 funciona como destino de exploração opcional (garagem), sem fechar o anel principal.

## 4. Prova: escolha de caminho e regresso ao início na mesma cena

A prova tem de ser **verificável dentro de uma única cena**, sem reload nem troca de ecrã.

### 4.1 Elementos de jogo propostos

1. **Marcadores direcionais**: cada ponto de referência e cada trecho de rota recebe um marcador
   3D, com a mesma infraestrutura já existente para os cones dos checkpoints
   (`render_checkpoints()` em `grau-web/src/renderer.rs`) — cor por rota:
   SP azul, PR1 verde, PR2 vermelho, X amarelo.
2. **Checkpoints de rota**: estado dedicado no controlador de jogo
   (análogo ao vector `LAP_CHECKPOINTS` / `checkpoints_crossed` já usado para a volta em
   `grau-web/src/game.rs`), com:
   - `RouteCheckpoint { id, route: 1|2, kind: Start|Landmark|Return }`;
   - raio de deteção igual ao raio de checkpoint já existente (8 m);
   - `route_crossed: Vec<bool>` a marcar a ordem de cruzamento.
3. **Dois estados de missão coexistentes**: `LapMode` (circuito de treino, anel de 5 checkpoints)
   e `UrbanMode` (exploração dos dois quarteirões). Só um está ativo de cada vez, na mesma cena.
4. **Indicador de rota ativa no HUD**: linha/texto "ROTA 1" / "ROTA 2", atualizado em tempo real
   quando o jogador muda de caminho em X.

### 4.2 Regra de conclusão (a prova)

- **Ida**: ao cruzar, em ordem, os checkpoints da Rota 1 (SP → contorno oeste → Transversal → PR1),
  o sistema regista `ROUTE 1 COMPLETED`.
- **Volta**: a partir de PR1, ao refazer o percurso pela Rota 2 (Rua Leste → Rua Base → SP) e
  cruzar o checkpoint de retorno em SP, o sistema regista
  **`ROUTE COMPLETED round_trip=true`** com o tempo total e a rota de ida e volta.
- **Consequências demonstradas**:
  1. o jogador **escolheu** a Rota 1 (o sistema regista a rota ativa; não há caminho forçado);
  2. pode **regressar** a SP por um caminho diferente do da ida;
  3. o ciclo **fechou na mesma cena** — o estado vive no controlador do jogo; a física, a câmera e
     o renderer continuam a correr; não há transição de cena, pausa, nem carregamento.

### 4.3 Prova automatizada

- **Autotest de rota (`?autotest=route`)**: script headless (Chrome, padrão já existente em
  `grau-web/scripts/autotest_lap.sh`) que simula a ida pela Rota 1, a inversão em PR1 e o retorno
  pela Rota 2, terminando em SP.
- **Critério de PASS**: presença do log `ROUTE COMPLETED round_trip=true` nos logs do browser,
  sem erros fatais de JS — exactamente o padrão de `?autotest=lap`, estendido a rotas.
- **Prova interativa (humana)**: com teclado, o jogador pode mudar de rota em X a qualquer momento
  e ver o HUD e os marcadores refletirem a rota ativa.

## 5. Entrada da moto numa fase posterior

Nesta fase **não há moto no mundo urbano** — a moto entra em **G02-F2 ou posterior**, pelo ponto
**SP (Posto de partida)**, que funciona como portal de transição:

1. O jogador completa o circuito de treino atual (`[GRAU][LAP] COMPLETED`).
2. No HUD aparece a ação **"CIDADE"** (tecla `U`), que já está prevista na fase actual de input
   (a tecla `U` está livre no esquema WASD/Setas/Space/Shift/Ctrl/C/R/ESC/H documentado em
   `grau-web/README.md`).
3. Ao activar, a moto é reposicionada em SP e o controlador muda de `LapMode` para `UrbanMode`;
   os cones do anel de treino desaparecem e a planta urbana (dois quarteirões, Rua Central,
   Rua Transversal, Rua Base, X, SP, PR1, PR2) é gerada pelo renderer a partir da mesma
   infraestrutura procedural de baixo nível já usada para os cones.
4. A prova da secção 4 é o **pré-requisito funcional** da transição: se o regresso a SP funciona
   na mesma cena, a entrada em modo urbano é uma troca de estado dentro da cena, sem reload — o
   que preserva a continuidade do jogo nativo.

**Ordem de fases proposta**

| Fase | Conteúdo | Estado |
|---|---|---|
| G02-F1 | Emenda de escopo, planta, rotas e critério de prova | CONCLUÍDA (proposta) |
| G02-F2 | Motor Godot para Windows: cena urbana, dois quarteirões, geometria procedural | PENDENTE |
| G02-F3 | Checkpoints de rota, indicador de rota, HUD urbano | PENDENTE |
| G02-F4 | Autotest `?autotest=route` e validação interactiva | PENDENTE |
| G02-F5 | Portal SP ↔ urbano e primeira entrada da moto | PENDENTE |

## 6. Estado Git no fecho desta fase

- Branch: `main` (remoto `origin` → `https://github.com/igorsantos109/grau.git`)
- Working tree limpa antes desta entrega; as alterações desta fase são apenas documentação.
- Nenhum ficheiro de código, asset ou configuração de build foi alterado.
