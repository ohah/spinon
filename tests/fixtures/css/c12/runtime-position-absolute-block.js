const c12AbsoluteNodes = Object.create(null);
const c12AbsoluteStyleRules = [
  'html,body{margin:0;padding:0}',
  '#c12-root{display:block;position:static;box-sizing:border-box;width:100vw;height:100vh;margin:0;padding:0;background-color:#101827}',
  '.case-row{display:block;position:static;box-sizing:border-box;width:320px;height:70px;margin:0;padding:0;background-color:#18253a}',
  '.c12-owner{background-color:#243b5a}',
  '.c12-absolute{background-color:#38bdf8}',
];

function c12Node(parent, id, caseId, declarations = '', className = '') {
  const element = document.createElement('div');
  element.id = id;
  element.setAttribute('data-c12-node', '');
  element.setAttribute('data-c12-case', caseId);
  const runtimeClassNames = [
    className,
    declarations.includes('position:relative') ? 'c12-owner' : '',
    declarations.includes('position:absolute') ? 'c12-absolute' : '',
  ].filter(Boolean);
  if (runtimeClassNames.length) element.className = runtimeClassNames.join(' ');
  if (declarations) c12AbsoluteStyleRules.push(`#${id}{${declarations}}`);
  c12AbsoluteNodes[id] = element;
  parent.appendChild(element);
  return element;
}

function c12Case(root, id, caseId, declarations = '') {
  return c12Node(root, id, caseId, declarations || 'display:block;position:static;width:320px;height:70px;margin:0;padding:0');
}

const fixture = document.createElement('div');
fixture.id = 'c12-root';
fixture.setAttribute('data-c12-node', '');
fixture.setAttribute('data-c12-case', 'root');
c12AbsoluteNodes['c12-root'] = fixture;

const viewportCase = c12Case(fixture, 'viewport-row', 'viewport-origin');
c12Node(viewportCase, 'viewport-absolute', 'viewport-origin', 'position:absolute;left:17px;top:11px;width:30px;height:20px');

const edgeCase = c12Case(fixture, 'edge-row', 'padding-edge');
const edgeOwner = c12Node(edgeCase, 'edge-owner', 'padding-edge', 'position:relative;box-sizing:border-box;width:200px;height:64px;border:4px solid;padding:10px');
const edgeStatic = c12Node(edgeOwner, 'edge-static-wrapper', 'padding-edge', 'position:static;width:120px;height:32px');
c12Node(edgeStatic, 'edge-absolute', 'padding-edge', 'position:absolute;left:5px;top:7px;width:30px;height:12px');

const skipCase = c12Case(fixture, 'skip-row', 'static-ancestor-skip');
const skipOwner = c12Node(skipCase, 'skip-owner', 'static-ancestor-skip', 'position:relative;box-sizing:border-box;width:200px;height:58px;padding:8px;border:2px solid');
const skipWrapper = c12Node(skipOwner, 'skip-static-wrapper', 'static-ancestor-skip', 'position:static;width:120px;height:30px');
c12Node(skipWrapper, 'skip-absolute', 'static-ancestor-skip', 'position:absolute;left:6px;top:4px;width:24px;height:10px');

const nestedCase = c12Case(fixture, 'nested-row', 'nearest-nested-owner');
const nestedOuter = c12Node(nestedCase, 'nested-outer', 'nearest-nested-owner', 'position:relative;width:250px;height:60px;padding:5px;border:3px solid');
const nestedStatic = c12Node(nestedOuter, 'nested-static-wrapper', 'nearest-nested-owner', 'position:static;width:180px;height:45px');
const nestedInner = c12Node(nestedStatic, 'nested-inner', 'nearest-nested-owner', 'position:relative;left:12px;top:4px;width:80px;height:34px;padding:6px;border:2px solid');
c12Node(nestedInner, 'nested-absolute', 'nearest-nested-owner', 'position:absolute;left:10px;top:8px;width:20px;height:12px');

const flowCase = c12Case(fixture, 'flow-row', 'out-of-flow-auto-parent-size', 'display:block;position:relative;box-sizing:content-box;width:300px;height:auto;margin:0;padding:0');
c12Node(flowCase, 'flow-before', 'out-of-flow-auto-parent-size', 'display:block;width:40px;height:12px');
c12Node(flowCase, 'flow-absolute', 'out-of-flow-auto-parent-size', 'position:absolute;left:0;top:0;width:50px;height:16px');
c12Node(flowCase, 'flow-after', 'out-of-flow-auto-parent-size', 'display:block;width:40px;height:12px');

const oppositeCase = c12Case(fixture, 'opposite-row', 'right-bottom-insets');
const oppositeOwner = c12Node(oppositeCase, 'opposite-owner', 'right-bottom-insets', 'position:relative;box-sizing:border-box;width:200px;height:58px;padding:8px;border:2px solid');
c12Node(oppositeOwner, 'opposite-absolute', 'right-bottom-insets', 'position:absolute;right:9px;bottom:6px;width:31px;height:13px');

