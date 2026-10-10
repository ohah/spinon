const css = String.raw`  :root { font-size: 1.25rem; }
  body { margin: 0; }
  #mount {
    box-sizing: border-box;
    display: flex;
    flex-direction: column;
    width: 300px;
    height: 780px;
    font-size: 10px;
    row-gap: 2px;
  }
  #em-inherit { width: 2em; height: 1em; }
  #em-parent { font-size: 1.2em; width: 2em; height: 1em; }
  #em-child { font-size: 1.5em; width: 1em; height: 1em; }
  #rem-target {
    font-size: 1.25rem;
    width: 2rem;
    height: 1rem;
    margin-left: 0.25rem;
    padding-left: 0.5rem;
  }
  #font-size-percent { font-size: 150%; width: 1em; height: 1em; }
  #zero { font-size: 0; width: 2em; height: 2em; }
  #var-em { font-size: 14px; --box: 2em; width: var(--box); height: 1em; }
  #var-rem { font-size: 14px; --box: 2rem; width: var(--box); height: 1em; }
  #spacing {
    box-sizing: border-box;
    display: flex;
    flex-direction: column;
    width: 120px;
    height: 60px;
    font-size: 12px;
    row-gap: 0.25em;
    column-gap: 0.5em;
    margin-left: 0.5em;
    padding-left: 0.25em;
  }
  #basis-a { flex: 0 0 2em; height: 1rem; }
  #basis-b { flex: 0 0 1rem; height: 1em; }`;

const mount = document.createElement("div");
mount.setAttribute("id", "mount");
mount.setAttribute("style", "background-color:#101827");
document.appendChild(mount);

const style = document.createElement("style");
style.setAttribute("type", "text/css");
style.appendChild(document.createTextNode(css));
mount.appendChild(style);

function appendBox(parent, id, color) {
  const node = document.createElement("div");
  node.setAttribute("id", id);
  node.setAttribute("style", `background-color:${color}`);
  parent.appendChild(node);
  return node;
}

appendBox(mount, "em-inherit", "#3366ff");
const emParent = appendBox(mount, "em-parent", "#cc55aa");
appendBox(emParent, "em-child", "#ff9933");
appendBox(mount, "rem-target", "#22aa66");
appendBox(mount, "font-size-percent", "#ff9933");
appendBox(mount, "zero", "#3366ff");
appendBox(mount, "var-em", "#cc55aa");
appendBox(mount, "var-rem", "#22aa66");
const spacing = appendBox(mount, "spacing", "#26354f");
appendBox(spacing, "basis-a", "#3366ff");
appendBox(spacing, "basis-b", "#22aa66");
