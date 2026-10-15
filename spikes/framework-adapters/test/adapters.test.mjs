import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import test from "node:test";
import { HostProjectionRecorder } from "../src/host-projection.mjs";
import { mountVueCounter } from "../src/vue-adapter.mjs";
import { mountSvelteCounter } from "../src/svelte-adapter.mjs";
import { runAdapterScenario } from "../src/run-scenario.mjs";

const oracle = JSON.parse(await readFile(new URL("../../../spec/internal/evidence/r07-framework-adapter-precomparison-2026-10-16/oracle.json", import.meta.url), "utf8"));
const allowedOperations = new Set([
  "CreateElement",
  "CreateText",
  "InsertBefore",
  "RemoveChild",
  "SetTextData",
  "SetAttribute",
  "RemoveAttribute",
  "AdapterListenerAdd",
  "AdapterListenerRemove",
  "HarnessClickDispatch",
]);

test("Vue custom renderer와 Svelte client output이 같은 Element/Text oracle로 투영된다", async () => {
  const results = [];
  const vue = await runAdapterScenario("Vue", "vue-root", mountVueCounter);
  results.push(vue);
  const svelte = await runAdapterScenario("Svelte", "svelte-root", mountSvelteCounter);
  results.push(svelte);

  for (const result of results) {
    assert.deepEqual(result.snapshots.mount, oracle.steps[0].tree, `${result.name}: mount`);
    assert.equal(result.clicks.mount, oracle.steps[0].clickCount, `${result.name}: initial click count`);
    assert.deepEqual(result.snapshots.click, oracle.steps[1].tree, `${result.name}: click`);
    assert.equal(result.clicks.click, oracle.steps[1].clickCount, `${result.name}: one click`);
    assert.deepEqual(result.snapshots.twoClicks, oracle.steps[2].tree, `${result.name}: two clicks`);
    assert.equal(result.clicks.twoClicks, oracle.steps[2].clickCount, `${result.name}: two clicks`);
    assert.deepEqual(result.snapshots.unmount, oracle.steps[3].tree, `${result.name}: unmount`);
    assert.equal(result.clicks.unmount, oracle.steps[3].clickCount, `${result.name}: click count after unmount`);
    assert.ok(result.operations.length > 0, `${result.name}: host trace exists`);
    assert.ok(result.operations.every((operation) => operation.ownerId === result.ownerId), `${result.name}: owner carried by every operation`);
    assert.ok(result.operations.every((operation) => allowedOperations.has(operation.operation)), `${result.name}: operation set is explicit`);
    assert.ok(result.operations.some((operation) => operation.operation === "SetTextData"), `${result.name}: text update reached the host`);
    assert.ok(result.operations.some((operation) => operation.operation === "SetAttribute" && operation.name === "aria-label"), `${result.name}: attribute reached the host`);
    assert.equal(result.operations.filter((operation) => operation.operation === "HarnessClickDispatch").length, 2, `${result.name}: each synthetic click dispatched once`);
    assert.equal(result.detachedClickErrorCode, "DetachedTarget", `${result.name}: detached click rejected`);
  }

  assert.ok(vue.operations.some((operation) => operation.operation === "CreateElement"));
  assert.ok(svelte.operations.some((operation) => operation.operation === "CreateElement"));
  assert.equal(svelte.rootClickListeners, 0, "Svelte delegated root listener removed after unmount");
  assert.equal(svelte.documentClickListeners, 0, "Svelte delegated document listener removed after unmount");
});

test("지원하지 않는 속성·이름·소유권은 조용히 수용하지 않는다", () => {
  const recorder = new HostProjectionRecorder();
  recorder.bindOwner("owner-a");
  const element = recorder.createElement("owner-a", "button");
  const beforeUnsupportedAttribute = recorder.operations.length;
  assert.throws(() => recorder.setAttribute("owner-a", element, "style", "color:red"), (error) => error.code === "UnsupportedAttribute");
  assert.equal(element.attributeMap.size, 0);
  assert.equal(recorder.operations.length, beforeUnsupportedAttribute);
  const beforeInvalidName = recorder.operations.length;
  assert.throws(() => recorder.createElement("owner-a", "svg:path"), (error) => error.code === "InvalidNodeName");
  assert.equal(recorder.operations.length, beforeInvalidName);

  const foreignParent = recorder.createElement("owner-b", "main");
  const beforeOwnerConflict = recorder.operations.length;
  assert.throws(() => recorder.insert("owner-a", element, foreignParent), (error) => error.code === "OwnerConflict");
  assert.equal(recorder.operations.length, beforeOwnerConflict);
  assert.equal(foreignParent.childNodes.length, 0);
  assert.equal(element.parentNode, null);
});
