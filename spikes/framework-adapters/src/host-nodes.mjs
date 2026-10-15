import { HostProjectionError } from "./errors.mjs";

export class HostNode {
  constructor(recorder, { nodeType, name, data = "", ownerId, root = false, internal = false, templateOnly = false }) {
    this.recorder = recorder;
    this.nodeType = nodeType;
    this.name = name;
    this.ownerId = ownerId ?? null;
    this.root = root;
    this.internal = internal;
    this.templateOnly = templateOnly;
    this.id = root || templateOnly || internal ? null : `n${recorder.nextNodeId++}`;
    this.ownerDocument = recorder.document;
    this.parentNode = null;
    this.childNodes = [];
    this._listeners = new Map();
    this.listenerOwnerId = null;
    this._data = String(data);
  }

  get firstChild() {
    return this.childNodes[0] ?? null;
  }

  get lastChild() {
    return this.childNodes.at(-1) ?? null;
  }

  get nextSibling() {
    if (this.parentNode === null) return null;
    const index = this.parentNode.childNodes.indexOf(this);
    return this.parentNode.childNodes[index + 1] ?? null;
  }

  get previousSibling() {
    if (this.parentNode === null) return null;
    const index = this.parentNode.childNodes.indexOf(this);
    return this.parentNode.childNodes[index - 1] ?? null;
  }

  get parentElement() {
    return this.parentNode instanceof HostElement && !this.parentNode.root ? this.parentNode : null;
  }

  get nodeName() {
    if (this.nodeType === 3) return "#text";
    if (this.nodeType === 8) return "#comment";
    return this.name === "#root" ? "#root" : this.name.toUpperCase();
  }

  get nodeValue() {
    return this.nodeType === 3 || this.nodeType === 8 ? this._data : null;
  }

  set nodeValue(value) {
    if (this.nodeType === 3 || this.nodeType === 8) this.data = value ?? "";
  }

  get textContent() {
    if (this.nodeType === 3 || this.nodeType === 8) return this._data;
    return this.childNodes.map((child) => child.textContent).join("");
  }

  set textContent(value) {
    const text = String(value ?? "");
    if (this instanceof HostText || this instanceof HostComment) {
      this.data = text;
      return;
    }
    if (this.templateOnly) {
      this.childNodes = [];
      if (text !== "") this._appendTemplateChild(new HostText(this.recorder, text, { templateOnly: true }));
      return;
    }
    if (this.root) throw new HostProjectionError("UnsupportedOperation", "HostRoot textContent 수정은 지원하지 않습니다");
    this.recorder.setElementText(this.ownerId, this, text);
  }

  get data() {
    return this._data;
  }

  set data(value) {
    if (this.nodeType === 3 && this.ownerId !== null && !this.internal && !this.templateOnly) {
      this.recorder.setText(this.ownerId, this, value);
    } else {
      this._data = String(value ?? "");
    }
  }

  get isConnected() {
    return this.recorder.isConnected(this);
  }

  appendChild(child) {
    return this.insertBefore(child, null);
  }

  insertBefore(child, anchor) {
    const ownerId = this.root ? this.listenerOwnerId ?? this.recorder.boundOwnerId : this.ownerId;
    if (this.templateOnly && child.templateOnly) {
      if (child.parentNode !== null) {
        child.parentNode.childNodes.splice(child.parentNode.childNodes.indexOf(child), 1);
      }
      const index = anchor === null ? this.childNodes.length : this.childNodes.indexOf(anchor);
      if (index < 0) throw new HostProjectionError("NotFoundError", "템플릿 기준 노드를 찾지 못했습니다");
      this.childNodes.splice(index, 0, child);
      child.parentNode = this;
      return child;
    }
    return this.recorder.insert(ownerId, child, this, anchor);
  }

  removeChild(child) {
    if (this.templateOnly && child.templateOnly) {
      const index = this.childNodes.indexOf(child);
      if (index < 0) throw new HostProjectionError("NotFoundError", "템플릿 노드를 찾지 못했습니다");
      this.childNodes.splice(index, 1);
      child.parentNode = null;
      return child;
    }
    const ownerId = this.root ? child.ownerId ?? this.listenerOwnerId ?? this.recorder.boundOwnerId : this.ownerId;
    return this.recorder.remove(ownerId, child, this);
  }

  before(node) {
    if (this.parentNode === null) return;
    this.parentNode.insertBefore(node, this);
  }

  after(node) {
    if (this.parentNode === null) return;
    this.parentNode.insertBefore(node, this.nextSibling);
  }

  remove() {
    this.parentNode?.removeChild(this);
  }

