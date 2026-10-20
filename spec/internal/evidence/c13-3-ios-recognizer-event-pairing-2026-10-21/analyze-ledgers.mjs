#!/usr/bin/env node
import { readdir, readFile, writeFile } from 'node:fs/promises';
import { basename, dirname, join, resolve } from 'node:path';

const [directoryArgument, scrollRunID, directTouchRunID] = process.argv.slice(2);
if (!directoryArgument || !scrollRunID || !directTouchRunID) {
  console.error('사용법: node analyze-ledgers.mjs <ledger-directory> <scroll-run-id> <direct-touch-run-id>');
  process.exit(2);
}
const runIDs = { scroll: scrollRunID, 'direct-touch': directTouchRunID };
for (const [mode, runID] of Object.entries(runIDs)) {
  if (!/^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/i.test(runID)) {
    console.error(`${mode}: run ID must be a UUID`);
    process.exit(2);
  }
}

const directory = resolve(directoryArgument);
const expectedNames = {
  scroll: ['continuous', 'discrete', 'union'],
  'direct-touch': ['direct-touch'],
};
const files = (await readdir(directory))
  .filter(name => /^c133-recognizer-ledger-.*\.json$/.test(name))
  .sort();
const errors = [];
const ledgers = [];

for (const mode of Object.keys(expectedNames)) {
  const runID = runIDs[mode];
  const expectedLaunchIDs = new Set([1, 2, 3].map(index => `${mode}-${index}`));
  const modeFiles = files.filter(name => name.includes(`-ledger-${mode}-${runID}-`));
  if (modeFiles.length !== 3) {
    errors.push(`${mode}: expected 3 launch files, found ${modeFiles.length}`);
  }
  const foundLaunchIDs = new Set();

  for (const file of modeFiles) {
    const path = join(directory, file);
    let ledger;
    try {
      ledger = JSON.parse(await readFile(path, 'utf8'));
    } catch (error) {
      errors.push(`${file}: JSON parse failed (${error.message})`);
      continue;
    }

    const rowErrors = [];
    const rows = Array.isArray(ledger.eventRows) ? ledger.eventRows : [];
    const recognizerNames = expectedNames[mode];
    if (ledger.schema !== 'spinon-c133-recognizer-event-pairing-v1') rowErrors.push('schema mismatch');
    if (ledger.mode !== mode || ledger.runID !== runID) rowErrors.push('mode or runID mismatch');
    if (ledger.applicationClass !== 'C133ProbeApplication') rowErrors.push('unexpected application class');
    if (!expectedLaunchIDs.has(ledger.launchID)) rowErrors.push('unexpected launchID');
    foundLaunchIDs.add(ledger.launchID);
    if (ledger.ready !== true) rowErrors.push('capture was not ready');
    if (ledger.overflow !== false) rowErrors.push('ledger overflow');
    if (ledger.mainThreadViolation !== false) rowErrors.push('main-thread violation');
    if (ledger.incompleteRows !== 0) rowErrors.push(`incompleteRows=${ledger.incompleteRows}`);
    if (!Array.isArray(ledger.eventRows) || rows.length === 0 || rows.length > 4_096) {
      rowErrors.push(`invalid row count ${rows?.length ?? 'missing'}`);
    }
    if (JSON.stringify(ledger.expectedRecognizerNames) !== JSON.stringify([...recognizerNames].sort())) {
      rowErrors.push('expected recognizer names mismatch');
    }

    const eventTypeCounts = {};
    const callbackRowsByEventType = {};
    const callbackDeltaTotalsByEventType = {};
    const callbackRows = Object.fromEntries(recognizerNames.map(name => [name, 0]));
    const callbackDeltaTotals = Object.fromEntries(recognizerNames.map(name => [name, 0]));
    const sameStateCallbackRows = Object.fromEntries(recognizerNames.map(name => [name, 0]));
    const stateChangedWithoutCallbackRows = Object.fromEntries(recognizerNames.map(name => [name, 0]));
    const previousCallbacks = Object.fromEntries(recognizerNames.map(name => [name, -1]));
    let mainThreadRows = 0;
    let completeSnapshotRows = 0;
    let monotonicTimeRegressions = 0;
    let lastReceiveTime = -1;
    let unionInvariantViolations = 0;

    for (let index = 0; index < (Array.isArray(rows) ? rows.length : 0); index += 1) {
      const row = rows[index];
      if (row.sequence !== index + 1) rowErrors.push(`sequence gap at row ${index + 1}: ${row.sequence}`);
      if (!Number.isSafeInteger(row.eventType)) rowErrors.push(`invalid eventType at sequence ${row.sequence}`);
      if (!Number.isSafeInteger(row.receiveMonotonicNs)) rowErrors.push(`invalid receive time at sequence ${row.sequence}`);
      if (row.receiveMonotonicNs < lastReceiveTime) monotonicTimeRegressions += 1;
      lastReceiveTime = row.receiveMonotonicNs;
      eventTypeCounts[row.eventType] = (eventTypeCounts[row.eventType] ?? 0) + 1;
      if (row.mainThread === true) mainThreadRows += 1;
      else rowErrors.push(`non-main event row at sequence ${row.sequence}`);
      if (!Number.isSafeInteger(row.superDispatchElapsedNs) || row.superDispatchElapsedNs < 0) {
        rowErrors.push(`invalid super duration at sequence ${row.sequence}`);
      }

      const preNames = Object.keys(row.pre ?? {}).sort();
      const postNames = Object.keys(row.post ?? {}).sort();
      if (JSON.stringify(preNames) !== JSON.stringify([...recognizerNames].sort())
          || JSON.stringify(postNames) !== JSON.stringify([...recognizerNames].sort())) {
        rowErrors.push(`snapshot names incomplete at sequence ${row.sequence}`);
        continue;
      }
      completeSnapshotRows += 1;

      let rowCallbackDeltaTotal = 0;
      for (const name of recognizerNames) {
        const before = row.pre[name];
        const after = row.post[name];
        if (!Number.isInteger(before.state) || !Number.isInteger(after.state)
            || before.state < 0 || after.state < 0) {
          rowErrors.push(`invalid ${name} state at sequence ${row.sequence}`);
        }
        if (!Number.isInteger(before.callbacks) || !Number.isInteger(after.callbacks)
            || before.callbacks < 0 || after.callbacks < before.callbacks) {
          rowErrors.push(`invalid ${name} callback count at sequence ${row.sequence}`);
          continue;
        }
        if (previousCallbacks[name] > after.callbacks) {
          rowErrors.push(`${name} callback counter regressed at sequence ${row.sequence}`);
        }
        previousCallbacks[name] = Math.max(previousCallbacks[name], after.callbacks);
        const delta = after.callbacks - before.callbacks;
        rowCallbackDeltaTotal += delta;
        callbackDeltaTotals[name] += delta;
        if (delta > 0) callbackRows[name] += 1;
        if (delta > 0 && after.state === before.state) sameStateCallbackRows[name] += 1;
        if (delta === 0 && after.state !== before.state) stateChangedWithoutCallbackRows[name] += 1;
      }
      callbackDeltaTotalsByEventType[row.eventType] = (callbackDeltaTotalsByEventType[row.eventType] ?? 0)
        + rowCallbackDeltaTotal;
      if (rowCallbackDeltaTotal > 0) {
        callbackRowsByEventType[row.eventType] = (callbackRowsByEventType[row.eventType] ?? 0) + 1;
      }

      if (mode === 'scroll') {
        const continuousDelta = row.post.continuous.callbacks - row.pre.continuous.callbacks;
        const discreteDelta = row.post.discrete.callbacks - row.pre.discrete.callbacks;
        const unionDelta = row.post.union.callbacks - row.pre.union.callbacks;
        if ((continuousDelta > 0 && continuousDelta !== unionDelta)
            || (discreteDelta > 0 && unionDelta === 0)) {
          unionInvariantViolations += 1;
        }
      }
    }

    if (monotonicTimeRegressions > 0) rowErrors.push(`monotonic time regressions=${monotonicTimeRegressions}`);
    if (mainThreadRows !== rows.length) rowErrors.push(`main-thread rows=${mainThreadRows}/${rows.length}`);
    if (completeSnapshotRows !== rows.length) rowErrors.push(`complete snapshots=${completeSnapshotRows}/${rows.length}`);
    if (mode === 'scroll' && unionInvariantViolations > 0) {
      rowErrors.push(`continuous/union invariant violations=${unionInvariantViolations}`);
    }
    if (mode === 'scroll' && (eventTypeCounts[10] ?? 0) === 0) {
      rowErrors.push('no UIEventType.scroll rows');
    }
    if (mode === 'direct-touch' && (callbackDeltaTotals['direct-touch'] ?? 0) === 0) {
      rowErrors.push('direct-touch callback positive control was not observed in event rows');
    }

    if (rowErrors.length > 0) errors.push(`${file}: ${rowErrors.join('; ')}`);
    ledgers.push({
      file: basename(file),
      mode,
      runID: ledger.runID,
      launchID: ledger.launchID,
      rows: rows.length,
      eventTypeCounts,
      callbackRowsByEventType,
      callbackDeltaTotalsByEventType,
      mainThreadRows,
      completeSnapshotRows,
      callbackRows,
      callbackDeltaTotals,
      sameStateCallbackRows,
      stateChangedWithoutCallbackRows,
      unionInvariantViolations,
      valid: rowErrors.length === 0,
      errors: rowErrors,
    });
  }

  for (const launchID of expectedLaunchIDs) {
    if (!foundLaunchIDs.has(launchID)) errors.push(`${mode}: missing launch ${launchID}`);
  }
}

if (files.length !== 6) errors.push(`expected exactly 6 ledgers, found ${files.length}`);
const report = {
  schema: 'spinon-c133-recognizer-event-pairing-analysis-v1',
  runIDs,
  inputFiles: files,
  ledgerCount: ledgers.length,
  valid: errors.length === 0 && ledgers.length === 6,
  errors,
  totals: {
    rows: ledgers.reduce((total, item) => total + item.rows, 0),
    eventTypes: ledgers.reduce((total, item) => {
      for (const [type, count] of Object.entries(item.eventTypeCounts)) {
        total[type] = (total[type] ?? 0) + count;
      }
      return total;
    }, {}),
  },
  ledgers,
};
const output = join(dirname(directory), 'analysis.json');
await writeFile(output, `${JSON.stringify(report, null, 2)}\n`);
console.log(JSON.stringify(report, null, 2));
if (!report.valid) process.exitCode = 1;
