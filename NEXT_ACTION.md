# NEXT_ACTION — grau-web

## Próxima ação (menor, testável)
Limpar warnings de build: remover deps não usadas (console_log, serde_json, web-time) do
Cargo.toml e remover/prefixar com _ os setters e imports mortos apontados pelo cargo check.
Critério: `cargo check` termina com 0 warnings (ou só warnings justificados).

## Depois dela
- Circuito com checkpoints/cones e placar de volta (P2)
- Indicador visual de dano no HUD (bike.damage)
- Gamepad API (P3)
