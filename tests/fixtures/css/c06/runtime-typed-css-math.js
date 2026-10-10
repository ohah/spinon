const css = String.raw`  body { margin: 0; }
  #mount {
    box-sizing: border-box;
    display: flex;
    flex-direction: column;
    width: 300px;
    height: 780px;
    row-gap: 1px;
    background-color: #101827;
  }
  #calc-mixed { box-sizing: border-box; width: calc(10px + 25%); flex: 0 0 8px; height: 6px; }
  #min-value { box-sizing: border-box; width: min(100px, 50%); flex: 0 0 8px; height: 6px; }
  #max-value { box-sizing: border-box; width: max(100px, 50%); flex: 0 0 8px; height: 6px; }
  #clamp-value { box-sizing: border-box; width: clamp(20px, 50%, 100px); flex: 0 0 8px; height: 6px; }
  #clamp-nan { box-sizing: border-box; width: clamp(10px, calc(0px / 0), 20px); flex: 0 0 8px; height: 6px; }
  #negative-margin { box-sizing: border-box; width: 30px; flex: 0 0 8px; height: 6px; margin-left: calc(5px - 10px); }
  #gap-row { display: flex; box-sizing: border-box; flex: 0 0 8px; flex-direction: row; width: 300px; column-gap: calc(5px + 5%); }
  #gap-a { box-sizing: border-box; flex: 0 0 30px; height: 6px; }
  #gap-b { box-sizing: border-box; flex: 0 0 30px; height: 6px; }
  #basis-row { display: flex; box-sizing: border-box; flex: 0 0 8px; flex-direction: row; width: 300px; }
  #basis-math { box-sizing: border-box; flex: 0 0 calc(40px + 10%); height: 6px; }`;

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

append(mount, "calc-mixed", "#3366ff");
append(mount, "min-value", "#cc55aa");
append(mount, "max-value", "#22aa66");
append(mount, "clamp-value", "#ff9933");
append(mount, "clamp-nan", "#f2cf5b");
append(mount, "negative-margin", "#ff6688");
const gapRow = append(mount, "gap-row", "#26354f");
append(gapRow, "gap-a", "#3366ff");
append(gapRow, "gap-b", "#22aa66");
const basisRow = append(mount, "basis-row", "#26354f");
append(basisRow, "basis-math", "#cc55aa");
