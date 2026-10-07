#!/usr/bin/env bun

import { readdirSync, readFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { spawnSync } from "node:child_process";

const matrixPath = process.argv[2];
if (!matrixPath) {
  console.error(
    "사용법: SPINON_TRACE_PROCESSOR=<경로> bun tools/benchmark/summarize-android-ui-runtime-attribution.mjs <matrix 디렉터리>",
  );
  process.exit(2);
}

const processor = process.env.SPINON_TRACE_PROCESSOR ?? "trace_processor";
const matrix = resolve(matrixPath);

function run(command, args) {
  const result = spawnSync(command, args, { encoding: "utf8" });
  if (result.error) throw result.error;
  if (result.status !== 0) {
    throw new Error(`${command} 실패 (${result.status}): ${result.stderr}`);
  }
  return result.stdout.trim();
}

function parseCsv(output) {
  const lines = output.split(/\r?\n/).filter(Boolean);
  const headerIndex = lines.findIndex((line) => line.startsWith('"'));
  if (headerIndex < 0) {
    throw new Error(`CSV 결과를 읽지 못했습니다: ${output}`);
  }
  const fields = (line) =>
    [...line.matchAll(/(?:^|,)(?:"((?:[^"]|"")*)"|([^,]*))/g)].map((match) =>
      (match[1] ?? match[2] ?? "").replaceAll('""', '"'),
    );
  const headers = fields(lines[headerIndex]);
  if (headerIndex + 1 >= lines.length) return [];
  return lines.slice(headerIndex + 1).map((line) => {
    const values = fields(line);
    return Object.fromEntries(headers.map((header, index) => [header, values[index] ?? ""]));
  });
}

function query(trace, sql) {
  return parseCsv(run(processor, ["query", trace, sql]));
}

function readKeyValues(path) {
  const values = {};
  for (const line of readFileSync(path, "utf8").split(/\r?\n/)) {
    const match = line.match(/^([a-z_]+)=(.*)$/);
    if (match) values[match[1]] = match[2];
  }
  return values;
}

function numeric(value) {
  const parsed = Number(value);
  return Number.isFinite(parsed) ? parsed : 0;
}

function percentile(values, percentileValue) {
  if (values.length === 0) return 0;
  const sorted = [...values].sort((left, right) => left - right);
  return sorted[Math.max(0, Math.ceil(percentileValue * sorted.length) - 1)];
}

function distribution(values) {
  return {
    count: values.length,
    p50: percentile(values, 0.5),
    p95: percentile(values, 0.95),
    max: values.length ? Math.max(...values) : 0,
  };
}

function formatDistribution(values, divisor = 1) {
  const stats = distribution(values);
  return `${stats.count}회; p50=${(stats.p50 / divisor).toFixed(3)}, p95=${(stats.p95 / divisor).toFixed(3)}, 최대=${(stats.max / divisor).toFixed(3)}`;
}

function slowCount(values, threshold) {
  return values.filter((value) => value > threshold).length;
}

const processorVersion = run(processor, ["--version"]).split("\n")[0];
const conditions = ["spinon-ui-only", "spinon-event"];
const schedulePath = join(matrix, "schedule.txt");
const scheduleText = readFileSync(schedulePath, "utf8");
const scheduleMetadata = readKeyValues(schedulePath);
const expectedInputCount = Number(scheduleMetadata.events_per_condition);
if (!Number.isSafeInteger(expectedInputCount) || expectedInputCount < 1) {
  throw new Error("schedule.txt에 올바른 events_per_condition 값이 없습니다");
}
const expectedRepeats = Number(scheduleMetadata.repeats_per_condition);
const scheduledRuns = scheduleText.split(/\r?\n/).flatMap((line) => {
  const match = line.match(/^(\d+)\s+(spinon-ui-only|spinon-event)$/);
  return match ? [{ round: Number(match[1]), scenario: match[2] }] : [];
});
const scheduledPairs = new Map();
for (const entry of scheduledRuns) {
  const scenarios = scheduledPairs.get(entry.round) ?? [];
  scenarios.push(entry.scenario);
  scheduledPairs.set(entry.round, scenarios);
}
const scheduleOrderCounts = {
  "spinon-ui-only→spinon-event": scheduledRuns.filter((entry, index) =>
    entry.scenario === "spinon-ui-only" && scheduledRuns[index + 1]?.round === entry.round
      && scheduledRuns[index + 1]?.scenario === "spinon-event").length,
  "spinon-event→spinon-ui-only": scheduledRuns.filter((entry, index) =>
    entry.scenario === "spinon-event" && scheduledRuns[index + 1]?.round === entry.round
      && scheduledRuns[index + 1]?.scenario === "spinon-ui-only").length,
};
const scheduleProblems = [];
if (!Number.isSafeInteger(expectedRepeats) || expectedRepeats < 1) {
  scheduleProblems.push("반복 횟수 metadata 없음");
} else if (scheduledRuns.length !== expectedRepeats * 2) {
  scheduleProblems.push(`schedule 입력 ${scheduledRuns.length}개, 기대 ${expectedRepeats * 2}개`);
}
for (let round = 1; round <= expectedRepeats; round++) {
  const pair = scheduledPairs.get(round) ?? [];
  if (pair.length !== 2 || new Set(pair).size !== 2) {
    scheduleProblems.push(`${round}회차 조건 쌍 누락/중복`);
  }
}
if (Math.abs(scheduleOrderCounts["spinon-ui-only→spinon-event"] - scheduleOrderCounts["spinon-event→spinon-ui-only"]) > 1) {
  scheduleProblems.push("조건 순서가 균형화되지 않음");
}
if (scheduleMetadata.condition_order === "balanced-alternating") {
  const firstCondition = scheduleMetadata.first_condition;
  if (!conditions.includes(firstCondition)) {
    scheduleProblems.push("balanced-alternating의 first_condition 값이 올바르지 않음");
  } else {
    const secondCondition = conditions.find((condition) => condition !== firstCondition);
    for (let round = 1; round <= expectedRepeats; round++) {
      const expectedFirst = round % 2 === 1 ? firstCondition : secondCondition;
      if (scheduledPairs.get(round)?.[0] !== expectedFirst) {
        scheduleProblems.push(`${round}회차 조건 순서가 교대 규칙과 다름`);
      }
    }
  }
}
const runDirectories = readdirSync(join(matrix, "runs"), { withFileTypes: true })
  .filter((entry) => entry.isDirectory())
  .map((entry) => join(matrix, "runs", entry.name))
  .sort();
