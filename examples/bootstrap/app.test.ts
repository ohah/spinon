import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { runInNewContext } from "node:vm";

test("예제 번들이 HostDocument 커밋·rollback·getter 재진입·이벤트를 확인한다", () => {
  const source = readFileSync("app.js", "utf8");
  const facadeSource = readFileSync("../../native/v8/src/spinon_dom_facade.inc", "utf8")
    .replace(/^R"SPINONJS\(/, "")
    .replace(/\)SPINONJS"\s*$/, "");
  const batches: Array<unknown[]> = [];
  const texts: string[] = [];
  const createdNodes: Array<[number, string]> = [];
  let eventHandler: ((nodeId: number) => void) | undefined;
  let active = false;
  let documentRevision = 0n;
  let renderTreeRevision = 0n;
  let nodes = new Map<number, "element" | "text">();
  let elementNames = new Map<number, string>();
  let textValues = new Map<number, string>();
  let connected = new Set<number>();
  let parents = new Map<number, number>();
  let children = new Map<number, number[]>([[0, []]]);
  let attributes = new Map<number, Map<string, string>>();
  const isWellFormedUtf16 = (value: string) => {
    for (let index = 0; index < value.length; index += 1) {
      const unit = value.charCodeAt(index);
      if (unit >= 0xd800 && unit <= 0xdbff) {
        const next = value.charCodeAt(index + 1);
        if (!(next >= 0xdc00 && next <= 0xdfff)) return false;
        index += 1;
      } else if (unit >= 0xdc00 && unit <= 0xdfff) {
        return false;
      }
    }
    return true;
  };

  const host = {
    createNode(id: number, tag: string) {
      createdNodes.push([id, tag]);
    },
    setText(text: string) {
      texts.push(text);
    },
    __internal: {
      commitDocumentBatch(operations: Array<Record<string, unknown>>) {
        if (!Array.isArray(operations)) throw new TypeError("배열 필요");
        if (active) throw new TypeError("중첩 호출");
        active = true;
        try {
          const operationCount = operations.length;
          if (operationCount > 256) throw new TypeError("작업 수 제한");
          const normalized: Array<Record<string, unknown>> = [];
          let batchStringUnits = 0;
          const readInteger = (value: unknown) => {
            if (
              typeof value !== "number" ||
              !Number.isInteger(value) ||
              value < -2_147_483_648 ||
              value > 2_147_483_647
            ) {
              throw new TypeError("정수 필드 형식 오류");
            }
            return value;
          };
          const readString = (value: unknown, maximum: number) => {
            if (typeof value !== "string" || value.length > maximum) {
              throw new TypeError("문자열 필드 형식 또는 길이 오류");
            }
            batchStringUnits += value.length;
            if (batchStringUnits > 1_048_576) throw new TypeError("묶음 문자열 상한");
            return value;
          };
          const readName = (value: unknown) => {
            const name = readString(value, 1024);
            if (!isWellFormedUtf16(name)) throw new TypeError("이름 UTF-16 오류");
            return name;
          };
          for (let index = 0; index < operationCount; index += 1) {
            const operation = operations[index];
            if (operation === undefined) throw new TypeError("빈 작업 슬롯");
            const type = operation.type;
            if (typeof type !== "string" || type.length > 32) {
              throw new TypeError("작업 종류 문자열 형식 또는 길이 오류");
            }
            const parsed: Record<string, unknown> = { type };
            if (type === "createElement") {
              parsed.id = readInteger(operation.id);
              parsed.name = readName(operation.name);
              parsed.namespace = readName(
                operation.namespace ?? "http://www.w3.org/1999/xhtml",
              );
            } else if (type === "createText") {
              parsed.id = readInteger(operation.id);
              parsed.data = readString(operation.data, 1_048_576);
            } else if (type === "append" || type === "remove") {
              parsed.parent = readInteger(operation.parent);
              parsed.node = readInteger(operation.node);
            } else if (type === "insertBefore") {
              parsed.parent = readInteger(operation.parent);
              parsed.node = readInteger(operation.node);
              parsed.before = readInteger(operation.before);
            } else if (type === "setText") {
              parsed.node = readInteger(operation.node);
              parsed.data = readString(operation.data, 1_048_576);
            } else if (type === "setAttribute") {
              parsed.node = readInteger(operation.node);
              parsed.name = readName(operation.name);
              parsed.value = readString(operation.value, 1_048_576);
            } else if (type === "removeAttribute") {
              parsed.node = readInteger(operation.node);
              parsed.name = readName(operation.name);
            } else {
              throw new TypeError("지원하지 않는 변경 종류");
            }
            normalized.push(parsed);
          }
          batches.push(normalized);

          const nextNodes = new Map(nodes);
          const nextElementNames = new Map(elementNames);
          const nextTextValues = new Map(textValues);
          const nextConnected = new Set(connected);
          const nextParents = new Map(parents);
          const nextChildren = new Map(
            [...children].map(([id, childIds]) => [id, [...childIds]]),
          );
          const nextAttributes = new Map(
            [...attributes].map(([id, values]) => [id, new Map(values)]),
          );
          let changed = false;
          for (const operation of normalized) {
            const type = operation.type;
            const id = operation.id as number | undefined;
            const node = operation.node as number | undefined;
            if (type === "createElement" || type === "createText") {
              if (id === undefined || id <= 0 || nextNodes.has(id)) {
                throw new TypeError("중복 노드");
              }
              nextNodes.set(id, type === "createElement" ? "element" : "text");
              nextChildren.set(id, []);
              if (type === "createElement") nextElementNames.set(id, operation.name as string);
              if (type === "createText") nextTextValues.set(id, operation.data as string);
              if (type === "createElement") nextAttributes.set(id, new Map());
              changed = true;
            } else if (type === "append") {
              const parent = operation.parent as number;
              const childId = node as number;
              if (!nextNodes.has(childId) || (parent !== 0 && !nextNodes.has(parent))) {
                throw new TypeError("없는 노드");
              }
              if (nextParents.has(childId)) {
                const oldParent = nextParents.get(childId)!;
                nextChildren.set(oldParent, nextChildren.get(oldParent)!.filter((id) => id !== childId));
              }
              nextParents.set(childId, parent);
              nextChildren.get(parent)!.push(childId);
              nextConnected.add(node as number);
              if (parent !== 0) nextConnected.add(parent);
              changed = true;
            } else if (type === "insertBefore") {
              const parent = operation.parent as number;
              const childId = node as number;
              const before = operation.before as number;
              if (!nextNodes.has(childId)) throw new TypeError("없는 노드");
              if (parent !== 0 && !nextNodes.has(parent)) {
                throw new TypeError("없는 부모");
              }
              const siblings = nextChildren.get(parent)!;
              if (!siblings.includes(before)) throw new TypeError("없는 기준 노드");
              if (childId === before) continue;
              if (nextParents.has(childId)) {
                const oldParent = nextParents.get(childId)!;
                nextChildren.set(oldParent, nextChildren.get(oldParent)!.filter((id) => id !== childId));
              }
              const insertionPoint = nextChildren.get(parent)!.indexOf(before);
              nextChildren.get(parent)!.splice(insertionPoint, 0, childId);
              nextParents.set(childId, parent);
              nextConnected.add(node as number);
              if (parent !== 0) nextConnected.add(parent);
              changed = true;
            } else if (type === "remove") {
              const parent = operation.parent as number;
              const childId = node as number;
              if (!nextNodes.has(childId) || nextParents.get(childId) !== parent) {
                throw new TypeError("없는 자식 노드");
              }
              nextChildren.set(parent, nextChildren.get(parent)!.filter((id) => id !== childId));
              nextParents.delete(childId);
              nextConnected.delete(node as number);
              changed = true;
            } else if (type === "setText") {
              if (nextNodes.get(node as number) !== "text") throw new TypeError("텍스트 노드 아님");
              const value = operation.data as string;
              if (nextTextValues.get(node as number) !== value) {
                nextTextValues.set(node as number, value);
                changed = true;
              }
            } else if (type === "setAttribute") {
              if (nextNodes.get(node as number) !== "element") throw new TypeError("요소가 아님");
              const name = operation.name as string;
              const value = operation.value as string;
              if (nextAttributes.get(node as number)!.get(name) !== value) {
                nextAttributes.get(node as number)!.set(name, value);
                changed = true;
              }
            } else if (type === "removeAttribute") {
              if (nextNodes.get(node as number) !== "element") throw new TypeError("요소가 아님");
              changed = nextAttributes.get(node as number)!.delete(operation.name as string) || changed;
            } else {
              throw new TypeError("지원하지 않는 작업");
            }
          }

          if (changed) {
            nodes = nextNodes;
            elementNames = nextElementNames;
            textValues = nextTextValues;
            connected = nextConnected;
            parents = nextParents;
            children = nextChildren;
            attributes = nextAttributes;
            documentRevision += 1n;
            if (normalized.some((operation) => operation.type === "append") ||
                normalized.some((operation) => operation.type === "setText" && connected.has(operation.node as number))) {
              renderTreeRevision += 1n;
            }
          }
          return {
            changed,
            documentRevision,
            renderTreeRevision,
            nodeCount: BigInt(nodes.size),
          };
        } finally {
          active = false;
        }
      },
      readDocument(kind: number, nodeId: number, index = 0, name = "") {
        if (kind === 8) {
          return { exists: true, value: Math.max(0, ...nodes.keys()) + 1, text: "" };
        }
        if (kind === 1) {
          const type = nodes.get(nodeId);
          if (type === undefined) throw new TypeError("없는 노드");
          const localName = type === "element" ? elementNames.get(nodeId)! : "#text";
          return {
            exists: true,
            value: type === "element" ? 1 : 3,
            text: type === "element" ? localName.toUpperCase() : localName,
          };
        }
        if (kind === 2) {
          return parents.has(nodeId)
            ? { exists: true, value: parents.get(nodeId)!, text: "" }
            : { exists: false, value: 0, text: "" };
        }
        if (kind === 3) {
          return { exists: true, value: children.get(nodeId)?.length ?? 0, text: "" };
        }
        if (kind === 4) {
          const childId = children.get(nodeId)?.[index];
          return childId === undefined
            ? { exists: false, value: 0, text: "" }
            : { exists: true, value: childId, text: "" };
        }
        if (kind === 5) {
          const parent = parents.get(nodeId);
          const siblings = parent === undefined ? [] : children.get(parent) ?? [];
          const nextId = siblings[siblings.indexOf(nodeId) + 1];
          return nextId === undefined
            ? { exists: false, value: 0, text: "" }
            : { exists: true, value: nextId, text: "" };
        }
        if (kind === 6) {
          const collect = (id: number): string =>
            nodes.get(id) === "text"
              ? textValues.get(id) ?? ""
              : (children.get(id) ?? []).map(collect).join("");
          return { exists: true, value: 0, text: collect(nodeId) };
        }
        if (kind === 7) {
          const value = attributes.get(nodeId)?.get(name);
          return value === undefined
            ? { exists: false, value: 0, text: "" }
            : { exists: true, value: 0, text: value };
        }
        throw new TypeError("알 수 없는 조회 종류");
      },
    },
    onEvent(handler: (nodeId: number) => void) {
      eventHandler = handler;
    },
  };

  const context = { spinon: host, TypeError };
  runInNewContext(facadeSource, context);
  runInNewContext(source, context);
  expect(batches.slice(0, 5)).toEqual([
    [
      {
        type: "createElement",
        id: 1,
        name: "div",
        namespace: "http://www.w3.org/1999/xhtml",
      },
      { type: "createText", id: 2, data: "ready" },
      { type: "append", parent: 0, node: 1 },
      { type: "append", parent: 1, node: 2 },
      { type: "setAttribute", node: 1, name: "class", value: "counter" },
    ],
    [{ type: "setText", node: 999, data: "must roll back" }],
    [
      { type: "createText", id: 3, data: "temporary" },
      { type: "append", parent: 1, node: 3 },
      { type: "insertBefore", parent: 1, node: 3, before: 2 },
      { type: "setText", node: 2, data: "\uD800" },
      { type: "setText", node: 2, data: "ready" },
      { type: "setAttribute", node: 1, name: "data-temp", value: "yes" },
      { type: "removeAttribute", node: 1, name: "data-temp" },
      { type: "remove", parent: 1, node: 3 },
    ],
    [],
    [{ type: "setText", node: 2, data: "ready" }],
  ]);
  expect(batches.slice(5, -1).every((batch) => batch.length <= 1)).toBe(true);
  expect(createdNodes).toEqual([[1, "view"]]);
  expect(texts).toEqual(["문서 revision 1 · 노드 2"]);
  expect(eventHandler).toBeDefined();

  eventHandler?.(7);
  expect(batches.at(-1)).toEqual([
    { type: "setText", node: 2, data: "이벤트:7" },
  ]);
  expect(createdNodes).toEqual([
    [1, "view"],
    [8, "text"],
  ]);
  expect(texts.at(-1)).toBe(`이벤트:7 · 문서 revision ${documentRevision}`);
});
