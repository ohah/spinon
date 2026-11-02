// examples/bootstrap/root-app.js
var body = document.body;
if (!body)
  throw new Error("parsed root HTML body is missing");
var gridCount = 0;
var marker = null;
for (let node = body.firstChild;node !== null; node = node.nextSibling) {
  if (node.nodeType === 1 && node.localName === "section" && node.getAttribute("class") === "grid") {
    gridCount += 1;
    if (marker === null)
      marker = node;
  }
}
if (gridCount !== 7 || marker === null) {
  throw new Error(`expected seven parsed grid sections, received ${gridCount}`);
}
marker.setAttribute("data-runtime", "ready");
if (marker.getAttribute("data-runtime") !== "ready") {
  throw new Error("parsed Root HTML mutation was not observed");
}
