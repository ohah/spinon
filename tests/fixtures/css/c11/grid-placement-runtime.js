(() => {
  const sceneRoot = document.createElement("div");
  sceneRoot.setAttribute("id", "c1121-runtime-scene-root");
  sceneRoot.setAttribute("style", "display:flex;width:360px;height:800px");
  document.appendChild(sceneRoot);

  const grid = document.createElement("div");
  grid.setAttribute("id", "c1121-numeric-grid");
  grid.setAttribute("data-c1121-grid-node", "");
  grid.setAttribute("style", "display:grid;box-sizing:border-box;width:250px;height:85px;grid-template-columns:80px 80px 80px;grid-template-rows:40px 40px;column-gap:5px;row-gap:5px;background-color:#edf1f7");
  sceneRoot.appendChild(grid);

  const positive = document.createElement("div");
  positive.setAttribute("id", "c1121-positive-lines");
  positive.setAttribute("data-c1121-grid-node", "");
  positive.setAttribute("style", "display:block;box-sizing:content-box;width:auto;height:auto;margin:0;padding:0;grid-column:1 / span 2;grid-row:1 / 2;background-color:#2866f6");
  grid.appendChild(positive);

  const negative = document.createElement("div");
  negative.setAttribute("id", "c1121-negative-lines");
  negative.setAttribute("data-c1121-grid-node", "");
  negative.setAttribute("style", "display:block;box-sizing:content-box;width:auto;height:auto;margin:0;padding:0;grid-column:-2 / -1;grid-row:-2 / -1;background-color:#e5484d");
  grid.appendChild(negative);

  return {
    sceneId: "numeric-lines",
    rootId: "c1121-numeric-grid",
    nodeIds: ["c1121-numeric-grid", "c1121-positive-lines", "c1121-negative-lines"],
  };
})()
