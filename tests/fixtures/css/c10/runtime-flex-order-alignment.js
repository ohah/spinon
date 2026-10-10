const root = document.createElement("div");
root.setAttribute("id", "c1032-runtime-root");
root.setAttribute(
  "style",
  "display:flex;box-sizing:border-box;width:320px;height:240px;flex-direction:column;flex-wrap:nowrap;justify-content:flex-start;align-items:flex-start;flex-grow:0;flex-shrink:0;min-width:0;min-height:0;row-gap:12px;margin:0;padding:0;border:0;background-color:#101827",
);
document.appendChild(root);

const createFlex = (id, width, height, background) => {
  const element = document.createElement("div");
  element.setAttribute("id", id);
  element.setAttribute(
    "style",
    `display:flex;box-sizing:border-box;width:${width}px;height:${height}px;flex-direction:row;flex-wrap:nowrap;justify-content:flex-start;align-items:flex-start;flex-grow:0;flex-shrink:0;min-width:0;min-height:0;margin:0;padding:0;border:0;background-color:${background}`,
  );
  root.appendChild(element);
  return element;
};

const basic = createFlex("c1032-runtime-basic", 180, 40, "#243047");
const basicItems = [
  ["a", 2, "#e5484d"],
  ["b", -1, "#28a745"],
  ["c", 0, "#2866f6"],
];
for (const [name, order, color] of basicItems) {
  const item = document.createElement("div");
  item.setAttribute("id", `c1032-runtime-basic-${name}`);
  item.setAttribute(
    "style",
    `display:block;box-sizing:border-box;width:60px;height:40px;min-width:0;min-height:0;flex-grow:0;flex-shrink:0;flex-basis:auto;order:${order};margin:0;padding:0;border:0;background-color:${color}`,
  );
  basic.appendChild(item);
}

const overlap = createFlex("c1032-runtime-overlap", 120, 40, "#243047");
const overlapItems = [
  ["a", 2, -30, "#e5484d"],
  ["b", -1, -30, "#28a745"],
  ["c", 0, -30, "#2866f6"],
];
for (const [name, order, marginLeft, color] of overlapItems) {
  const item = document.createElement("div");
  item.setAttribute("id", `c1032-runtime-overlap-${name}`);
  item.setAttribute(
    "style",
    `display:block;box-sizing:border-box;width:80px;height:40px;min-width:0;min-height:0;flex-grow:0;flex-shrink:0;flex-basis:auto;order:${order};margin:0 0 0 ${marginLeft}px;padding:0;border:0;background-color:${color}`,
  );
  overlap.appendChild(item);
}
