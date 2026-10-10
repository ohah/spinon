const css = String.raw`html, body { margin: 0; padding: 0; }
#mount { box-sizing: border-box; display: block; width: 100vw; height: 100vh; background-color: #101827; }
#row { box-sizing: border-box; display: flex; flex-direction: row; align-items: flex-start; gap: 6px; width: 100%; height: 42px; }
#content-box { box-sizing: content-box; width: 48px; height: 24px; padding: 4px; border: 3px solid red; background-color: #3366ff; }
#sibling { width: 28px; height: 24px; background-color: #22aa66; }
#border-box { box-sizing: border-box; width: 64px; height: 36px; padding: 4px; border-width: 3px 5px; border-style: dashed; background-color: #cc55aa; }
#none-border { box-sizing: content-box; width: 48px; height: 24px; border: 6px solid red; border-style: none; background-color: #ff9933; }`;

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

for (const id of ["content-box", "sibling", "border-box", "none-border"]) {
  const node = document.createElement("div");
  node.setAttribute("id", id);
  row.appendChild(node);
}