if (runDirectories.length === 0) throw new Error("행렬 실행 폴더가 없습니다");

const records = runDirectories.map((directory) => {
  const metadata = readKeyValues(join(directory, "metadata.txt"));
  const trace = join(directory, "frame-attribution.pftrace");
  const pid = Number(metadata.app_pid);
  if (!Number.isSafeInteger(pid)) throw new Error(`앱 PID가 없습니다: ${directory}`);

  // FrameTimeline은 surface/display 행이 함께 있으므로 프레임은 token으로 집계합니다.
  const frames = query(
    trace,
    `SELECT
      COUNT(*) AS all_frame_rows,
      SUM(CASE WHEN layer_name GLOB '*MainActivity*' THEN 1 ELSE 0 END) AS app_frame_rows,
      COUNT(DISTINCT CASE WHEN layer_name GLOB '*MainActivity*' THEN surface_frame_token END) AS app_frames,
      SUM(CASE WHEN layer_name GLOB '*MainActivity*' AND instr(COALESCE(jank_type, ''), 'App Deadline Missed') > 0 THEN 1 ELSE 0 END) AS app_deadline_missed_rows,
      COUNT(DISTINCT CASE WHEN layer_name GLOB '*MainActivity*' AND instr(COALESCE(jank_type, ''), 'App Deadline Missed') > 0 THEN surface_frame_token END) AS app_deadline_missed,
      SUM(CASE WHEN instr(COALESCE(jank_type, ''), 'SurfaceFlinger') > 0 THEN 1 ELSE 0 END) AS surfaceflinger_deadline_rows,
      COUNT(DISTINCT CASE WHEN instr(COALESCE(jank_type, ''), 'SurfaceFlinger') > 0 THEN display_frame_token END) AS surfaceflinger_deadline_frames,
      SUM(CASE WHEN instr(COALESCE(jank_type, ''), 'Dropped Frame') > 0 THEN 1 ELSE 0 END) AS dropped_frame_rows,
      COUNT(DISTINCT CASE WHEN instr(COALESCE(jank_type, ''), 'Dropped Frame') > 0 THEN display_frame_token END) AS dropped_frame_display_tokens,
      SUM(CASE WHEN layer_name GLOB '*MainActivity*' AND present_type = 'Late Present' THEN 1 ELSE 0 END) AS app_late_present_rows,
      COUNT(DISTINCT CASE WHEN layer_name GLOB '*MainActivity*' AND present_type = 'Late Present' THEN surface_frame_token END) AS app_late_present,
      (SELECT COUNT(*) FROM expected_frame_timeline_slice WHERE layer_name GLOB '*MainActivity*') AS app_expected_frame_rows,
      (SELECT COUNT(DISTINCT surface_frame_token) FROM expected_frame_timeline_slice WHERE layer_name GLOB '*MainActivity*') AS app_expected_frames,
      (SELECT COUNT(*) FROM (
        SELECT DISTINCT surface_frame_token FROM actual_frame_timeline_slice WHERE layer_name GLOB '*MainActivity*'
        EXCEPT
        SELECT DISTINCT surface_frame_token FROM expected_frame_timeline_slice WHERE layer_name GLOB '*MainActivity*'
      )) AS app_actual_only_tokens,
      (SELECT COUNT(*) FROM (
        SELECT DISTINCT surface_frame_token FROM expected_frame_timeline_slice WHERE layer_name GLOB '*MainActivity*'
        EXCEPT
        SELECT DISTINCT surface_frame_token FROM actual_frame_timeline_slice WHERE layer_name GLOB '*MainActivity*'
      )) AS app_expected_only_tokens
    FROM actual_frame_timeline_slice`,
  )[0];

  const mainStates = query(
    trace,
    `SELECT state, ROUND(SUM(dur) / 1e6, 3) AS duration_ms
     FROM thread_state JOIN thread USING (utid)
     WHERE tid = ${pid}
     GROUP BY state`,
  );

  const quality = query(
    trace,
    `SELECT
      COALESCE(MAX(CASE WHEN name = 'ftrace_setup_errors' THEN CAST(value AS INT) END), 0) AS setup_errors,
      COALESCE(MAX(CASE WHEN name = 'ftrace_cpu_dropped_events_delta' THEN CAST(value AS INT) END), 0) AS ftrace_drops,
      COALESCE(MAX(CASE WHEN name IN ('traced_buf_data_loss_read_gap', 'traced_buf_incremental_sequences_dropped', 'traced_buf_sequence_packet_loss', 'traced_buf_trace_writer_packet_loss') THEN CAST(value AS INT) END), 0) AS packet_loss,
      COALESCE(MAX(CASE WHEN name IN ('traced_buf_chunks_overwritten', 'traced_buf_chunks_discarded') THEN CAST(value AS INT) END), 0) AS discarded_chunks
     FROM stats`,
  )[0];

  const counters = query(
    trace,
    `SELECT
      COALESCE(MAX(CASE WHEN ct.name = 'SpinonR05RuntimeDispatchCount' THEN c.value END), 0) AS dispatch_count,
      COALESCE(MAX(CASE WHEN ct.name = 'SpinonR05SpinonInputCount' THEN c.value END), 0) AS input_count
     FROM counter c JOIN counter_track ct ON c.track_id = ct.id`,
  )[0];

  const log = readFileSync(join(directory, "logcat.txt"), "utf8");
  const dispatchReports = log
    .split(/\r?\n/)
    .filter((line) => line.includes("SPINON_RUNTIME_DISPATCH="))
    .map((line) => {
      const report = line.slice(line.indexOf("SPINON_RUNTIME_DISPATCH=") + "SPINON_RUNTIME_DISPATCH=".length);
      return Object.fromEntries([...report.matchAll(/([a-z0-9_]+)=([^\s]+)/g)].map((match) => [match[1], match[2]]));
    }).sort((left, right) => numeric(left.seq) - numeric(right.seq));

  const runtimeWorkerCalls = query(
    trace,
    `SELECT row_number() OVER (ORDER BY s.ts) AS call_index,
      ROUND(s.dur / 1e3, 3) AS duration_us, s.ts,
      t.tid AS caller_tid
     FROM slice s
     JOIN thread_track tt ON s.track_id = tt.id
     JOIN thread t USING (utid)
     WHERE s.name = 'SpinonR05:runtime-worker-call'
     ORDER BY s.ts`,
  );
  const reportsByCaller = new Map();
  for (const report of dispatchReports) {
    const reports = reportsByCaller.get(String(report.caller_tid)) ?? [];
    reports.push(report);
    reportsByCaller.set(String(report.caller_tid), reports);
  }
  const callsByCaller = new Map();
  for (const call of runtimeWorkerCalls) {
    const calls = callsByCaller.get(String(call.caller_tid)) ?? [];
    calls.push(call);
    callsByCaller.set(String(call.caller_tid), calls);
  }
  const callReports = runtimeWorkerCalls.map((call) => {
    const caller = String(call.caller_tid);
    const index = callsByCaller.get(caller).findIndex(
      (candidate) => candidate.call_index === call.call_index,
    );
    return { ...call, report: reportsByCaller.get(caller)?.[index] ?? null };
  });
  const callCorrelationFailures = callReports.filter((call) => call.report === null).length
    + [...reportsByCaller.entries()].reduce((total, [caller, reports]) =>
      total + Math.max(0, reports.length - (callsByCaller.get(caller)?.length ?? 0)), 0);
  const reportValues = callReports
    .filter((call) => call.report)
    .map((call) => `(${Number(call.call_index)}, ${Number(call.report.caller_tid)}, ${Number(call.report.owner_tid)})`)
    .join(", ");

  const runtimeThreadStates = reportValues
    ? query(
        trace,
        `WITH reports(call_index, caller_tid, owner_tid) AS (VALUES ${reportValues}),
        calls AS (
          SELECT row_number() OVER (ORDER BY s.ts) AS call_index, s.ts, s.dur
          FROM slice s WHERE s.name = 'SpinonR05:runtime-worker-call'
        ),
        overlaps AS (
          SELECT r.call_index, t.tid, t.name, ts.state,
            SUM(MAX(0, MIN(ts.ts + ts.dur, c.ts + c.dur) - MAX(ts.ts, c.ts))) AS overlap_ns
          FROM calls c JOIN reports r USING (call_index)
          JOIN thread_state ts ON ts.ts < c.ts + c.dur AND ts.ts + ts.dur > c.ts
          JOIN thread t USING (utid)
          WHERE t.tid IN (r.caller_tid, r.owner_tid)
          GROUP BY r.call_index, t.tid, t.name, ts.state
        )
        SELECT call_index, tid, name, state, ROUND(overlap_ns / 1e6, 3) AS overlap_ms
        FROM overlaps ORDER BY call_index, tid, overlap_ms DESC`,
      )
    : [];

  const runtimePhases = dispatchReports.length
    ? query(
        trace,
        `WITH calls AS (
          SELECT row_number() OVER (ORDER BY ts) AS call_index, ts, dur
          FROM slice WHERE name = 'SpinonR05:runtime-worker-call'
        )
        SELECT c.call_index, s.name,
          ROUND(SUM(MAX(0, MIN(s.ts + s.dur, c.ts + c.dur)
            - MAX(s.ts, c.ts))) / 1e3, 3) AS duration_us
        FROM calls c JOIN slice s
          ON s.ts < c.ts + c.dur AND s.ts + s.dur > c.ts
        WHERE s.name IN (
          'SpinonR05:runtime-submit',
          'SpinonR05:runtime-enqueue',
          'SpinonR05:reply-channel-create',
          'SpinonR05:reply-receive',
          'SpinonR05:scheduler-enqueue',
          'SpinonR05:actor-condvar-wait',
          'SpinonR05:native-session-dispatch',
          'SpinonR05:native-session-ffi',
          'SpinonR05:native-session-logcat',
          'SpinonR05:native-session-byte-array',
          'SpinonR05:v8-handler-call',
          'SpinonR05:node-callback',
          'SpinonR05:document-commit-bridge',
          'SpinonR05:document-commit-callback',
          'SpinonR05:text-callback',
          'SpinonR05:v8-microtasks',
          'SpinonR05:document-safe-point'
        )
        GROUP BY c.call_index, s.name
        ORDER BY c.call_index, s.name`,
      )
    : [];

  const runtimeScheduling = reportValues
    ? query(
        trace,
        `WITH reports(call_index, caller_tid, owner_tid) AS (VALUES ${reportValues}),
        calls AS (
          SELECT row_number() OVER (ORDER BY s.ts) AS call_index, s.ts, s.dur
          FROM slice s WHERE s.name = 'SpinonR05:runtime-worker-call'
        ),
        participants AS (
          SELECT call_index, 'caller' AS role, caller_tid AS tid FROM reports
          UNION ALL
          SELECT call_index, 'owner' AS role, owner_tid AS tid FROM reports
        ),
        wake_candidates AS (
          SELECT p.call_index, p.role, p.tid, e.ts AS wake_ts,
            cpu.int_value AS target_cpu,
            row_number() OVER (PARTITION BY p.call_index, p.role ORDER BY e.ts) AS wake_rank
          FROM calls c JOIN participants p USING (call_index)
          JOIN __intrinsic_ftrace_event e ON e.name = 'sched_wakeup' AND e.ts BETWEEN c.ts AND c.ts + c.dur
          JOIN args pid ON pid.arg_set_id = e.arg_set_id AND pid.key = 'pid' AND pid.int_value = p.tid
          JOIN args cpu ON cpu.arg_set_id = e.arg_set_id AND cpu.key = 'target_cpu'
        ),
        first_wake AS (
          SELECT call_index, role, tid, wake_ts, target_cpu
          FROM wake_candidates WHERE wake_rank = 1
        ),
        run_candidates AS (
          SELECT w.call_index, w.role, w.tid, w.wake_ts, w.target_cpu,
            e.ts AS run_ts, e.ucpu AS run_cpu,
            row_number() OVER (PARTITION BY w.call_index, w.role ORDER BY e.ts) AS run_rank
          FROM first_wake w JOIN calls c USING (call_index)
          JOIN __intrinsic_ftrace_event e ON e.name = 'sched_switch' AND e.ts >= w.wake_ts AND e.ts <= c.ts + c.dur
          JOIN args next_pid ON next_pid.arg_set_id = e.arg_set_id AND next_pid.key = 'next_pid' AND next_pid.int_value = w.tid
        ),
        first_run AS (
          SELECT call_index, role, tid, wake_ts, target_cpu, run_ts, run_cpu
          FROM run_candidates WHERE run_rank = 1
        ),
        cpu_occupants AS (
          SELECT fr.call_index, fr.role, t.tid, t.name, s.end_state, s.priority,
            SUM(MAX(0, MIN(s.ts + s.dur, fr.run_ts) - MAX(s.ts, fr.wake_ts))) AS overlap_ns
          FROM first_run fr
          JOIN __intrinsic_sched_slice s ON s.ucpu = fr.target_cpu AND s.ts < fr.run_ts AND s.ts + s.dur > fr.wake_ts
          JOIN thread t USING (utid)
          GROUP BY fr.call_index, fr.role, t.tid, t.name, s.end_state, s.priority
        ),
        ranked_occupants AS (
          SELECT *, row_number() OVER (PARTITION BY call_index, role ORDER BY overlap_ns DESC) AS occupant_rank
          FROM cpu_occupants
        )
        SELECT fr.call_index, fr.role, fr.tid, fr.wake_ts, fr.run_ts,
          (fr.run_ts - fr.wake_ts) / 1e6 AS wake_to_run_ms,
          fr.target_cpu, fr.run_cpu, o.tid AS longest_tid, o.name AS longest_thread,
          o.priority AS longest_priority, o.end_state AS longest_end_state,
          o.overlap_ns / 1e6 AS longest_overlap_ms
        FROM first_run fr
        LEFT JOIN ranked_occupants o ON o.call_index = fr.call_index
          AND o.role = fr.role AND o.occupant_rank = 1
        ORDER BY fr.call_index, fr.role`,
      )
    : [];

  const tapTimelineSql =
    "WITH event_calls AS (" +
    " SELECT row_number() OVER (ORDER BY ts) AS call_index, ts, dur" +
    " FROM slice WHERE name = 'SpinonR05:runtime-dispatch'" +
    "), worker_calls AS (" +
    " SELECT row_number() OVER (ORDER BY ts) AS call_index, ts, dur" +
    " FROM slice WHERE name = 'SpinonR05:runtime-worker-call'" +
    "), ui_handoffs AS (" +
    " SELECT row_number() OVER (ORDER BY ts) AS call_index, ts, dur" +
    " FROM slice WHERE name = 'SpinonR05:runtime-main-thread-handoff'" +
    "), ui_posts AS (" +
    " SELECT row_number() OVER (ORDER BY ts) AS call_index, ts, dur" +
    " FROM slice WHERE name = 'SpinonR05:runtime-main-thread-post'" +
    "), ui_completions AS (" +
    " SELECT row_number() OVER (ORDER BY ts) AS call_index, ts, dur" +
    " FROM slice WHERE name = 'SpinonR05:runtime-main-thread-completion'" +
    "), status_updates AS (" +
    " SELECT row_number() OVER (ORDER BY ts) AS call_index, dur" +
    " FROM slice WHERE name = 'SpinonR05:spinon-event-status-update'" +
    "), log_overlap AS (" +
    " SELECT e.call_index," +
    " SUM(MAX(0, MIN(l.ts + l.dur, e.ts + e.dur) - MAX(l.ts, e.ts))) AS overlap_ns" +
    " FROM event_calls e LEFT JOIN slice l ON l.name = 'SpinonR05:runtime-log-flush'" +
    " AND l.ts < e.ts + e.dur AND l.ts + l.dur > e.ts" +
    " GROUP BY e.call_index" +
    "), handoff_states AS (" +
    " SELECT h.call_index, ts.state," +
    " SUM(MAX(0, MIN(ts.ts + ts.dur, h.ts + h.dur) - MAX(ts.ts, h.ts))) AS overlap_ns" +
    " FROM ui_handoffs h JOIN thread t ON t.tid = " + pid +
    " JOIN thread_state ts USING (utid)" +
    " WHERE ts.ts < h.ts + h.dur AND ts.ts + ts.dur > h.ts" +
    " GROUP BY h.call_index, ts.state" +
    "), handoff_state_summary AS (" +
    " SELECT call_index," +
    " GROUP_CONCAT(state || '=' || ROUND(overlap_ns / 1e6, 3), ', ') AS states_ms" +
    " FROM handoff_states GROUP BY call_index" +
    ") SELECT e.call_index," +
    " ROUND(e.dur / 1e3, 3) AS event_async_us," +
    " ROUND((w.ts - e.ts) / 1e3, 3) AS event_to_worker_us," +
    " ROUND(w.dur / 1e3, 3) AS worker_call_us," +
    " ROUND((e.ts + e.dur - w.ts - w.dur) / 1e3, 3) AS worker_end_to_ui_completion_us," +
    " ROUND(h.dur / 1e3, 3) AS main_thread_handoff_us," +
    " ROUND(p.dur / 1e3, 3) AS main_thread_post_us," +
    " ROUND(c.dur / 1e3, 3) AS main_thread_completion_us," +
    " COALESCE(ROUND(status.dur / 1e3, 3), -1) AS input_status_update_us," +
    " ROUND(COALESCE(log.overlap_ns, 0) / 1e3, 3) AS ui_log_flush_overlap_us," +
    " COALESCE(ss.states_ms, '상태 미관측') AS main_thread_handoff_states_ms" +
    " FROM event_calls e LEFT JOIN worker_calls w USING (call_index)" +
    " LEFT JOIN ui_handoffs h USING (call_index)" +
    " LEFT JOIN ui_posts p USING (call_index)" +
    " LEFT JOIN ui_completions c USING (call_index)" +
    " LEFT JOIN status_updates status USING (call_index)" +
    " LEFT JOIN log_overlap log USING (call_index)" +
    " LEFT JOIN handoff_state_summary ss USING (call_index)" +
    " ORDER BY e.call_index";
  const tapTimeline = query(trace, tapTimelineSql);

  const apkHash = readFileSync(join(directory, "installed-apk.sha256"), "utf8").trim().split(/\s+/)[0];
  return {
    directory,
    scenario: metadata.scenario,
    metadata,
    apkHash,
    sourceHash: metadata.source_tree_sha256,
    frames: Object.fromEntries(Object.entries(frames).map(([key, value]) => [key, numeric(value)])),
    mainStates: Object.fromEntries(mainStates.map((row) => [row.state, numeric(row.duration_ms)])),
    quality: Object.fromEntries(Object.entries(quality).map(([key, value]) => [key, numeric(value)])),
    counters: Object.fromEntries(Object.entries(counters).map(([key, value]) => [key, numeric(value)])),
    dispatchReports,
    callReports,
    callCorrelationFailures,
    runtimeThreadStates,
    runtimePhases,
    runtimeWorkerCalls,
    runtimeScheduling,
    tapTimeline,
  };
});

