const root = document.createElement("div");
root.setAttribute("id", "c102-runtime-root");
root.setAttribute(
  "style",
  "display:flex;box-sizing:border-box;width:320px;height:240px;flex-direction:column;flex-wrap:nowrap;justify-content:flex-start;align-items:flex-start;flex-grow:0;flex-shrink:0;min-width:0;min-height:0;margin:0;padding:0;border:0;background-color:#101827;--c102-first:#3366ff;--c102-second:#12b981;--c102-third:#f59e0b",
);
document.appendChild(root);

const flex = document.createElement("div");
flex.setAttribute("id", "c102-runtime-flex");
flex.setAttribute(
  "style",
  "display:flex;box-sizing:border-box;width:304px;height:24px;flex-direction:row;flex-wrap:nowrap;justify-content:flex-start;align-items:flex-start;flex-grow:0;flex-shrink:0;min-width:0;min-height:0;margin:8px;padding:0;border:0;row-gap:0;column-gap:0;background-color:#243047",
);
root.appendChild(flex);

const tiles = [
  { id: "c102-runtime-item-1", limit: "max-width:80px", color: "--c102-first" },
  { id: "c102-runtime-item-2", limit: "min-width:120px", color: "--c102-second" },
  { id: "c102-runtime-item-3", limit: "", color: "--c102-third" },
];

for (const tile of tiles) {
  const item = document.createElement("div");
  item.setAttribute("id", tile.id);
  item.setAttribute(
    "style",
    `display:block;box-sizing:border-box;height:20px;min-width:0;min-height:0;${tile.limit};flex:1 0 60px;margin:0;padding:0;border:0;background-color:var(${tile.color})`,
  );
  flex.appendChild(item);
}