const percentCase = c12Case(fixture, 'percent-row', 'percentage-axis-bases');
const percentOwner = c12Node(percentCase, 'percent-owner', 'percentage-axis-bases', 'position:relative;box-sizing:border-box;width:200px;height:120px;padding:10px;border:4px solid');
c12Node(percentOwner, 'percent-absolute', 'percentage-axis-bases', 'position:absolute;inset:10% 5% 15% 25%;width:20px;height:10px');

const mathCase = c12Case(fixture, 'math-row', 'calc-insets');
const mathOwner = c12Node(mathCase, 'math-owner', 'calc-insets', 'position:relative;box-sizing:border-box;width:200px;height:120px;padding:10px;border:4px solid');
c12Node(mathOwner, 'math-absolute', 'calc-insets', 'position:absolute;left:calc(10% + 4px);top:calc(10% + 2px);width:20px;height:10px');

const stretchCase = c12Case(fixture, 'stretch-row', 'auto-size-stretch');
const stretchOwner = c12Node(stretchCase, 'stretch-owner', 'auto-size-stretch', 'position:relative;box-sizing:border-box;width:200px;height:80px;padding:10px;border:4px solid');
c12Node(stretchOwner, 'stretch-absolute', 'auto-size-stretch', 'position:absolute;left:8px;right:12px;top:5px;bottom:7px;width:auto;height:auto');

const overCase = c12Case(fixture, 'over-row', 'overconstrained-ltr');
const overOwner = c12Node(overCase, 'over-owner', 'overconstrained-ltr', 'position:relative;direction:ltr;box-sizing:border-box;width:200px;height:58px;padding:8px;border:2px solid');
c12Node(overOwner, 'over-absolute', 'overconstrained-ltr', 'position:absolute;left:7px;right:11px;top:4px;width:30px;height:12px');

const marginCase = c12Case(fixture, 'margin-row', 'auto-margins');
const marginOwner = c12Node(marginCase, 'margin-owner', 'auto-margins', 'position:relative;box-sizing:border-box;width:200px;height:58px;padding:8px;border:2px solid');
c12Node(marginOwner, 'margin-absolute', 'auto-margins', 'position:absolute;left:0;right:0;top:5px;width:40px;height:12px;margin-left:auto;margin-right:auto');

const minmaxCase = c12Case(fixture, 'minmax-row', 'min-max-with-stretch');
const minmaxOwner = c12Node(minmaxCase, 'minmax-owner', 'min-max-with-stretch', 'position:relative;box-sizing:border-box;width:200px;height:58px;padding:0;border:0');
c12Node(minmaxOwner, 'minmax-absolute', 'min-max-with-stretch', 'position:absolute;left:10px;right:10px;top:5px;width:auto;height:12px;min-width:80px;max-width:130px');

const sizingCase = c12Case(fixture, 'sizing-row', 'box-sizing-border-padding');
const sizingOwner = c12Node(sizingCase, 'sizing-owner', 'box-sizing-border-padding', 'position:relative;box-sizing:border-box;width:200px;height:58px');
c12Node(sizingOwner, 'border-box-absolute', 'box-sizing-border-padding', 'position:absolute;left:4px;top:4px;box-sizing:border-box;width:60px;height:30px;padding:8px;border:2px solid');
c12Node(sizingOwner, 'content-box-absolute', 'box-sizing-border-padding', 'position:absolute;left:80px;top:4px;box-sizing:content-box;width:60px;height:30px;padding:8px;border:2px solid');

const staticCase = c12Case(fixture, 'static-row', 'auto-insets-static-position', 'display:block;position:relative;box-sizing:border-box;width:200px;height:auto');
c12Node(staticCase, 'static-before', 'auto-insets-static-position', 'display:block;width:36px;height:12px');
c12Node(staticCase, 'static-absolute', 'auto-insets-static-position', 'position:absolute;box-sizing:border-box;width:40px;height:10px');
c12Node(staticCase, 'static-after', 'auto-insets-static-position', 'display:block;width:32px;height:14px');

const inlineCase = c12Case(fixture, 'inline-row', 'absolute-inline-blockification');
const inlineOwner = c12Node(inlineCase, 'inline-owner', 'absolute-inline-blockification', 'position:relative;box-sizing:border-box;width:180px;height:48px');
c12Node(inlineOwner, 'inline-absolute', 'absolute-inline-blockification', 'display:inline;position:absolute;left:3px;top:4px;width:30px;height:12px');

const bfcCase = c12Case(fixture, 'bfc-row', 'independent-block-formatting-context');
const bfcOwner = c12Node(bfcCase, 'bfc-owner', 'independent-block-formatting-context', 'position:relative;box-sizing:border-box;width:180px;height:48px');
const bfcAbsolute = c12Node(bfcOwner, 'bfc-absolute', 'independent-block-formatting-context', 'display:block;position:absolute;left:3px;top:4px;box-sizing:border-box;width:80px;height:auto;margin:0;padding:0;border:0');
c12Node(bfcAbsolute, 'bfc-child', 'independent-block-formatting-context', 'display:block;width:30px;height:12px;margin-top:10px');

