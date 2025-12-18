#!/usr/bin/env python3
"""Validate that the git tag matches crate versions.

Usage:
    python .github/scripts/check_tag.py [tag]

If no tag is supplied explicitly, the script falls back to the
`GITHUB_REF_NAME` environment variable (as provided by GitHub Actions).
"""

from __future__ import annotations

import os
import sys
import tomllib
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]

def read_package_version(path: Path) -> str:
    try:
        data = tomllib.loads(path.read_text("utf-8"))
    except FileNotFoundError as exc:  # pragma: no cover - defensive guard for CI
        raise SystemExit(f"unable to read {path}: {exc}") from exc

    try:
        return data["package"]["version"]
    except KeyError as exc:  # pragma: no cover - ensures clear error reporting
        raise SystemExit(f"missing package.version in {path}") from exc


def resolve_tag() -> str:
    tag = sys.argv[1] if len(sys.argv) > 1 else os.getenv("GITHUB_REF_NAME")
    if not tag:
        raise SystemExit("release tag not provided via argument or GITHUB_REF_NAME")
    return tag


def parse_tag(tag: str) -> str:
    if tag.startswith("v"):
        return tag[1:]

    raise SystemExit(
        "release tags must either start with 'v' or use the '<kind>-<version>' format"
    )


def main() -> None:
    tag = resolve_tag()
    version = parse_tag(tag)

    crate_version = read_package_version(ROOT / "Cargo.toml")
    if crate_version != version:
        raise SystemExit(f"tag version {version} does not match lybic-sdk-rs crate version {crate_version}")

    print(f"tag {tag} validated against crate(s): lybic-sdk-rs {crate_version}")


if __name__ == "__main__":
    main()
