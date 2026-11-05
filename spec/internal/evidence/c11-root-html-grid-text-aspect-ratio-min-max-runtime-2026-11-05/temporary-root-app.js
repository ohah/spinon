const root = document.documentElement;
if (!root || root.localName !== "html" || !document.head || !document.body ||
    document.head.parentNode !== root || document.body.parentNode !== root) {
  throw new Error("Root HTML fixture was not parsed into the expected document tree");
}
root.setAttribute("data-runtime", "ready");
if (root.getAttribute("data-runtime") !== "ready") {
  throw new Error("Root HTML mutation did not persist in the default app session");
}
