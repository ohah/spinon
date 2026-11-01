import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import { fileURLToPath } from "node:url";
import { dirname, resolve } from "node:path";

const here = dirname(fileURLToPath(import.meta.url));
const fixturePath = resolve(
  here,
  "../../../../tests/fixtures/css/c11/root-html-grid-anonymous-text-precomparison.html",
);
const expected = JSON.parse(await readFile(resolve(here, "expected.json"), "utf8"));
const observation = JSON.parse(await readFile(resolve(here, "chrome-observation.json"), "utf8"));
const fixture = await readFile(fixturePath, "utf8");

assert.equal(expected.fixture_id, "C11.1-root-html-grid-anonymous-text-v1");
assert.match(fixture, /<meta name="viewport" content="width=device-width, initial-scale=1">/);
assert.match(fixture, /grid-template-columns:\s*40px 80px/);
assert.match(fixture, /grid-template-rows:\s*70px/);
assert.match(fixture, /white-space:\s*normal/);
assert.match(fixture, /<main id="layout">AB<\/main>/);

const actual = observation.fixture;
assert.match(observation.browser.user_agent, /HeadlessChrome\/156\.0\.0\.0/);
assert.deepEqual(actual.viewport, {
  width: expected.viewport.width_css_px,
  height: expected.viewport.height_css_px,
  device_scale_factor: expected.viewport.device_scale_factor,
});
assert.deepEqual(actual.root.rect, {
  x: expected.grid.origin_css_px[0],
  y: expected.grid.origin_css_px[1],
  width: expected.grid.size_css_px[0],
  height: expected.grid.size_css_px[1],
});
assert.equal(actual.root.columns, "40px 80px");
assert.equal(actual.root.rows, "70px");
assert.equal(actual.root.gap, "0px");
assert.equal(actual.root.white_space, expected.text_style.white_space);
assert.equal(actual.root.font, expected.text_style.font);
assert.equal(actual.root.color, expected.text_style.color);
assert.deepEqual(actual.root.child_node_types, [
  { type: 3, name: "#text", value: expected.anonymous_item.source_text },
]);
assert.deepEqual(
  [actual.text.bounding_rect.x, actual.text.bounding_rect.y],
  expected.text_range.origin_css_px,
);
assert.equal(actual.text.range_rects.length, expected.text_range.line_count);
assert.ok(
  actual.text.bounding_rect.x + actual.text.bounding_rect.width <=
    expected.grid.columns_css_px[0],
  "text Range should remain within the first fixed column",
);

const whitespace = actual.whitespace_only_control;
assert.equal(whitespace.text, expected.whitespace_only_control.text);
assert.equal(whitespace.range_rect_count, expected.whitespace_only_control.range_rect_count);
assert.deepEqual(whitespace.root_rect, {
  x: expected.grid.origin_css_px[0],
  y: expected.grid.origin_css_px[1],
  width: expected.whitespace_only_control.grid_size_css_px[0],
  height: expected.whitespace_only_control.grid_size_css_px[1],
});
assert.equal(whitespace.columns, "40px 80px");
assert.equal(whitespace.rows, "70px");

process.stdout.write(
  `${JSON.stringify({
    fixture: expected.fixture_id,
    browser: "HeadlessChrome/156.0.0.0",
    root: actual.root.rect,
    tracks: { columns: actual.root.columns, rows: actual.root.rows },
    anonymousTextRange: actual.text.bounding_rect,
    whitespaceRangeRectCount: whitespace.range_rect_count,
    result: "PASS",
  }, null, 2)}\n`,
);
