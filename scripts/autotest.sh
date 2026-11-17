#!/usr/bin/env bash
# Autotest: build + serve + Chrome headless REAL (sem virtual-time) com ?autotest=1.
# O autotest interno roda por ~220 frames e loga [GRAU][AUTOTEST] RESULT/PASS.
set -u
cd "$(dirname "$0")/.."
PORTA="${1:-8899}"
export PATH="$HOME/.cargo/bin:$PATH"

echo "[autotest] build release..."
wasm-pack build --target web --out-dir pkg --release > /tmp/grau_build.log 2>&1 || { echo "[autotest] FALHA build"; tail -20 /tmp/grau_build.log; exit 1; }
cp pkg/grau_web.js pkg/grau_web_bg.wasm public/

python3 -m http.server "$PORTA" --directory public > /tmp/grau_http.log 2>&1 &
HTTP_PID=$!
trap 'kill $HTTP_PID 2>/dev/null' EXIT
sleep 1

echo "[autotest] abrindo ?autotest=1 por 20s (tempo real, sem virtual-time)..."
timeout 20 google-chrome --headless=new --no-sandbox --disable-gpu \
  --enable-unsafe-swiftshader \
  --enable-logging=stderr --v=0 \
  --window-size=1280,720 \
  --remote-debugging-port=0 \
  "http://127.0.0.1:$PORTA/index.html?autotest=1" > /tmp/grau_autotest.log 2>&1
RC=$?
echo "[autotest] chrome exit: $RC (124=timeout esperado)"

echo "[autotest] --- resultado AUTOTEST ---"
grep -E "AUTOTEST" /tmp/grau_autotest.log || echo "(nenhum log AUTOTEST)"
echo "[autotest] --- ticks executados ---"
grep -oE "tick #[0-9]+" /tmp/grau_autotest.log | wc -l
echo "[autotest] --- erros fatais ---"
grep -qiE "Uncaught|TypeError|ReferenceError" /tmp/grau_autotest.log && echo "ERRO FATAL DE JS" || echo "sem erro fatal de JS"

if grep -q "AUTOTEST] PASS" /tmp/grau_autotest.log; then
  echo "[autotest] PASS GERAL: moto acelerou no autotest"
  exit 0
else
  echo "[autotest] FAIL/inconclusivo (veja /tmp/grau_autotest.log)"
  exit 2
fi
