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
const grid = firstElementNamed(body, "section");
const items = [];
if (grid !== null) {
  for (let child = grid.firstChild; child !== null; child = child.nextSibling) {
    if (child.nodeType === 1) items.push(child);
  }
}

const expectedClasses = ["cell-a", "cell-b", "cell-c", "cell-d"];
if (!root || root.localName !== "html" || !head || !body ||
    !grid || grid.getAttribute("id") !== "gap-grid" || items.length !== 4 ||
    expectedClasses.some((className, index) => items[index].getAttribute("class") !== className)) {
  throw new Error("parsed C11.1 Root HTML Grid gap fixture did not reach the V8 document");
}
