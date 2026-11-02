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
const classes = [];
if (grid !== null) {
  for (let child = grid.firstChild; child !== null; child = child.nextSibling) {
    if (child.nodeType === 1) {
      ids.push(child.getAttribute("id"));
      classes.push(child.getAttribute("class"));
    }
  }
}
const expectedIds = [
  "tile-10", "tile-09", "tile-08", "tile-07", "tile-06",
  "tile-05", "tile-04", "tile-03", "tile-02", "tile-01",
];
if (root === null || root.localName !== "html" || head === null || body === null ||
    grid === null || grid.getAttribute("id") !== "percentage-rows-grid" ||
    ids.length !== expectedIds.length || expectedIds.some((id, index) => ids[index] !== id) ||
    classes.some((className) => className !== "grid-cell-v2")) {
  throw new Error("parsed percentage row-list fixture lost its renamed, reversed source order");
}
grid.setAttribute("data-runtime", "ready");
if (grid.getAttribute("data-runtime") !== "ready") {
  throw new Error("Root HTML fixture mutation was not observable in the V8 document");
}
