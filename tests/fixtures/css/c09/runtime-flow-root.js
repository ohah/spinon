const root = document.createElement("div");
root.setAttribute("id", "c093-runtime-root");
root.setAttribute(
  "style",
  "display:block;width:auto;height:240px;min-width:0;min-height:0;background-color:#101827",
);
document.appendChild(root);

const before = document.createElement("div");
before.setAttribute("id", "c093-runtime-before");
before.setAttribute(
  "style",
  "display:block;width:220px;height:10px;margin-bottom:30px;background-color:#f97316",
);
root.appendChild(before);

const flowRoot = document.createElement("div");
flowRoot.setAttribute("id", "c093-runtime-flow-root");
flowRoot.setAttribute(
  "style",
  "display:flow-root;width:220px;margin-top:20px;margin-bottom:30px;background-color:#243047",
);
root.appendChild(flowRoot);

const child = document.createElement("div");
child.setAttribute("id", "c093-runtime-child");
child.setAttribute(
  "style",
  "display:block;width:180px;height:24px;margin-top:20px;margin-bottom:30px;background-color:#3366ff",
);
flowRoot.appendChild(child);

const after = document.createElement("div");
after.setAttribute("id", "c093-runtime-after");
after.setAttribute(
  "style",
  "display:block;width:220px;height:10px;margin-top:20px;background-color:#60a5fa",
);
root.appendChild(after);