const actualScheduleMismatch = records.some((record, index) =>
  scheduledRuns[index]?.scenario !== record.scenario,
);
if (actualScheduleMismatch) scheduleProblems.push("schedule 순서와 캡처 폴더 순서 불일치");
const fields = [
  "submission_lock_wait_us",
  "control_lock_wait_us",
  "scheduler_lock_wait_us",
  "queue_residence_us",
  "v8_call_us",
  "post_v8_us",
  "report_build_us",
  "response_report_append_us",
  "report_finalize_us",
  "actor_before_reply_us",
  "enqueue_total_us",
  "response_wait_us",
  "submit_total_us",
];
const appDeadlines = records.reduce((total, record) => total + record.frames.app_deadline_missed, 0);
const frameCount = records.reduce((total, record) => total + record.frames.app_frames, 0);
const frameTokenMismatches = records.filter((record) => {
  const frames = record.frames;
  return frames.app_frame_rows !== frames.app_frames
    || frames.app_deadline_missed_rows !== frames.app_deadline_missed
    || frames.app_expected_frame_rows !== frames.app_expected_frames
    || frames.app_frames !== frames.app_expected_frames
    || frames.app_actual_only_tokens !== 0
    || frames.app_expected_only_tokens !== 0;
});
const qualityFailures = records.filter((record) => Object.values(record.quality).some((value) => value !== 0));
const inconsistentRuns = records.filter((record) => {
  const expectedDispatches = record.scenario === "spinon-event" ? expectedInputCount : 0;
  return record.counters.dispatch_count !== expectedDispatches
    || record.counters.input_count !== expectedInputCount
    || record.dispatchReports.length !== expectedDispatches
    || record.runtimeWorkerCalls.length !== expectedDispatches
    || record.callCorrelationFailures !== 0;
});

