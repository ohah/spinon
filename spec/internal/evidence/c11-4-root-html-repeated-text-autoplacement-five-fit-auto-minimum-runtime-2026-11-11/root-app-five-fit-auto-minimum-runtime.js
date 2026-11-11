const firstElementNamed = (parent, name) => {
  if (parent === null) return null;
  for (let child = parent.firstChild; child !== null; child = child.nextSibling) {
    if (child.nodeType === 1 && child.localName === name) return child;
  }
  return null;
};

const elementChildren = (parent) => {
  const result = [];
  for (let child = parent.firstChild; child !== null; child = child.nextSibling) {
    if (child.nodeType === 1) result.push(child);
  }
  return result;
};

const root = document.documentElement;
const head = document.head;
const body = document.body;
const app = firstElementNamed(body, "main");
const grids = app === null ? [] : elementChildren(app);
const expectedTexts = [
  "A A A A A A",
  "AAAA AAAA A",
  "AA AA AA AA",
  "AAA AAA AAA",
  "AAA AAA",
  "A A A A A A",
  "A A A A A A A A",
  "AA",
];

if (!root || root.localName !== "html" || !head || !body ||
    root.getAttribute("lang") !== "en" || !app || app.id !== "app" ||
    grids.length !== 2 || grids[0].id !== "grid-light" || grids[1].id !== "grid-dark") {
  throw new Error("five-fit Root HTML smoke fixture has the expected root and scheme grids");
}

for (const [scheme, grid] of [["light", grids[0]], ["dark", grids[1]]]) {
  const items = elementChildren(grid);
  if (items.length !== 9 || items[0].id !== `${scheme}-item-0` ||
      elementChildren(items[0]).length !== 0) {
    throw new Error(`${scheme} Grid must contain the empty explicit-row control and eight text items`);
  }
  for (let index = 1; index <= 8; index += 1) {
    const item = items[index];
    const payload = firstElementNamed(item, "div");
    if (item.id !== `${scheme}-item-${index}` || !payload ||
        payload.getAttribute("class") !== "payload" || payload.textContent !== expectedTexts[index - 1]) {
      throw new Error(`${scheme} repeated text item ${index} does not match the fixed smoke input`);
    }
  }
}

app.setAttribute("data-runtime", "five-fit-ready");
if (app.getAttribute("data-runtime") !== "five-fit-ready") {
  throw new Error("five-fit Root HTML marker was not retained by the same runtime document");
}
