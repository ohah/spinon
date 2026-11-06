import { readFileSync } from "node:fs";

const root = new URL("./", import.meta.url);
const expected = JSON.parse(readFileSync(new URL("expected-values.json", root), "utf8"));
const observed = JSON.parse(readFileSync(new URL("chrome-observation.json", root), "utf8"));
const tolerance = expected.tolerance_css_px;
let compared = 0;
let maxCoordinateError = 0;

for (const gapScene of observed.cases) {
  const gap = gapScene.row_gap_css_px;
  const expectedCases = expected.cases_by_row_gap_css_px[String(gap)];
  if (!expectedCases || gapScene.alignments.length !== 10) {
    throw new Error(`unexpected row-gap scene: ${gap}`);
  }

  for (const scene of gapScene.alignments) {
    const wanted = expectedCases[scene.computed];
    if (!wanted || scene.row_gap_css_px !== gap) {
      throw new Error(`unexpected alignment or gap: ${scene.computed}, ${gap}`);
    }
    if (scene.used_rows_css_px.length !== wanted.used_rows.length) {
      throw new Error(`track count mismatch: ${scene.computed}, ${gap}`);
    }
    for (let i = 0; i < wanted.used_rows.length; i += 1) {
      if (scene.used_rows_css_px[i] !== wanted.used_rows[i]) {
        throw new Error(`track mismatch: ${scene.computed}, ${gap}, row ${i}`);
      }
    }

    if (scene.item_rects_css_px.length !== 3) {
      throw new Error(`item count mismatch: ${scene.computed}, ${gap}`);
    }
    for (let i = 0; i < scene.item_rects_css_px.length; i += 1) {
      const rect = scene.item_rects_css_px[i];
      const wantedY = wanted.item_y[i];
      const error = Math.abs(rect[1] - wantedY);
      maxCoordinateError = Math.max(maxCoordinateError, error);
      if (error > tolerance || rect[0] !== 0 || rect[2] !== 40 || rect[3] !== wanted.used_rows[i]) {
        throw new Error(`frame mismatch: ${scene.computed}, ${gap}, item ${i}, ${JSON.stringify(rect)}`);
      }
    }
    compared += 1;
  }
}

if (compared !== 40 || observed.cases.length !== 4) {
  throw new Error(`expected 40 alignments over 4 gap values, got ${compared}`);
}

process.stdout.write(JSON.stringify({
  result: "passed",
  independent_expected_scenes: compared,
  row_gap_values_css_px: observed.cases.map(({ row_gap_css_px }) => row_gap_css_px),
  coordinate_tolerance_css_px: tolerance,
  max_observed_coordinate_error_css_px: maxCoordinateError,
}));
