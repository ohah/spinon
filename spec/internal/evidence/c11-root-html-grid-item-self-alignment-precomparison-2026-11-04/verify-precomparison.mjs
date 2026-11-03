import fs from 'node:fs';
const dir = new URL('.', import.meta.url);
const actual = JSON.parse(fs.readFileSync(new URL('chrome-observation.json', dir), 'utf8'));
const expected = JSON.parse(fs.readFileSync(new URL('expected.json', dir), 'utf8'));
const mismatches = [];
for (let i = 0; i < expected.scenes.length; i++) {
  const e = expected.scenes[i];
  const a = actual[i];
  if (!a || a.id !== e.id ||
      a.parent.alignItems !== e.parentAlignment.alignItems ||
      a.parent.justifyItems !== e.parentAlignment.justifyItems ||
      a.self.alignSelf !== e.alignSelf || a.self.justifySelf !== e.justifySelf ||
      a.item.x !== e.expectedItemFrame.x || a.item.y !== e.expectedItemFrame.y ||
      a.item.width !== e.expectedItemFrame.width || a.item.height !== e.expectedItemFrame.height) {
    mismatches.push({index: i, id: e.id, expected: e, actual: a});
  }
}
if (actual.length !== expected.scenes.length) {
  mismatches.push({expectedSceneCount: expected.scenes.length, actualSceneCount: actual.length});
}
const result = {sceneCount: expected.scenes.length, mismatches, pass: mismatches.length === 0};
process.stdout.write(JSON.stringify(result, null, 2) + '\n');
if (mismatches.length !== 0) process.exitCode = 1;
