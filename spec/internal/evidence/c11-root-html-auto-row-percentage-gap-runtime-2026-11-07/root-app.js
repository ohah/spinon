if (!document.body) {
  throw new Error("Root HTML body should exist before the bootstrap script runs");
}

document.body.setAttribute("data-runtime", "ready");
if (document.body.getAttribute("data-runtime") !== "ready") {
  throw new Error("Root HTML attribute update should remain in the V8 document session");
}
