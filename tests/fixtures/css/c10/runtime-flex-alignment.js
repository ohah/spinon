const root = document.createElement("div");
root.setAttribute("id", "c1033-runtime-root");
root.setAttribute(
  "style",
  "display:flex;box-sizing:border-box;width:320px;height:240px;flex-direction:column;flex-wrap:nowrap;justify-content:flex-start;align-items:stretch;flex-grow:0;flex-shrink:0;min-width:0;min-height:0;margin:0;padding:0;border:0;background-color:#101827",
);
document.appendChild(root);

const flex = document.createElement("div");
flex.setAttribute("id", "c1033-runtime-flex");
flex.setAttribute(
  "style",
  "display:flex;box-sizing:border-box;width:230px;height:160px;flex-direction:row;flex-wrap:wrap;justify-content:space-between;align-items:normal;align-content:space-evenly;row-gap:8px;column-gap:8px;flex-grow:0;flex-shrink:0;min-width:0;min-height:0;margin:12px;padding:8px;border:2px solid #52627a;background-color:#243047",
);
root.appendChild(flex);

const items = [
  { color: "#3366ff", alignment: "auto", height: "36px" },
  { color: "#12b981", alignment: "safe center", height: "20px" },
  { color: "#f59e0b", alignment: "flex-end", height: "36px" },
  { color: "#ef4444", alignment: "stretch", height: "36px" },
];
for (let index = 0; index < items.length; index += 1) {
  const tile = document.createElement("div");
  tile.setAttribute("id", `c1033-runtime-item-${index + 1}`);
  tile.setAttribute(
    "style",
    `display:block;box-sizing:border-box;width:100px;height:${items[index].height};min-width:0;min-height:0;flex-grow:0;flex-shrink:0;flex-basis:auto;align-self:${items[index].alignment};margin:0;padding:0;border:0;background-color:${items[index].color}`,
  );
  flex.appendChild(tile);
}

const overflowFlex = document.createElement("div");
overflowFlex.setAttribute("id", "c1033-runtime-negative-auto-flex");
overflowFlex.setAttribute(
  "style",
  "display:flex;box-sizing:border-box;width:60px;height:10px;flex-direction:row;flex-wrap:nowrap;align-items:center;flex-grow:0;flex-shrink:0;min-width:0;min-height:0;margin:0;padding:0;border:0;background-color:#475569",
);
root.appendChild(overflowFlex);

const overflowItem = document.createElement("div");
overflowItem.setAttribute("id", "c1033-runtime-negative-auto-item");
overflowItem.setAttribute(
  "style",
  "display:block;box-sizing:border-box;width:20px;height:20px;flex-grow:0;flex-shrink:0;flex-basis:auto;align-self:flex-end;margin-top:auto;margin-right:0;margin-bottom:0;margin-left:0;padding:0;border:0;background-color:#f97316",
);
overflowFlex.appendChild(overflowItem);
