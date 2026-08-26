"""Shared helpers for research-program-as-code validators."""

from __future__ import annotations

import hashlib
import json
from pathlib import Path
import re
import tomllib
from typing import Any


DIGEST = re.compile(r"^[0-9a-f]{64}$", re.IGNORECASE)
PLACEHOLDERS = {"", "tbd", "todo", "n/a", "na", "-", "...", "?", "none"}


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def placeholder(value: object) -> bool:
    text = str(value).strip().strip("-*` ").lower()
    return text in PLACEHOLDERS or (text.startswith("<") and text.endswith(">"))


def require_text(value: object, field: str, errors: list[str]) -> str:
    if not isinstance(value, str) or placeholder(value):
        errors.append(f"{field} must be non-placeholder text")
        return ""
    return value


def require_list(value: object, field: str, errors: list[str]) -> list[Any]:
    if not isinstance(value, list) or not value:
        errors.append(f"{field} must be a non-empty list")
        return []
    return value


def require_digest(value: object, field: str, errors: list[str]) -> str:
    text = require_text(value, field, errors)
    if text and not DIGEST.fullmatch(text):
        errors.append(f"{field} must be a 64-character SHA-256 digest")
    return text


def load_toml(path: Path) -> dict[str, Any]:
    value = tomllib.loads(path.read_text(encoding="utf-8"))
    if not isinstance(value, dict):
        raise ValueError("top-level TOML value must be a table")
    return value


def load_json(path: Path) -> dict[str, Any]:
    value = json.loads(path.read_text(encoding="utf-8"))
    if not isinstance(value, dict):
        raise ValueError("top-level JSON value must be an object")
    return value


def artifact_path(receipt: Path, value: object) -> Path:
    path = Path(str(value))
    return path if path.is_absolute() else (receipt.parent / path).resolve()
