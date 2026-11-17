# DECISIONS — grau-web

## D1 (2026-11-16) Engine: Rust/WASM em vez de Unity
- O repositório não tinha projeto Unity; havia um APK Android (`projeto-grau-mod...apk`) e um rewrite web em Rust já existente (`grau-web/`).
- Justificativa: build reproduzível via terminal (cargo/wasm-pack disponíveis), sem editor Unity neste ambiente, entrega jogável via navegador.
- Tradeoff: sem assets 3D ricos; usamos geometria procedural + WebGL2 custom.

## D2 (2026-11-16) 100% original, sem copiar GTA III nem o jogo original
- Mantemos apenas mecânicas genéricas (wheelie/drift) com nomes e parâmetros próprios.
- Sem assets, áudios, logos ou nomes do APK original. Toda arte é procedural; todo áudio é sintetizado.

## D3 (2026-11-16) Artefatos gerados fora do git
- `pkg/`, `target/`, `public/grau_web.js`, `public/grau_web_bg.wasm` são gerados e gitignorados.
- `public/index.html` é fonte (tracked). Build deve copiar pkg → public antes de servir.

## D4 (2026-11-16) Validação runtime adiada para iteração 2
- Sem navegador headless confirmado nesta sessão; registramos como próxima ação (P0) em vez de bloquear.
- Suposição: o build compila ⇒ alta chance de rodar; risco real está em erros de runtime WebGL/JS.

## D5 (2026-11-16) Aprovação humana
- Política da sessão mudou para "ask approval"; operações que exigem aprovação podem falhar se não houver respondente. Preferimos mudanças locais reversíveis (arquivos do projeto) e commits locais.
