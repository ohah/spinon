import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFile, writeFile } from 'node:fs/promises';
import { dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { parse } from 'parse5';

const repositoryRoot = resolve(dirname(fileURLToPath(import.meta.url)), '../..');
const inventoryPath = 'tests/fixtures/css/c11/grid-layout-inventory.json';
const htmlPath = 'tests/fixtures/css/c11/grid-layout.html';
const referencePath = 'tests/fixtures/css/references/c11-grid-precomparison-v2.json';
const outputPath = 'tests/fixtures/css/c11/runtime-grid-matrix.json';
const expectedSceneCount = 10;
const expectedNodeCount = 40;
const expectedPalette = new Set([
  'track-surface', 'blue', 'red', 'green', 'yellow', 'purple', 'cyan', 'orange', 'pink',
]);
const expectedNonPaletteRules = new Map([
  ['html, body', 'margin:0;padding:0'],
  ['body', 'direction:ltr;writing-mode:horizontal-tb;color-scheme:light'],
  ['*, *::before, *::after', 'box-sizing:content-box'],
  ['#stage', 'position:absolute;top:0;left:0;width:360px;height:800px'],
  ['.scene', 'position:absolute;top:0;left:0;display:none;width:360px;height:800px'],
  ['.scene.active', 'display:block'],
  ['.scene-root', 'position:absolute;top:0;left:0;width:360px;height:800px;margin:0;padding:0;border:0;direction:ltr;writing-mode:horizontal-tb'],
  ['[data-node]', 'margin:0;padding:0;border:0;box-sizing:content-box'],
]);
const expectedPaletteColors = new Map([
  ['track-surface', '#edf1f7'], ['blue', '#2866f6'], ['red', '#e5484d'],
  ['green', '#19a974'], ['yellow', '#ffc857'], ['purple', '#8758ff'],
  ['cyan', '#0ea5e9'], ['orange', '#f97316'], ['pink', '#ec4899'],
]);
const expectedInlineStyleProperties = new Set([
  'display', 'width', 'height', 'grid-template-columns', 'grid-template-rows',
  'column-gap', 'row-gap', 'flex-direction', 'align-items', 'margin', 'margin-left',
  'background-color',
]);
const runtimeSceneWrapperStyle = 'display:flex;width:360px;height:800px';

const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');

function attributes(node) {
  return new Map((node.attrs ?? []).map(({ name, value }) => [name, value]));
}

function childrenOf(node) {
  return (node.childNodes ?? []).filter(({ nodeName }) => nodeName !== '#comment');
}

function descendants(node, predicate) {
  const matches = [];
  const visit = (current) => {
    if (predicate(current)) matches.push(current);
    for (const child of childrenOf(current)) visit(child);
  };
  visit(node);
  return matches;
}

function textContent(node) {
  if (node.nodeName === '#text') return node.value;
  return childrenOf(node).map(textContent).join('');
}

function stylesheetText(document) {
  const styleElements = descendants(document, (node) => node.tagName === 'style');
  assert.equal(styleElements.length, 1, 'C11 HTML fixture의 style 요소 수가 달라졌습니다.');
  return textContent(styleElements[0]);
}

function paletteFromStylesheet(css) {
  const palette = new Map();
  const nonPaletteRules = new Map();
  const rulePattern = /([^{}]+)\{([^{}]*)\}/g;
  let consumed = '';
  for (const match of css.matchAll(rulePattern)) {
    consumed += match[0];
    const selector = match[1].trim().replace(/\s+/g, ' ');
    const declarations = match[2].split(';').map((value) => value.trim()).filter(Boolean)
      .map((value) => value.replace(/\s*:\s*/, ':')).join(';');
    const paletteClass = /^\.([a-z][a-z0-9-]*)$/i.exec(selector)?.[1];
    if (paletteClass && !expectedNonPaletteRules.has(selector)) {
      assert.equal(expectedPalette.has(paletteClass), true, `알 수 없는 paint class입니다: ${selector}`);
      assert.equal(palette.has(paletteClass), false, `배경색 class가 중복되었습니다: ${selector}`);
      assert.match(declarations, /^background-color:#[0-9a-f]{6}$/i,
        `${selector}: 배경색 외 속성 또는 잘못된 값이 있습니다.`);
      palette.set(paletteClass, declarations.slice('background-color:'.length).toLowerCase());
    } else {
      assert.equal(expectedNonPaletteRules.has(selector), true, `지원하지 않는 CSS selector입니다: ${selector}`);
      assert.equal(nonPaletteRules.has(selector), false, `CSS selector가 중복되었습니다: ${selector}`);
      nonPaletteRules.set(selector, declarations);
    }
  }
  assert.equal(consumed.replace(/\s/g, ''), css.replace(/\s/g, ''),
    'CSS 규칙 밖의 토큰 또는 중괄호가 있습니다.');
  assert.deepEqual(nonPaletteRules, expectedNonPaletteRules,
    'fixture reset/scene CSS가 고정된 값과 다릅니다.');
  assert.deepEqual(new Set(palette.keys()), expectedPalette,
    'C11 HTML fixture의 paint-only 색상 palette가 바뀌었습니다.');
  assert.deepEqual(palette, expectedPaletteColors, 'C11 HTML fixture의 고정 paint 색이 바뀌었습니다.');
  return palette;
}

function stylePropertyMap(style, description) {
  const properties = new Map();
  for (const declaration of style.split(';').map((value) => value.trim()).filter(Boolean)) {
    const colon = declaration.indexOf(':');
    assert.ok(colon > 0, `${description}: 잘못된 inline style declaration입니다: ${declaration}`);
    const name = declaration.slice(0, colon).trim().toLowerCase();
    const value = declaration.slice(colon + 1).trim();
    assert.equal(properties.has(name), false, `${description}: style property가 중복되었습니다: ${name}`);
    properties.set(name, value);
  }
  return properties;
}

function findFixtureStructure(html, inventory) {
  const parseErrors = [];
  const parsed = parse(html, {
    sourceCodeLocationInfo: true,
    onParseError(error) { parseErrors.push(error); },
  });
  assert.deepEqual(parseErrors, [], 'C11 HTML fixture에 HTML parse error가 있습니다.');
  assert.equal(parsed.mode, 'no-quirks', 'C11 HTML fixture가 standards mode가 아닙니다.');

  const css = stylesheetText(parsed);
  const palette = paletteFromStylesheet(css);
  const sceneElements = descendants(parsed, (node) =>
    node.tagName === 'section' && attributes(node).has('data-scene'));
  assert.equal(sceneElements.length, inventory.cases.length, 'HTML scene 수가 inventory와 다릅니다.');

  const globalIDs = new Set();
  const scenes = sceneElements.map((section) => {
    const sectionAttrs = attributes(section);
    const sceneId = sectionAttrs.get('data-scene');
    assert.ok(sceneId, 'scene ID가 비어 있습니다.');
    assert.deepEqual([...sectionAttrs.keys()].sort(), ['class', 'data-scene'],
      `${sceneId}: section attribute 계약이 바뀌었습니다.`);
    assert.equal(sectionAttrs.get('class'), 'scene', `${sceneId}: 예상하지 못한 scene class입니다.`);
    const wrappers = descendants(section, (node) =>
      node.tagName === 'div' && attributes(node).get('class') === 'scene-root');
    assert.equal(wrappers.length, 1, `${sceneId}: scene-root wrapper가 하나가 아닙니다.`);
    assert.deepEqual([...attributes(wrappers[0]).keys()].sort(), ['class', 'id'],
      `${sceneId}: scene-root attribute 계약이 바뀌었습니다.`);
    const allElements = descendants(section, (node) => Boolean(node.tagName));
    for (const node of allElements) {
      const attrs = attributes(node);
      if (node === section || node === wrappers[0]) continue;
      assert.equal(node.tagName, 'div', `${sceneId}: 지원하지 않는 element입니다: ${node.tagName}`);
      assert.ok(attrs.has('data-node'), `${sceneId}: data-node가 없는 wrapper element가 있습니다.`);
      for (const name of attrs.keys()) {
        assert.ok(['id', 'data-node', 'class', 'style'].includes(name),
          `${sceneId}/${attrs.get('id')}: 지원하지 않는 attribute입니다: ${name}`);
      }
    }

    const nodes = [];
    const visit = (node, nearestNodeId = null) => {
      const attrs = attributes(node);
      let nextParentId = nearestNodeId;
      if (attrs.has('data-node')) {
        const id = attrs.get('id');
        assert.ok(id, `${sceneId}: data-node에 id가 없습니다.`);
        assert.equal(globalIDs.has(id), false, `C11 HTML fixture에서 ID가 중복되었습니다: ${id}`);
        globalIDs.add(id);
        const className = attrs.get('class') ?? '';
        const classNames = className.split(/\s+/).filter(Boolean);
        assert.ok(classNames.length <= 1, `${sceneId}/${id}: paint class가 하나보다 많습니다.`);
        let style = attrs.get('style') ?? '';
        const styleProperties = stylePropertyMap(style, `${sceneId}/${id}`);
        for (const property of styleProperties.keys()) {
          assert.equal(expectedInlineStyleProperties.has(property), true,
            `${sceneId}/${id}: 지원하지 않는 inline style 속성입니다: ${property}`);
        }
        if (classNames.length === 1) {
          const classColor = palette.get(classNames[0]);
          assert.ok(classColor, `${sceneId}/${id}: 배경색 palette class를 찾지 못했습니다: ${classNames[0]}`);
          assert.equal(styleProperties.has('background-color'), false,
            `${sceneId}/${id}: inline color와 class color가 동시에 있습니다.`);
          style = `${style}${style.length > 0 && !style.trimEnd().endsWith(';') ? ';' : ''}background-color:${classColor}`;
        }
        nodes.push({ id, parentId: nearestNodeId, className: classNames[0] ?? null, style });
        nextParentId = id;
      }
      for (const child of childrenOf(node)) visit(child, nextParentId);
    };
    visit(section);
    return { id: sceneId, nodes };
  });
  return { scenes, palette };
}

function htmlSceneSources({ inventoryBytes, htmlBytes, referenceBytes }) {
  const inventory = JSON.parse(inventoryBytes.toString('utf8'));
  const html = htmlBytes.toString('utf8');
  const reference = JSON.parse(referenceBytes.toString('utf8'));
  assert.equal(inventory.schema, 'spinon-css-c11-grid-inventory/v1');
  assert.equal(inventory.fixtureId, 'C11-grid-precomparison-v1');
  assert.equal(inventory.cases.length, expectedSceneCount);
  assert.equal(reference.schema, 'spinon-css-c11-grid-precomparison/v1');
  assert.equal(reference.fixture.inventorySha256, sha256(inventoryBytes), 'inventory fixture hash가 기준과 다릅니다.');
  assert.equal(reference.fixture.htmlSha256, sha256(htmlBytes), 'HTML fixture hash가 기준과 다릅니다.');
  assert.equal(reference.observations.find(({ deviceScaleFactor }) => deviceScaleFactor === 1)?.cases.length,
    expectedSceneCount, 'DPR 1 Chromium reference scene 수가 예상과 다릅니다.');

  const structure = findFixtureStructure(html, inventory);
  const htmlSceneIDs = structure.scenes.map(({ id }) => id);
  assert.deepEqual(htmlSceneIDs, inventory.cases.map(({ id }) => id),
    'HTML scene ID 또는 순서가 inventory와 다릅니다.');
  const referenceCases = reference.observations
    .find(({ deviceScaleFactor }) => deviceScaleFactor === 1).cases;
  const cases = structure.scenes.map((scene, index) => {
    const inventoryCase = inventory.cases[index];
    const referenceCase = referenceCases[index];
    assert.equal(scene.id, inventoryCase.id);
    assert.equal(referenceCase.id, scene.id);
    assert.equal(referenceCase.rootId, inventoryCase.rootId);
    const nodeIds = scene.nodes.map(({ id }) => id);
    assert.deepEqual(nodeIds, inventoryCase.nodeIds, `${scene.id}: HTML과 inventory NodeId 순서가 다릅니다.`);
    assert.deepEqual(nodeIds, referenceCase.nodes.map(({ id }) => id),
      `${scene.id}: HTML과 Chromium NodeId 순서가 다릅니다.`);
    assert.deepEqual(scene.nodes.map(({ id, parentId }) => ({ id, parentId })),
      referenceCase.nodes.map(({ id, parentId }) => ({ id, parentId })),
      `${scene.id}: HTML과 Chromium parent 관계가 다릅니다.`);
    assert.equal(scene.nodes[0].id, inventoryCase.rootId, `${scene.id}: root ID가 첫 NodeId가 아닙니다.`);
    const referenceNodes = new Map(referenceCase.nodes.map((node) => [node.id, node]));
    for (const node of scene.nodes) {
      const referenceNode = referenceNodes.get(node.id);
      const properties = stylePropertyMap(node.style, `${scene.id}/${node.id}`);
      for (const [property, value] of properties) {
        if (!inventory.comparison.computedProperties.includes(property)) continue;
        if (property.startsWith('grid-template-') && /%|fr\b/i.test(value)) continue;
        const normalizedValue = value.trim().replace(/\s+/g, ' ');
        assert.equal(referenceNode.properties[property], normalizedValue,
          `${scene.id}/${node.id}: inline ${property}와 Chromium computed value가 다릅니다.`);
      }
    }

    const lines = [
      '(() => {',
      'const refs = Object.create(null);',
      'const nodeIds = [];',
      'const sceneRoot = document.createElement("div");',
      'sceneRoot.setAttribute("id", "c11-runtime-scene-root");',
      'sceneRoot.setAttribute("class", "scene-root");',
      `sceneRoot.setAttribute("style", ${JSON.stringify(runtimeSceneWrapperStyle)});`,
      'sceneRoot.setAttribute("data-c11-grid-wrapper", "");',
      'document.appendChild(sceneRoot);',
    ];
    for (const [nodeIndex, node] of scene.nodes.entries()) {
      const variable = `node_${nodeIndex}`;
      lines.push(`const ${variable} = document.createElement("div");`);
      lines.push(`${variable}.setAttribute("id", ${JSON.stringify(node.id)});`);
      lines.push(`${variable}.setAttribute("data-c11-grid-node", "");`);
      if (node.className) lines.push(`${variable}.setAttribute("class", ${JSON.stringify(node.className)});`);
      lines.push(`${variable}.setAttribute("style", ${JSON.stringify(node.style)});`);
      const parent = node.parentId === null ? 'sceneRoot' : `refs[${JSON.stringify(node.parentId)}]`;
      if (node.parentId !== null) {
        assert.ok(nodeIds.slice(0, nodeIndex).includes(node.parentId),
          `${scene.id}/${node.id}: parent가 preorder에서 자식보다 뒤에 있습니다.`);
      }
      lines.push(`${parent}.appendChild(${variable});`);
      lines.push(`refs[${JSON.stringify(node.id)}] = ${variable};`);
      lines.push(`nodeIds.push(${JSON.stringify(node.id)});`);
    }
    lines.push(`return { sceneId:${JSON.stringify(scene.id)}, rootId:${JSON.stringify(inventoryCase.rootId)}, nodeIds };`);
    lines.push('})()');
    const source = lines.join('\n');
    return {
      index,
      id: scene.id,
      rootId: inventoryCase.rootId,
      nodeIds,
      parentIds: scene.nodes.map(({ parentId }) => parentId),
      source,
      sourceSha256: sha256(Buffer.from(source)),
      referenceNodeCount: referenceCase.nodes.length,
    };
  });
  assert.equal(cases.reduce((total, fixtureCase) => total + fixtureCase.nodeIds.length, 0), expectedNodeCount);
  return {
    schema: 'spinon-css-c11-grid-runtime-matrix/v1',
    parser: { name: 'parse5', version: '7.3.0' },
    inputs: {
      inventory: { path: inventoryPath, sha256: sha256(inventoryBytes) },
      html: { path: htmlPath, sha256: sha256(htmlBytes) },
      chromiumReference: { path: referencePath, sha256: sha256(referenceBytes), referenceId: reference.referenceId },
      generator: { path: 'tools/css-reference/build-c11-grid-runtime-matrix.mjs' },
      palette: Object.fromEntries([...structure.palette].sort(([left], [right]) => left.localeCompare(right))),
    },
    viewportCssPx: inventory.viewport,
    environment: inventory.environment,
    runtimeSceneWrapperStyle,
    cases,
    summary: { sceneCount: cases.length, nodeCount: expectedNodeCount },
  };
}

export async function buildC11GridRuntimeMatrix(inputs) {
  const [inventoryBytes, htmlBytes, referenceBytes] = inputs
    ? [Buffer.from(inputs.inventory), Buffer.from(inputs.html), Buffer.from(inputs.reference)]
    : await Promise.all([
      readFile(resolve(repositoryRoot, inventoryPath)),
      readFile(resolve(repositoryRoot, htmlPath)),
      readFile(resolve(repositoryRoot, referencePath)),
    ]);
  return htmlSceneSources({ inventoryBytes, htmlBytes, referenceBytes });
}

async function main() {
  const matrix = await buildC11GridRuntimeMatrix();
  const generatorPath = resolve(repositoryRoot, 'tools/css-reference/build-c11-grid-runtime-matrix.mjs');
  matrix.inputs.generator.sha256 = sha256(await readFile(generatorPath));
  const output = `${JSON.stringify(matrix, null, 2)}\n`;
  const target = resolve(repositoryRoot, outputPath);
  if (process.argv.includes('--check')) {
    assert.equal(await readFile(target, 'utf8'), output,
      `${outputPath}가 입력 fixture로 재생성한 결과와 다릅니다. 생성 명령을 먼저 실행하세요.`);
  } else {
    await writeFile(target, output);
  }
  process.stdout.write(`C11.1 runtime matrix: ${matrix.summary.sceneCount}개 scene, ${matrix.summary.nodeCount}개 node, `
    + `HTML·inventory·Chrome reference 일치${process.argv.includes('--check') ? ' · 고정 결과 확인' : ' · 생성'}\n`);
}

if (process.argv[1] && resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  main().catch((error) => {
    process.stderr.write(`${error.stack ?? error}\n`);
    process.exitCode = 1;
  });
}
