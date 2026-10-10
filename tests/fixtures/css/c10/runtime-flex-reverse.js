const root = document.createElement("div");
root.setAttribute("id", "c1031-runtime-root");
root.setAttribute(
  "style",
  "display:flex;box-sizing:border-box;width:320px;height:240px;flex-direction:column;flex-wrap:nowrap;justify-content:flex-start;align-items:flex-start;flex-grow:0;flex-shrink:0;min-width:0;min-height:0;margin:0;padding:0;border:0;background-color:#101827",
);
document.appendChild(root);

const flex = document.createElement("div");
flex.setAttribute("id", "c1031-runtime-flex");
flex.setAttribute(
  "style",
  "display:flex;box-sizing:border-box;width:170px;height:68px;flex-direction:row-reverse;flex-wrap:wrap-reverse;justify-content:flex-start;align-items:flex-start;flex-grow:0;flex-shrink:0;min-width:0;min-height:0;row-gap:4px;column-gap:6px;margin:8px;padding:0;border:0;background-color:#243047",
);
root.appendChild(flex);

const tiles = ["#3366ff", "#12b981", "#f59e0b", "#ef4444", "#8b5cf6"];
for (let index = 0; index < tiles.length; index += 1) {
  const tile = document.createElement("div");
  tile.setAttribute("id", `c1031-runtime-item-${index + 1}`);
  tile.setAttribute(
    "style",
    `display:block;box-sizing:border-box;width:70px;height:20px;min-width:0;min-height:0;flex-grow:0;flex-shrink:0;flex-basis:auto;margin:0;padding:0;border:0;background-color:${tiles[index]}`,
  );
  flex.appendChild(tile);
}
