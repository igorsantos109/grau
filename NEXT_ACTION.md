# NEXT_ACTION — grau-web

## Próxima ação (menor, testável)
Adicionar validação interativa automatizada: estender scripts/smoke_test.sh (ou novo script
scripts/input_test.sh) para, após carregar a página no Chrome headless com remote debugging,
disparar teclas (W por 2s, depois Space) via CDP (Chrome DevTools Protocol) e verificar no
console/DOM que a velocidade (HUD #speed-value) mudou de 0 e que não há erros.
Alternativa mais simples: injetar no index.html um "modo teste" (query ?autotest=1) que
simula input por código e loga [GRAU][AUTOTEST] speed=... após N frames — validável só de
ler o console headless, sem CDP.

## Depois dela
- Limpar warnings de dead code e deps não usadas (P1)
- Circuito com checkpoints/cones e placar de volta (P2)
- Gamepad API (P3)
