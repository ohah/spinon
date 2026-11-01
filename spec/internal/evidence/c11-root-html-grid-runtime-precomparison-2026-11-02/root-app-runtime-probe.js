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
const items = [];
if (grid !== null) {
  for (let child = grid.firstChild; child !== null; child = child.nextSibling) {
    if (child.nodeType === 1) items.push(child);
  }
}

if (!root || root.localName !== "html" || !head || !body ||
    !grid || grid.localName !== "main" || items.length !== 2 ||
    items[0].getAttribute("class") !== "item-a" ||
    items[1].getAttribute("class") !== "item-b") {
  throw new Error("parsed C11.1 root HTML Grid fixture did not reach the V8 document");
}
