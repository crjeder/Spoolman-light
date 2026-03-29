#!/usr/bin/env bash
# run-e2e.sh — Build, start, test, teardown.
#
# Usage: ./scripts/run-e2e.sh
#
# Requires: docker, docker compose, node/npm (for Playwright)
# Tip: on Windows, run this from WSL or Docker Desktop Dev Containers.

set -euo pipefail

COMPOSE_FILE="$(cd "$(dirname "$0")/.." && pwd)/docker-compose.test.yml"
E2E_DIR="$(cd "$(dirname "$0")/.." && pwd)/tests/e2e"
BASE_URL="http://localhost:8000"
HEALTHCHECK_URL="${BASE_URL}/api/v1/info"
TIMEOUT=60

# ── Teardown trap ─────────────────────────────────────────────────────────────
teardown() {
  echo "→ Tearing down test container…"
  docker compose -f "$COMPOSE_FILE" down --remove-orphans 2>/dev/null || true
}
trap teardown EXIT

# ── Build & start ─────────────────────────────────────────────────────────────
echo "→ Building image and starting container…"
docker compose -f "$COMPOSE_FILE" up --build -d

# ── Wait for server ───────────────────────────────────────────────────────────
echo "→ Waiting for server at ${HEALTHCHECK_URL} (timeout: ${TIMEOUT}s)…"
elapsed=0
until curl -sf "${HEALTHCHECK_URL}" > /dev/null 2>&1; do
  if [ "$elapsed" -ge "$TIMEOUT" ]; then
    echo "✗ Server did not respond within ${TIMEOUT}s. Aborting."
    exit 1
  fi
  sleep 2
  elapsed=$((elapsed + 2))
done
echo "✓ Server is ready."

# ── Run Playwright tests ──────────────────────────────────────────────────────
echo "→ Installing Playwright dependencies…"
npm ci --prefix "$E2E_DIR"

echo "→ Running Playwright tests…"
# cd into the E2E dir so Playwright finds playwright.config.ts.
# Pass PLAYWRIGHT_BASE_URL so playwright.config.ts can pick it up if needed.
cd "$E2E_DIR"
PLAYWRIGHT_BASE_URL="$BASE_URL" npx playwright test

echo "✓ All tests passed."
