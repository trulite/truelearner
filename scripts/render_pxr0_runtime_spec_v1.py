#!/usr/bin/env python3
"""Render the exhaustive PXR0 active specification as one legible A4 page."""

from __future__ import annotations

import hashlib
import json
from pathlib import Path

from reportlab.lib.pagesizes import A4
from reportlab.pdfbase.pdfmetrics import stringWidth
from reportlab.pdfgen.canvas import Canvas


ROOT = Path(__file__).resolve().parents[1]
DATA = ROOT / "experiments/pxr0_single_file_physical_runtime_spec_v1.json"
SOURCE = ROOT / "crates/pxr0-physical-runtime/src/lib.rs"
OUTPUT = ROOT / "output/pdf/pxr0_single_file_physical_runtime_spec_v1.pdf"
FONT = "Helvetica"
FONT_BOLD = "Helvetica-Bold"
SIZE = 10
LEADING = 12
MARGIN = 36


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
        lines = wrap(f"{name} - {detail}", width)
        for line in lines:
            canvas.drawString(x, y, line)
            y -= LEADING
        y -= 1
    return y


def main() -> None:
    data = json.loads(DATA.read_text())
    OUTPUT.parent.mkdir(parents=True, exist_ok=True)
    width, height = A4
    canvas = Canvas(str(OUTPUT), pagesize=A4, pageCompression=1)
    canvas.setTitle(data["title"])
    canvas.setFont(FONT_BOLD, 14)
    canvas.drawString(MARGIN, height - MARGIN, data["title"])
    y = height - MARGIN - 18
    canvas.setFont(FONT, SIZE)
    y = draw_wrapped(canvas, data["authority"], MARGIN, y, width - 2 * MARGIN)
    source_hash = hashlib.sha256(SOURCE.read_bytes()).hexdigest()
    y = draw_wrapped(canvas, f"Canonical source SHA-256: {source_hash}", MARGIN, y, width - 2 * MARGIN)
    y -= 2
    for text in data["sections"]:
        y = draw_wrapped(canvas, text, MARGIN, y, width - 2 * MARGIN)
    y -= 4
    canvas.line(MARGIN, y, width - MARGIN, y)
    y -= 14
    gap = 18
    column_width = (width - 2 * MARGIN - gap) / 2
    canvas.setFont(FONT_BOLD, 11)
    canvas.drawString(MARGIN, y, "All active types/state (13)")
    canvas.drawString(MARGIN + column_width + gap, y, "All active functions/methods (15)")
    y -= 15
    canvas.setFont(FONT, SIZE)
    left_y = draw_entries(canvas, data["types"], MARGIN, y, column_width)
    right_y = draw_entries(canvas, data["functions"], MARGIN + column_width + gap, y, column_width)
    bottom = min(left_y, right_y)
    if bottom < MARGIN + 18:
        raise SystemExit(f"one-page content overflow: bottom={bottom}")
    canvas.setFont(FONT, SIZE)
    canvas.drawString(MARGIN, MARGIN, "28/28 entries; no hidden production module; development readiness only; authority unchanged.")
    canvas.showPage()
    canvas.save()
    print(f"PXR0_ONE_PAGE_SPEC_RENDERED path={OUTPUT} bottom={bottom:.1f}")


if __name__ == "__main__":
    main()
