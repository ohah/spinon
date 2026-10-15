import { HostProjectionRecorder } from "./host-projection.mjs";
import { HostProjectionError } from "./errors.mjs";

export async function runAdapterScenario(name, ownerId, mountAdapter) {
  const recorder = new HostProjectionRecorder();
  const app = await mountAdapter(recorder, ownerId);
  let unmounted = false;
  try {
    const snapshots = { mount: recorder.snapshot() };
    const clicks = { mount: recorder.dispatchCount };
    const button = recorder.find("button");
    if (button === null) throw new HostProjectionError("MissingFixtureNode", `${name}: button이 mount되지 않았습니다`);

    recorder.dispatchClick(button);
    await app.settle();
    snapshots.click = recorder.snapshot();
    clicks.click = recorder.dispatchCount;

    recorder.dispatchClick(button);
    await app.settle();
    snapshots.twoClicks = recorder.snapshot();
    clicks.twoClicks = recorder.dispatchCount;

    await app.unmount();
    unmounted = true;
    snapshots.unmount = recorder.snapshot();
    clicks.unmount = recorder.dispatchCount;

    let detachedClickErrorCode = null;
    try {
      recorder.dispatchClick(button);
    } catch (error) {
      detachedClickErrorCode = error instanceof HostProjectionError ? error.code : "UnknownError";
    }

    return {
      name,
      ownerId,
      snapshots,
      clicks,
      detachedClickErrorCode,
      rootClickListeners: recorder.listenerCount(recorder.root, "click"),
      documentClickListeners: recorder.document === null ? null : recorder.listenerCount(recorder.document, "click"),
      operations: [...recorder.operations],
      recorder,
      button,
      compileMetadata: app.compileMetadata ?? null,
    };
  } finally {
    try {
      if (!unmounted) await app.unmount();
    } finally {
      if (typeof app.dispose === "function") await app.dispose();
    }
  }
}
