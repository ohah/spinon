spinon.createNode(1, "view");

const initialDocumentReceipt = spinon.__internal.commitDocumentBatch([
  { type: "createElement", id: 1, name: "div" },
  { type: "createText", id: 2, data: "ready" },
  { type: "append", parent: 0, node: 1 },
  { type: "append", parent: 1, node: 2 },
  { type: "setAttribute", node: 1, name: "class", value: "counter" },
]);
let invalidCallShapeRejected = false;
try {
  spinon.__internal.commitDocumentBatch({});
} catch (error) {
  invalidCallShapeRejected = error instanceof TypeError;
}

let malformedNameRejected = false;
try {
  spinon.__internal.commitDocumentBatch([
    { type: "createElement", id: 4, name: "\uD800" },
  ]);
} catch {
  malformedNameRejected = true;
}

let unknownOperationRejected = false;
try {
  spinon.__internal.commitDocumentBatch([{ type: "unsupported" }]);
} catch {
  unknownOperationRejected = true;
}

let nonIntegerFieldRejected = false;
try {
  spinon.__internal.commitDocumentBatch([
    { type: "createText", id: 1.5, data: "must reject" },
  ]);
} catch {
  nonIntegerFieldRejected = true;
}

if (
  !invalidCallShapeRejected ||
  !malformedNameRejected ||
  !unknownOperationRejected ||
  !nonIntegerFieldRejected ||
  initialDocumentReceipt.documentRevision !== 1n ||
  initialDocumentReceipt.renderTreeRevision !== 1n ||
  initialDocumentReceipt.nodeCount !== 2n ||
  !initialDocumentReceipt.changed
) {
  throw new Error(
    `초기 HostDocument 영수증이 예상과 다릅니다: ${JSON.stringify({
      invalidCallShapeRejected,
      malformedNameRejected,
      unknownOperationRejected,
      nonIntegerFieldRejected,
      documentRevision: initialDocumentReceipt.documentRevision.toString(),
      renderTreeRevision: initialDocumentReceipt.renderTreeRevision.toString(),
      nodeCount: initialDocumentReceipt.nodeCount.toString(),
      changed: initialDocumentReceipt.changed,
    })}`,
  );
}

let invalidBatchRejected = false;
try {
  spinon.__internal.commitDocumentBatch([
    { type: "setText", node: 999, data: "must roll back" },
  ]);
} catch {
  invalidBatchRejected = true;
}
const getterError = new Error("getter failure sentinel");
let getterErrorPreserved = false;
try {
  spinon.__internal.commitDocumentBatch([
    {
      get type() {
        throw getterError;
      },
    },
  ]);
} catch (error) {
  getterErrorPreserved = error === getterError;
}

let sparseSlotRejected = false;
try {
  const sparseBatch = [];
  sparseBatch.length = 1;
  spinon.__internal.commitDocumentBatch(sparseBatch);
} catch {
  sparseSlotRejected = true;
}

let operationLimitRejected = false;
try {
  spinon.__internal.commitDocumentBatch(
    Array.from({ length: 257 }, (_, index) => ({
      type: "createText",
      id: 100 + index,
      data: "x",
    })),
  );
} catch {
  operationLimitRejected = true;
}

let longOperationTypeRejected = false;
try {
  spinon.__internal.commitDocumentBatch([
    { type: "x".repeat(33), id: 899, data: "must reject before conversion" },
  ]);
} catch {
  longOperationTypeRejected = true;
}

let stringLimitRejected = false;
try {
  spinon.__internal.commitDocumentBatch([
    { type: "createText", id: 900, data: "a".repeat(600_000) },
    { type: "createText", id: 901, data: "b".repeat(500_001) },
  ]);
} catch {
  stringLimitRejected = true;
}