console.log(`# Android UI/runtime 귀속 행렬`);
console.log("");
console.log(`- Trace Processor: ${processorVersion}`);
console.log(`- 행렬: ${matrix}`);
console.log(`- 실행 수: ${records.length}회 (${conditions.map((condition) => `${condition} ${records.filter((record) => record.scenario === condition).length}회`).join(", ")})`);
console.log(`- APK SHA-256 고유 개수: ${new Set(records.map((record) => record.apkHash)).size}`);
console.log(`- 캡처 시점 작업 트리 SHA-256 고유 개수: ${new Set(records.map((record) => record.sourceHash)).size}`);
console.log(`- 조건 순서 수: UI-only→event ${scheduleOrderCounts["spinon-ui-only→spinon-event"]}, event→UI-only ${scheduleOrderCounts["spinon-event→spinon-ui-only"]}${scheduleProblems.length ? ` · 경고: ${scheduleProblems.join("; ")}` : " · 균형/실제 순서 일치"}`);
console.log(`- 회차별 입력 수: ${expectedInputCount}`);
console.log(`- FrameTimeline 앱 프레임: ${frameCount}개 고유 surface token; App Deadline Missed: ${appDeadlines}개 고유 app token`);
console.log(`- 앱 actual/expected token 집합 불일치 회차: ${frameTokenMismatches.length}; trace 품질 실패: ${qualityFailures.length}; 입력·counter·report/thread 귀속 불일치: ${inconsistentRuns.length}`);
console.log("");
console.log("## 조건별 프레임·UI thread");
console.log("");
console.log("| 조건 | 앱 실제 surface frames (고유 token) | App Deadline Missed (고유 token) | Dropped Frame (고유 display token) | SurfaceFlinger deadline (고유 display token) | main thread D (회차별 중앙값 ms) | main thread Running (회차별 중앙값 ms) |");
console.log("| --- | ---: | ---: | ---: | ---: | ---: | ---: |");
for (const condition of conditions) {
  const subset = records.filter((record) => record.scenario === condition);
  const sum = (key) => subset.reduce((total, record) => total + record.frames[key], 0);
  const stateMedian = (state) => percentile(subset.map((record) => record.mainStates[state] ?? 0), 0.5).toFixed(3);
  console.log(`| ${condition} | ${sum("app_frames")} | ${sum("app_deadline_missed")} | ${sum("dropped_frame_display_tokens")} | ${sum("surfaceflinger_deadline_frames")} | ${stateMedian("D")} | ${stateMedian("Running")} |`);
}
console.log("");
console.log("## 런타임 dispatch 단계 (us, Spinon event 조건)");
console.log("");
console.log("| 단계 | 관측값 | >16.67 ms |");
console.log("| --- | --- | ---: |");
const eventReports = records.filter((record) => record.scenario === "spinon-event").flatMap((record) => record.dispatchReports);
for (const field of fields) {
  const values = eventReports.map((report) => numeric(report[field]));
  console.log(`| ${field} | ${formatDistribution(values)} | ${slowCount(values, 16667)} |`);
}
console.log("");
console.log("## dispatch 내부 trace 구간");
console.log("");
console.log("Perfetto ATrace 구간의 실행 시간입니다. `v8_call_us`와 별개로 읽지 말고, 같은 dispatch의 중첩 구간 위치를 확인하는 데 사용합니다.");
console.log("");
console.log("| 구간 | 관측값 (us) | >16.67 ms |");
console.log("| --- | --- | ---: |");
const phaseLabels = new Map([
  ["SpinonR05:runtime-submit", "Rust submit 전체 (enqueue + 응답 수신)"],
  ["SpinonR05:runtime-enqueue", "Rust enqueue 준비·잠금·삽입"],
  ["SpinonR05:reply-channel-create", "응답 채널 생성"],
  ["SpinonR05:reply-receive", "호출자 응답 수신 대기"],
  ["SpinonR05:scheduler-enqueue", "스케줄러 잠금·큐 삽입·깨우기"],
  ["SpinonR05:actor-condvar-wait", "actor 조건변수 대기"],
  ["SpinonR05:native-session-dispatch", "JNI nativeSessionDispatch 전체"],
  ["SpinonR05:native-session-ffi", "JNI → Rust FFI 전체"],
  ["SpinonR05:native-session-logcat", "동기 Android Logcat 출력"],
  ["SpinonR05:native-session-byte-array", "JNI report byte-array 변환"],
  ["SpinonR05:v8-handler-call", "V8 이벤트 handler 호출"],
  ["SpinonR05:node-callback", "JS createNode → Rust callback"],
  ["SpinonR05:document-commit-bridge", "DOM 문서 묶음 변환·commit bridge"],
  ["SpinonR05:document-commit-callback", "HostDocument Rust commit callback"],
  ["SpinonR05:text-callback", "JS setText → Rust callback"],
  ["SpinonR05:v8-microtasks", "V8 microtask checkpoint"],
  ["SpinonR05:document-safe-point", "HostDocument safe-point 회수"],
]);
const eventPhases = records.filter((record) => record.scenario === "spinon-event").flatMap((record) => record.runtimePhases);
for (const [name, label] of phaseLabels) {
  const rows = eventPhases.filter((row) => row.name === name);
  const values = rows.map((row) => numeric(row.duration_us));
  console.log(`| ${label} | ${formatDistribution(values)} | ${slowCount(values, 16667)} |`);
}
console.log("");
console.log("## 탭 완료 경로 분해");
console.log("");
console.log("비동기 탭 시작부터 메인 스레드 완료 callback까지의 시간과 Rust/V8 worker 구간을 나눕니다. main-thread handoff는 백그라운드 결과 처리 직후 runOnUiThread를 게시한 시점부터 메인 callback 첫 줄까지이며, 그 안의 스레드 상태와 탭별 로그 flush 겹침도 함께 집계합니다.");
console.log("");
console.log("| 구간 | 전체 (us) | 첫 입력 (us) | 후속 입력 (us) |");
console.log("| --- | --- | --- | --- |");
const tapTimelineRows = records.filter((record) => record.scenario === "spinon-event")
  .flatMap((record) => record.tapTimeline);
