import { createHash } from "node:crypto";
import { readFileSync, statSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";
import { spawnSync } from "node:child_process";

const runDir = resolve(process.argv[2] ?? "");
const processor = process.argv[3] ?? process.env.SPINON_TRACE_PROCESSOR ?? "trace_processor";
if (!process.argv[2]) {
  console.error("사용법: bun tools/benchmark/validate-android-trace-overhead-run.mjs <실행 디렉터리> [trace_processor 경로]");
  process.exit(2);
}

function readKeyValues(path) {
  return Object.fromEntries(readFileSync(path, "utf8").split(/\r?\n/)
    .filter((line) => line.includes("="))
    .map((line) => {
      const separator = line.indexOf("=");
      return [line.slice(0, separator), line.slice(separator + 1)];
    }));
}

function requireCondition(condition, message) {
  if (!condition) throw new Error(message);
}

function run(command, args) {
  const result = spawnSync(command, args, { encoding: "utf8" });
  if (result.error) throw result.error;
  if (result.status !== 0) {
    throw new Error(`${command} 실패 (${result.status}): ${result.stderr}`);
  }
  return result.stdout.trim();
}

function csvFields(line) {
  return [...line.matchAll(/(?:^|,)(?:"((?:[^"]|"")*)"|([^,]*))/g)]
    .map((match) => (match[1] ?? match[2] ?? "").replaceAll('""', '"'));
}

function parseCsv(output) {
  const rows = output.split(/\r?\n/).filter((line) => line.trim());
  const headerIndex = rows.findIndex((line) => line.startsWith('"check_name"'));
  requireCondition(headerIndex >= 0, `Trace Processor SQL 결과가 없습니다: ${output}`);
  const headers = csvFields(rows[headerIndex]);
  return rows.slice(headerIndex + 1).map((line) => {
    const values = csvFields(line);
    return Object.fromEntries(headers.map((header, index) => [header, values[index] ?? ""]));
  });
}

const metadata = readKeyValues(resolve(runDir, "metadata.txt"));
requireCondition(metadata.scenario === "spinon-event", "spinon-event 실행이 아닙니다");
requireCondition(metadata.perfetto_enabled === "true", "Perfetto trace-on 실행이 아닙니다");
requireCondition(metadata.device_kind === "에뮬레이터" && metadata.android_api === "36", "계획된 Android API 36 에뮬레이터가 아닙니다");
requireCondition(/^\d+$/.test(metadata.app_pid ?? ""), "앱 PID metadata가 없습니다");
const expectedInputs = Number(metadata.input_count);
requireCondition(Number.isSafeInteger(expectedInputs) && expectedInputs > 0, "입력 수 metadata가 올바르지 않습니다");

const tracePath = resolve(runDir, "frame-attribution.pftrace");
const traceBytes = statSync(tracePath).size;
requireCondition(traceBytes > 0, "Perfetto 원본 trace가 비어 있습니다");
const runnerLog = readFileSync(resolve(runDir, "perfetto-runner.txt"), "utf8");
const bytesWritten = runnerLog.match(/Wrote (\d+) bytes into \/data\/misc\/perfetto-traces\//)?.[1];
requireCondition(bytesWritten && Number(bytesWritten) === traceBytes, "Perfetto runner 기록 크기와 내려받은 trace 크기가 다릅니다");
requireCondition(runnerLog.includes("Connected to the Perfetto traced service"), "Perfetto service 연결 증거가 없습니다");

const sql = `
SELECT 'nonzero_data_loss' AS check_name, CAST(COUNT(*) AS TEXT) AS value
FROM stats WHERE severity = 'data_loss' AND CAST(value AS REAL) != 0
UNION ALL
SELECT 'nonzero_error_stats', CAST(COUNT(*) AS TEXT)
FROM stats WHERE severity = 'error' AND CAST(value AS REAL) != 0
UNION ALL
SELECT 'ftrace_setup_errors', CAST(COALESCE(MAX(CAST(value AS REAL)), 0) AS TEXT)
FROM stats WHERE name = 'ftrace_setup_errors'
UNION ALL
SELECT 'runtime_dispatch_markers', CAST(COUNT(*) AS TEXT)
FROM slice WHERE name = 'SpinonR05:runtime-dispatch'
UNION ALL
SELECT 'v8_handler_markers', CAST(COUNT(*) AS TEXT)
FROM slice WHERE name = 'SpinonR05:v8-handler-call'
UNION ALL
SELECT 'runtime_completion_markers', CAST(COUNT(*) AS TEXT)
FROM slice WHERE name = 'SpinonR05:runtime-main-thread-completion'
UNION ALL
SELECT 'runtime_dispatch_counter', CAST(COALESCE(MAX(c.value), 0) AS TEXT)
FROM counter c JOIN counter_track ct ON c.track_id = ct.id
WHERE ct.name = 'SpinonR05RuntimeDispatchCount'
UNION ALL
SELECT 'spinon_input_counter', CAST(COALESCE(MAX(c.value), 0) AS TEXT)
FROM counter c JOIN counter_track ct ON c.track_id = ct.id
WHERE ct.name = 'SpinonR05SpinonInputCount'
UNION ALL
SELECT 'display_frame_tokens', CAST(COUNT(DISTINCT display_frame_token) AS TEXT)
FROM actual_frame_timeline_slice
UNION ALL
SELECT 'surface_frame_tokens', CAST(COUNT(DISTINCT surface_frame_token) AS TEXT)
FROM actual_frame_timeline_slice`;

const version = run(processor, ["--version"]).split(/\r?\n/)[0];
const queryOutput = run(processor, ["query", tracePath, sql]);
const checks = Object.fromEntries(parseCsv(queryOutput).map(({ check_name, value }) => [check_name, Number(value)]));
const expected = {
  nonzero_data_loss: 0,
  nonzero_error_stats: 0,
  ftrace_setup_errors: 0,
  runtime_dispatch_markers: expectedInputs,
  v8_handler_markers: expectedInputs,
  runtime_completion_markers: expectedInputs,
  runtime_dispatch_counter: expectedInputs,
  spinon_input_counter: expectedInputs,
};
for (const [name, value] of Object.entries(expected)) {
  requireCondition(checks[name] === value, `${name} 불일치: ${checks[name]} (기대 ${value})`);
}
requireCondition(checks.display_frame_tokens > 0, "FrameTimeline display frame token이 없습니다");
requireCondition(checks.surface_frame_tokens > 0, "FrameTimeline surface frame token이 없습니다");

const result = {
  status: "pass",
  processor_version: version,
  trace_bytes: traceBytes,
  trace_sha256: createHash("sha256").update(readFileSync(tracePath)).digest("hex"),
  app_pid: Number(metadata.app_pid),
  checks,
};
writeFileSync(resolve(runDir, "trace-validation.json"), `${JSON.stringify(result, null, 2)}\n`);
console.log(`검증 통과 · Android Perfetto · packet/data loss 0 · 앱 표식·counter ${expectedInputs}/${expectedInputs} · FrameTimeline ${checks.display_frame_tokens} display / ${checks.surface_frame_tokens} surface · ${version}`);
