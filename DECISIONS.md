# DECISIONS — grau-web

## D1 (2026-11-16) Engine: Rust/WASM em vez de Unity
- O repo não tinha projeto Unity; havia APK Android + rewrite web em Rust existente.
- Build reproduzível via terminal (cargo/wasm-pack), sem editor Unity no ambiente.
- Tradeoff: sem assets 3D ricos; geometria procedural + WebGL2 custom.

## D2 (2026-11-16) 100% original
- Apenas mecânicas genéricas (wheelie/drift) com nomes/parâmetros próprios.
- Sem assets, áudios, logos ou nomes do APK original. Arte procedural, áudio sintetizado.

## D3 (2026-11-16) Artefatos gerados fora do git
- pkg/, target/, public/grau_web.js, public/grau_web_bg.wasm gitignorados.
- public/index.html é fonte (tracked). Build copia pkg→public antes de servir.

## D4 (2026-11-17) Áudio só inicia após gesto do usuário
- AudioContext exige gesto; criamos tela inicial com botão JOGAR que dispara evento
  "grau-start" e chama audio.init()+resume(). Resultado: 0 warnings de autoplay no headless.

## D5 (2026-11-17) Persistência via localStorage (não save file)
- Em WASM no navegador, localStorage é o mecanismo natural de save simples.
- Salvamos best_wheelie_time/best_wheelie_angle; carregamos no start e salvamos em reset/tombo.

## D6 (2026-11-17) Validação headless com virtual-time-budget
- Chrome headless --virtual-time-budget roda poucos ticks reais; serve para provar que o
  WASM inicia sem erro fatal, mas não valida gameplay interativo. Próximo passo: autotest.

## D7 (2026-11-16) Aprovação humana
- Política da sessão: "ask approval". Preferimos mudanças locais reversíveis e commits locais.