for (const [field, label] of [
  ["event_async_us", "탭 제출→메인 완료 callback"],
  ["event_to_worker_us", "탭 제출→worker 시작"],
  ["worker_call_us", "네이티브/Rust/V8 worker 호출"],
  ["worker_end_to_ui_completion_us", "worker 반환→UI 완료 callback"],
  ["main_thread_handoff_us", "runOnUiThread 게시→main callback 시작"],
  ["main_thread_post_us", "main callback 게시 비용"],
  ["main_thread_completion_us", "main 완료 callback 본문"],
  ["ui_log_flush_overlap_us", "탭 처리와 겹친 로그 UI flush"],
]) {
  const all = tapTimelineRows.map((row) => numeric(row[field]));
  const first = tapTimelineRows.filter((row) => numeric(row.call_index) === 1)
    .map((row) => numeric(row[field]));
  const followups = tapTimelineRows.filter((row) => numeric(row.call_index) > 1)
    .map((row) => numeric(row[field]));
  console.log("| " + label + " | " + formatDistribution(all) + " | "
      + formatDistribution(first) + " | " + formatDistribution(followups) + " |");
}
const inputStatusUpdateValues = tapTimelineRows
  .map((row) => Number(row.input_status_update_us))
  .filter((value) => Number.isFinite(value) && value >= 0);
