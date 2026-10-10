const css = String.raw`html, body { margin: 0; padding: 0; }
#mount { box-sizing: border-box; display: block; width: 100vw; height: 100vh; background-color: #101827; }
.row { box-sizing: border-box; display: flex; flex-direction: row; align-items: center; width: 100%; height: 18px; }
#minimum { width: 28px; height: 12px; min-width: 72px; }
#maximum { width: 120px; height: 12px; max-width: 70px; }
#content-box { box-sizing: content-box; width: 46px; height: 12px; padding: 0 8px; max-width: 50px; }
#percent-row { width: 200px; }
#percentage { width: 30px; height: 12px; min-width: 25%; }
#flex-row { width: 200px; }
#flex-a, #flex-b { box-sizing: border-box; flex: 1 1 0; min-width: 0; max-width: 55px; height: 12px; }`;

const mount = document.createElement("div");
mount.setAttribute("id", "mount");
document.appendChild(mount);

const style = document.createElement("style");
style.setAttribute("type", "text/css");
style.appendChild(document.createTextNode(css));
mount.appendChild(style);

function append(parent, id, color) {
  const node = document.createElement("div");
  node.setAttribute("id", id);
  node.setAttribute("style", `background-color:${color}`);
  parent.appendChild(node);
  return node;
}

const basicRow = append(mount, "basic-row", "#26354f");
append(basicRow, "minimum", "#3366ff");
append(basicRow, "maximum", "#22aa66");
append(basicRow, "content-box", "#cc55aa");

const percentRow = append(mount, "percent-row", "#26354f");
append(percentRow, "percentage", "#ff9933");

const flexRow = append(mount, "flex-row", "#26354f");
append(flexRow, "flex-a", "#f2cf5b");
append(flexRow, "flex-b", "#44c7d9");
