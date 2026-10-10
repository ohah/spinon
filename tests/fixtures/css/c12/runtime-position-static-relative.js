const stylesheet = String.raw`html, body { margin: 0; padding: 0; }
#c12-root { display: block; position: static; box-sizing: border-box; width: 100vw; height: 100vh; margin: 0; padding: 0; background-color: #101827; }
.case-row { display: block; position: static; box-sizing: border-box; width: 300px; height: 40px; margin: 0; padding: 0; }
.case-row.tall { height: 72px; }
.box { display: block; box-sizing: border-box; width: 40px; height: 12px; margin: 0; padding: 0; background-color: #3366ff; }
.small { width: 20px; height: 8px; background-color: #22aa66; }
#opposite-row, #flex-row { display: flex; flex-direction: row; align-items: flex-start; gap: 8px; }
#percent-container, #calc-container { box-sizing: content-box; width: 200px; height: 40px; padding: 10px; background-color: #24324a; }
#percent-target { position: relative; left: 10%; top: 25%; }
#calc-target { --move-x: 4px; position: relative; left: calc(10% + var(--move-x)); top: calc(-2px - var(--rise, 3px)); }
@layer c12-base, c12-adjust;
@layer c12-base {
  #layered-target { position: relative; inset: 2px 9px 3px 4px; top: 6px; }
  #important-target { position: relative; inset: 3px; left: 13px !important; }
}
@layer c12-adjust {
  .layered-target { left: 11px; }
}
#nested-parent { position: relative; left: 7px; top: 5px; width: 240px; height: 48px; background-color: #24324a; }
#nested-child { position: relative; left: 3px; top: 2px; width: 80px; height: 28px; background-color: #4c3478; }
#hidden-parent { display: none; position: relative; left: 20px; top: 10px; width: 100px; height: 30px; }
#hidden-child { position: relative; left: 12px; top: 4px; width: 20px; height: 10px; }
#dynamic-parent { position: static; width: 200px; height: 24px; background-color: #24324a; }
#dynamic-target { position: static; left: 15px; top: 2px; width: 40px; height: 12px; }
#dynamic-nested-parent { position: static; width: 200px; height: 24px; }
#dynamic-nested-child { position: relative; left: 8px; top: 0; width: 40px; height: 12px; }
`;

const fixture = document.createElement("div");
fixture.setAttribute("id", "c12-root");
fixture.setAttribute("data-c12-node", "");

const style = document.createElement("style");
style.setAttribute("type", "text/css");
style.appendChild(document.createTextNode(stylesheet));
fixture.appendChild(style);

function node(parent, id, inlineStyle = "", className = "") {
  const element = document.createElement("div");
  element.setAttribute("id", id);
  element.setAttribute("data-c12-node", "");
  if (className) element.setAttribute("class", className);
  if (inlineStyle) element.setAttribute("style", inlineStyle);
  parent.appendChild(element);
  return element;
}

function row(id, tall = false) {
  return node(fixture, id, "", tall ? "case-row tall" : "case-row");
}

const staticRow = row("static-row");
node(staticRow, "static-target", "position:static;left:12px;top:6px;width:40px;height:12px;background-color:#3366ff");
node(staticRow, "static-after", "width:20px;height:8px;background-color:#22aa66");

const relativeRow = row("relative-row");
node(relativeRow, "relative-target", "position:relative;left:12px;top:-3px;width:40px;height:12px;background-color:#3366ff");
node(relativeRow, "relative-after", "width:20px;height:8px;background-color:#22aa66");

const oppositeRow = row("opposite-row");
node(oppositeRow, "right-only", "position:relative;right:8px;width:32px;height:12px;background-color:#3366ff");
node(oppositeRow, "bottom-only", "position:relative;bottom:4px;width:32px;height:12px;background-color:#22aa66");
node(oppositeRow, "auto-insets", "position:relative;top:auto;right:auto;bottom:auto;left:auto;width:32px;height:12px;background-color:#cc55aa");

const overconstrainedRow = row("overconstrained-row");
node(overconstrainedRow, "overconstrained-target", "position:relative;left:5px;right:40px;top:3px;bottom:19px;width:40px;height:12px;background-color:#3366ff");
node(overconstrainedRow, "overconstrained-after", "width:20px;height:8px;background-color:#22aa66");

const percentRow = row("percent-row", true);
const percentContainer = node(percentRow, "percent-container");
node(percentContainer, "percent-target", "width:20px;height:10px;background-color:#3366ff");

const calcRow = row("calc-row", true);
const calcContainer = node(calcRow, "calc-container");
node(calcContainer, "calc-target", "width:20px;height:10px;background-color:#3366ff");

const cascadeRow = row("cascade-row");
node(cascadeRow, "layered-target", "width:40px;height:12px;background-color:#3366ff", "layered-target");
node(cascadeRow, "important-target", "width:40px;height:12px;background-color:#22aa66");

const nestedRow = row("nested-row", true);
const nestedParent = node(nestedRow, "nested-parent");
const nestedChild = node(nestedParent, "nested-child");
node(nestedChild, "nested-grandchild", "position:static;width:20px;height:8px;background-color:#22aa66");
node(nestedParent, "nested-parent-after", "width:40px;height:8px;background-color:#cc55aa");
node(nestedRow, "nested-outside-after", "width:20px;height:8px;background-color:#f09b32");

const shorthandRow = row("shorthand-row", true);
node(shorthandRow, "inset-one", "position:relative;inset:6px;width:30px;height:10px;background-color:#3366ff");
node(shorthandRow, "inset-two", "position:relative;inset:7px 8px;width:30px;height:10px;background-color:#22aa66");
node(shorthandRow, "inset-three", "position:relative;inset:9px 10px 11px;width:30px;height:10px;background-color:#cc55aa");
node(shorthandRow, "inset-four", "position:relative;inset:1px 2px 3px 4px;width:30px;height:10px;background-color:#f09b32");

const flexRow = row("flex-row");
node(flexRow, "flex-relative", "position:relative;left:10px;top:4px;width:40px;height:12px;background-color:#3366ff");
node(flexRow, "flex-sibling", "width:40px;height:12px;background-color:#22aa66");

const hiddenRow = row("hidden-row");
const hiddenParent = node(hiddenRow, "hidden-parent");
node(hiddenParent, "hidden-child");
node(hiddenRow, "hidden-sibling", "width:20px;height:8px;background-color:#22aa66");

const mutationRow = row("mutation-row", true);
const dynamicParent = node(mutationRow, "dynamic-parent");
node(dynamicParent, "dynamic-target");
const dynamicNestedParent = node(mutationRow, "dynamic-nested-parent");
node(dynamicNestedParent, "dynamic-nested-child");
node(mutationRow, "dynamic-after", "width:20px;height:8px;background-color:#22aa66");

const mountPoint = document.body || document;
mountPoint.appendChild(fixture);
