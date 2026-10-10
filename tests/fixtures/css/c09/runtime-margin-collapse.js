const root = document.createElement("div");
root.setAttribute("id", "c092-margin-collapse-root");
root.setAttribute(
  "style",
  "display:block;width:auto;min-width:0;min-height:0;background-color:#101827",
);
document.appendChild(root);

const first = document.createElement("div");
first.setAttribute("id", "c092-margin-collapse-first");
first.setAttribute(
  "style",
  "display:block;width:160px;height:12px;margin-bottom:24px;background-color:#3366ff",
);
root.appendChild(first);

const empty = document.createElement("div");
empty.setAttribute("id", "c092-margin-collapse-empty");
empty.setAttribute(
  "style",
  "display:block;width:160px;height:0;margin-top:30px;margin-bottom:-12px",
);
root.appendChild(empty);

const last = document.createElement("div");
last.setAttribute("id", "c092-margin-collapse-last");
last.setAttribute(
  "style",
  "display:block;width:160px;height:16px;margin-top:10px;background-color:#ff6333",
);
root.appendChild(last);
