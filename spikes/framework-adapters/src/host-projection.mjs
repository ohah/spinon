import { HostProjectionError, assertOwnerId, assertNodeName } from "./errors.mjs";
import { HostComment, HostDocumentFragment, HostElement, HostEvent, HostNode, HostText } from "./host-nodes.mjs";

export class HostProjectionRecorder {
  constructor() {
    this.sequence = 0;
    this.nextNodeId = 1;
    this.dispatchCount = 0;
    this.operations = [];
    this.boundOwnerId = null;
    this.document = null;
    this.root = new HostElement(this, "#root", {
      root: true,
      ownerId: null,
    });
  }

  bindOwner(ownerId) {
    assertOwnerId(ownerId);
    this.boundOwnerId = ownerId;
    this.root.listenerOwnerId = ownerId;
  }

  createElement(ownerId, name, options = {}) {
    assertOwnerId(ownerId);
    assertNodeName(name);
    const node = new HostElement(this, name, { ownerId, ...options });
    if (!node.templateOnly && !node.internal) {
      this.record(ownerId, "CreateElement", { node: node.id, name });
    }
    return node;
  }

  createText(ownerId, data = "", options = {}) {
    assertOwnerId(ownerId);
    const node = new HostText(this, data, { ownerId, ...options });
    if (!node.templateOnly && !node.internal) {
      this.record(ownerId, "CreateText", { node: node.id, data: String(data) });
    }
    return node;
  }

  createComment(ownerId, data = "", options = {}) {
    assertOwnerId(ownerId);
    return new HostComment(this, data, { ownerId, internal: true, ...options });
  }

  insert(ownerId, child, parent, anchor = null) {
    assertOwnerId(ownerId);
    this.assertOwnedNode(ownerId, child);
    if (!(parent instanceof HostNode) || (!parent.root && parent.ownerId !== ownerId)) {
      throw new HostProjectionError("OwnerConflict", "호스트 부모의 OwnerId가 다릅니다");
    }
    if (anchor !== null && anchor.parentNode !== parent) {
      throw new HostProjectionError("NotFoundError", "삽입 기준 노드가 부모의 자식이 아닙니다");
    }
    if (child === parent || isAncestor(child, parent)) {
      throw new HostProjectionError("HierarchyRequestError", "호스트 노드를 자기 자손 아래에 넣을 수 없습니다");
    }

    if (child instanceof HostDocumentFragment) {
      for (const fragmentChild of [...child.childNodes]) {
        this.insert(ownerId, fragmentChild, parent, anchor);
      }
      return child;
    }

    if (child.templateOnly) {
      throw new HostProjectionError("UnmaterializedNode", "정적 템플릿 노드는 복제한 뒤 삽입해야 합니다");
    }
    if (!child.internal && child.ownerId !== ownerId) {
      throw new HostProjectionError("OwnerConflict", "호스트 노드의 OwnerId가 다릅니다");
    }

    if (child.parentNode === parent) {
      const oldIndex = parent.childNodes.indexOf(child);
      const anchorIndex = anchor === null ? parent.childNodes.length : parent.childNodes.indexOf(anchor);
      if (oldIndex >= 0 && (oldIndex === anchorIndex || oldIndex + 1 === anchorIndex)) {
        return child;
      }
    }
    if (child.parentNode !== null) {
      const oldParent = child.parentNode;
      oldParent.childNodes.splice(oldParent.childNodes.indexOf(child), 1);
      child.parentNode = null;
    }

    const index = anchor === null ? parent.childNodes.length : parent.childNodes.indexOf(anchor);
    parent.childNodes.splice(index, 0, child);
    child.parentNode = parent;
    if (!child.internal && !parent.internal) {
      this.record(ownerId, "InsertBefore", {
        parent: parent.root ? "root" : parent.id,
        node: child.id,
        before: anchor?.internal ? null : (anchor?.id ?? null),
      });
    }
    return child;
  }

  remove(ownerId, child, parent) {
    assertOwnerId(ownerId);
    this.assertOwnedNode(ownerId, child);
    if (child.parentNode !== parent) {
      throw new HostProjectionError("NotFoundError", "제거할 노드가 부모의 자식이 아닙니다");
    }
    const index = parent.childNodes.indexOf(child);
    parent.childNodes.splice(index, 1);
    child.parentNode = null;
    if (!child.internal && !parent.internal) {
      this.record(ownerId, "RemoveChild", {
        parent: parent.root ? "root" : parent.id,
        node: child.id,
      });
    }
    return child;
  }

  setText(ownerId, node, data) {
    assertOwnerId(ownerId);
    this.assertOwnedNode(ownerId, node);
    if (!(node instanceof HostText)) {
      throw new HostProjectionError("TypeError", "SetTextData 대상은 Text여야 합니다");
    }
    const next = String(data);
    if (node.data === next) return;
    const previous = node.data;
    node._data = next;
    if (!node.internal) {
      this.record(ownerId, "SetTextData", { node: node.id, previous, data: next });
    }
  }

  setAttribute(ownerId, node, name, value) {
    assertOwnerId(ownerId);
    this.assertOwnedNode(ownerId, node);
    if (!(node instanceof HostElement) || node.root) {
      throw new HostProjectionError("TypeError", "SetAttribute 대상은 Element여야 합니다");
    }
    if (name !== "aria-label" || typeof value !== "string") {
      throw new HostProjectionError("UnsupportedAttribute", `지원하지 않는 속성: ${name}`);
    }
    const previous = node.attributeMap.get(name) ?? null;
    if (previous === value) return;
    node.attributeMap.set(name, value);
    if (!node.templateOnly && !node.internal) {
      this.record(ownerId, "SetAttribute", { node: node.id, name, previous, value });
    }
  }

