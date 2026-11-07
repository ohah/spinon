// examples/bootstrap/root-app.js
var body = document.body;
if (!body)
  throw new Error("Root HTML fixture has no body");
body.setAttribute("data-root-app-marker", "ready");
