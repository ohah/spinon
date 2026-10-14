(() => {
  const ids = [
    'visible', 'clip-x', 'clip-y', 'clip-both', 'clip-margin-padding',
    'clip-margin-content', 'clip-margin-border', 'clip-margin-hidden',
    'mixed-visible-hidden', 'mixed-clip-auto', 'positioned-contained',
    'positioned-escape', 'nested-clip',
  ];
  const rect = (element) => {
    const value = element.getBoundingClientRect();
    return { x: value.x, y: value.y, width: value.width, height: value.height };
  };
  const samplesFor = (id, x, y) => {
    if (id === 'positioned-contained') return [
      ['paint', x + 61, y + 12], ['target-beyond-frame', x + 70, y + 12],
      ['x-overflow', x + 66, y + 12], ['y-overflow', x + 61, y + 15],
    ];
    if (id === 'positioned-escape') return [
      ['paint', x + 61, y + 12], ['target-beyond-frame', x + 70, y + 12],
      ['x-overflow', x + 66, y + 12], ['y-overflow', x + 61, y + 16],
    ];
    if (id === 'nested-clip') return [
      ['paint', x + 25, y + 10], ['target-beyond-frame', x + 60, y + 10],
      ['x-overflow', x + 52, y + 10], ['y-overflow', x + 25, y + 16],
    ];
    return [
      ['paint', x + 5, y + 6], ['target-beyond-frame', x + 70, y + 6],
      ['x-overflow', x + 66, y + 6], ['y-overflow', x + 5, y + 15],
    ];
  };
  const cases = ids.map((id, index) => {
    const x = (index % 3) * 100;
    const y = Math.floor(index / 3) * 20;
    const section = document.getElementById(`c131-surface-case-${id}`);
    const box = document.getElementById(`c131-surface-box-${id}`);
    const target = document.getElementById(`c131-surface-target-${id}`);
    const inner = id === 'nested-clip' ? document.getElementById('c131-surface-inner-nested-clip') : null;
    const style = getComputedStyle(box);
    const innerStyle = inner ? getComputedStyle(inner) : null;
    const frame = {
      section: rect(section), box: rect(box), target: rect(target),
      inner: inner ? rect(inner) : null,
      overflowX: style.overflowX, overflowY: style.overflowY,
      overflowClipMargin: style.overflowClipMargin, position: style.position,
      boxBackground: style.backgroundColor,
      innerOverflowX: innerStyle?.overflowX ?? null,
      innerOverflowY: innerStyle?.overflowY ?? null,
      innerOverflowClipMargin: innerStyle?.overflowClipMargin ?? null,
    };
    const samples = samplesFor(id, x, y).map(([role, px, py]) => {
      const hit = document.elementFromPoint(px, py);
      return {
        role, x: px, y: py,
        hitId: hit?.id ?? null,
        hitNode: hit?.getAttribute('data-c131-surface-node') === '' ? 'fixture-node' : (hit?.tagName ?? null),
        computedBackground: hit ? getComputedStyle(hit).backgroundColor : null,
      };
    });
    return { id, frame, samples };
  });
  return {
    metadata: {
      href: location.href,
      userAgent: navigator.userAgent,
      innerWidth, innerHeight, devicePixelRatio,
      scrollX, scrollY,
      root: rect(document.getElementById('c131-surface-root')),
      nodeCount: Object.keys(globalThis.spinonC131CompactSurfaceMatrixNodes).length,
    },
    cases,
  };
})()
