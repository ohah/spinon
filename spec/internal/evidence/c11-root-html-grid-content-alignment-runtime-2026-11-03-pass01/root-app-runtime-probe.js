const findElementById = (parent, id) => {
  if (parent === null) return null;
  for (let child = parent.firstChild; child !== null; child = child.nextSibling) {
    if (child.nodeType !== 1) continue;
    if (child.getAttribute("id") === id) return child;
    const nested = findElementById(child, id);
    if (nested !== null) return nested;
  }
  return null;
};

const root = document.documentElement;
const head = document.head;
const body = document.body;
const grid = findElementById(body, "grid");
const cells = [];
if (grid !== null) {
  for (let child = grid.firstChild; child !== null; child = child.nextSibling) {
    if (child.nodeType === 1) cells.push(child);
  }
}
const expectedCells = ["a", "b", "c", "d"];

if (!root || root.localName !== "html" || !head || !body || !grid ||
    cells.length !== expectedCells.length ||
    expectedCells.some((value, index) => cells[index].getAttribute("data-cell") !== value)) {
  throw new Error("C11 Root HTML Grid content-alignment fixture did not reach the V8 document");
}

grid.setAttribute("data-runtime", "ready");
if (grid.getAttribute("data-runtime") !== "ready") {
  throw new Error("C11 Grid attribute update did not stay in the same V8 document session");
}
