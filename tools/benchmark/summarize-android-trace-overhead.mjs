import { createHash } from "node:crypto";
import { existsSync, readFileSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";

const matrixDir = resolve(process.argv[2] ?? "");
if (!process.argv[2]) {
  console.error("사용법: bun tools/benchmark/summarize-android-trace-overhead.mjs <행렬 디렉터리>");
  process.exit(2);
}

const pairsPath = resolve(matrixDir, "pairs.tsv");
const pairLines = readFileSync(pairsPath, "utf8").trim().split(/\r?\n/);
if (pairLines[0] !== "round\tattempt\torder\tperfetto_on_dir\tperfetto_off_dir") {
  throw new Error("pairs.tsv 헤더가 현재 형식과 다릅니다");
}
const pairRows = pairLines.slice(1).map((line) => line.split("\t"));
if (pairRows.length !== 10) {
  throw new Error(`사전 모델은 유효 pair 10개를 요구합니다. 현재 ${pairRows.length}개입니다.`);
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

const orderCounts = new Map([["on/off", 0], ["off/on", 0]]);
const seenRounds = new Set();
const matrixMetadata = readKeyValues(resolve(matrixDir, "schedule.txt"));
requireCondition(matrixMetadata.repeats_per_condition === "10" && matrixMetadata.duration_seconds === "20"
  && matrixMetadata.input_count === "10" && matrixMetadata.android_api === "36"
  && matrixMetadata.device_model === "sdk_gphone64_arm64", "사전 비교와 행렬 조건이 다릅니다");
requireCondition(/^[0-9a-f]{64}$/.test(matrixMetadata.source_tree_sha256 ?? ""), "행렬 전체 source digest가 없습니다");
const scheduleLines = readFileSync(resolve(matrixDir, "schedule.txt"), "utf8").split(/\r?\n/);
const scheduleHeaderIndex = scheduleLines.indexOf("round\tattempt\torder\tstatus\trun_on\trun_off");
requireCondition(scheduleHeaderIndex >= 0, "schedule.txt에 실행 순서 표가 없습니다");
const passedScheduleRows = scheduleLines.slice(scheduleHeaderIndex + 1)
  .filter((line) => line.trim())
  .map((line) => line.split("\t"))
  .filter((row) => row[3] === "PASS");
requireCondition(passedScheduleRows.length === 10, `schedule PASS 행이 10개가 아닙니다: ${passedScheduleRows.length}`);
for (let index = 0; index < passedScheduleRows.length; index += 1) {
  requireCondition(passedScheduleRows[index].length === 6, `schedule 행의 열 수가 잘못됐습니다: ${passedScheduleRows[index].join("\t")}`);
}

const fields = ["queue_residence_us", "v8_call_us", "actor_before_reply_us", "response_wait_us"];

function parseReports(directory, expectedMode, expectedMatrix) {
  const metadata = readFileSync(resolve(directory, "metadata.txt"), "utf8");
  if (!metadata.includes("device_kind=에뮬레이터") || !metadata.includes("android_api=36")) {
    throw new Error(`에뮬레이터/API metadata가 맞지 않습니다: ${directory}`);
  }
  const metadataValues = readKeyValues(resolve(directory, "metadata.txt"));
  requireCondition(metadataValues.scenario === "spinon-event", `spinon-event 조건이 아닙니다: ${directory}`);
  requireCondition(metadataValues.perfetto_enabled === String(expectedMode === "on"), `Perfetto ${expectedMode} metadata가 다릅니다: ${directory}`);
  requireCondition(metadataValues.adb_serial === expectedMatrix.adb_serial, `adb serial이 행렬 metadata와 다릅니다: ${directory}`);
  requireCondition(metadataValues.device_model === expectedMatrix.device_model, `모델이 행렬 metadata와 다릅니다: ${directory}`);
  requireCondition(metadataValues.android_api === expectedMatrix.android_api, `API가 행렬 metadata와 다릅니다: ${directory}`);
  requireCondition(metadataValues.duration_seconds === expectedMatrix.duration_seconds, `측정 시간이 다릅니다: ${directory}`);
  requireCondition(metadataValues.input_count === expectedMatrix.input_count, `입력 수가 다릅니다: ${directory}`);
  requireCondition(metadataValues.app_pid === metadataValues.app_pid_end, `실행 중 앱 PID가 바뀌었습니다: ${directory}`);
  requireCondition(metadataValues.source_tree_sha256 === expectedMatrix.source_tree_sha256,
    `행렬 시작 시 source digest와 run이 다릅니다: ${directory}`);
  requireCondition(metadataValues.diagnostic_ui_mutations_suppressed === "true", `진단 UI 변경 억제 조건이 다릅니다: ${directory}`);
  requireCondition(/^[0-9a-f]{64}$/.test(metadataValues.source_tree_sha256 ?? ""), `source digest가 없습니다: ${directory}`);
  requireCondition(existsSync(resolve(directory, "installed-apk.sha256")), `설치 APK checksum이 없습니다: ${directory}`);
  const apkHash = readFileSync(resolve(directory, "installed-apk.sha256"), "utf8").trim().split(/\s+/)[0];
  requireCondition(apkHash === expectedMatrix.apk_sha256, `설치 APK가 행렬 빌드와 다릅니다: ${directory}`);

  let traceValidation = null;
  if (expectedMode === "on") {
    const validationPath = resolve(directory, "trace-validation.json");
    requireCondition(existsSync(validationPath), `Perfetto trace 품질 검증 결과가 없습니다: ${directory}`);
    const validation = JSON.parse(readFileSync(validationPath, "utf8"));
    const trace = readFileSync(resolve(directory, "frame-attribution.pftrace"));
    const traceHash = createHash("sha256").update(trace).digest("hex");
    requireCondition(validation.status === "pass", `Perfetto trace 검증이 통과하지 않았습니다: ${directory}`);
    requireCondition(validation.trace_sha256 === traceHash, `검증 후 Perfetto 원본이 바뀌었습니다: ${directory}`);
    requireCondition(validation.processor_version === expectedMatrix.trace_processor_version, `Trace Processor 버전이 행렬과 다릅니다: ${directory}`);
    requireCondition(validation.checks?.nonzero_data_loss === 0 && validation.checks?.nonzero_error_stats === 0,
      `Perfetto packet/data loss 또는 오류 통계가 있습니다: ${directory}`);
    requireCondition(validation.checks?.runtime_dispatch_markers === Number(expectedMatrix.input_count), `runtime 표식 수가 입력 수와 다릅니다: ${directory}`);
    requireCondition(validation.checks?.v8_handler_markers === Number(expectedMatrix.input_count), `V8 표식 수가 입력 수와 다릅니다: ${directory}`);
    requireCondition(validation.checks?.runtime_completion_markers === Number(expectedMatrix.input_count), `완료 표식 수가 입력 수와 다릅니다: ${directory}`);
    requireCondition(validation.checks?.runtime_dispatch_counter === Number(expectedMatrix.input_count), `runtime counter가 입력 수와 다릅니다: ${directory}`);
    requireCondition(validation.checks?.display_frame_tokens > 0 && validation.checks?.surface_frame_tokens > 0,
      `FrameTimeline token이 없습니다: ${directory}`);
    traceValidation = validation;
  } else {
    requireCondition(existsSync(resolve(directory, "perfetto-disabled.txt")), `Perfetto off marker가 없습니다: ${directory}`);
    requireCondition(!existsSync(resolve(directory, "frame-attribution.pftrace")), `Perfetto off 조건에 trace 원본이 있습니다: ${directory}`);
  }

  const log = readFileSync(resolve(directory, "logcat.txt"), "utf8");
  const lines = log.split(/\r?\n/).filter((line) => line.includes("SPINON_RUNTIME_DISPATCH="));
  if (lines.length !== 10) throw new Error(`dispatch report 10개가 아닙니다 (${lines.length}): ${directory}`);
  const reports = lines.map((line) => {
    const payload = line.slice(line.indexOf("SPINON_RUNTIME_DISPATCH=") + "SPINON_RUNTIME_DISPATCH=".length);
    return Object.fromEntries([...payload.matchAll(/([a-z0-9_]+)=([^\s]+)/g)].map((match) => [match[1], match[2]]));
  }).sort((left, right) => Number(left.seq) - Number(right.seq));
  for (let index = 0; index < reports.length; index += 1) {
    const report = reports[index];
    if (report.status !== "0" || !Number.isInteger(Number(report.seq))
      || (index > 0 && Number(report.seq) !== Number(reports[index - 1].seq) + 1)) {
      throw new Error(`status/연속 순번 불일치: ${directory} #${index + 1}`);
    }
    for (const field of fields) {
      if (!Number.isFinite(Number(report[field]))) throw new Error(`${field} 누락: ${directory} #${index + 1}`);
    }
  }
  return { reports, metadata: metadataValues, traceValidation };
}

function median(values) {
  const sorted = [...values].sort((left, right) => left - right);
  const middle = Math.floor(sorted.length / 2);
  return sorted.length % 2 === 0
    ? (sorted[middle - 1] + sorted[middle]) / 2
    : sorted[middle];
}

function describe(values) {
  const min = Math.min(...values);
  const max = Math.max(...values);
  return `${median(values).toFixed(3)} (${min.toFixed(3)}–${max.toFixed(3)})`;
}

const paired = pairRows.map(([roundText, attemptText, order, onDir, offDir], index) => {
  requireCondition(pairRows[index].length === 5, `pairs.tsv 열 수가 잘못됐습니다: 회차 ${roundText}`);
  const round = Number(roundText);
  requireCondition(round === index + 1 && !seenRounds.has(round), `회차가 중복되거나 1..10 순서가 아닙니다: ${roundText}`);
  seenRounds.add(round);
  requireCondition(Number.isInteger(Number(attemptText)) && Number(attemptText) >= 1 && Number(attemptText) <= 3,
    `시도 번호가 1..3 범위가 아닙니다: ${attemptText}`);
  requireCondition(orderCounts.has(order), `실행 순서 표기가 잘못됐습니다: ${order}`);
  orderCounts.set(order, orderCounts.get(order) + 1);
  requireCondition(onDir && offDir, `회차 ${round} 실행 경로가 비어 있습니다`);
  const onPath = resolve(onDir);
  const offPath = resolve(offDir);
  const scheduled = passedScheduleRows[index];
  requireCondition(scheduled[0] === roundText && scheduled[1] === attemptText && scheduled[2] === order
    && scheduled[4] === onDir && scheduled[5] === offDir, `schedule과 pairs.tsv가 다릅니다: 회차 ${round}`);
  const onResult = parseReports(onPath, "on", matrixMetadata);
  const offResult = parseReports(offPath, "off", matrixMetadata);
  const onStarted = Date.parse(onResult.metadata.started_utc);
  const offStarted = Date.parse(offResult.metadata.started_utc);
  requireCondition(Number.isFinite(onStarted) && Number.isFinite(offStarted) && onStarted !== offStarted,
    `짝 안의 실행 시작 시각을 확인할 수 없습니다: 회차 ${round}`);
  const observedOrder = onStarted < offStarted ? "on/off" : "off/on";
  requireCondition(observedOrder === order, `schedule 순서와 metadata 시작 시각이 다릅니다: 회차 ${round}`);
  const onReports = onResult.reports;
  const offReports = offResult.reports;
  requireCondition(onResult.metadata.source_tree_sha256 === offResult.metadata.source_tree_sha256,
    `source tree가 쌍 안에서 달라졌습니다: 회차 ${round}`);
  requireCondition(onResult.metadata.tap_x === offResult.metadata.tap_x && onResult.metadata.tap_y === offResult.metadata.tap_y,
    `입력 좌표가 쌍 안에서 달라졌습니다: 회차 ${round}`);
  if (onReports.map((report) => report.seq).join(",") !== offReports.map((report) => report.seq).join(",")) {
    throw new Error(`짝 안에서 dispatch 순번이 다릅니다: 회차 ${round}`);
  }
  const values = {};
  for (const field of fields) {
    const onMedian = median(onReports.map((report) => Number(report[field])));
    const offMedian = median(offReports.map((report) => Number(report[field])));
    values[field] = { onMedian, offMedian, delta: onMedian - offMedian };
  }
  return {
    round,
    attempt: Number(attemptText),
    order,
    onDir: onPath,
    offDir: offPath,
    traceValidation: onResult.traceValidation,
    values,
  };
});
requireCondition(orderCounts.get("on/off") === 5 && orderCounts.get("off/on") === 5,
  `AB/BA 순서가 5회씩 균형을 이루지 않습니다: ${JSON.stringify(Object.fromEntries(orderCounts))}`);

const lines = [
  "# R05 · Android 에뮬레이터 계측 오버헤드 요약",
  "",
  `행렬: \`${matrixDir}\``,
  "",
  "같은 debug APK와 Activity에서 Perfetto session 유무만 바꾼 짝 비교다. 각 조건은 10회 dispatch이고, 표는 실행 내 10개 값의 중앙값을 먼저 계산한 뒤 10쌍의 중앙값 차이(`on - off`)를 요약한다. 괄호는 10개 pair의 최소–최대다. 양수는 trace-on에서 더 큰 벽시계 값, 음수는 더 작은 값을 뜻한다.",
  "",
  "이 표본은 진단 오버헤드 관찰이며 p95/p99, 실기기·release·제품 renderer 성능, 인과적 플랫폼 비용을 확정하지 않는다.",
  "",
  "| 지표 (µs) | trace on 중앙값 (범위) | trace off 중앙값 (범위) | 쌍별 차이 중앙값 (범위) |",
  "|---|---:|---:|---:|",
];

for (const field of fields) {
  const onValues = paired.map((pair) => pair.values[field].onMedian);
  const offValues = paired.map((pair) => pair.values[field].offMedian);
  const deltas = paired.map((pair) => pair.values[field].delta);
  lines.push(`| \`${field}\` | ${describe(onValues)} | ${describe(offValues)} | ${describe(deltas)} |`);
}

lines.push("", "## 실행별 쌍 차이", "", "| 회차 | queue residence | V8 호출 | actor 응답 전 | 응답 대기 |", "|---:|---:|---:|---:|---:|");
for (const pair of paired) {
  lines.push(`| ${pair.round} | ${pair.values.queue_residence_us.delta.toFixed(3)} | ${pair.values.v8_call_us.delta.toFixed(3)} | ${pair.values.actor_before_reply_us.delta.toFixed(3)} | ${pair.values.response_wait_us.delta.toFixed(3)} |`);
}
const traceValidations = paired.map((pair) => pair.traceValidation);
const displayTokens = traceValidations.map((validation) => validation.checks.display_frame_tokens);
const surfaceTokens = traceValidations.map((validation) => validation.checks.surface_frame_tokens);
lines.push(
  "",
  "## Perfetto 원본 품질",
  "",
  `trace-on 원본 ${traceValidations.length}개를 ${matrixMetadata.trace_processor_version}로 읽었다. packet/data loss, error 통계, ftrace 설정 오류는 모든 원본에서 0이었다. runtime/V8/completion 표식과 입력·dispatch counter는 각 10개로 일치했다. FrameTimeline 고유 token 범위는 display ${Math.min(...displayTokens)}–${Math.max(...displayTokens)}, surface ${Math.min(...surfaceTokens)}–${Math.max(...surfaceTokens)}개다.`,
  "",
  "## 한계",
  "",
  "- 에뮬레이터의 CPU·표시 경로는 Mac host 가상화의 영향을 받는다.",
  "- trace-off에서도 앱이 `android.os.Trace` 함수를 호출하지만 활성 Perfetto session이 없어 기록하지 않는다.",
  "- `response_wait_us`와 actor wall time은 스레드 CPU 시간이 아니다.",
  "- on 실행만 Perfetto FrameTimeline을 갖는다. 이를 trace-off `gfxinfo`와 동일 지표로 간주하지 않았다.",
  "",
);

const report = lines.join("\n");
const outputPath = resolve(matrixDir, "summary.md");
writeFileSync(outputPath, report);
console.log(report);
console.log(`\n저장: ${outputPath}`);