if (inputStatusUpdateValues.length === 0) {
  console.log("| 탭 시 상태 TextView 갱신 | 벤치마크 계측 모드에서 생략 | 벤치마크 계측 모드에서 생략 | 벤치마크 계측 모드에서 생략 |");
} else {
  const firstStatusUpdates = tapTimelineRows
    .filter((row) => Number(row.input_status_update_us) >= 0 && numeric(row.call_index) === 1)
    .map((row) => Number(row.input_status_update_us));
  const followupStatusUpdates = tapTimelineRows
    .filter((row) => Number(row.input_status_update_us) >= 0 && numeric(row.call_index) > 1)
    .map((row) => Number(row.input_status_update_us));
  console.log("| 탭 시 상태 TextView 갱신 | "
    + formatDistribution(inputStatusUpdateValues) + " | "
    + formatDistribution(firstStatusUpdates) + " | "
    + formatDistribution(followupStatusUpdates) + " |");
}
console.log("");
console.log("### 16.67 ms 초과 탭");
console.log("");
console.log("| 실행 / 호출 | 전체 async ms | worker us | worker→UI callback ms | UI handoff ms | 겹친 로그 flush ms | main handoff 스레드 상태 ms |");
console.log("| --- | ---: | ---: | ---: | ---: | ---: | --- |");
for (const record of records.filter((item) => item.scenario === "spinon-event")) {
  for (const row of record.tapTimeline) {
    if (numeric(row.event_async_us) <= 16670) continue;
    console.log("| " + record.directory.split("/").at(-1) + " #" + row.call_index + " | "
      + (numeric(row.event_async_us) / 1000).toFixed(3) + " | " + row.worker_call_us
      + " | " + (numeric(row.worker_end_to_ui_completion_us) / 1000).toFixed(3)
      + " | " + (numeric(row.main_thread_handoff_us) / 1000).toFixed(3)
      + " | " + (numeric(row.ui_log_flush_overlap_us) / 1000).toFixed(3)
      + " | " + row.main_thread_handoff_states_ms + " |");
  }
}
console.log("");
console.log("### 회차 내 첫 호출과 후속 호출");
console.log("");
console.log(`각 앱 프로세스에서 첫 입력(seq=2)과 나머지 ${Math.max(0, expectedInputCount - 1)}개 후속 입력을 나눠 봅니다. cold/warm 성능 등급이 아니라 이 행렬 안의 관측 위치입니다.`);
console.log("");
console.log("| 위치 | 호출 수 | queue residence us (p50/p95/max) | V8 wall us (p50/p95/max) | 총 호출 us (p50/p95/max) |");
console.log("| --- | ---: | --- | --- | --- |");
for (const [label, position] of [["첫 입력", 0], ["후속 입력", 1]]) {
  const reports = records.filter((record) => record.scenario === "spinon-event").flatMap((record) =>
    position === 0 ? (record.dispatchReports[0] ? [record.dispatchReports[0]] : []) : record.dispatchReports.slice(1),
  );
  console.log(`| ${label} | ${reports.length} | ${formatDistribution(reports.map((report) => numeric(report.queue_residence_us)))} | ${formatDistribution(reports.map((report) => numeric(report.v8_call_us)))} | ${formatDistribution(reports.map((report) => numeric(report.submit_total_us)))} |`);
}
console.log("");
console.log("## 긴 runtime-worker-call thread 상태");
console.log("");
console.log("`SpinonR05:runtime-worker-call`이 16.67 ms를 넘은 호출만 표시합니다. caller와 V8 owner 각각의 첫 `sched_wakeup`→실행 전환을 구분합니다. CPU 열은 wake target CPU와 실제 실행 CPU를 함께 표시합니다. CPU slice는 wake target CPU에서 겹친 최장 관측 thread이며 지연 원인 증거로 단정하지 않습니다. `swapper`는 CPU idle 상태입니다.");
console.log("");
console.log("| 실행 / 호출 | Java worker ms | Rust submit ms | actor 응답 전 ms | 보고서 후처리 us | JNI→Rust FFI ms | Logcat ms | queue ms | V8 ms | caller Running ms | owner Running ms | caller wake→run / CPU slice | owner wake→run / CPU slice |");
console.log("| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | --- | --- |");
for (const record of records.filter((item) => item.scenario === "spinon-event")) {
  record.callReports.forEach((call, index) => {
    const report = call.report;
    if (!report) return;
    const workerCallUs = numeric(call.duration_us);
    if (numeric(report.submit_total_us) <= 16670 && workerCallUs <= 16670) return;
    const stateRows = record.runtimeThreadStates.filter((row) => numeric(row.call_index) === index + 1);
    const phaseDurationUs = (name) => record.runtimePhases
      .filter((row) => numeric(row.call_index) === index + 1 && row.name === name)
      .reduce((total, row) => total + numeric(row.duration_us), 0);
    const stateDuration = (tid, state) =>
      stateRows
        .filter((row) => numeric(row.tid) === Number(tid) && row.state === state)
        .reduce((total, row) => total + numeric(row.overlap_ms), 0);
    const schedulingLabel = (role) => {
      const scheduling = record.runtimeScheduling.find((row) =>
        numeric(row.call_index) === index + 1 && row.role === role,
      );
      if (!scheduling) return "—";
      const wakeToRunMs = numeric(scheduling.wake_to_run_ms);
      const cpuRoute = `wake cpu${scheduling.target_cpu}→run cpu${scheduling.run_cpu}`;
      if (Number(scheduling.target_cpu) !== Number(scheduling.run_cpu)) {
        return `${wakeToRunMs.toFixed(3)}ms · ${cpuRoute}; target CPU 점유는 원인 귀속에서 제외`;
      }
      const slice = scheduling.longest_thread
        ? `${cpuRoute} ${scheduling.longest_thread} ${numeric(scheduling.longest_overlap_ms).toFixed(3)}ms ${scheduling.longest_end_state}`
        : `${cpuRoute} slice 미관측`;
      return `${wakeToRunMs.toFixed(3)}ms · ${slice}`;
    };
    console.log(`| ${record.directory.split("/").at(-1)} #${index + 1} | ${(workerCallUs / 1000).toFixed(3)} | ${(numeric(report.submit_total_us) / 1000).toFixed(3)} | ${(numeric(report.actor_before_reply_us) / 1000).toFixed(3)} | ${numeric(report.report_finalize_us)} | ${(phaseDurationUs("SpinonR05:native-session-ffi") / 1000).toFixed(3)} | ${(phaseDurationUs("SpinonR05:native-session-logcat") / 1000).toFixed(3)} | ${(numeric(report.queue_residence_us) / 1000).toFixed(3)} | ${(numeric(report.v8_call_us) / 1000).toFixed(3)} | ${stateDuration(report.caller_tid, "Running").toFixed(3)} | ${stateDuration(report.owner_tid, "Running").toFixed(3)} | ${schedulingLabel("caller")} | ${schedulingLabel("owner")} |`);
  });
}
console.log("");
console.log("## trace 품질·입력 확인");
console.log("");
console.log("| 조건 | trace 수 | setup errors | ftrace drops | packet loss | discarded chunks | input counter | dispatch counter |");
console.log("| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |");
for (const condition of conditions) {
  const subset = records.filter((record) => record.scenario === condition);
  const sum = (key) => subset.reduce((total, record) => total + record.quality[key], 0);
  console.log(`| ${condition} | ${subset.length} | ${sum("setup_errors")} | ${sum("ftrace_drops")} | ${sum("packet_loss")} | ${sum("discarded_chunks")} | ${subset.reduce((total, record) => total + record.counters.input_count, 0)} | ${subset.reduce((total, record) => total + record.counters.dispatch_count, 0)} |`);
}
console.log("");
console.log("## 회차 원본");
console.log("");
console.log("| 시작 시각 | 조건 | PID | 앱 실제 surface frames | App Deadline Missed | main D ms | dispatch 수 | trace 품질 / token 일치 |");
console.log("| --- | --- | ---: | ---: | ---: | ---: | ---: | --- |");
for (const record of records) {
  const traceQuality = Object.values(record.quality).every((value) => value === 0) ? "정상" : "실패";
  const frameTokensMatch = frameTokenMismatches.includes(record) ? "불일치" : "일치";
  console.log(`| ${record.directory.split("/").at(-1).slice(0, 16)} | ${record.scenario} | ${record.metadata.app_pid} | ${record.frames.app_frames} | ${record.frames.app_deadline_missed} | ${(record.mainStates.D ?? 0).toFixed(3)} | ${record.counters.dispatch_count} | ${traceQuality} / ${frameTokensMatch} |`);
}
