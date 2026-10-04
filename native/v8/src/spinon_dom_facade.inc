R"SPINONJS(
(() => {
  const query = {
    nodeInfo: 1,
    parent: 2,
    childCount: 3,
    childAt: 4,
    nextSibling: 5,
    textContent: 6,
    attribute: 7,
    nextId: 8,
  };
  const htmlNamespace = "http://www.w3.org/1999/xhtml";
  const wrappers = new Map();
  const nodeIds = new WeakMap();
  const exceptionValues = new WeakMap();
  const constructionKey = {};

  class DOMException extends Error {
    constructor(message = "", name = "Error") {
      const exceptionMessage = toDOMString(message);
      super(exceptionMessage);
      delete this.message;
      exceptionValues.set(this, {
        name: toDOMString(name),
        message: exceptionMessage,
      });
    }
  }

  Object.defineProperties(DOMException.prototype, {
    name: {
      get() {
        const values = exceptionValues.get(this);
        if (values === undefined) throw new TypeError("DOMException receiver가 아닙니다");
        return values.name;
      },
      enumerable: true,
      configurable: true,
    },
    message: {
      get() {
        const values = exceptionValues.get(this);
        if (values === undefined) throw new TypeError("DOMException receiver가 아닙니다");
        return values.message;
      },
      enumerable: true,
      configurable: true,
    },
  });
  Object.defineProperty(DOMException.prototype, Symbol.toStringTag, {
    value: "DOMException",
    configurable: true,
  });

  function fail(name, message) {
    throw new DOMException(message, name);
  }

  function toDOMString(value) {
    if (typeof value === "symbol") throw new TypeError("Symbol은 DOMString으로 변환할 수 없습니다");
    return `${value}`;
  }

  function requireArguments(actual, required, method) {
    if (actual < required) {
      throw new TypeError(`${method} 호출에 인수가 부족합니다`);
    }
  }

  function read(kind, nodeId, index = 0, name = "") {
    return spinon.__internal.readDocument(kind, nodeId, index, name);
  }

  function nodeId(node) {
    const id = nodeIds.get(node);
    if (id === undefined) throw new TypeError("Node 객체가 필요합니다");
    return id;
  }

  function nodeType(node) {
    const id = nodeId(node);
    return id === 0 ? 9 : read(query.nodeInfo, id).value;
  }

  function readParent(node) {
    const id = nodeId(node);
    if (id === 0) return null;
    const parent = read(query.parent, id);
    return parent.exists ? wrap(parent.value) : null;
  }

  function requireNodeType(node, expected, label) {
    if (nodeType(node) !== expected) {
      throw new TypeError(`${label} 객체가 필요합니다`);
    }
  }

  function getAttributeValue(node, name) {
    requireNodeType(node, 1, "Element");
    const result = read(query.attribute, nodeId(node), 0, attributeName(name));
    return result.exists ? result.text : null;
  }

  function setAttributeValue(node, name, value) {
    requireNodeType(node, 1, "Element");
    commit([{
      type: "setAttribute",
      node: nodeId(node),
      name: attributeName(name),
      value: toDOMString(value),
    }], "InvalidCharacterError");
  }

  function removeAttributeValue(node, name) {
    requireNodeType(node, 1, "Element");
    commit([{
      type: "removeAttribute",
      node: nodeId(node),
      name: attributeName(name),
    }], "InvalidCharacterError");
  }

  function setTextValue(node, value) {
    requireNodeType(node, 3, "Text");
    commit([{
      type: "setText",
      node: nodeId(node),
      data: toDOMString(value),
    }]);
  }

  function commit(operations, failureName = "InvalidStateError") {
    try {
      spinon.__internal.commitDocumentBatch(operations);
    } catch (error) {
      if (error instanceof DOMException) throw error;
      const message = error && error.message ? error.message : String(error);
      if (error && error.name === "QuotaExceededError") {
        fail("QuotaExceededError", message);
      }
      fail(failureName, message);
    }
  }

  function asciiLower(value) {
    return value.replace(/[A-Z]/g, (letter) => letter.toLowerCase());
  }

  function attributeName(value) {
    const name = toDOMString(value);
    if (!/^[A-Za-z_:][A-Za-z0-9_.:-]*$/.test(name)) {
      fail("InvalidCharacterError", "속성 이름이 지원 범위에 맞지 않습니다");
    }
    return asciiLower(name);
  }

  class Node {
    constructor(key, id) {
      if (key !== constructionKey) throw new TypeError("Node를 직접 만들 수 없습니다");
      nodeIds.set(this, id);
    }

    get nodeType() {
      return nodeType(this);
    }

    get nodeName() {
      const id = nodeId(this);
      return id === 0 ? "#document" : read(query.nodeInfo, id).text;
    }

    get parentNode() {
      return readParent(this);
    }

    get parentElement() {
      const parent = readParent(this);
      return parent !== null && nodeType(parent) === 1 ? parent : null;
    }

    get firstChild() {
      const child = read(query.childAt, nodeId(this), 0);
      return child.exists ? wrap(child.value) : null;
    }

    get nextSibling() {
      const id = nodeId(this);
      if (id === 0) return null;
      const next = read(query.nextSibling, id);
      return next.exists ? wrap(next.value) : null;
    }

    get textContent() {
      const id = nodeId(this);
      return id === 0 ? null : read(query.textContent, id).text;
    }

    get nodeValue() {
      return nodeType(this) === 3 ? read(query.textContent, nodeId(this)).text : null;
    }

    set nodeValue(value) {
      if (nodeType(this) === 3) setTextValue(this, value);
    }

    get ownerDocument() {
      return nodeId(this) === 0 ? null : document;
    }

    hasChildNodes() {
      return read(query.childCount, nodeId(this)).value !== 0;
    }

    isSameNode(other) {
      nodeId(this);
      requireArguments(arguments.length, 1, "isSameNode");
      if (other === undefined) other = null;
      if (other !== null) nodeId(other);
      return other === this;
    }

    contains(other) {
      nodeId(this);
      requireArguments(arguments.length, 1, "contains");
      if (other === undefined) other = null;
      if (other === null) return false;
      nodeId(other);
      for (let current = other; current !== null; current = readParent(current)) {
        if (current === this) return true;
      }
      return false;
    }

    appendChild(child) {
      requireArguments(arguments.length, 1, "appendChild");
      return this.insertBefore(child, null);
    }

    insertBefore(child, before) {
      requireArguments(arguments.length, 2, "insertBefore");
      if (before === undefined) before = null;
      const parentId = nodeId(this);
      const childId = nodeId(child);
      const parentType = nodeType(this);
      const childType = nodeType(child);
      if (childId === 0 || childType === 9) {
        fail("HierarchyRequestError", "문서 노드는 자식으로 삽입할 수 없습니다");
      }
      if (before !== null) {
        nodeId(before);
        if (readParent(before) !== this) {
          fail("NotFoundError", "기준 노드가 이 부모의 자식이 아닙니다");
        }
        if (before === child) return child;
      }
      if (parentType !== 9 && parentType !== 1) {
        fail("HierarchyRequestError", "이 노드 종류에는 자식을 넣을 수 없습니다");
      }
      if (parentType === 9 && childType !== 1) {
        fail("HierarchyRequestError", "앱 문서 루트에는 Element만 넣을 수 있습니다");
      }
      for (let ancestor = this; ancestor !== null; ancestor = readParent(ancestor)) {
        if (ancestor === child) {
          fail("HierarchyRequestError", "노드를 자기 자손 아래로 옮길 수 없습니다");
        }
      }
      const operation = before === null
        ? { type: "append", parent: parentId, node: childId }
        : {
            type: "insertBefore",
            parent: parentId,
            node: childId,
            before: nodeId(before),
          };
      commit([operation], "HierarchyRequestError");
      return child;
    }

    removeChild(child) {
      requireArguments(arguments.length, 1, "removeChild");
      const parentId = nodeId(this);
      const childId = nodeId(child);
      if (readParent(child) !== this) {
        fail("NotFoundError", "제거할 노드가 이 부모의 자식이 아닙니다");
      }
      commit([{ type: "remove", parent: parentId, node: childId }], "NotFoundError");
      return child;
    }
  }

  class Element extends Node {
    get localName() {
      requireNodeType(this, 1, "Element");
      return read(query.nodeInfo, nodeId(this)).text.toLowerCase();
    }

    get tagName() {
      requireNodeType(this, 1, "Element");
      return read(query.nodeInfo, nodeId(this)).text;
    }

    get id() {
      return getAttributeValue(this, "id") || "";
    }

    set id(value) {
      setAttributeValue(this, "id", value);
    }

    get className() {
      return getAttributeValue(this, "class") || "";
    }

    set className(value) {
      setAttributeValue(this, "class", value);
    }

    getAttribute(name) {
      requireArguments(arguments.length, 1, "getAttribute");
      return getAttributeValue(this, name);
    }

    hasAttribute(name) {
      requireArguments(arguments.length, 1, "hasAttribute");
      return getAttributeValue(this, name) !== null;
    }

    setAttribute(name, value) {
      requireArguments(arguments.length, 2, "setAttribute");
      setAttributeValue(this, name, value);
    }

    removeAttribute(name) {
      requireArguments(arguments.length, 1, "removeAttribute");
      removeAttributeValue(this, name);
    }
  }

  class Text extends Node {
    get data() {
      requireNodeType(this, 3, "Text");
      return read(query.textContent, nodeId(this)).text;
    }

    set data(value) {
      setTextValue(this, value);
    }

  }

  function wrap(id) {
    if (id === 0) return document;
    const cached = wrappers.get(id);
    if (cached !== undefined) return cached;
    const info = read(query.nodeInfo, id);
    const node = info.value === 1
      ? new Element(constructionKey, id)
      : new Text(constructionKey, id);
    wrappers.set(id, node);
    return node;
  }

  class Document extends Node {
    constructor(key) {
      super(key, 0);
    }

    createElement(name) {
      if (this !== document) throw new TypeError("document 객체가 필요합니다");
      requireArguments(arguments.length, 1, "createElement");
      const rawName = toDOMString(name);
      if (!/^[A-Za-z][A-Za-z0-9._:-]*$/.test(rawName)) {
        fail("InvalidCharacterError", "요소 이름이 지원 범위에 맞지 않습니다");
      }
      const id = read(query.nextId, 0).value;
      commit([{
        type: "createElement",
        id,
        namespace: htmlNamespace,
        name: asciiLower(rawName),
      }], "InvalidCharacterError");
      return wrap(id);
    }

    createTextNode(data) {
      if (this !== document) throw new TypeError("document 객체가 필요합니다");
      requireArguments(arguments.length, 1, "createTextNode");
      const id = read(query.nextId, 0).value;
      commit([{ type: "createText", id, data: toDOMString(data) }]);
      return wrap(id);
    }
  }

  const document = new Document(constructionKey);
  Object.defineProperty(globalThis, "DOMException", {
    configurable: true,
    value: DOMException,
    writable: true,
  });
  Object.defineProperty(globalThis, "document", {
    configurable: false,
    enumerable: true,
    value: document,
    writable: false,
  });
  Object.defineProperty(globalThis, "Node", {
    configurable: true,
    value: Node,
    writable: true,
  });
  Object.defineProperty(globalThis, "Element", {
    configurable: true,
    value: Element,
    writable: true,
  });
  Object.defineProperty(globalThis, "Text", {
    configurable: true,
    value: Text,
    writable: true,
  });
  wrappers.set(0, document);
  nodeIds.set(document, 0);
})();
)SPINONJS"
