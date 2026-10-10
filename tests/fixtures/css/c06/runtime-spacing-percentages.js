const root = document.createElement("div");
root.setAttribute(
  "style",
  "display:flex;box-sizing:border-box;width:100%;height:100%;flex-direction:column;background-color:#123456"
);
document.appendChild(root);

const row = document.createElement("div");
row.setAttribute(
  "style",
  "display:flex;box-sizing:border-box;width:90%;height:40%;padding:4%;column-gap:5%;background-color:#26354f"
);
root.appendChild(row);

const rowLeft = document.createElement("div");
rowLeft.setAttribute(
  "style",
  "display:block;box-sizing:border-box;flex:0 0 35%;height:55%;margin:5%;padding:5%;background-color:#3366ff"
);
row.appendChild(rowLeft);

const rowRight = document.createElement("div");
rowRight.setAttribute(
  "style",
  "display:block;box-sizing:border-box;flex:0 0 35%;height:55%;margin:-2%;padding:2%;background-color:#22aa66"
);
row.appendChild(rowRight);

const column = document.createElement("div");
column.setAttribute(
  "style",
  "display:flex;box-sizing:border-box;flex-direction:column;width:90%;height:40%;padding:4%;row-gap:8%;background-color:#26354f"
);
root.appendChild(column);

const columnTop = document.createElement("div");
columnTop.setAttribute(
  "style",
  "display:block;box-sizing:border-box;flex:0 0 20%;width:65%;margin:2%;padding:3%;background-color:#ff9933"
);
column.appendChild(columnTop);

const columnBottom = document.createElement("div");
columnBottom.setAttribute(
  "style",
  "display:block;box-sizing:border-box;flex:0 0 20%;width:65%;margin:2%;padding:3%;background-color:#cc55aa"
);
column.appendChild(columnBottom);
