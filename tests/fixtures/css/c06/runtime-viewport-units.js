const css = String.raw`body { margin: 0; }
#mount { box-sizing: border-box; display: block; width: 100vw; height: 100vh; background-color: #101827; }
#default-units { width: 50vw; height: 5vh; }
#small-units { width: calc(25svw + 1svi + 1svmin); height: calc(5svh + 1svb + 1svmax); }
#large-units { width: calc(25lvw + 1lvi + 1lvmin); height: calc(5lvh + 1lvb + 1lvmax); }
#dynamic-units { width: calc(25dvw + 1dvi + 1dvmin); height: calc(5dvh + 1dvb + 1dvmax); }
#logical-units { width: 10vi; height: 5vb; }
#minmax-units { width: 10vmin; height: 5vmax; }
#math-units { width: calc(10vw + 10px); height: min(5vh, 45px); }
#var-units { --viewport-width: 12vw; width: var(--viewport-width); height: 6px; }
#spacing-units { box-sizing: border-box; width: 30px; height: 6px; margin-left: 5vw; padding-left: 2vw; }
#gap-row { box-sizing: border-box; display: flex; flex-direction: row; width: 50vw; height: 6px; column-gap: 5vw; }
#gap-a, #gap-b { box-sizing: border-box; flex: 0 0 10vw; height: 6px; }
#basis-row { box-sizing: border-box; display: flex; flex-direction: row; width: 50vw; height: 6px; }
#basis-item { box-sizing: border-box; flex: 0 0 25vw; height: 6px; }`;

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

for (const [id, color] of [
  ["default-units", "#3366ff"], ["small-units", "#cc55aa"],
  ["large-units", "#22aa66"], ["dynamic-units", "#ff9933"],
  ["logical-units", "#f2cf5b"], ["minmax-units", "#ff6688"],
  ["math-units", "#8c62ff"], ["var-units", "#44c7d9"],
  ["spacing-units", "#d9e2f2"],
]) append(mount, id, color);

const gapRow = append(mount, "gap-row", "#26354f");
append(gapRow, "gap-a", "#3366ff");
append(gapRow, "gap-b", "#22aa66");
const basisRow = append(mount, "basis-row", "#26354f");
append(basisRow, "basis-item", "#cc55aa");
