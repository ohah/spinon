const root = document.createElement("div");
root.setAttribute("id", "root");
root.setAttribute(
  "style",
  "box-sizing:border-box;width:280px;background-color:#101827",
);
document.appendChild(root);

function appendBlock(parent, id, style) {
  const element = document.createElement("div");
  element.setAttribute("id", id);
  element.setAttribute("style", style);
  parent.appendChild(element);
  return element;
}

const hero = appendBlock(
  root,
  "hero",
  "display:block;height:32px;background-color:#3366ff;color:#f9fafb;font-size:20px;font-family:Arial,sans-serif",
);
appendBlock(hero, "hero-child", "display:block;height:10px;background-color:#60a5fa");

const group = appendBlock(root, "group", "display:block;background-color:transparent");
appendBlock(group, "first", "display:block;height:20px;background-color:#e11d48");
appendBlock(
  group,
  "second",
  "display:block;width:120px;height:18px;background-color:#f97316",
);

const hidden = appendBlock(
  root,
  "hidden",
  "display:none;height:40px;background-color:#ff0000",
);
appendBlock(hidden, "hidden-child", "display:block;height:14px;background-color:#00ff00");
appendBlock(root, "last", "display:block;height:16px;background-color:#22c55e");