const operationCoverageReceipt = spinon.__internal.commitDocumentBatch([
  { type: "createText", id: 3, data: "temporary" },
  { type: "append", parent: 1, node: 3 },
  { type: "insertBefore", parent: 1, node: 3, before: 2 },
  { type: "setText", node: 2, data: "\uD800" },
  { type: "setText", node: 2, data: "ready" },
  { type: "setAttribute", node: 1, name: "data-temp", value: "yes" },
  { type: "removeAttribute", node: 1, name: "data-temp" },
  { type: "remove", parent: 1, node: 3 },
]);
if (
  operationCoverageReceipt.documentRevision !== 2n ||
  operationCoverageReceipt.nodeCount !== 3n ||
  !operationCoverageReceipt.changed
) {
  throw new Error("V8 문서 변경 종류 8개의 실행 결과가 예상과 다릅니다");
}

const emptyReceipt = spinon.__internal.commitDocumentBatch([]);
if (
  !invalidBatchRejected ||
  !getterErrorPreserved ||
  !sparseSlotRejected ||
  !operationLimitRejected ||
  !longOperationTypeRejected ||
  !stringLimitRejected ||
  emptyReceipt.changed ||
  emptyReceipt.documentRevision !== operationCoverageReceipt.documentRevision ||
  emptyReceipt.nodeCount !== operationCoverageReceipt.nodeCount
) {
  throw new Error("거부된 HostDocument 묶음이 원자적으로 복구되지 않았습니다");
}

const growingBatch = [];
growingBatch.push({
  get type() {
    for (let index = 0; index < 300; index += 1) {
      growingBatch.push({ type: "unsupported" });
    }
    let nestedRejected = false;
    try {
      spinon.__internal.commitDocumentBatch([]);
    } catch {
      nestedRejected = true;
    }
    if (!nestedRejected) throw new Error("중첩 문서 묶음이 거부되지 않았습니다");
    return "setText";
  },
  node: 2,
  data: "ready",
});
const getterReceipt = spinon.__internal.commitDocumentBatch(growingBatch);
if (
  getterReceipt.changed ||
  getterReceipt.documentRevision !== operationCoverageReceipt.documentRevision ||
  getterReceipt.nodeCount !== operationCoverageReceipt.nodeCount
) {
  throw new Error("배열 getter 처리 중 변경 묶음 길이가 달라졌습니다");
}

if (
  document.nodeType !== 9 ||
  document.firstChild.nodeType !== 1 ||
  document.firstChild.nodeName !== "DIV" ||
  document.firstChild.localName !== "div" ||
  document.firstChild !== document.firstChild ||
  document.firstChild.className !== "counter"
) {
  throw new Error("DOM façade의 기존 HostDocument 조회가 예상과 다릅니다");
}

