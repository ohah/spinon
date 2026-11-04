(() => ({
  userAgent: navigator.userAgent,
  viewport: [innerWidth, innerHeight],
  dpr: devicePixelRatio,
  scenes: Array.from(document.querySelectorAll('.scene')).map(scene => {
    const root = scene.getBoundingClientRect();
    return {
      id: scene.id,
      items: Array.from(scene.querySelectorAll('[data-id]')).map(item => {
        const r = item.getBoundingClientRect();
        const css = getComputedStyle(item);
        return {
          id: item.dataset.id,
          x: r.x - root.x,
          y: r.y - root.y,
          width: r.width,
          height: r.height,
          display: css.display,
          visibility: css.visibility,
        };
      }),
    };
  }),
}))()
