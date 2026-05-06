#!/usr/bin/env python3
"""Validate modified TDD artifact sections in execution plans."""

from __future__ import annotations

import argparse
from pathlib import Path
import re
import sys


ROOT = Path.cwd()
DEFAULT_PATHS = [ROOT / "docs/exec-plans/plan-template.md"]
ACTIVE_DIR = ROOT / "docs/exec-plans/active"

REQUIRED_HEADINGS = [
    "Modified TDD artifacts",
    "Feature Info",
    "Research Notes",
    "Reviewed Plan",
    "Architecture Pseudocode",
    "Reviewed Architecture",
    "Skeleton Checklist",
    "Mock Test Checklist",
    "Implementation Checklist",
    "Validation Log",
    "Review Notes",
]


def heading_present(text: str, heading: str) -> bool:
    pattern = rf"(?m)^#+\s+{re.escape(heading)}\s*$"
    return re.search(pattern, text) is not None


def default_plan_paths() -> list[Path]:
    paths = list(DEFAULT_PATHS)
    if ACTIVE_DIR.exists():
        paths.extend(
            sorted(
                path
                for path in ACTIVE_DIR.glob("*.md")
                if path.name.lower() != "readme.md"
            )
        )
    return paths


def validate(path: Path) -> list[str]:
    if not path.exists():
        return [f"{path}: file does not exist"]
    text = path.read_text(encoding="utf-8")
    missing = [
        heading for heading in REQUIRED_HEADINGS if not heading_present(text, heading)
    ]
    return [f"{path}: missing heading `{heading}`" for heading in missing]


def main() -> int:
    parser = argparse.ArgumentParser(
        description="Check execution plans for required modified TDD sections."
    )
    parser.add_argument(
        "paths",
        nargs="*",
        type=Path,
        help="Plan files to validate. Defaults to the plan template and active plans.",
    )
    args = parser.parse_args()

    paths = args.paths or default_plan_paths()
    errors: list[str] = []
    for path in paths:
        errors.extend(validate(path))

    if errors:
        print("Modified TDD workflow validation failed:")
        for error in errors:
            print(f"- {error}")
        return 1

    print(f"Modified TDD workflow validation passed for {len(paths)} file(s).")
    return 0


if __name__ == "__main__":
    sys.exit(main())