const hiddenCase = c12Case(fixture, 'hidden-row', 'display-none-subtree');
const hiddenParent = c12Node(hiddenCase, 'hidden-parent', 'display-none-subtree', 'display:none;position:relative;width:80px;height:20px');
c12Node(hiddenParent, 'hidden-absolute', 'display-none-subtree', 'position:absolute;left:4px;top:3px;width:20px;height:10px');
c12Node(hiddenCase, 'hidden-after', 'display-none-subtree', 'display:block;width:30px;height:12px');

const absoluteOwnerCase = c12Case(fixture, 'abs-owner-row', 'absolute-ancestor-owner');
const absoluteOuter = c12Node(absoluteOwnerCase, 'abs-outer', 'absolute-ancestor-owner', 'position:relative;box-sizing:border-box;width:220px;height:60px;padding:6px;border:2px solid');
const absoluteParent = c12Node(absoluteOuter, 'abs-parent', 'absolute-ancestor-owner', 'position:absolute;left:15px;top:8px;box-sizing:border-box;width:70px;height:42px;padding:5px;border:2px solid');
c12Node(absoluteParent, 'abs-nested', 'absolute-ancestor-owner', 'position:absolute;left:7px;top:5px;width:18px;height:9px');

const edgeMarginCase = c12Case(fixture, 'edge-margin-row', 'margin-and-negative-inset');
const edgeMarginOwner = c12Node(edgeMarginCase, 'edge-margin-owner', 'margin-and-negative-inset', 'position:relative;box-sizing:border-box;width:200px;height:58px;padding:8px;border:2px solid');
c12Node(edgeMarginOwner, 'edge-margin-absolute', 'margin-and-negative-inset', 'position:absolute;left:-4px;top:-3px;width:32px;height:12px;margin-left:5px;margin-top:2px');

const cascadeCase = c12Case(fixture, 'cascade-row', 'inset-shorthand-cascade');
const cascadeOwner = c12Node(cascadeCase, 'cascade-owner', 'inset-shorthand-cascade', 'position:relative;box-sizing:border-box;width:200px;height:58px;padding:8px;border:2px solid');
c12Node(cascadeOwner, 'cascade-absolute', 'inset-shorthand-cascade', 'position:absolute;width:30px;height:12px', 'cascade-target');
c12AbsoluteStyleRules.push('@layer c12-base,c12-override;@layer c12-base{#cascade-absolute{inset:5px 7px 9px 11px}}@layer c12-override{.cascade-target{left:15px}}#cascade-absolute{left:17px!important}');

const ratioCase = c12Case(fixture, 'ratio-row', 'aspect-ratio-with-auto-height');
const ratioOwner = c12Node(ratioCase, 'ratio-owner', 'aspect-ratio-with-auto-height', 'position:relative;box-sizing:border-box;width:200px;height:58px');
c12Node(ratioOwner, 'ratio-absolute', 'aspect-ratio-with-auto-height', 'position:absolute;left:5px;top:4px;width:40px;height:auto;aspect-ratio:2/1');

const singleAutoCase = c12Case(fixture, 'single-auto-row', 'one-auto-inset');
const singleAutoOwner = c12Node(singleAutoCase, 'single-auto-owner', 'one-auto-inset', 'position:relative;box-sizing:border-box;width:200px;height:58px;padding:8px;border:2px solid');
c12Node(singleAutoOwner, 'single-auto-absolute', 'one-auto-inset', 'position:absolute;right:13px;top:6px;width:24px;height:10px');

const staticDiffCase = c12Case(fixture, 'static-diff-row', 'static-position-owner-differs');
const staticDiffOwner = c12Node(staticDiffCase, 'static-diff-owner', 'static-position-owner-differs', 'position:relative;box-sizing:border-box;width:220px;height:70px;padding:6px;border:2px solid');
const staticDiffWrapper = c12Node(staticDiffOwner, 'static-diff-wrapper', 'static-position-owner-differs', 'position:static;width:120px;height:50px');
c12Node(staticDiffWrapper, 'static-diff-before', 'static-position-owner-differs', 'display:block;width:40px;height:12px');
c12Node(staticDiffWrapper, 'static-diff-absolute', 'static-position-owner-differs', 'position:absolute;width:40px;height:10px;margin-left:3px;margin-top:2px');
c12Node(staticDiffWrapper, 'static-diff-after', 'static-position-owner-differs', 'display:block;width:30px;height:14px');

const rootStyle = document.createElement('style');
rootStyle.type = 'text/css';
rootStyle.appendChild(document.createTextNode(c12AbsoluteStyleRules.join('\n')));
fixture.insertBefore(rootStyle, fixture.firstChild);
const mountPoint = document.body || document;
mountPoint.appendChild(fixture);

globalThis.spinonC12AbsoluteFixture = {
  nodes: c12AbsoluteNodes,
  sourceOrder: Object.keys(c12AbsoluteNodes),
};
