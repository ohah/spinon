const firstElementNamed = (parent, name) => {
  if (parent === null) return null;
  for (let child = parent.firstChild; child !== null; child = child.nextSibling) {
    if (child.nodeType === 1 && child.localName === name) return child;
  }
  return null;
};

const root = document.documentElement;
const body = document.body;
const grid = firstElementNamed(body, "main");
let directDivs = 0;
if (grid !== null) {
  for (let child = grid.firstChild; child !== null; child = child.nextSibling) {
    if (child.nodeType === 1 && child.localName === "div") directDivs += 1;
  }
}
if (!root || root.localName !== "html" || !body || body.parentNode !== root || !grid || directDivs !== 4) {
  throw new Error("column-flow Root HTML fixture did not produce its four direct items");
}
grid.setAttribute("data-runtime", "ready");
if (grid.getAttribute("data-runtime") !== "ready") {
  throw new Error("Root HTML runtime marker did not persist in the same V8 document");
}
