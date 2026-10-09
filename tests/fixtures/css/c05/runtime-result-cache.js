{
const state = globalThis.__spinonC053RuntimeResultCache;
const fixture = globalThis.__spinonC052FixtureState;
if (!fixture) throw new Error("C05.3은 C05.2 HostDocument fixture를 먼저 필요로 합니다");

if (!state) {
  const node = document.createElement("div");
  node.setAttribute("id", "c053-detached-probe");
  node.setAttribute(
    "style",
    "display:block;width:27px;height:14px;background-color:#22aa66"
  );
  globalThis.__spinonC053RuntimeResultCache = { node, step: "detached" };
} else if (state.step === "detached") {
  fixture.app.appendChild(state.node);
  state.step = "attached";
} else if (state.step === "attached") {
  fixture.app.removeChild(state.node);
  state.step = "removed";
} else if (state.step === "removed") {
  state.node.setAttribute(
    "style",
    "display:block;width:39px;height:18px;background-color:#cc4466"
  );
  state.step = "changed-while-detached";
} else {
  fixture.app.appendChild(state.node);
  state.step = "attached";
}
}
