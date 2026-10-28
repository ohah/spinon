#!/usr/bin/env python3
"""Derive fixed C11.4 visibility-hidden flex-track expectations independently."""

from fractions import Fraction
import json
from pathlib import Path


def main() -> None:
    hidden_min_content = 15 * 10
    visible_a_min_content = 10 * 10
    fixed_track = 40
    gap_total = 2 * 10
    available_flex = 300 - fixed_track - gap_total
    initial_fraction = Fraction(available_flex, 3)
    first_base = max(hidden_min_content, visible_a_min_content)
    assert first_base > initial_fraction
    second_flexible_track = available_flex - first_base
    tracks = [first_base, fixed_track, second_flexible_track]
    assert tracks == [150, 40, 90]
    assert sum(tracks) + gap_total == 300

    expected = {
        "basis": {
            "ahem_font_size_css_px": 10,
            "hidden_text_characters": 15,
            "hidden_text_min_content_css_px": hidden_min_content,
            "visible_a_text_characters": 10,
            "visible_a_min_content_css_px": visible_a_min_content,
            "fixed_track_css_px": fixed_track,
            "two_gaps_css_px": gap_total,
            "available_flexible_space_css_px": available_flex,
            "initial_fr_size_css_px": str(initial_fraction),
            "first_track_freezes_at_css_px": first_base,
            "remaining_2fr_size_css_px": second_flexible_track,
        },
        "scenes": {
            "column-hidden": {
                "axis_tracks_css_px": tracks,
                "root": [0, 0, 300, 120],
                "items": {
                    "a": [0, 0, 150, 120],
                    "hidden": [0, 0, 150, 120],
                    "b": [160, 0, 24, 16],
                    "c": [210, 0, 90, 120],
                },
                "hidden_display": "block",
                "hidden_visibility": "hidden",
                "hidden_client_rect_count": 1,
                "surface_sample_root_css_px": [120, 60],
                "surface_sample_rgb": [40, 100, 220],
                "surface_sample_hex": "#2864dc",
            },
            "column-visible-control": {
                "axis_tracks_css_px": tracks,
                "root": [0, 0, 300, 120],
                "items": {
                    "a": [0, 0, 150, 120],
                    "hidden": [0, 0, 150, 120],
                    "b": [160, 0, 24, 16],
                    "c": [210, 0, 90, 120],
                },
                "hidden_display": "block",
                "hidden_visibility": "visible",
                "hidden_client_rect_count": 1,
                "surface_sample_root_css_px": [120, 60],
                "surface_sample_rgb": [142, 68, 173],
                "surface_sample_hex": "#8e44ad",
            },
            "row-hidden": {
                "axis_tracks_css_px": tracks,
                "root": [0, 0, 120, 300],
                "items": {
                    "a": [0, 0, 120, 150],
                    "hidden": [0, 0, 120, 150],
                    "b": [0, 160, 24, 16],
                    "c": [0, 210, 120, 90],
                },
                "hidden_display": "block",
                "hidden_visibility": "hidden",
                "hidden_client_rect_count": 1,
                "surface_sample_root_css_px": [60, 125],
                "surface_sample_rgb": [40, 100, 220],
                "surface_sample_hex": "#2864dc",
            },
            "row-visible-control": {
                "axis_tracks_css_px": tracks,
                "root": [0, 0, 120, 300],
                "items": {
                    "a": [0, 0, 120, 150],
                    "hidden": [0, 0, 120, 150],
                    "b": [0, 160, 24, 16],
                    "c": [0, 210, 120, 90],
                },
                "hidden_display": "block",
                "hidden_visibility": "visible",
                "hidden_client_rect_count": 1,
                "surface_sample_root_css_px": [60, 125],
                "surface_sample_rgb": [142, 68, 173],
                "surface_sample_hex": "#8e44ad",
            },
        },
    }
    target = Path(__file__).with_name("expected.json")
    target.write_text(json.dumps(expected, indent=2) + "\n")


if __name__ == "__main__":
    main()
