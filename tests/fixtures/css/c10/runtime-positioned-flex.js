const fixtureInventory = globalThis.__spinonC1035FixtureInventory;
const runtimeInventory = globalThis.__spinonC1035RuntimeInventory;
if (!fixtureInventory || !runtimeInventory) {
  throw new Error("C10.3.5 runtime inventory가 없습니다");
}

const root = document.createElement("div");
root.setAttribute("id", "c1035-runtime-root");
root.setAttribute("style", runtimeInventory.rootStyle);
document.appendChild(root);

function runtimeStyle(sourceStyle) {
  return sourceStyle.replace(/(^|;)background:/g, "$1background-color:");
}

const runtimeNodeIds = ["c1035-runtime-root"];
for (const runtimeCase of runtimeInventory.cases) {
  const fixtureCase = fixtureInventory.cases.find(({ id }) => id === runtimeCase.id);
  if (!fixtureCase) throw new Error(`C10.3.5 case가 없습니다: ${runtimeCase.id}`);

  const parents = new Map();
  for (const node of fixtureCase.nodes) {
    for (const childId of node.children) parents.set(childId, node.id);
  }

  const elements = new Map();
  for (const node of fixtureCase.nodes) {
    const element = document.createElement("div");
    element.setAttribute("id", node.id);
    let style = runtimeStyle(node.style);
    if (node === fixtureCase.nodes[0] && runtimeCase.rootStyleAppend) {
      style += `;${runtimeCase.rootStyleAppend}`;
    }
    element.setAttribute("style", style);
    elements.set(node.id, element);
    runtimeNodeIds.push(node.id);
  }

  for (const node of fixtureCase.nodes) {
    const parentId = parents.get(node.id);
    const parent = parentId ? elements.get(parentId) : root;
    if (!parent) throw new Error(`C10.3.5 부모 node가 없습니다: ${runtimeCase.id}/${node.id}`);
    parent.appendChild(elements.get(node.id));
  }
}

globalThis.__spinonC1035RuntimeNodeIds = runtimeNodeIds;
