import { createHash } from 'node:crypto';
import { existsSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
import { spawnSync } from 'node:child_process';
import { tmpdir } from 'node:os';
import { dirname, join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import postcss from 'postcss';

const here = dirname(fileURLToPath(import.meta.url));
const generatedPath = join(here, '.output', 'generated.css');
const converterPath = resolve(here, '../style-layout/compile-css.mjs');
const converterPackage = JSON.parse(readFileSync(resolve(here, '../style-layout/package.json'), 'utf8'));
const lightningCssVersion = converterPackage.dependencies?.lightningcss;
if (lightningCssVersion !== '1.33.0') throw new Error(`mobile converter Lightning CSS 버전 drift: ${lightningCssVersion}`);
if (!existsSync(generatedPath)) throw new Error('먼저 `bun run build:css`로 generated CSS를 만들어야 합니다.');

const css = readFileSync(generatedPath, 'utf8');
const root = postcss.parse(css, { from: generatedPath });
const generatedSha256 = createHash('sha256').update(css).digest('hex');
const expectedGeneratedSha256 = 'c11cf6cff5c8e4b3053da8d328cda229884a6b59f9b4f3910d5b1a1f2023740b';
if (generatedSha256 !== expectedGeneratedSha256) throw new Error(`generated CSS hash drift: ${generatedSha256}`);
const rules = [];
const layerStatements = [];
const layerBlocks = new Set();
const atRules = [];
const variables = [];

root.walkAtRules(atRule => {
  atRules.push({ name: atRule.name, params: atRule.params });
  if (atRule.name === 'layer') {
    if (atRule.nodes) layerBlocks.add(atRule.params.trim());
    else layerStatements.push(atRule.params.replace(/\s/g, ''));
  }
});
root.walkRules(rule => {
  const ancestors = [];
  for (let parent = rule.parent; parent && parent.type !== 'root'; parent = parent.parent) {
    if (parent.type === 'atrule') ancestors.unshift({ name: parent.name, params: parent.params });
  }
  const declarations = [];
  rule.walkDecls(declaration => {
    declarations.push({ property: declaration.prop, value: declaration.value, important: declaration.important });
    if (declaration.prop.startsWith('--')) {
      variables.push({ name: declaration.prop, value: declaration.value, selector: rule.selector, ancestors });
    }
  });
  rules.push({ selector: rule.selector, ancestors, declarations, node: rule });
});

function requireRule(description, predicate) {
  const found = rules.find(predicate);
  if (!found) throw new Error(`생성 CSS에서 ${description} 규칙을 찾지 못했습니다.`);
  return found;
}

function declaration(rule, property) {
  return rule.declarations.find(item => item.property === property);
}

function requireDeclaration(rule, property, predicate) {
  const found = declaration(rule, property);
  if (!found || !predicate(found)) throw new Error(`${rule.selector}의 ${property} 사전 기준을 충족하지 않습니다.`);
  return found;
}

const classRules = new Map([
  'flex', 'items-center', 'gap-2', 'rounded-md', 'bg-brand-600', 'p-4', 'font-semibold', 'text-white',
].map(name => [name, requireRule(`.${name}`, rule => rule.selector === `.${name}` && rule.ancestors.some(a => a.name === 'layer' && a.params.trim() === 'utilities'))]));
const hoverRule = requireRule('hover variant', rule => rule.selector.includes('.hover\\:bg-brand-700'));
const responsiveRule = requireRule('responsive variant', rule => rule.selector.includes('.sm\\:p-6'));
const utilitySelectors = rules
  .filter(rule => rule.ancestors.some(a => a.name === 'layer' && a.params.trim() === 'utilities'))
  .map(rule => rule.selector)
  .sort();
const expectedUtilitySelectors = [[...classRules.keys()].map(name => `.${name}`), [hoverRule.selector, responsiveRule.selector]].flat().sort();
if (JSON.stringify(utilitySelectors) !== JSON.stringify(expectedUtilitySelectors)) {
  throw new Error(`source-scoped utility selector set이 다릅니다: ${JSON.stringify(utilitySelectors)}`);
}
const selectorKinds = {
  global: rules.filter(rule => /(^|[\s,])\*/.test(rule.selector)).map(rule => rule.selector),
  pseudo: rules.filter(rule => /::|:(?:hover|where|host|root|is|not)\b/.test(rule.selector)).map(rule => rule.selector),
  attribute: rules.filter(rule => /\[[^\]]+\]/.test(rule.selector)).map(rule => rule.selector),
  class: utilitySelectors,
};
for (const [kind, selectors] of Object.entries(selectorKinds)) {
  if (!selectors.length) throw new Error(`generated CSS selector 유형 ${kind}이 없습니다.`);
}
const themeRule = requireRule('theme token', rule => rule.selector === ':root, :host' && rule.ancestors.some(a => a.name === 'layer' && a.params.trim() === 'theme'));
const baseRule = requireRule('Preflight global reset', rule => rule.selector.includes('::file-selector-button') && rule.selector.includes('::after'));
const headingRule = requireRule('Preflight heading reset', rule => rule.selector.includes('h1') && rule.selector.includes('h6'));
const listRule = requireRule('Preflight list reset', rule => rule.selector.includes('ol') && rule.selector.includes('ul') && rule.selector.includes('menu'));
const imageRule = requireRule('Preflight replaced element display', rule => rule.selector.includes('img') && rule.selector.includes('svg'));
const imageSizingRule = requireRule('Preflight image sizing', rule => rule.selector.includes('img') && declaration(rule, 'max-width'));
const hiddenRule = requireRule('Preflight hidden behavior', rule => rule.selector.includes('[hidden]'));

for (const layer of ['theme', 'base', 'utilities']) {
  if (!layerBlocks.has(layer)) throw new Error(`@layer ${layer} 출력이 없습니다.`);
}
if (!layerStatements.includes('theme,base,components,utilities')) throw new Error('공식 layer 순서 선언이 다릅니다.');
for (const token of ['--color-brand-600', '--color-brand-700']) {
  if (!declaration(themeRule, token)) throw new Error(`${token} theme token이 출력되지 않았습니다.`);
}
requireDeclaration(classRules.get('bg-brand-600'), 'background-color', item => item.value.includes('var(--color-brand-600)'));
requireDeclaration(hoverRule, 'background-color', item => item.value.includes('var(--color-brand-700)'));
requireDeclaration(baseRule, 'box-sizing', item => item.value === 'border-box');
requireDeclaration(baseRule, 'margin', item => item.value === '0');
requireDeclaration(baseRule, 'padding', item => item.value === '0');
requireDeclaration(baseRule, 'border', item => item.value === '0 solid');
requireDeclaration(headingRule, 'font-size', item => item.value === 'inherit');
requireDeclaration(headingRule, 'font-weight', item => item.value === 'inherit');
requireDeclaration(listRule, 'list-style', item => item.value === 'none');
requireDeclaration(imageRule, 'display', item => item.value === 'block');
requireDeclaration(imageSizingRule, 'max-width', item => item.value === '100%');
requireDeclaration(imageSizingRule, 'height', item => item.value === 'auto');
requireDeclaration(hiddenRule, 'display', item => item.value === 'none' && item.important);

const mediaCondition = rule => rule.ancestors.filter(a => a.name === 'media').map(a => a.params);
if (!mediaCondition(hoverRule).some(value => value.includes('hover: hover'))) throw new Error('hover variant media 조건이 없습니다.');
if (!mediaCondition(responsiveRule).some(value => value.includes('40rem'))) throw new Error('sm variant media 조건이 없습니다.');

function withMediaAncestors(rule) {
  let result = rule.node.toString();
  for (const ancestor of rule.ancestors.filter(a => a.name === 'media').reverse()) {
    result = `@media ${ancestor.params} {\n${result}\n}`;
  }
  return result;
}

function runConverter(name, source, expected, expectedDiagnostic) {
  const directory = mkdtempSync(join(tmpdir(), 'spinon-r14-converter-'));
  const input = join(directory, `${name}.css`);
  const output = join(directory, `${name}.json`);
  writeFileSync(input, source);
  const run = spawnSync(process.execPath, [converterPath, input, output], { encoding: 'utf8' });
  const text = `${run.stdout || ''}\n${run.stderr || ''}`;
  const diagnostic = run.status === 0
    ? `PASS: ${(run.stdout || '').trim().split(/\r?\n/).at(-1)?.replace(/^.*?: /, '') || '변환 성공'}`
    : text.split(/\r?\n/).filter(line => line.includes('Error:')).at(-1) || text.trim().split(/\r?\n/).at(-1) || '';
  const outputExists = existsSync(output);
  const parsed = outputExists ? JSON.parse(readFileSync(output, 'utf8')) : null;
  rmSync(directory, { recursive: true, force: true });
  const passed = expected === 'accept'
    ? run.status === 0 && outputExists && parsed?.version === 1 && parsed?.styles?.flex?.display === 'flex' && Object.keys(parsed.styles).length === 1
    : run.status !== 0 && !outputExists && diagnostic.includes(expectedDiagnostic);
  if (!passed) throw new Error(`${name} converter control 불일치: exit=${run.status}, output=${outputExists}, diagnostic=${diagnostic}`);
  return { name, expected, exitCode: run.status, outputExists, diagnostic, parsed };
}

const backgroundRule = requireRule('.bg-brand-600', rule => rule.selector === '.bg-brand-600');
const paddingRule = requireRule('.p-4', rule => rule.selector === '.p-4');
const preflightRule = requireRule('Preflight global selector', rule => rule === baseRule);
const converter = [
  runConverter('whole-generated-css', css, 'reject', 'layer-statement'),
  runConverter('tailwind-flex-positive', classRules.get('flex').node.toString(), 'accept'),
  runConverter('spacing-token-negative', paddingRule.node.toString(), 'reject', 'unparsed:'),
  runConverter('color-token-negative', backgroundRule.node.toString(), 'reject', 'unparsed:'),
  runConverter('hover-selector-negative', hoverRule.node.toString(), 'reject', '단일 클래스 선택자'),
  runConverter('hover-media-negative', withMediaAncestors(hoverRule), 'reject', 'media:'),
  runConverter('responsive-media-negative', withMediaAncestors(responsiveRule), 'reject', 'media:'),
  runConverter('preflight-global-selector-negative', preflightRule.node.toString(), 'reject', '단일 클래스 선택자'),
];

const summary = {
  generator: { tailwindcss: '4.3.3', cli: '@tailwindcss/cli 4.3.3', lightningcss: lightningCssVersion },
  generatedCss: {
    bytes: Buffer.byteLength(css),
    lines: css.split(/\r?\n/).length - 1,
    sha256: generatedSha256,
    ruleCount: rules.length,
    layerOrder: layerStatements,
    layerBlocks: [...layerBlocks],
    atRuleCounts: Object.fromEntries([...new Set(atRules.map(a => a.name))].sort().map(name => [name, atRules.filter(a => a.name === name).length])),
    customProperties: variables,
    utilityRuleCount: utilitySelectors.length,
    utilitySelectors,
    selectorKinds,
    preflight: {
      globalReset: { selector: baseRule.selector, declarations: baseRule.declarations },
      headings: { selector: headingRule.selector, declarations: headingRule.declarations },
      lists: { selector: listRule.selector, declarations: listRule.declarations },
      imageDisplay: { selector: imageRule.selector, declarations: imageRule.declarations },
      imageSizing: { selector: imageSizingRule.selector, declarations: imageSizingRule.declarations },
      hidden: { selector: hiddenRule.selector, declarations: hiddenRule.declarations },
    },
    variants: { hover: mediaCondition(hoverRule), responsive: mediaCondition(responsiveRule) },
  },
  converter,
};
process.stdout.write(`${JSON.stringify(summary, null, 2)}\n`);
