"""Build and run the integration tests."""

# ruff: noqa: S603, S607, T201

import subprocess
import sys

COMPOSE_FILE = "tests_integration/docker-compose-sqlite.yml"


def run(args: list[str]) -> int:
    """Run a command and return its exit code."""
    return subprocess.run(args, stdin=subprocess.DEVNULL).returncode  # noqa: S603


if __name__ == "__main__":
    print("Building and running integration tests...")
    print("Building Spoolman...")
    if run(["docker", "build", "-t", "donkie/spoolman:test", "."]) != 0:
        print("Failed to build Spoolman!")
        sys.exit(1)
    print("Building Spoolman tester...")
    if run(["docker", "build", "-t", "donkie/spoolman-tester:latest", "tests_integration"]) != 0:
        print("Failed to build Spoolman tester!")
        sys.exit(1)

    print("Running integration tests...")
    run(["docker", "compose", "-f", COMPOSE_FILE, "down", "-v"])
    result = run([
        "docker", "compose", "-f", COMPOSE_FILE,
        "up", "--abort-on-container-exit", "--exit-code-from", "tester",
    ])
    run(["docker", "compose", "-f", COMPOSE_FILE, "down", "-v"])
    if result != 0:
        print("Integration tests failed!")
        sys.exit(1)

    print("Integration tests passed!")
