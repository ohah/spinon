(() => {
  const state = globalThis.__spinonC054Precomparison;
  if (!state) {
    const app = document.createElement('div');
    app.id = 'c054-app';
    app.style.cssText = 'display:flex;gap:10px;width:300px;height:100px;margin:0;padding:0';
    document.body.appendChild(app);

    const left = document.createElement('div');
    left.id = 'c054-left';
    left.style.cssText = 'display:flex;flex-direction:column;gap:4px;width:140px;height:80px;--tile-size:32px';
    app.appendChild(left);
    const leftA = document.createElement('div');
    leftA.id = 'c054-left-a';
    leftA.style.cssText = 'display:block;flex-shrink:0;width:var(--tile-size);height:10px;margin:0;padding:0';
    left.appendChild(leftA);
    const leftB = document.createElement('div');
    leftB.id = 'c054-left-b';
    leftB.style.cssText = 'display:block;flex-shrink:0;width:var(--tile-size);height:10px;margin:0;padding:0';
    left.appendChild(leftB);

    const right = document.createElement('div');
    right.id = 'c054-right';
    right.style.cssText = 'display:flex;flex-direction:column;gap:4px;width:140px;height:80px;--tile-size:41px';
    app.appendChild(right);
    const rightA = document.createElement('div');
    rightA.id = 'c054-right-a';
    rightA.style.cssText = 'display:block;flex-shrink:0;width:var(--tile-size);height:10px;margin:0;padding:0';
    right.appendChild(rightA);
    const rightB = document.createElement('div');
    rightB.id = 'c054-right-b';
    rightB.style.cssText = 'display:block;flex-shrink:0;width:var(--tile-size);height:10px;margin:0;padding:0';
    right.appendChild(rightB);
    globalThis.__spinonC054Precomparison = { left, size: 32 };
  } else {
    state.size = state.size === 32 ? 46 : 32;
    state.left.style.setProperty('--tile-size', `${state.size}px`);
  }
})();
