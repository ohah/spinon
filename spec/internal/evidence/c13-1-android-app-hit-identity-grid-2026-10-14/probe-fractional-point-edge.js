(async () => {
  const width = innerWidth;
  const height = innerHeight;
  const nodeIds = Object.keys(globalThis.spinonC131CompactSurfaceMatrixNodes ?? {});
  const nodeIndex = new Map(nodeIds.map((id, index) => [id, index]));
  const root = document.getElementById('c131-surface-root');
  const rootRect = root?.getBoundingClientRect();
  const rows = [];
  const histogram = new Array(nodeIds.length).fill(0);
  const identityBytes = new Uint8Array(width * height);
  const unexpected = [];
  let runCount = 0;
  let queryCount = 0;

  if (!root || nodeIds.length !== 41 || width !== 300 || height !== 100
      || devicePixelRatio !== 1 || scrollX !== 0 || scrollY !== 0
      || rootRect.x !== 0 || rootRect.y !== 0
      || rootRect.width !== width || rootRect.height !== height) {
    return {
      status: 'invalid-environment', href: location.href, userAgent: navigator.userAgent,
      width, height, devicePixelRatio, scrollX, scrollY, nodeCount: nodeIds.length,
      root: rootRect?.toJSON() ?? null,
    };
  }

  for (let y = 0; y < height; y += 1) {
    const rowRuns = [];
    let currentIndex = -1;
    let runStart = 0;
    for (let x = 0; x < width; x += 1) {
      const hit = document.elementFromPoint(x + 0.5, y + 0.5);
      const id = hit?.getAttribute('data-c131-surface-node') === '' ? hit.id : null;
      const index = nodeIndex.get(id);
      queryCount += 1;
      if (index === undefined) {
        identityBytes[y * width + x] = 255;
        if (unexpected.length < 32) unexpected.push({ xCssPx: x, yCssPx: y, tag: hit?.tagName ?? null, id });
      } else {
        identityBytes[y * width + x] = index;
        histogram[index] += 1;
      }
      if (x === 0) {
        currentIndex = index ?? 255;
        runStart = 0;
      } else if ((index ?? 255) !== currentIndex) {
        rowRuns.push([runStart, x, currentIndex]);
        runCount += 1;
        runStart = x;
        currentIndex = index ?? 255;
      }
    }
    rowRuns.push([runStart, width, currentIndex]);
    runCount += 1;
    rows.push(rowRuns);
  }

  const digest = await crypto.subtle.digest('SHA-256', identityBytes);
  const pixelCenterIdentitySha256 = Array.from(new Uint8Array(digest), byte => byte.toString(16).padStart(2, '0')).join('');
  return {
    status: unexpected.length === 0 ? 'captured' : 'unexpected-hit-target',
    browser: { userAgent: navigator.userAgent },
    viewport: { width, height, devicePixelRatio, scrollX, scrollY },
    root: rootRect.toJSON(),
    queryConvention: 'elementFromPoint(xCssPx + 0.5, yCssPx + 0.5)',
    nodeIds,
    queryCount,
    rowCount: rows.length,
    runCount,
    histogram,
    unexpected,
    pixelCenterIdentitySha256,
    rows,
  };
})()
