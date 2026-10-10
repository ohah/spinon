const root = document.createElement("div");
root.setAttribute("id", "c091-centered-root");
root.setAttribute(
  "style",
  "display:block;width:auto;min-width:0;min-height:0;background-color:#101827",
);
document.appendChild(root);

const parent = document.createElement("div");
parent.setAttribute("id", "c091-centered-parent");
parent.setAttribute("style", "display:block;width:200px;background-color:#1f2937");
root.appendChild(parent);

const child = document.createElement("div");
child.setAttribute("id", "c091-centered-child");
child.setAttribute(
  "style",
  "display:block;width:120px;height:10px;margin-left:auto;margin-right:auto;background-color:#3366ff",
);
parent.appendChild(child);
