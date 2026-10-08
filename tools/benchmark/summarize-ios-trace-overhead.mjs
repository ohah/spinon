import { existsSync, readFileSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";

const matrixDir = resolve(process.argv[2] ?? "");
if (!process.argv[2]) {
  console.error("사용법: bun tools/benchmark/summarize-ios-trace-overhead.mjs <행렬 디렉터리>");
  process.exit(2);
}
const pairLines = readFileSync(resolve(matrixDir, "pairs.tsv"), "utf8").trim().split(/\r?\n/);
if (pairLines[0] !== "round\tattempt\torder\ttrace_on_dir\ttrace_off_dir") {
  throw new Error("pairs.tsv 헤더가 현재 형식과 다릅니다");
}
const pairs = pairLines.slice(1).map((line) => line.split("\t"));
if (pairs.length !== 10) throw new Error(`유효 pair 10개가 필요합니다. 현재 ${pairs.length}개입니다.`);
const schedulePath = resolve(matrixDir, "schedule.txt");
const scheduleValues = Object.fromEntries(readFileSync(schedulePath, "utf8").split(/\r?\n/)
  .filter((line) => line.includes("="))
  .map((line) => {
    const separator = line.indexOf("=");
    return [line.slice(0, separator), line.slice(separator + 1)];
  }));
if (scheduleValues.repeats_per_condition !== "10" || scheduleValues.duration_seconds !== "12") {
  throw new Error("사전 비교와 행렬 조건이 다릅니다");
}
if (!/^[0-9a-f]{64}$/.test(scheduleValues.source_tree_sha256 ?? "")) {
  throw new Error("행렬 전체 source digest가 없습니다");
}
const scheduleLines = readFileSync(schedulePath, "utf8").split(/\r?\n/);
const scheduleHeaderIndex = scheduleLines.indexOf("round\tattempt\torder\tstatus\trun_on\trun_off");
if (scheduleHeaderIndex < 0) throw new Error("schedule.txt에 실행 순서 표가 없습니다");
const passedScheduleRows = scheduleLines.slice(scheduleHeaderIndex + 1)
  .filter((line) => line.trim())
  .map((line) => line.split("\t"))
  .filter((row) => row[3] === "PASS");
if (passedScheduleRows.length !== 10 || passedScheduleRows.some((row) => row.length !== 6)) {
  throw new Error("schedule.txt의 PASS 실행 행이 10개가 아니거나 열 수가 잘못됐습니다");
}

const fields = [
  ["queue_us", "runtime queue"],
  ["ffi_us", "Rust FFI"],
  ["v8_us", "V8 호출"],
  ["actor_us", "actor 응답 전"],
  ["main_queue_us", "main queue handoff"],
  ["callback_to_display_tick_us", "callback → CADisplayLink"],
  ["total_to_display_tick_us", "입력 → CADisplayLink"],
];

function median(values) {
  const sorted = [...values].sort((left, right) => left - right);
  const middle = Math.floor(sorted.length / 2);
  return sorted.length % 2 === 0 ? (sorted[middle - 1] + sorted[middle]) / 2 : sorted[middle];
}

function requireCondition(condition, message) {
  if (!condition) throw new Error(message);
}

function parseCompactUtc(value) {
  const match = value?.match(/^(\d{4})(\d{2})(\d{2})T(\d{2})(\d{2})(\d{2})Z$/);
  if (!match) return Number.NaN;
  const [, year, month, day, hour, minute, second] = match;
  return Date.UTC(Number(year), Number(month) - 1, Number(day), Number(hour), Number(minute), Number(second));
}

function describe(values) {
  return `${median(values).toFixed(3)} (${Math.min(...values).toFixed(3)}–${Math.max(...values).toFixed(3)})`;
}

function summarizeRun(directory, expectedMode, expectedSchedule) {
  const metadata = Object.fromEntries(readFileSync(resolve(directory, "metadata.txt"), "utf8")
    .split(/\r?\n/).filter((line) => line.includes("="))
    .map((line) => {
      const separator = line.indexOf("=");
      return [line.slice(0, separator), line.slice(separator + 1)];
    }));
  requireCondition(metadata.mode === expectedMode, `실행 mode가 다릅니다: ${directory}`);
  requireCondition(metadata.simulator_udid === expectedSchedule.simulator_udid, `시뮬레이터 UDID가 행렬과 다릅니다: ${directory}`);
  requireCondition(metadata.simulator_runtime === "iOS 26.2", `iOS runtime이 다릅니다: ${directory}`);
  requireCondition(metadata.app_bundle_sha256 === expectedSchedule.app_bundle_sha256, `앱 bundle digest가 행렬과 다릅니다: ${directory}`);
  requireCondition(metadata.source_tree_sha256 === expectedSchedule.source_tree_sha256,
    `행렬 시작 시 source digest와 run이 다릅니다: ${directory}`);
  requireCondition(metadata.duration_seconds === expectedSchedule.duration_seconds, `실행 시간이 쌍과 다릅니다: ${directory}`);
  requireCondition(/^[0-9a-f]{64}$/.test(metadata.source_tree_sha256 ?? ""), `source digest가 없습니다: ${directory}`);
  requireCondition(/^[0-9a-f]{64}$/.test(metadata.app_binary_sha256 ?? ""), `app binary digest가 없습니다: ${directory}`);
  requireCondition(/^\d+$/.test(metadata.app_pid ?? ""), `앱 PID가 없습니다: ${directory}`);

  const log = readFileSync(resolve(directory, "unified-log.txt"), "utf8");
  const lines = log.split(/\r?\n/).filter((line) => line.includes("SPINON_R05_IOS_SAMPLE"));
  if (lines.length !== 32) throw new Error(`sample 32개가 아닙니다: ${directory}`);
  let previousEpochMs = Number.NEGATIVE_INFINITY;
  const reports = lines.map((line, index) => {
    const timestamp = line.match(/^(\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}\.\d+)/)?.[1];
    const pid = line.match(/SpinonBootstrap\[(\d+):/)?.[1];
    const sequence = Number(line.match(/SPINON_R05_IOS_SAMPLE seq=(\d+)/)?.[1]);
    const status = line.match(/\bstatus=(-?\d+)/)?.[1];
    requireCondition(timestamp && pid === metadata.app_pid && sequence === index + 1 && status === "0",
      `sample timestamp/PID/순번/status가 맞지 않습니다: ${directory} #${index + 1}`);
    const epochMs = Date.parse(timestamp.replace(" ", "T"));
    requireCondition(Number.isFinite(epochMs), `sample timestamp를 해석할 수 없습니다: ${directory} #${index + 1}`);
    requireCondition(epochMs >= previousEpochMs, `sample 시간이 역행했습니다: ${directory} #${index + 1}`);
    previousEpochMs = epochMs;
    const values = Object.fromEntries(fields.map(([field]) => {
      const value = line.match(new RegExp(`\\b${field}=([^\\s]+)`))?.[1];
      if (value === undefined || !Number.isFinite(Number(value))) throw new Error(`${field} 누락: ${directory}`);
      return [field, Number(value)];
    }));
    return { timestamp, epochMs, ...values };
  });

  if (expectedMode === "on") {
    const tocPath = resolve(directory, "trace-toc.xml");
    requireCondition(existsSync(tocPath), `Time Profiler TOC가 없습니다: ${directory}`);
    const toc = readFileSync(tocPath, "utf8");
    requireCondition(toc.includes('<template-name>Time Profiler</template-name>'), `Time Profiler template이 아닙니다: ${directory}`);
    requireCondition(toc.includes(`name="SpinonBootstrap" pid="${metadata.app_pid}"`), `Time Profiler PID가 앱 PID와 다릅니다: ${directory}`);
    const startMs = Date.parse(toc.match(/<start-date>([^<]+)<\/start-date>/)?.[1] ?? "");
    const endMs = Date.parse(toc.match(/<end-date>([^<]+)<\/end-date>/)?.[1] ?? "");
    requireCondition(Number.isFinite(startMs) && Number.isFinite(endMs) && endMs > startMs,
      `Time Profiler 기록 구간이 잘못됐습니다: ${directory}`);
    requireCondition(reports.every((report) => report.epochMs >= startMs && report.epochMs <= endMs),
      `앱 표본 전체가 Time Profiler 기록 구간에 있지 않습니다: ${directory}`);
  } else {
    requireCondition(existsSync(resolve(directory, "trace-disabled.txt")), `trace-off marker가 없습니다: ${directory}`);
    requireCondition(!existsSync(resolve(directory, "time-profiler.trace")), `trace-off 조건에 profiler 원본이 있습니다: ${directory}`);
  }

  return {
    metadata,
    values: Object.fromEntries(fields.map(([field]) => [field, median(reports.map((report) => report[field]))])),
  };
}

const orderCounts = new Map([["on/off", 0], ["off/on", 0]]);
const seenRounds = new Set();
const paired = pairs.map(([roundText, attemptText, order, onDir, offDir], index) => {
  requireCondition(pairs[index].length === 5, `pairs.tsv 열 수가 잘못됐습니다: 회차 ${roundText}`);
  const round = Number(roundText);
  requireCondition(round === index + 1 && !seenRounds.has(round), `회차가 중복되거나 1..10 순서가 아닙니다: ${roundText}`);
  seenRounds.add(round);
  requireCondition(Number.isInteger(Number(attemptText)) && Number(attemptText) >= 1 && Number(attemptText) <= 3,
    `시도 번호가 1..3 범위가 아닙니다: ${attemptText}`);
  requireCondition(orderCounts.has(order), `실행 순서 표기가 잘못됐습니다: ${order}`);
  orderCounts.set(order, orderCounts.get(order) + 1);
  requireCondition(onDir && offDir, `회차 ${round} 실행 경로가 비어 있습니다`);
  const scheduled = passedScheduleRows[index];
  requireCondition(scheduled[0] === roundText && scheduled[1] === attemptText && scheduled[2] === order
    && scheduled[4] === onDir && scheduled[5] === offDir, `schedule과 pairs.tsv가 다릅니다: 회차 ${round}`);
  const onPath = resolve(onDir);
  const offPath = resolve(offDir);
  const onRun = summarizeRun(onPath, "on", scheduleValues);
  const offRun = summarizeRun(offPath, "off", scheduleValues);
  const onStarted = parseCompactUtc(onRun.metadata.started_utc);
  const offStarted = parseCompactUtc(offRun.metadata.started_utc);
  requireCondition(Number.isFinite(onStarted) && Number.isFinite(offStarted) && onStarted !== offStarted,
    `짝 안의 실행 시작 시각을 확인할 수 없습니다: 회차 ${round}`);
  const observedOrder = onStarted < offStarted ? "on/off" : "off/on";
  requireCondition(observedOrder === order, `schedule 순서와 metadata 시작 시각이 다릅니다: 회차 ${round}`);
  requireCondition(onRun.metadata.source_tree_sha256 === offRun.metadata.source_tree_sha256,
    `source tree가 쌍 안에서 달라졌습니다: 회차 ${round}`);
  requireCondition(onRun.metadata.app_binary_sha256 === offRun.metadata.app_binary_sha256,
    `app binary가 쌍 안에서 달라졌습니다: 회차 ${round}`);
  const off = offRun.values;
  return { round, attempt: Number(attemptText), order, deltas: Object.fromEntries(fields.map(([field]) => [field, onRun.values[field] - off[field]])) };
});
requireCondition(orderCounts.get("on/off") === 5 && orderCounts.get("off/on") === 5,
  `AB/BA 순서가 5회씩 균형을 이루지 않습니다: ${JSON.stringify(Object.fromEntries(orderCounts))}`);

const lines = [
  "# R05 · iOS Simulator 계측 오버헤드 요약",
  "",
  `행렬: \`${matrixDir}\``,
  "",
  "같은 debug app의 자동 R05 probe를 trace-on `Time Profiler`와 trace-off로 짝 비교했다. 각 실행은 32개 sample의 중앙값을 계산하고, 표에는 10쌍의 차이(`on - off`) 중앙값과 최소–최대를 표시한다. 단위는 µs다.",
  "",
  "이는 Time Profiler가 앱 내부 진단 측정에 미친 영향을 시뮬레이터에서 관찰한 것이다. CPU sample은 OS thread state나 화면 present 시각이 아니며, Android Perfetto와 직접 비교하지 않는다.",
  "",
  "| 앱 내부 측정 | 쌍별 차이 중앙값 (범위) |",
  "|---|---:|",
];
for (const [field, label] of fields) {
  lines.push(`| ${label} | ${describe(paired.map((pair) => pair.deltas[field]))} |`);
}
lines.push("", "## 실행별 쌍 차이", "", `| 회차 | ${fields.map(([, label]) => label).join(" | ")} |`, `|---:|${fields.map(() => "---:").join("|")}|`);
for (const pair of paired) lines.push(`| ${pair.round} | ${fields.map(([field]) => pair.deltas[field].toFixed(3)).join(" | ")} |`);
lines.push("", "## 한계", "", "- iOS 자동 selector 기반 debug probe이며 물리 터치를 재현하지 않는다.", "- iPhone 17 Pro / iOS 26.2 Simulator 한정이며 실기기·release·제품 renderer 결과가 아니다.", "- 10쌍의 기술통계다. 정식 p95/p99나 계측 오버헤드 0을 주장하지 않는다.", "- `CADisplayLink`는 display tick 관측값이며 실제 픽셀 발광 시각이 아니다.", "");

const report = lines.join("\n");
const outputPath = resolve(matrixDir, "summary.md");
writeFileSync(outputPath, report);
console.log(report);
console.log(`\n저장: ${outputPath}`);
