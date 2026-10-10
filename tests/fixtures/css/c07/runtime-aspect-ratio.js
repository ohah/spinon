const css = String.raw`html, body { margin: 0; padding: 0; }
#mount { box-sizing: border-box; display: block; width: 100vw; height: 100vh; background-color: #101827; }
#row { box-sizing: border-box; display: flex; flex-direction: row; align-items: flex-start; gap: 8px; width: 100%; height: 100%; padding: 8px; }
#wide { box-sizing: border-box; width: 88px; aspect-ratio: 16 / 9; background-color: #3366ff; }
#square { box-sizing: border-box; width: 64px; aspect-ratio: 1; background-color: #22aa66; }
#tall { box-sizing: border-box; width: 42px; aspect-ratio: 3 / 4; background-color: #cc55aa; }`;

const mount = document.createElement("div");
mount.setAttribute("id", "mount");
document.appendChild(mount);

const style = document.createElement("style");
style.setAttribute("type", "text/css");
style.appendChild(document.createTextNode(css));
mount.appendChild(style);

const row = document.createElement("div");
row.setAttribute("id", "row");
mount.appendChild(row);

for (const id of ["wide", "square", "tall"]) {
  const node = document.createElement("div");
  node.setAttribute("id", id);
  row.appendChild(node);
}
