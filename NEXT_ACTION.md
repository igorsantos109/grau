# NEXT_ACTION — grau-web

## Próxima ação (menor, testável)
Criar validação runtime automatizada mínima: script `scripts/smoke_test.sh` (ou node) que
1) gera build release, 2) copia pkg→public, 3) sobe `python3 -m http.server` em public,
4) faz fetch de index/js/wasm (200) e 5) se houver Chromium/Chrome headless disponível, abre a página e captura erros de console por ~5s.

Se headless não existir, registrar em TEST_REPORT.md e pedir validação manual guiada ao humano (abrir URL, confirmar render + WASD + HUD).

## Depois dela
P0 do BACKLOG: tela inicial com gesto de áudio + limpar warnings.
