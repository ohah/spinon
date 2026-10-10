const root = document.createElement("div");
root.setAttribute("id", "c1034-runtime-root");
root.setAttribute(
  "style",
  "display:flex;box-sizing:border-box;width:320px;height:640px;flex-direction:column;align-items:stretch;justify-content:flex-start;min-width:0;min-height:0;margin:0;padding:0;border:0;background-color:#101827",
);
document.appendChild(root);

function appendFlexRow(id, alignment, children) {
  const row = document.createElement("div");
  row.setAttribute("id", id);
  row.setAttribute(
    "style",
    `display:flex;box-sizing:border-box;width:280px;height:100px;flex:0 0 auto;flex-direction:row;flex-wrap:nowrap;align-items:${alignment};justify-content:flex-start;min-width:0;min-height:0;margin:0;padding:0;border:0;background-color:#243047`,
  );
  root.appendChild(row);
  for (const childSpec of children) {
    const child = document.createElement("div");
    child.setAttribute("id", childSpec.id);
    child.setAttribute(
      "style",
      `display:block;box-sizing:border-box;width:48px;height:${childSpec.height}px;flex:0 0 auto;align-self:${childSpec.alignment ?? "auto"};margin:${childSpec.margin ?? "0"};padding:0;border:0;background-color:${childSpec.color}`,
    );
    row.appendChild(child);
  }
  return row;
}

appendFlexRow("c1034-runtime-first", "baseline", [
  { id: "c1034-runtime-first-a", height: 20, color: "#3366ff" },
  { id: "c1034-runtime-first-b", height: 40, color: "#12b981" },
]);

appendFlexRow("c1034-runtime-last", "last baseline", [
  { id: "c1034-runtime-last-a", height: 20, color: "#f59e0b" },
  { id: "c1034-runtime-last-b", height: 40, color: "#ef4444" },
]);

const nestedRow = document.createElement("div");
nestedRow.setAttribute("id", "c1034-runtime-nested-row");
nestedRow.setAttribute(
  "style",
  "display:flex;box-sizing:border-box;width:280px;height:100px;flex:0 0 auto;flex-direction:row;flex-wrap:nowrap;align-items:last baseline;justify-content:flex-start;min-width:0;min-height:0;margin:0;padding:0;border:0;background-color:#334155",
);
root.appendChild(nestedRow);

const nested = document.createElement("div");
nested.setAttribute("id", "c1034-runtime-nested-flex");
nested.setAttribute(
  "style",
  "display:flex;box-sizing:border-box;width:100px;height:60px;flex:0 0 auto;flex-direction:row;flex-wrap:wrap;align-items:flex-start;align-content:flex-start;justify-content:flex-start;min-width:0;min-height:0;margin:0;padding:0;border:0;background-color:#475569",
);
nestedRow.appendChild(nested);

for (const childSpec of [
  { id: "c1034-runtime-nested-a", height: 20, color: "#22d3ee" },
  { id: "c1034-runtime-nested-b", height: 40, color: "#a78bfa" },
]) {
  const child = document.createElement("div");
  child.setAttribute("id", childSpec.id);
  child.setAttribute(
    "style",
    `display:block;box-sizing:border-box;width:60px;height:${childSpec.height}px;flex:0 0 auto;align-self:auto;margin:0;padding:0;border:0;background-color:${childSpec.color}`,
  );
  nested.appendChild(child);
}

const peer = document.createElement("div");
peer.setAttribute("id", "c1034-runtime-nested-peer");
peer.setAttribute(
  "style",
  "display:block;box-sizing:border-box;width:48px;height:25px;flex:0 0 auto;align-self:auto;margin:0;padding:0;border:0;background-color:#fb7185",
);
nestedRow.appendChild(peer);
