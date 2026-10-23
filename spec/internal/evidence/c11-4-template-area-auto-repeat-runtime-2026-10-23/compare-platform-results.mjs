import assert from 'node:assert/strict';
import { readFile, writeFile } from 'node:fs/promises';

const [androidPath, iosPath, outputPath] = process.argv.slice(2);
assert.ok(androidPath && iosPath && outputPath, 'usage: compare-platform-results.mjs android.json ios.json output.json');

const [android, ios] = await Promise.all([
  readFile(androidPath, 'utf8').then(JSON.parse),
  readFile(iosPath, 'utf8').then(JSON.parse),
]);
const normalizeFrame = ({ order, node, x, y, width, height }) => ({
  order, node, x, y, width, height,
});
const byScene = (result) => new Map(result.observations.map((observation) => [
  observation.sceneId,
  observation,
]));
const androidScenes = byScene(android);
const iosScenes = byScene(ios);
assert.equal(android.sceneCount, 13);
assert.equal(ios.sceneCount, 13);
assert.deepEqual([...androidScenes.keys()], [...iosScenes.keys()]);

const comparisons = [];
for (const [sceneId, androidScene] of androidScenes) {
  const iosScene = iosScenes.get(sceneId);
  assert.equal(androidScene.drawPresented, true, `${sceneId}: Android draw/present`);
  assert.equal(iosScene.drawPresented, true, `${sceneId}: iOS draw/present`);
  const androidFrames = androidScene.observedNodeFrames.map(normalizeFrame);
  const iosFrames = iosScene.observedNodeFrames.map(normalizeFrame);
  assert.deepEqual(androidFrames, iosFrames, `${sceneId}: node frames`);
  assert.deepEqual(androidScene.tracks, iosScene.tracks, `${sceneId}: used tracks`);
  const tracks = Object.values(androidScene.tracks).flat();
  comparisons.push({
    sceneId,
    nodeCount: androidFrames.length,
    trackCount: tracks.length,
    collapsedTrackCount: tracks.filter((track) => track.collapsed).length,
    framesEqual: true,
    tracksEqual: true,
    androidDrawPresented: androidScene.drawPresented,
    iosDrawPresented: iosScene.drawPresented,
  });
}

const summary = {
  schemaVersion: 1,
  androidRunPrefix: android.runPrefix,
  iosRunPrefix: ios.runPrefix,
  sceneCount: comparisons.length,
  exactFrameAndTrackMatches: comparisons.length,
  comparisons,
};
await writeFile(outputPath, `${JSON.stringify(summary, null, 2)}\n`);
process.stdout.write(`${JSON.stringify({ outputPath, sceneCount: summary.sceneCount, exactFrameAndTrackMatches: summary.exactFrameAndTrackMatches })}\n`);
