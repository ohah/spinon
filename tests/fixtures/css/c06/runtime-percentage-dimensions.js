const root = document.createElement("div");
root.setAttribute(
  "style",
  "display:flex;box-sizing:border-box;width:100%;height:100%;flex-direction:row;background-color:#123456"
);
document.appendChild(root);

const left = document.createElement("div");
left.setAttribute(
  "style",
  "display:block;box-sizing:border-box;width:50%;height:50%;flex-basis:50%;flex-shrink:0;background-color:#3366ff"
);
root.appendChild(left);

const right = document.createElement("div");
right.setAttribute(
  "style",
  "display:block;box-sizing:border-box;width:50%;height:50%;flex-basis:50%;flex-shrink:0;background-color:#22aa66"
);
root.appendChild(right);
