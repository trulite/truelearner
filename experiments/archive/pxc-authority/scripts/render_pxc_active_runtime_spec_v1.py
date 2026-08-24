#!/usr/bin/env python3
"""Render the exhaustive PX-C active specification as one legible A4 page."""

from __future__ import annotations

import hashlib
import json
from pathlib import Path

from reportlab.lib.pagesizes import A4
from reportlab.pdfbase.pdfmetrics import stringWidth
from reportlab.pdfgen.canvas import Canvas


ROOT = Path(__file__).resolve().parents[1]
DATA = ROOT / "experiments/pxc_active_runtime_spec_v1.json"
SOURCE = ROOT / "crates/pxr0-physical-runtime/src/lib.rs"
OUTPUT = ROOT / "output/pdf/pxc_active_runtime_spec_v1.pdf"
FONT = "Helvetica"
FONT_BOLD = "Helvetica-Bold"
SIZE = 9.4
LEADING = 11.2
MARGIN = 34


def wrap(text: str, width: float) -> list[str]:
    words = text.split()
    lines: list[str] = []
    current = ""
    for word in words:
        candidate = word if not current else f"{current} {word}"
        if stringWidth(candidate, FONT, SIZE) <= width:
            current = candidate
        else:
            if current:
                lines.append(current)
            current = word
    if current:
        lines.append(current)
    return lines


def draw_wrapped(canvas: Canvas, text: str, x: float, y: float, width: float) -> float:
    for line in wrap(text, width):
        canvas.drawString(x, y, line)
        y -= LEADING
    return y


def draw_entries(canvas: Canvas, entries: list[list[str]], x: float, y: float, width: float) -> float:
    for name, detail in entries:
        for line in wrap(f"{name} - {detail}", width):
            canvas.drawString(x, y, line)
            y -= LEADING
        y -= 0.8
    return y


def main() -> None:
    data = json.loads(DATA.read_text())
    OUTPUT.parent.mkdir(parents=True, exist_ok=True)
    width, height = A4
    canvas = Canvas(str(OUTPUT), pagesize=A4, pageCompression=1, invariant=1)
    canvas.setTitle(data["title"])
    canvas.setFont(FONT_BOLD, 13)
    canvas.drawString(MARGIN, height - MARGIN, data["title"])
    y = height - MARGIN - 17
    canvas.setFont(FONT, SIZE)
    y = draw_wrapped(canvas, data["authority"], MARGIN, y, width - 2 * MARGIN)
    source_hash = hashlib.sha256(SOURCE.read_bytes()).hexdigest()
    y = draw_wrapped(canvas, f"Canonical source SHA-256: {source_hash}", MARGIN, y, width - 2 * MARGIN)
    y -= 1
    for section in data["sections"]:
        y = draw_wrapped(canvas, section, MARGIN, y, width - 2 * MARGIN)
    y -= 3
    canvas.line(MARGIN, y, width - MARGIN, y)
    y -= 13
    gap = 16
    column_width = (width - 2 * MARGIN - gap) / 2
    canvas.setFont(FONT_BOLD, 10.4)
    canvas.drawString(MARGIN, y, "All active types/state (13)")
    canvas.drawString(MARGIN + column_width + gap, y, "All active functions/methods (16)")
    y -= 14
    canvas.setFont(FONT, SIZE)
    left_y = draw_entries(canvas, data["types"], MARGIN, y, column_width)
    right_y = draw_entries(canvas, data["functions"], MARGIN + column_width + gap, y, column_width)
    bottom = min(left_y, right_y)
    if bottom < MARGIN + 18:
        raise SystemExit(f"one-page content overflow: bottom={bottom}")
    canvas.drawString(MARGIN, MARGIN, "29/29 entries; one production file; no hidden module or auxiliary surface.")
    canvas.showPage()
    canvas.save()
    print(f"PXC_ONE_PAGE_SPEC_RENDERED path={OUTPUT} bottom={bottom:.1f}")


if __name__ == "__main__":
    main()
