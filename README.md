# Grau Web — Jogo de Moto com Manobra (Wheelie) 100% Original

Protótipo jogável em web (Rust → WebAssembly/WebGL2) de um jogo de motocicleta focado em
manobra "grau" (wheelie), drift e zerinho (donut). Todo o código, arte e áudio são
**originais e procedurais** — nenhum asset, nome, marca ou código de terceiros é usado.

> 🎮 **Jogar agora (online):** [https://igorsantos109.github.io/grau/](https://igorsantos109.github.io/grau/)
> (build automático via GitHub Actions → GitHub Pages a cada push na `main`)

## 🎮 Como jogar

```bash
cd grau-web
./scripts/smoke_test.sh 8899     # build + teste headless automático
# ou só servir:
wasm-pack build --target web --out-dir pkg --release
cp pkg/grau_web.js pkg/grau_web_bg.wasm public/
python3 -m http.server 8899 --directory public
```

Abra **http://127.0.0.1:8899**, clique em **JOGAR** (inicia o áudio — navegador exige gesto) e jogue.

### Controles

| Tecla | Ação |
|-------|------|
| `W` / `↑` | Acelerar |
| `S` / `↓` | Frear / Ré |
| `A` / `←` | Virar esquerda |
| `D` / `→` | Virar direita |
| `ESPAÇO` / `SHIFT` | Grau (wheelie) |
| `CTRL` | Drift (derrapagem) |
| `C` | Trocar câmera (5 modos) |
| `R` | Resetar moto |
| `ESC` / `P` | Pausar |
| `H` | Ajuda |

**Mobile:** toque na esquerda para virar, direita para acelerar/frear, canto inferior direito para o grau.

## ✨ Funcionalidades

- Física de moto com wheelie (sustentação, ângulo crítico, force reverse), drift e zerinho
- **Circuito de checkpoints**: 5 checkpoints em anel com marcadores visuais (cones 3D coloridos)
- Sistema de volta (?autotest=lap): cruzar checkpoints em ordem, loga [GRAU][LAP] COMPLETED com tempo
- 5 modos de câmera (terceira pessoa, primeira, capô, cinemática, drone)
- Renderer WebGL2 com shaders próprios: moto procedural, chão infinito com grid, skybox procedural
- HUD em HTML/CSS: velocidade, RPM, marcha, ângulo/tempo de grau, recordes, distância, FPS, barras de input
- Tela inicial com botão JOGAR (inicia áudio via gesto do usuário — sem warnings de autoplay)
- Pausa (ESC/P), reset (R), reset automático ao tombar
- Recordes (melhor ângulo/tempo de grau) persistem em `localStorage`
- Áudio procedural (Web Audio): motor por RPM, vento por velocidade, derrapagem
- Suporte a toque (mobile)

## 🚀 Deploy automático (CI/CD)

- Workflow `.github/workflows/build-and-deploy.yml` builda o WASM (`wasm-pack --release`)
  e publica em **GitHub Pages** a cada push na `main`.
- Site jogável: https://igorsantos109.github.io/grau/
- Validado em headless contra o site publicado: autotest PASS (moto acelera 16.8 km/h, 6.1 m).

## 🧪 Testes automatizados

```bash
./scripts/smoke_test.sh 8899    # build + serve + Chrome headless: valida que o WASM inicia sem erro
./scripts/autotest.sh 8899      # abre ?autotest=1: simula input e prova que a moto acelera (PASS)
./scripts/autotest_lap.sh 8899  # abre ?autotest=lap: testa circuito de checkpoints e volta (PASS)
```

- `cargo check --target wasm32-unknown-unknown` → **0 warnings, 0 erros**
- Autotest headless confirma: moto acelera até ~16.8 km/h e percorre ~6 m em 220 frames

## 🏗️ Arquitetura

```
grau-web/
├── Cargo.toml          # deps: wasm-bindgen, web-sys, nalgebra, rand, serde
├── src/
│   ├── lib.rs          # entry point (main)
│   ├── game.rs         # loop principal, estados, autotest (?autotest=1)
│   ├── physics.rs      # motor de física (wheelie/drift/zerinho)
│   ├── bike.rs         # entidade da moto + recordes (localStorage)
│   ├── renderer.rs     # WebGL2 + shaders procedurais
│   ├── camera.rs       # 5 modos de câmera + shake
│   ├── input.rs        # teclado + toque
│   ├── audio.rs        # Web Audio procedural (motor/vento/derrapagem)
│   └── ui.rs           # HUD/overlays em HTML/CSS
├── public/index.html   # página estática (fonte)
├── scripts/            # smoke_test.sh e autotest.sh (Chrome headless)
└── pkg/                # saída WASM gerada (gitignored)
```

## 📋 Documentação de estado (loop de desenvolvimento)

O projeto segue um loop incremental com arquivos de estado na raiz de `grau-web/`:

- `PROJECT_STATE.md` — estado atual, o que funciona, última build
- `BACKLOG.md` — backlog priorizado por fases
- `DECISIONS.md` — decisões técnicas com justificativa
- `TEST_REPORT.md` — resultados de build/testes
- `NEXT_ACTION.md` — próxima ação concreta
- `CHANGELOG.md` — mudanças por iteração
- `HANDOFF.md` — handoff entre sessões/agentes

## 🔧 Build

Pré-requisitos: Rust estable, `rustup target add wasm32-unknown-unknown`, `cargo install wasm-pack`.

```bash
wasm-pack build --target web --out-dir pkg --release   # release (~145 KB wasm, wasm-opt)
wasm-pack build --target web --out-dir pkg --dev       # debug
```

## Licença

Código deste repositório: uso educacional/pessoal. Nenhum asset de terceiros é distribuído.
