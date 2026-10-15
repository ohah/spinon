import { parseFragment } from "parse5";
import {
  HostComment,
  HostDocumentFragment,
  HostElement,
  HostNode,
  HostText,
} from "./host-nodes.mjs";
import { HostProjectionError } from "./errors.mjs";

export function installSvelteDomFacade(recorder, ownerId) {
  recorder.bindOwner(ownerId);
  const document = new HostDocumentFacade(recorder, ownerId);
  recorder.document = document;
  recorder.root.ownerDocument = document;

  const keys = ["window", "document", "navigator", "Node", "Element", "Text", "Comment", "Document", "DocumentFragment"];
  const previous = new Map(keys.map((key) => [key, Object.getOwnPropertyDescriptor(globalThis, key)]));
  const values = {
    window: { document, trustedTypes: null },
    document,
    navigator: { userAgent: "Spinon R07 host facade" },
    Node: HostNode,
    Element: HostElement,
    Text: HostText,
    Comment: HostComment,
    Document: HostDocumentFacade,
    DocumentFragment: HostDocumentFragment,
  };
  for (const [key, value] of Object.entries(values)) {
    Object.defineProperty(globalThis, key, {
      configurable: true,
      enumerable: true,
      writable: true,
      value,
    });
  }

  return () => {
    for (const [key, descriptor] of previous) {
      if (descriptor === undefined) delete globalThis[key];
      else Object.defineProperty(globalThis, key, descriptor);
    }
  };
}

class HostDocumentFacade {
  constructor(recorder, ownerId) {
    this.recorder = recorder;
    this.ownerId = ownerId;
    this.nodeType = 9;
    this.nodeName = "#document";
    this.ownerDocument = null;
    this._listeners = new Map();
  }

  createElement(name) {
    if (String(name).toLowerCase() === "template") {
      return new HostTemplateElement(this.recorder, this);
    }
    return this.recorder.createElement(this.ownerId, String(name));
  }

  createElementNS(namespace, name) {
    if (namespace === "http://www.w3.org/1999/xhtml") return this.createElement(name);
    throw new HostProjectionError("UnsupportedNamespace", `지원하지 않는 namespace: ${namespace}`);
  }

  createTextNode(value = "") {
    return this.recorder.createText(this.ownerId, String(value), {
      internal: String(value) === "",
    });
  }

  createComment(value = "") {
    return this.recorder.createComment(this.ownerId, String(value));
  }

  createDocumentFragment() {
    return new HostDocumentFragment(this.recorder, { ownerId: this.ownerId });
  }

  importNode(node, deep = false) {
    return node.cloneNode(deep);
  }

  addEventListener(type, listener) {
    this.recorder.addListener(this.ownerId, this, type, listener);
  }

  removeEventListener(type, listener) {
    this.recorder.removeListener(this.ownerId, this, type, listener);
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
}

class HostTemplateElement extends HostElement {
  constructor(recorder, ownerDocument) {
    super(recorder, "template", {
      ownerId: ownerDocument.ownerId,
      internal: true,
    });
    this.ownerDocument = ownerDocument;
    this.content = new HostDocumentFragment(recorder, {
      ownerId: ownerDocument.ownerId,
      templateOnly: true,
    });
    this._innerHTML = "";
  }

  get innerHTML() {
    return this._innerHTML;
  }

  set innerHTML(value) {
    this._innerHTML = String(value);
    this.content.childNodes = [];
    for (const parsed of parseFragment(this._innerHTML).childNodes) {
      const child = templateNode(this.recorder, this.ownerDocument, parsed);
      this.content._appendTemplateChild(child);
    }
  }
}

function templateNode(recorder, ownerDocument, parsed) {
  if (parsed.nodeName === "#text") {
    const node = new HostText(recorder, parsed.value, { templateOnly: true });
    node.ownerDocument = ownerDocument;
    return node;
  }
  if (parsed.nodeName === "#comment") {
    const node = new HostComment(recorder, parsed.data, { templateOnly: true });
    node.ownerDocument = ownerDocument;
    return node;
  }
  if (typeof parsed.tagName === "string") {
    const node = new HostElement(recorder, parsed.tagName, { templateOnly: true });
    node.ownerDocument = ownerDocument;
    for (const attribute of parsed.attrs ?? []) {
      node.attributeMap.set(attribute.name, attribute.value);
    }
    for (const parsedChild of parsed.childNodes ?? []) {
      node._appendTemplateChild(templateNode(recorder, ownerDocument, parsedChild));
    }
    return node;
  }
  throw new HostProjectionError("UnsupportedTemplateNode", `지원하지 않는 Svelte template node: ${parsed.nodeName}`);
}