  addEventListener(type, listener) {
    const ownerId = this.root ? this.listenerOwnerId ?? this.recorder.boundOwnerId : this.ownerId;
    this.recorder.addListener(ownerId, this, type, listener);
  }

  removeEventListener(type, listener) {
    const ownerId = this.root ? this.listenerOwnerId ?? this.recorder.boundOwnerId : this.ownerId;
    this.recorder.removeListener(ownerId, this, type, listener);
  }

  _addListener(type, listener) {
    if (typeof listener !== "function") return;
    let listeners = this._listeners.get(type);
    if (listeners === undefined) {
      listeners = new Set();
      this._listeners.set(type, listeners);
    }
    listeners.add(listener);
  }

  _removeListener(type, listener) {
    const listeners = this._listeners.get(type);
    listeners?.delete(listener);
    if (listeners?.size === 0) this._listeners.delete(type);
  }

  _dispatchAtTarget(event) {
    for (const listener of [...(this._listeners.get(event.type) ?? [])]) listener.call(this, event);
  }

  _appendTemplateChild(child) {
    child.parentNode = this;
    this.childNodes.push(child);
  }

  cloneNode(deep = false) {
    const ownerId = this.ownerId ?? this.ownerDocument?.ownerId;
    const cloneOptions = this.templateOnly ? { ownerId } : { ownerId: this.ownerId, internal: this.internal };
    let clone;
    if (this instanceof HostElement) {
      clone = this.recorder.createElement(ownerId, this.name, cloneOptions);
      for (const [name, value] of this.attributeMap) clone.setAttribute(name, value);
    } else if (this instanceof HostText) {
      clone = this.recorder.createText(ownerId, this.data, cloneOptions);
    } else if (this instanceof HostComment) {
      clone = this.recorder.createComment(ownerId, this.data, cloneOptions);
    } else if (this instanceof HostDocumentFragment) {
      clone = new HostDocumentFragment(this.recorder, { ownerId, templateOnly: this.templateOnly });
    } else {
      throw new HostProjectionError("UnsupportedOperation", "이 노드 종류는 복제할 수 없습니다");
    }
    if (deep) {
      for (const child of this.childNodes) {
        const childClone = child.cloneNode(true);
        if (clone.templateOnly && childClone.templateOnly) clone._appendTemplateChild(childClone);
        else clone.appendChild(childClone);
      }
    }
    return clone;
  }
}

export class HostElement extends HostNode {
  constructor(recorder, name, options = {}) {
    super(recorder, { ...options, nodeType: 1, name });
    this.localName = name === "#root" ? "#root" : name.toLowerCase();
    this.tagName = name === "#root" ? "#root" : name.toUpperCase();
    this.namespaceURI = "http://www.w3.org/1999/xhtml";
    this.attributeMap = new Map();
  }

  get attributes() {
    return [...this.attributeMap.entries()].map(([name, value]) => ({ name, value }));
  }

  getAttribute(name) {
    return this.attributeMap.get(String(name)) ?? null;
  }

  hasAttribute(name) {
    return this.attributeMap.has(String(name));
  }

  setAttribute(name, value) {
    if (this.templateOnly) {
      this.attributeMap.set(String(name), String(value));
      return;
    }
    const ownerId = this.ownerId;
    this.recorder.setAttribute(ownerId, this, String(name), String(value));
  }

  removeAttribute(name) {
    if (this.templateOnly) {
      this.attributeMap.delete(String(name));
      return;
    }
    this.recorder.removeAttribute(this.ownerId, this, String(name));
  }
}

export class HostText extends HostNode {
  constructor(recorder, data = "", options = {}) {
    super(recorder, { ...options, nodeType: 3, name: "#text", data });
  }
}

export class HostComment extends HostNode {
  constructor(recorder, data = "", options = {}) {
    super(recorder, { ...options, nodeType: 8, name: "#comment", data });
  }
}

export class HostDocumentFragment extends HostNode {
  constructor(recorder, options = {}) {
    super(recorder, { ...options, nodeType: 11, name: "#document-fragment", internal: options.internal ?? true });
  }
}

export class HostEvent {
  constructor(type, target, path) {
    this.type = type;
    this.target = target;
    this.currentTarget = null;
    this.bubbles = true;
    this.cancelable = true;
    this.composed = true;
    this.cancelBubble = false;
    this.defaultPrevented = false;
    this._path = path;
  }

  composedPath() {
    return [...this._path];
  }

  preventDefault() {
    this.defaultPrevented = true;
  }

  stopPropagation() {
    this.cancelBubble = true;
  }
}