const root = document.firstChild;
const movedElement = document.createElement("DiV");
const movedText = document.createTextNode("한글🌐");
const target = document.createElement("section");
const defaultDOMException = new DOMException();
const messageOnlyDOMException = new DOMException("메시지만");
const namedDOMException = new DOMException("실패", "NotFoundError");
const domExceptionNameDescriptor = Object.getOwnPropertyDescriptor(
  DOMException.prototype,
  "name",
);
const domExceptionMessageDescriptor = Object.getOwnPropertyDescriptor(
  DOMException.prototype,
  "message",
);
const domExceptionTagDescriptor = Object.getOwnPropertyDescriptor(
  DOMException.prototype,
  Symbol.toStringTag,
);
const constructorGlobalDescriptors = ["DOMException", "Node", "Element", "Text"].map(
  (name) => Object.getOwnPropertyDescriptor(globalThis, name),
);
let rejectedDOMExceptionSymbolArguments = 0;
for (const args of [[Symbol("message")], ["", Symbol("name")]]) {
  try {
    new DOMException(...args);
  } catch (error) {
    if (error instanceof TypeError) rejectedDOMExceptionSymbolArguments += 1;
  }
}
const undefinedBeforeParent = document.createElement("aside");
const undefinedBeforeChild = document.createElement("span");
undefinedBeforeParent.insertBefore(undefinedBeforeChild, undefined);
const rootText = document.createTextNode("루트 텍스트");
Object.defineProperty(rootText, "nodeType", { configurable: true, value: 1 });
let illegalReceiverErrors = 0;
for (const call of [
  () => Node.prototype.isSameNode.call({}, document),
  () => Object.getOwnPropertyDescriptor(Node.prototype, "ownerDocument").get.call({}),
  () => Object.getPrototypeOf(document).createElement.call({}, "article"),
  () => document.contains({}),
  () => Element.prototype.getAttribute.call(movedText, "id"),
  () => Object.getOwnPropertyDescriptor(Text.prototype, "data").get.call(movedElement),
  () => domExceptionNameDescriptor.get.call({}),
  () => domExceptionMessageDescriptor.get.call({}),
]) {
  try {
    call();
  } catch (error) {
    if (error.name === "TypeError") illegalReceiverErrors += 1;
  }
}
const beforeMissingArguments = spinon.__internal.commitDocumentBatch([]).documentRevision;
let missingRequiredArguments = 0;
for (const call of [
  () => document.createElement(),
  () => document.createTextNode(),
  () => movedElement.setAttribute("data-missing-value"),
  () => movedElement.insertBefore(movedText),
]) {
  try {
    call();
  } catch (error) {
    if (error.name === "TypeError") missingRequiredArguments += 1;
  }
}
if (
  missingRequiredArguments !== 4 ||
  spinon.__internal.commitDocumentBatch([]).documentRevision !== beforeMissingArguments ||
  movedText.firstChild !== null ||
  movedText.hasChildNodes() ||
  movedText.nodeValue !== "한글🌐" ||
  movedElement.nodeValue !== null ||
  document.nodeValue !== null ||
  defaultDOMException.name !== "Error" ||
  defaultDOMException.message !== "" ||
  messageOnlyDOMException.name !== "Error" ||
  messageOnlyDOMException.message !== "메시지만" ||
  namedDOMException.name !== "NotFoundError" ||
  namedDOMException.message !== "실패" ||
  !(defaultDOMException instanceof Error) ||
  defaultDOMException.toString() !== "Error" ||
  defaultDOMException.code !== undefined ||
  namedDOMException.code !== undefined ||
  DOMException.NOT_FOUND_ERR !== undefined ||
  DOMException.name !== "DOMException" ||
  DOMException.prototype.constructor !== DOMException ||
  Object.prototype.toString.call(defaultDOMException) !== "[object DOMException]" ||
  domExceptionTagDescriptor?.value !== "DOMException" ||
  domExceptionTagDescriptor?.writable !== false ||
  domExceptionTagDescriptor?.enumerable !== false ||
  domExceptionTagDescriptor?.configurable !== true ||
  constructorGlobalDescriptors.some(
    (descriptor) =>
      descriptor?.writable !== true ||
      descriptor.enumerable !== false ||
      descriptor.configurable !== true,
  ) ||
  Object.getOwnPropertyDescriptor(defaultDOMException, "name") !== undefined ||
  Object.getOwnPropertyDescriptor(defaultDOMException, "message") !== undefined ||
  domExceptionNameDescriptor?.enumerable !== true ||
  domExceptionNameDescriptor?.configurable !== true ||
  domExceptionNameDescriptor?.set !== undefined ||
  typeof domExceptionNameDescriptor?.get !== "function" ||
  domExceptionMessageDescriptor?.enumerable !== true ||
  domExceptionMessageDescriptor?.configurable !== true ||
  domExceptionMessageDescriptor?.set !== undefined ||
  typeof domExceptionMessageDescriptor?.get !== "function" ||
  Reflect.set(defaultDOMException, "name", "Changed") ||
  Reflect.set(defaultDOMException, "message", "Changed") ||
  defaultDOMException.name !== "Error" ||
  defaultDOMException.message !== "" ||
  rejectedDOMExceptionSymbolArguments !== 2 ||
  illegalReceiverErrors !== 8 ||
  document.isSameNode(undefined) ||
  document.contains(undefined) ||
  undefinedBeforeParent.firstChild !== undefinedBeforeChild ||
  undefinedBeforeChild.parentNode !== undefinedBeforeParent
) {
  throw new Error(`DOM façade 결과가 예상과 다릅니다: ${JSON.stringify({
    missingRequiredArguments,
    textFirstChild: movedText.firstChild,
    textHasChildren: movedText.hasChildNodes(),
    textNodeValue: movedText.nodeValue,
    elementNodeValue: movedElement.nodeValue,
    documentNodeValue: document.nodeValue,
    defaultDOMException: [defaultDOMException.name, defaultDOMException.message],
    namedDOMException: [namedDOMException.name, namedDOMException.message],
    nameOwnProperty: Object.getOwnPropertyDescriptor(defaultDOMException, "name"),
    messageOwnProperty: Object.getOwnPropertyDescriptor(defaultDOMException, "message"),
    namePrototypeDescriptor: domExceptionNameDescriptor,
    messagePrototypeDescriptor: domExceptionMessageDescriptor,
    tagPrototypeDescriptor: domExceptionTagDescriptor,
    illegalReceiverErrors,
    isSameNodeUndefined: document.isSameNode(undefined),
    containsUndefined: document.contains(undefined),
    undefinedBeforeParent: undefinedBeforeChild.parentNode === undefinedBeforeParent,
  })}`);
}
const beforeRejectedRootText = spinon.__internal.commitDocumentBatch([]).documentRevision;
let rootTextErrorName = "";
try {
  document.appendChild(rootText);
} catch (error) {
  rootTextErrorName = error.name;
}
if (
  rootTextErrorName !== "HierarchyRequestError" ||
  rootText.parentNode !== null ||
  rootText.nodeValue !== "루트 텍스트" ||
  spinon.__internal.commitDocumentBatch([]).documentRevision !== beforeRejectedRootText
) {
  throw new Error("앱 문서 루트의 Text 거부가 원자적으로 처리되지 않았습니다");
}
movedElement.nodeValue = "무시";
document.nodeValue = "무시";
if (movedElement.nodeValue !== null || document.nodeValue !== null) {
  throw new Error("Text가 아닌 노드의 nodeValue 설정이 문서 값을 바꿨습니다");
}
let symbolStringFailures = 0;
for (let index = 0; index < 300; index += 1) {
  try {
    document.createTextNode(Symbol("unsupported"));
  } catch (error) {
    if (error.name === "TypeError") symbolStringFailures += 1;
  }
}
const textAfterSymbolFailures = document.createTextNode("after symbol failures");
if (symbolStringFailures !== 300 || textAfterSymbolFailures.data !== "after symbol failures") {
  throw new Error("DOMString의 Symbol 변환이 동기 거부되지 않았습니다");
}
root.appendChild(movedElement);
movedElement.appendChild(movedText);
document.appendChild(target);
target.appendChild(movedElement);
if (
  target.firstChild !== movedElement ||
  movedElement.parentNode !== target ||
  movedElement.parentElement !== target ||
  movedText.parentNode !== movedElement ||
  movedElement.textContent !== "한글🌐" ||
  movedText.nodeName !== "#text" ||
  movedText.nodeType !== 3 ||
  !(movedElement instanceof Node) ||
  !(movedElement instanceof Element) ||
  !(movedText instanceof Text) ||
  movedElement.ownerDocument !== document ||
  !target.contains(movedText) ||
  root.contains(movedElement) ||
  !movedText.isSameNode(movedElement.firstChild)
) {
  throw new Error("DOM façade의 동기 이동·관계·텍스트 조회가 예상과 다릅니다");
}

