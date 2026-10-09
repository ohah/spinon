{
const state = globalThis.__spinonC054IncrementalRestyle;
if (!state) {
  const app = document.createElement("div");
  app.setAttribute(
    "style",
    "display:flex;box-sizing:border-box;width:301px;height:100px;align-items:flex-start;column-gap:10px;background-color:#123456"
  );
  document.appendChild(app);

  const left = document.createElement("div");
  left.setAttribute(
    "style",
    "display:flex;box-sizing:border-box;width:140px;height:80px;--tile-size:32px"
  );
  const leftTile = document.createElement("div");
  leftTile.setAttribute(
    "style",
    "display:block;flex-shrink:0;width:var(--tile-size);height:24px;background-color:#3366ff"
  );
  left.appendChild(leftTile);
  app.appendChild(left);

  const right = document.createElement("div");
  right.setAttribute(
    "style",
    "display:flex;box-sizing:border-box;width:140px;height:80px;--tile-size:41px"
  );
  const rightTile = document.createElement("div");
  rightTile.setAttribute(
    "style",
    "display:block;flex-shrink:0;width:var(--tile-size);height:24px;background-color:#22aa66"
  );
  right.appendChild(rightTile);
  app.appendChild(right);

  globalThis.__spinonC054IncrementalRestyle = { left, size: 32 };
} else {
  state.size = state.size === 32 ? 46 : 32;
  state.left.setAttribute(
    "style",
    `display:flex;box-sizing:border-box;width:140px;height:80px;--tile-size:${state.size}px`
  );
}
}
