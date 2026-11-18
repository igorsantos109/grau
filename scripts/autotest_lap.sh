#!/usr/bin/env bash
# Autotest de volta (P05-F2): build + serve + Chrome headless com ?autotest=lap.
# REGRA AINDA NÃO IMPLEMENTADA: nenhum log [GRAU][LAP] deve aparecer.
# Este teste FALHA (exit 2) pelo motivo esperado: ausência da regra de contagem de volta.
set -u
cd "$(dirname "$0")/.."
PORTA="${1:-8898}"
export PATH="$HOME/.cargo/bin:$PATH"

echo "[lap-test] build release..."
wasm-pack build --target web --out-dir pkg --release > /tmp/grau_lap_build.log 2>&1 || { echo "[lap-test] FALHA build (erro de compilação NÃO é o motivo esperado)"; tail -20 /tmp/grau_lap_build.log; exit 1; }
cp pkg/grau_web.js pkg/grau_web_bg.wasm public/

python3 -m http.server "$PORTA" --directory public > /tmp/grau_lap_http.log 2>&1 &
HTTP_PID=$!
trap 'kill $HTTP_PID 2>/dev/null' EXIT
sleep 1

echo "[lap-test] abrindo ?autotest=lap por 60s (tempo real)..."
timeout 60 google-chrome --headless=new --no-sandbox --disable-gpu \
  --enable-unsafe-swiftshader \
  --enable-logging=stderr --v=0 \
  --window-size=1280,720 \
  --remote-debugging-port=0 \
  "http://127.0.0.1:$PORTA/index.html?autotest=lap" > /tmp/grau_lap_autotest.log 2>&1
RC=$?
echo "[lap-test] chrome exit: $RC (124=timeout esperado)"

echo "[lap-test] --- resultado LAP ---"
grep -E "\[GRAU\]\[LAP\]" /tmp/grau_lap_autotest.log || echo "(nenhum log [GRAU][LAP])"
echo "[lap-test] --- erros fatais de JS ---"
grep -qiE "Uncaught|TypeError|ReferenceError" /tmp/grau_lap_autotest.log && echo "ERRO FATAL DE JS" || echo "sem erro fatal de JS"

# P05-F3: sucesso = a regra de volta registrou [GRAU][LAP] COMPLETED com tempo.
if grep -q "\[GRAU\]\[LAP\] COMPLETED" /tmp/grau_lap_autotest.log; then
  echo "[lap-test] PASS GERAL: volta completada e logada"
  exit 0
else
  echo "[lap-test] FAIL: nenhum [GRAU][LAP] COMPLETED (veja /tmp/grau_lap_autotest.log)"
  exit 2
fi