  removeAttribute(ownerId, node, name) {
    assertOwnerId(ownerId);
    this.assertOwnedNode(ownerId, node);
    if (!(node instanceof HostElement) || node.root) {
      throw new HostProjectionError("TypeError", "RemoveAttribute 대상은 Element여야 합니다");
    }
    if (name !== "aria-label") {
      throw new HostProjectionError("UnsupportedAttribute", `지원하지 않는 속성: ${name}`);
    }
    const previous = node.attributeMap.get(name);
    if (previous === undefined) return;
    node.attributeMap.delete(name);
    if (!node.templateOnly && !node.internal) {
      this.record(ownerId, "RemoveAttribute", { node: node.id, name, previous });
    }
  }

  setElementText(ownerId, element, text) {
    assertOwnerId(ownerId);
    this.assertOwnedNode(ownerId, element);
    if (!(element instanceof HostElement) || element.root) {
      throw new HostProjectionError("TypeError", "요소 텍스트 대상은 Element여야 합니다");
    }
    const next = String(text);
    if (next === "") {
      for (const child of [...element.childNodes]) {
        this.remove(ownerId, child, element);
      }
      return;
    }
    if (element.childNodes.length === 1 && element.childNodes[0] instanceof HostText) {
      this.setText(ownerId, element.childNodes[0], next);
      return;
    }
    for (const child of [...element.childNodes]) {
      this.remove(ownerId, child, element);
    }
    const child = this.createText(ownerId, next);
    this.insert(ownerId, child, element, null);
  }

  addListener(ownerId, target, type, listener) {
    assertOwnerId(ownerId);
    if (target !== this.document && !(target instanceof HostNode)) {
      throw new HostProjectionError("TypeError", "이벤트 대상은 호스트 노드 또는 문서여야 합니다");
    }
    if (target instanceof HostNode && !target.root && target.ownerId !== ownerId) {
      throw new HostProjectionError("OwnerConflict", "이벤트 대상의 OwnerId가 다릅니다");
    }
    target._addListener(type, listener);
    this.record(ownerId, "AdapterListenerAdd", {
      target: target instanceof HostNode ? (target.root ? "root" : target.id) : "document",
      type,
    });
  }

  removeListener(ownerId, target, type, listener) {
    assertOwnerId(ownerId);
    target._removeListener(type, listener);
    this.record(ownerId, "AdapterListenerRemove", {
      target: target instanceof HostNode ? (target.root ? "root" : target.id) : "document",
      type,
    });
  }

  dispatchClick(target) {
    if (!(target instanceof HostElement) || target.root || !this.isConnected(target)) {
      throw new HostProjectionError("DetachedTarget", "분리된 노드에는 click을 전달할 수 없습니다");
    }
    this.dispatchCount += 1;
    const path = [];
    for (let node = target; node !== null; node = node.parentNode) path.push(node);
    if (this.document !== null) path.push(this.document);
    const event = new HostEvent("click", target, path);
    for (const node of path) {
      node?._dispatchAtTarget?.(event);
      if (event.cancelBubble) break;
    }
    this.record(target.ownerId, "HarnessClickDispatch", { node: target.id, type: "click" });
  }

  isConnected(node) {
    for (let current = node; current !== null; current = current.parentNode) {
      if (current === this.root) return true;
    }
    return false;
  }

  find(name, parent = this.root) {
    for (const child of parent.childNodes) {
      if (child instanceof HostElement && child.name === name) return child;
      if (child instanceof HostElement) {
        const found = this.find(name, child);
        if (found) return found;
      }
    }
    return null;
  }

  snapshot() {
    const project = (node) => {
      if (node.internal || node.templateOnly || node instanceof HostComment) return null;
      if (node instanceof HostText) return { type: "text", value: node.data };
      if (node instanceof HostElement) {
        return {
          type: "element",
          name: node.name,
          attributes: Object.fromEntries([...node.attributeMap.entries()].sort(([a], [b]) => a.localeCompare(b))),
          children: node.childNodes.map(project).filter(Boolean),
        };
      }
      return null;
    };
    return {
      type: "root",
      children: this.root.childNodes.map(project).filter(Boolean),
    };
  }

  listenerCount(target, type = null) {
    if (type !== null) return target._listeners.get(type)?.size ?? 0;
    let count = 0;
    for (const [eventType, listeners] of target._listeners) {
      if (eventType) count += listeners.size;
    }
    return count;
  }

  assertOwnedNode(ownerId, node) {
    if (!(node instanceof HostNode) || node.root || node.templateOnly) {
      throw new HostProjectionError("InvalidNode", "작업 대상이 materialized HostNode가 아닙니다");
    }
    if (!node.internal && node.ownerId !== ownerId) {
      throw new HostProjectionError("OwnerConflict", "호스트 노드의 OwnerId가 다릅니다");
    }
  }

  record(ownerId, operation, fields) {
    assertOwnerId(ownerId);
    this.operations.push({ sequence: ++this.sequence, ownerId, operation, ...fields });
  }
}

function isAncestor(node, maybeDescendant) {
  for (let current = maybeDescendant; current !== null; current = current.parentNode) {
    if (current === node) return true;
  }
  return false;
}
