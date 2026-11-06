// examples/bootstrap/root-app.js
var firstElementNamed = (parent, name) => {
  if (parent === null)
    return null;
  for (let child = parent.firstChild;child !== null; child = child.nextSibling) {
    if (child.nodeType === 1 && child.localName === name)
      return child;
  }
  return null;
};
var firstMeta = (parent, value) => {
  if (parent === null)
    return null;
  for (let child = parent.firstChild;child !== null; child = child.nextSibling) {
    if (child.nodeType === 1 && child.localName === "meta" && child.getAttribute("name") === value)
      return child;
  }
  return null;
};
var root = document.documentElement;
var head = document.head;
var body = document.body;
var viewport = firstMeta(head, "viewport");
var colorScheme = firstMeta(head, "color-scheme");
var style = firstElementNamed(head, "style");
var app = firstElementNamed(body, "main");
if (!root || root.localName !== "html" || root.getAttribute("lang") !== "ko" || !head || head.parentNode !== root || !body || body.parentNode !== root || !viewport || viewport.getAttribute("content") !== "width=device-width, initial-scale=1" || !colorScheme || colorScheme.getAttribute("content") !== "light dark" || !style || style.textContent !== ":root { color-scheme: light dark; }" || !app || app.id !== "app" || app.textContent !== "Spinon 루트 HTML") {
  throw new Error("기본 index.html 문서가 예상한 V8 DOM으로 초기화되지 않았습니다");
}
app.setAttribute("data-runtime", "ready");
if (app.getAttribute("data-runtime") !== "ready") {
  throw new Error("루트 HTML 문서의 속성 변경이 같은 V8 세션에 반영되지 않았습니다");
}
