globalThis.__spinonS03LifecycleProbeV1 = (() => {
  const probe = {
    reset() {
      spinon.onEvent(() => {});
      delete globalThis.__spinonLifecycleHeld;
      delete globalThis.__spinonLifecycleAttachedWeak;
      delete globalThis.__spinonLifecycleWeak;
      delete globalThis.__spinonLifecycleCallbackWeak;
      delete globalThis.__spinonLifecycleCallbackValue;

      let node = document.firstChild;
      while (node !== null) {
        const next = node.nextSibling;
        if (
          node.nodeType === 1 &&
          (node.getAttribute("data-spinon-lifecycle") === "parent" ||
            node.hasAttribute("data-spinon-lifecycle-stress"))
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
  };

  return Object.freeze(probe);
})();
