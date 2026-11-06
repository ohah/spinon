const findById = (node, id) => {
  if (node === null) return null;
  if (node.nodeType === 1 && node.getAttribute("id") === id) return node;
  for (let child = node.firstChild; child !== null; child = child.nextSibling) {
    const found = findById(child, id);
    if (found !== null) return found;
  }
  return null;
};

const root = document.documentElement;
const grid = findById(root, "grid");
const expectedOrder = ["first", "long-text", "short-text", "last"];
const actualOrder = [];
if (grid !== null) {
  for (let child = grid.firstChild; child !== null; child = child.nextSibling) {
    if (child.nodeType === 1) actualOrder.push(child.getAttribute("id"));
  }
}
const longText = findById(root, "long-text");
const shortText = findById(root, "short-text");
if (root === null || grid === null || longText === null || shortText === null ||
    actualOrder.join(",") !== expectedOrder.join(",") ||
    longText.textContent !== "A A A A A A" || shortText.textContent !== "A") {
  throw new Error("multiple-text Root HTML fixture DOM was not initialized as expected");
}
