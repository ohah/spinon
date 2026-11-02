const body = document.body;
const root = document.documentElement;
const grids = [];
if (body !== null) {
  for (let child = body.firstChild; child !== null; child = child.nextSibling) {
    if (child.nodeType === 1 && child.localName === "main") grids.push(child);
  }
}

const gridItems = grids.map((grid) => {
  const items = [];
  for (let child = grid.firstChild; child !== null; child = child.nextSibling) {
    if (child.nodeType === 1) items.push(child);
  }
  return items;
});

if (
  root === null || root.localName !== "html" || body === null || grids.length !== 2 ||
  gridItems[0].length !== 9 || gridItems[0].some((item) => item.getAttribute("class") !== "marker") ||
  grids[1].getAttribute("class") !== "overflow-grid" || gridItems[1].length !== 2 ||
  gridItems[1][0].getAttribute("class") !== "overflow-first" ||
  gridItems[1][1].getAttribute("class") !== "overflow-second"
) {
  throw new Error("parsed C11.1 percentage Grid fixture did not reach the V8 document");
}
