const firstElementNamed = (parent, name) => {
  if (parent === null) return null;
  for (let child = parent.firstChild; child !== null; child = child.nextSibling) {
    if (child.nodeType === 1 && child.localName === name) return child;
  }
  return null;
};

const root = document.documentElement;
const head = document.head;
const body = document.body;
const grid = firstElementNamed(body, "main");
const ids = [];
if (grid !== null) {
  for (let child = grid.firstChild; child !== null; child = child.nextSibling) {
    if (child.nodeType === 1) ids.push(child.getAttribute("id"));
  }
}
const expectedIds = ["a", "b", "c", "d", "e", "f"];
if (!root || root.localName !== "html" || !head || !body ||
    !grid || grid.getAttribute("id") !== "grid" || ids.length !== expectedIds.length ||
    expectedIds.some((id, index) => ids[index] !== id)) {
  throw new Error("parsed percentage-column implicit-row fixture did not reach the V8 document in source order");
}
grid.setAttribute("data-runtime", "ready");
if (grid.getAttribute("data-runtime") !== "ready") {
  throw new Error("Root HTML fixture mutation was not observable in the V8 document");
}
