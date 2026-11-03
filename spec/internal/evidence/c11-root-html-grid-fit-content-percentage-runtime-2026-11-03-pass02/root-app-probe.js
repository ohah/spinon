const firstElementNamed = (parent, name) => {
  if (parent === null) return null;
  for (let child = parent.firstChild; child !== null; child = child.nextSibling) {
    if (child.nodeType === 1 && child.localName === name) return child;
  }
  return null;
};
const html = document.documentElement;
const main = firstElementNamed(document.body, "main");
const item = firstElementNamed(main, "div");
if (!html || html.localName !== "html" || !main || main.id !== "root" ||
    !item || item.id !== "item") {
  throw new Error("C11 fit-content percentage Root HTML fixture did not initialize");
}
html.setAttribute("data-c11-fit-content-percentage-runtime", "ready");