const precedingText = document.createTextNode("앞");
movedElement.insertBefore(precedingText, movedText);
if (
  movedElement.firstChild !== precedingText ||
  precedingText.nextSibling !== movedText ||
  movedElement.textContent !== "앞한글🌐"
) {
  throw new Error("DOM façade의 자식 순서가 예상과 다릅니다");
}

movedText.nodeValue = "변경";
movedElement.setAttribute("CLASS", "first second");
movedElement.setAttribute("data-number", 17);
if (
  movedText.nodeValue !== "변경" ||
  movedText.data !== "변경" ||
  movedElement.getAttribute("class") !== "first second" ||
  movedElement.className !== "first second" ||
  movedElement.getAttribute("data-number") !== "17"
) {
  throw new Error("DOM façade의 Text·속성 반영이 예상과 다릅니다");
}
movedElement.id = "dom-proof";
if (movedElement.getAttribute("id") !== "dom-proof") {
  throw new Error("DOM façade의 id 속성 반영이 예상과 다릅니다");
}
movedElement.removeAttribute("id");
movedElement.removeAttribute("data-number");
if (movedElement.hasAttribute("id") || movedElement.getAttribute("id") !== null) {
  throw new Error("DOM façade의 속성 제거가 예상과 다릅니다");
}

const beforeNoOp = spinon.__internal.commitDocumentBatch([]).documentRevision;
movedElement.insertBefore(precedingText, precedingText);
const afterNoOp = spinon.__internal.commitDocumentBatch([]).documentRevision;
if (beforeNoOp !== afterNoOp || movedElement.firstChild !== precedingText) {
  throw new Error("DOM façade의 자기 자신 앞 삽입이 no-op이 아닙니다");
}

