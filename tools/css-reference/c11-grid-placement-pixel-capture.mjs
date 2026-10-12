export function hasRunSummary(log, marker, runID, status = 'captured') {
  return log.split(/\r?\n/).some((line) =>
    line.includes(`${marker} `) && line.includes(`run_id=${runID}`)
      && line.includes(`status=${status}`));
}
