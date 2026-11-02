const root = document.documentElement;
const head = document.head;
const body = document.body;
const grid = body?.querySelector("main#root") ?? null;
const expectedIds = ["item1", "item2", "item3", "item4", "item5", "item6", "item7", "item8"];
const ids = grid === null
  ? []
  : Array.from(grid.children, child => child.getAttribute("id"));
if (root?.localName !== "html" || head === null || body === null || grid === null ||
    ids.length !== expectedIds.length || expectedIds.some((id, index) => ids[index] !== id)) {
  throw new Error("parsed fixed-px auto-row-list fixture did not reach V8 in source order");
}
grid.setAttribute("data-runtime", "ready");
if (grid.getAttribute("data-runtime") !== "ready") {
  throw new Error("Root HTML fixture mutation was not observable in the V8 document");
}
