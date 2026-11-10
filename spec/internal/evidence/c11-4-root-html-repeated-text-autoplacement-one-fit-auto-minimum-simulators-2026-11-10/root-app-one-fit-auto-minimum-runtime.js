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
let hasLightDarkColorScheme = false;
if (head !== null) {
  for (let child = head.firstChild; child !== null; child = child.nextSibling) {
    if (child.nodeType === 1 && child.localName === "meta" &&
        child.getAttribute("name") === "color-scheme" &&
        child.getAttribute("content") === "light dark") {
      hasLightDarkColorScheme = true;
    }
  }
}

if (!root || root.localName !== "html" || !head || !body || !app ||
    app.getAttribute("id") !== "app" ||
    !hasLightDarkColorScheme) {
  throw new Error("C11.4 one-fit Root HTML smoke fixture is incomplete");
}

app.setAttribute("data-runtime", "ready");
if (app.getAttribute("data-runtime") !== "ready") {
  throw new Error("C11.4 one-fit automatic-minimum Root HTML did not retain the same V8 document");
}
