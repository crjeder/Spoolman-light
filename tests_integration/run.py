"""Build and run the integration tests."""

# ruff: noqa: S603, S607, T201

import subprocess
import sys

VALID_TARGETS = ["postgres", "sqlite", "mariadb", "cockroachdb"]


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

    # Support input arguments for running only specific tests
    if len(sys.argv) > 1:
        targets = sys.argv[1:]
        for target in targets:
            if target not in VALID_TARGETS:
                print(f"Unknown target: {target}")
                sys.exit(1)
    else:
        print("No targets specified, running all tests...")
        targets = list(VALID_TARGETS)

    for target in targets:
        compose_file = f"tests_integration/docker-compose-{target}.yml"
        print(f"Running integration tests against {target}...")
        run(["docker", "compose", "-f", compose_file, "down", "-v"])
        result = run([
            "docker", "compose", "-f", compose_file,
            "up", "--abort-on-container-exit", "--exit-code-from", "tester",
        ])
        run(["docker", "compose", "-f", compose_file, "down", "-v"])
        if result != 0:
            print(f"Integration tests against {target} failed!")
            sys.exit(1)

    print("Integration tests passed!")
