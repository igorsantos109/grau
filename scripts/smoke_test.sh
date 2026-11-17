#!/usr/bin/env bash
# Smoke test do grau-web: build + serve + navegador headless capturando console.
# Uso: ./scripts/smoke_test.sh [porta]
set -u
cd "$(dirname "$0")/.."
PORTA="${1:-8899}"
export PATH="$HOME/.cargo/bin:$PATH"

echo "[smoke] build release..."
wasm-pack build --target web --out-dir pkg --release > /tmp/grau_build.log 2>&1 || { echo "[smoke] FALHA no build"; tail -20 /tmp/grau_build.log; exit 1; }
cp pkg/grau_web.js pkg/grau_web_bg.wasm public/
[ -f pkg/index.html ] && cp pkg/index.html public/index.html

echo "[smoke] subindo servidor em :$PORTA ..."
python3 -m http.server "$PORTA" --directory public > /tmp/grau_http.log 2>&1 &
HTTP_PID=$!
trap 'kill $HTTP_PID 2>/dev/null' EXIT
sleep 1

for f in index.html grau_web.js grau_web_bg.wasm; do
  code=$(curl -s -o /dev/null -w "%{http_code}" "http://127.0.0.1:$PORTA/$f")
  echo "[smoke] GET /$f -> $code"
  [ "$code" = "200" ] || { echo "[smoke] FALHA: $f nao retornou 200"; exit 1; }
done

echo "[smoke] abrindo no Chrome headless por 8s e capturando console..."
# Captura console via --enable-logging; redireciona stderr do chrome (onde o console.log cai no headless)
timeout 12 google-chrome --headless=new --no-sandbox --disable-gpu \
  --enable-logging=stderr --v=0 \
  --virtual-time-budget=8000 \
  --window-size=1280,720 \
  "http://127.0.0.1:$PORTA/index.html" > /tmp/grau_console.log 2>&1
RC=$?
echo "[smoke] chrome exit code: $RC (124=timeout esperado)"

echo "[smoke] --- linhas de console relevantes ---"
grep -E "GRAU|CONSOLE|ERROR|Error|error|Uncaught|failed|Failed" /tmp/grau_console.log | head -40 || true
echo "[smoke] --- fim console ---"

# Critério: presença de logs [GRAU] do WASM indica que o loop iniciou; ausência de Uncaught indica sem erro fatal de JS
if grep -q "\[GRAU\]" /tmp/grau_console.log && ! grep -qiE "Uncaught|TypeError|ReferenceError" /tmp/grau_console.log; then
  echo "[smoke] PASS: WASM iniciou ([GRAU] logs presentes) e sem erro fatal de JS"
  exit 0
else
  echo "[smoke] VERIFICAR: resultado inconclusivo ou falha (veja /tmp/grau_console.log)"
  exit 2
fi