const stableParent = movedElement.parentNode;
let notFoundErrorName = "";
let notFoundErrorIsDOMException = false;
try {
  root.removeChild(movedElement);
} catch (error) {
  notFoundErrorName = error.name;
  notFoundErrorIsDOMException = error instanceof DOMException && error instanceof Error;
}
let hierarchyErrorName = "";
try {
  movedElement.appendChild(target);
} catch (error) {
  hierarchyErrorName = error.name;
}
let invalidNameErrorName = "";
try {
  document.createElement("bad name");
} catch (error) {
  invalidNameErrorName = error.name;
}
if (
  notFoundErrorName !== "NotFoundError" ||
  !notFoundErrorIsDOMException ||
  hierarchyErrorName !== "HierarchyRequestError" ||
  invalidNameErrorName !== "InvalidCharacterError" ||
  movedElement.parentNode !== stableParent
) {
  throw new Error("DOM façade 오류 또는 실패 뒤 기존 트리 보존이 예상과 다릅니다");
}

target.removeChild(movedElement);
if (movedElement.parentNode !== null || movedElement.firstChild !== precedingText) {
  throw new Error("DOM façade의 분리 노드 조회가 예상과 다릅니다");
}
target.appendChild(movedElement);
if (target.firstChild !== movedElement || movedElement.parentNode !== target) {
  throw new Error("DOM façade의 분리 노드 재삽입이 예상과 다릅니다");
}

const parentNodeDescriptor = Object.getOwnPropertyDescriptor(Node.prototype, "parentNode");
let containsWithPatchedParent = false;
let cycleWithPatchedParent = "";
Object.defineProperty(Node.prototype, "parentNode", {
  configurable: true,
  get() {
    return this;
  },
});
try {
  containsWithPatchedParent = target.contains(movedText);
  try {
    movedElement.appendChild(target);
  } catch (error) {
    cycleWithPatchedParent = error.name;
  }
} finally {
  Object.defineProperty(Node.prototype, "parentNode", parentNodeDescriptor);
}
if (!containsWithPatchedParent || cycleWithPatchedParent !== "HierarchyRequestError") {
  throw new Error("Node getter 재정의가 트리 포함·순환 검증을 바꿨습니다");
}

spinon.setText(
  `문서 revision ${initialDocumentReceipt.documentRevision} · 노드 ${initialDocumentReceipt.nodeCount}`,
);

spinon.onEvent((nodeId) => {
  spinon.createNode(nodeId + 1, "text");
  const eventReceipt = spinon.__internal.commitDocumentBatch([
    { type: "setText", node: 2, data: `이벤트:${nodeId}` },
  ]);
  spinon.setText(
    `이벤트:${nodeId} · 문서 revision ${eventReceipt.documentRevision}`,
  );
});
