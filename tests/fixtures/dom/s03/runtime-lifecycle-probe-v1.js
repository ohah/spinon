globalThis.__spinonS03LifecycleProbeV1 = (() => {
  const largeRegistryNodeCount = 16_385;
  const largeRegistryTextCount = largeRegistryNodeCount - 1;
  const largeRegistryFirstId = 1_000_000_000;
  const largeRegistryMarker = "large-registry";

  const probe = {
    reset() {
      spinon.onEvent(() => {});
      delete globalThis.__spinonLifecycleHeld;
      delete globalThis.__spinonLifecycleAttachedWeak;
      delete globalThis.__spinonLifecycleWeak;
      delete globalThis.__spinonLifecycleCallbackWeak;
      delete globalThis.__spinonLifecycleCallbackValue;
      delete globalThis.__spinonS03LargeRegistryNodes;
      delete globalThis.__spinonS03LargeRegistryContainer;
      delete globalThis.__spinonS03LargeRegistryFirst;
      delete globalThis.__spinonS03LargeRegistryLast;

      let node = document.firstChild;
      while (node !== null) {
        const next = node.nextSibling;
        if (
          node.nodeType === 1 &&
          (node.getAttribute("data-spinon-lifecycle") === "parent" ||
            node.hasAttribute("data-spinon-lifecycle-stress") ||
            node.getAttribute("data-spinon-lifecycle-large") === largeRegistryMarker)
        ) {
          document.removeChild(node);
        }
        node = next;
      }
      spinon.__internal.requestLifecycleCollectionForTesting();
    },

    setupRootCases() {
      const parent = document.createElement("div");
      parent.setAttribute("data-spinon-lifecycle", "parent");
      let child = document.createTextNode("attached-child");
      globalThis.__spinonLifecycleAttachedWeak = new WeakRef(child);
      document.appendChild(parent);
      parent.appendChild(child);
      child = null;

      const detachedParent = document.createElement("section");
      detachedParent.setAttribute("data-spinon-lifecycle", "detached-parent");
      const detachedChild = document.createTextNode("held");
      detachedParent.appendChild(detachedChild);
      globalThis.__spinonLifecycleHeld = detachedChild;

      let orphan = document.createTextNode("orphan");
      globalThis.__spinonLifecycleWeak = new WeakRef(orphan);
      orphan = null;
      spinon.__internal.requestLifecycleCollectionForTesting();
    },

    verifyRootCases() {
      let parent = document.firstChild;
      while (
        parent !== null &&
        parent.getAttribute("data-spinon-lifecycle") !== "parent"
      ) {
        parent = parent.nextSibling;
      }
      const oldWrapperExpired =
        globalThis.__spinonLifecycleAttachedWeak.deref() === undefined;
      const recreatedChild = parent === null ? null : parent.firstChild;
      if (
        parent === null ||
        !oldWrapperExpired ||
        recreatedChild === null ||
        recreatedChild.textContent !== "attached-child" ||
        parent.firstChild !== recreatedChild ||
        globalThis.__spinonLifecycleHeld.textContent !== "held" ||
        globalThis.__spinonLifecycleHeld.parentNode.getAttribute(
          "data-spinon-lifecycle",
        ) !== "detached-parent" ||
        globalThis.__spinonLifecycleWeak.deref() !== undefined
      ) {
        throw new Error("weak wrapper GC root preservation fixture failed");
      }
      spinon.__internal.requestLifecycleCollectionForTesting();
    },

    cleanupRootCases() {
      let parent = document.firstChild;
      while (parent !== null) {
        const next = parent.nextSibling;
        if (
          parent.nodeType === 1 &&
          parent.getAttribute("data-spinon-lifecycle") === "parent"
        ) {
          document.removeChild(parent);
        }
        parent = next;
      }
      delete globalThis.__spinonLifecycleHeld;
      delete globalThis.__spinonLifecycleAttachedWeak;
      delete globalThis.__spinonLifecycleWeak;
      spinon.onEvent(() => {});
      spinon.__internal.requestLifecycleCollectionForTesting();
    },

    setupCallbackClosureRoot() {
      const parent = document.createElement("aside");
      const child = document.createTextNode("callback-root");
      parent.appendChild(child);
      globalThis.__spinonLifecycleCallbackWeak = new WeakRef(child);
      spinon.onEvent(
        ((held) => () => {
          globalThis.__spinonLifecycleCallbackValue = held.textContent;
        })(child),
      );
      spinon.__internal.requestLifecycleCollectionForTesting();
    },

    verifyCallbackClosureRoot() {
      const held = globalThis.__spinonLifecycleCallbackWeak.deref();
      if (
        held === undefined ||
        held.textContent !== "callback-root" ||
        globalThis.__spinonLifecycleCallbackValue !== "callback-root"
      ) {
        throw new Error("native callback closure failed to retain its wrapper root");
      }
    },

    replaceCallbackClosureRoot() {
      spinon.onEvent(() => {});
      delete globalThis.__spinonLifecycleCallbackValue;
      spinon.__internal.requestLifecycleCollectionForTesting();
    },

    verifyCallbackClosureReleased() {
      if (globalThis.__spinonLifecycleCallbackWeak.deref() !== undefined) {
        throw new Error("replaced native callback still retains its wrapper root");
      }
      delete globalThis.__spinonLifecycleCallbackWeak;
    },

    stressRound() {
      for (let index = 0; index < 32; index += 1) {
        const parent = document.createElement("div");
        parent.setAttribute("data-spinon-lifecycle-stress", `${index}-🧪`);
        const child = document.createTextNode(`cycle-${index}-한글🧪`);
        parent.appendChild(child);
        document.appendChild(parent);
        document.removeChild(parent);
      }
      spinon.__internal.requestLifecycleCollectionForTesting();
    },

    setupLargeRegistry() {
      const containerId = largeRegistryFirstId;
      spinon.__internal.commitDocumentBatch([
        {
          type: "createElement",
          id: containerId,
          namespace: "http://www.w3.org/1999/xhtml",
          name: "div",
        },
        {
          type: "setAttribute",
          node: containerId,
          name: "data-spinon-lifecycle-large",
          value: largeRegistryMarker,
        },
        { type: "append", parent: 0, node: containerId },
      ]);

      let container = document.firstChild;
      while (
        container !== null &&
        (container.nodeType !== 1 ||
          container.getAttribute("data-spinon-lifecycle-large") !== largeRegistryMarker)
      ) {
        container = container.nextSibling;
      }
      if (container === null) throw new Error("large registry container was not wrapped");
      globalThis.__spinonS03LargeRegistryContainer = container;

      let created = 0;
      while (created < largeRegistryTextCount) {
        const count = Math.min(128, largeRegistryTextCount - created);
        const operations = [];
        for (let index = 0; index < count; index += 1) {
          const id = containerId + created + index + 1;
          operations.push({ type: "createText", id, data: "" });
          operations.push({ type: "append", parent: containerId, node: id });
        }
        spinon.__internal.commitDocumentBatch(operations);
        created += count;
      }

      const nodes = [container];
      let child = container.firstChild;
      while (child !== null) {
        nodes.push(child);
        child = child.nextSibling;
      }
      if (nodes.length !== largeRegistryNodeCount) {
        throw new Error(`large registry expected ${largeRegistryNodeCount} wrappers, got ${nodes.length}`);
      }
      globalThis.__spinonS03LargeRegistryNodes = nodes;
      globalThis.__spinonS03LargeRegistryFirst = new WeakRef(nodes[0]);
      globalThis.__spinonS03LargeRegistryLast = new WeakRef(nodes[nodes.length - 1]);
      spinon.__internal.requestLifecycleCollectionForTesting();
    },

    verifyLargeRegistry() {
      const nodes = globalThis.__spinonS03LargeRegistryNodes;
      if (
        !Array.isArray(nodes) ||
        nodes.length !== largeRegistryNodeCount ||
        nodes[0] !== globalThis.__spinonS03LargeRegistryContainer ||
        nodes[1].nodeType !== 3 ||
        nodes[largeRegistryNodeCount - 1].nodeType !== 3 ||
        globalThis.__spinonS03LargeRegistryFirst.deref() !== nodes[0] ||
        globalThis.__spinonS03LargeRegistryLast.deref() !== nodes[nodes.length - 1]
      ) {
        throw new Error("large weak wrapper registry did not retain all boundary nodes");
      }
    },

    releaseLargeRegistry() {
      const container = globalThis.__spinonS03LargeRegistryContainer;
      if (container !== undefined && container.parentNode === document) {
        document.removeChild(container);
      }
      globalThis.__spinonS03LargeRegistryNodes = null;
      globalThis.__spinonS03LargeRegistryContainer = null;
      spinon.__internal.requestLifecycleCollectionForTesting();
    },

    verifyLargeRegistryReleased() {
      spinon.__internal.requestLifecycleCollectionForTesting();
      const first = globalThis.__spinonS03LargeRegistryFirst;
      const last = globalThis.__spinonS03LargeRegistryLast;
      if (first === undefined || last === undefined ||
          first.deref() !== undefined || last.deref() !== undefined) {
        throw new Error("large weak wrapper registry retained released boundary nodes");
      }
      delete globalThis.__spinonS03LargeRegistryFirst;
      delete globalThis.__spinonS03LargeRegistryLast;
    },
  };

  return Object.freeze(probe);
})();
