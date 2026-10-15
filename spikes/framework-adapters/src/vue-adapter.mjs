import { createRenderer, nextTick } from "@vue/runtime-core";
import { createVueCounter } from "../fixtures/vue-counter.mjs";

export async function mountVueCounter(recorder, ownerId = "vue-root") {
  recorder.bindOwner(ownerId);
  const renderer = createRenderer({
    createElement: (name) => recorder.createElement(ownerId, name),
    createText: (data) => recorder.createText(ownerId, data),
    createComment: (data) => recorder.createComment(ownerId, data),
    setText: (node, data) => recorder.setText(ownerId, node, data),
    setElementText: (node, text) => recorder.setElementText(ownerId, node, text),
    insert: (node, parent, anchor = null) => recorder.insert(ownerId, node, parent, anchor),
    remove: (node) => {
      if (node.parentNode !== null) recorder.remove(ownerId, node, node.parentNode);
    },
    parentNode: (node) => node.parentNode,
    nextSibling: (node) => node.nextSibling,
    patchProp: (element, key, previous, next) => {
      if (key === "onClick") {
        if (previous != null) element.removeEventListener("click", previous);
        if (next == null) return;
        if (typeof next === "function") element.addEventListener("click", next);
        else throw new TypeError("R07 Vue fixture는 단일 click callback만 허용합니다");
        return;
      }
      if (key === "aria-label") {
        if (next == null) recorder.removeAttribute(ownerId, element, key);
        else if (typeof next === "string") recorder.setAttribute(ownerId, element, key, next);
        else throw new TypeError("R07 fixture 속성은 문자열이어야 합니다");
        return;
      }
      throw new TypeError(`R07 Vue adapter가 지원하지 않는 prop: ${key}`);
    },
  });

  const app = renderer.createApp(createVueCounter());
  app.mount(recorder.root);
  await nextTick();
  return {
    async settle() {
      await nextTick();
    },
    unmount() {
      app.unmount();
    },
  };
}
