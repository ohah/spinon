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
const app = firstElementNamed(body, "main");

if (!root || root.localName !== "html" || !head || !body || !app ||
    app.getAttribute("id") !== "app") {
  throw new Error("C11.4 root HTML smoke fixture is incomplete");
}

app.setAttribute("data-runtime", "ready");
if (app.getAttribute("data-runtime") !== "ready") {
  throw new Error("C11.4 four-fit automatic-minimum Root HTML smoke did not retain the same V8 document");
}
