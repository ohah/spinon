import { readFileSync, existsSync } from "node:fs";
import { resolve } from "node:path";

const runDir = resolve(process.argv[2] ?? "");
if (!process.argv[2]) {
  console.error("사용법: bun tools/benchmark/validate-ios-trace-overhead-run.mjs <실행 디렉터리>");
  process.exit(2);
}

function readKeyValue(file) {
  return Object.fromEntries(readFileSync(file, "utf8").split(/\r?\n/)
    .filter((line) => line.includes("="))
    .map((line) => {
      const separator = line.indexOf("=");
      return [line.slice(0, separator), line.slice(separator + 1)];
    }));
}

function requireCondition(condition, message) {
  if (!condition) throw new Error(message);
}

const metadata = readKeyValue(resolve(runDir, "metadata.txt"));
const mode = metadata.mode;
requireCondition(mode === "on" || mode === "off", `mode가 on/off가 아닙니다: ${mode}`);
requireCondition(metadata.app_pid && /^\d+$/.test(metadata.app_pid), "앱 PID metadata가 없습니다");
requireCondition(metadata.simulator_runtime === "iOS 26.2", "계획한 iOS 26.2 runtime이 아닙니다");
requireCondition(/^[0-9a-f]{64}$/.test(metadata.app_bundle_sha256 ?? ""), "앱 bundle digest가 없습니다");
requireCondition(/^[0-9a-f]{64}$/.test(metadata.app_binary_sha256 ?? ""), "앱 실행 파일 digest가 없습니다");
requireCondition(/^[0-9a-f]{64}$/.test(metadata.source_tree_sha256 ?? ""), "source tree digest가 없습니다");
const log = readFileSync(resolve(runDir, "unified-log.txt"), "utf8");
const startLines = log.split(/\r?\n/).filter((line) => line.includes("SPINON_R05_IOS_START"));
const sampleLines = log.split(/\r?\n/).filter((line) => line.includes("SPINON_R05_IOS_SAMPLE"));
const doneLines = log.split(/\r?\n/).filter((line) => line.includes("SPINON_R05_IOS_DONE"));
requireCondition(startLines.length === 1, `probe 시작 로그가 1개가 아닙니다: ${startLines.length}`);
requireCondition(sampleLines.length === 32, `probe sample이 32개가 아닙니다: ${sampleLines.length}`);
requireCondition(doneLines.length === 1 && doneLines[0].includes("samples=32"), "완료 sample 로그가 없습니다");
requireCondition(!log.includes("SPINON_R05_IOS_OVERLAP"), "sample overlap이 기록됐습니다");
requireCondition(!log.includes("SPINON_R05_IOS_REJECTED"), "runtime queue 거부가 기록됐습니다");

let previousEpochMs = Number.NEGATIVE_INFINITY;
const sampleInfo = sampleLines.map((line) => {
  const timestamp = line.match(/^(\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}\.\d+)/)?.[1];
  const pid = line.match(/SpinonBootstrap\[(\d+):/)?.[1];
  const sequence = Number(line.match(/SPINON_R05_IOS_SAMPLE seq=(\d+)/)?.[1]);
  const status = line.match(/\bstatus=(-?\d+)/)?.[1];
  requireCondition(timestamp && pid && Number.isFinite(sequence), `sample timestamp/PID/seq 누락: ${line}`);
  requireCondition(status === "0", `sample status가 0이 아닙니다: ${line}`);
  const epochMs = Date.parse(timestamp.replace(" ", "T"));
  requireCondition(Number.isFinite(epochMs), `sample timestamp를 해석할 수 없습니다: ${line}`);
  requireCondition(epochMs >= previousEpochMs, `sample 시간이 역행했습니다: ${line}`);
  previousEpochMs = epochMs;
  return { timestamp, pid, sequence, epochMs };
});
for (let index = 0; index < sampleInfo.length; index += 1) {
  requireCondition(sampleInfo[index].sequence === index + 1, `sample 순번이 1..32가 아닙니다: ${index + 1}`);
  requireCondition(sampleInfo[index].pid === metadata.app_pid, "로그 PID와 앱 PID metadata가 다릅니다");
}

if (mode === "on") {
  const tocPath = resolve(runDir, "trace-toc.xml");
  requireCondition(existsSync(tocPath), "trace-on에 Time Profiler TOC가 없습니다");
  const toc = readFileSync(tocPath, "utf8");
  requireCondition(toc.includes('<template-name>Time Profiler</template-name>'), "원본 template이 Time Profiler가 아닙니다");
  requireCondition(toc.includes(`name="SpinonBootstrap" pid="${metadata.app_pid}"`), "TOC 앱 PID가 실행 PID와 다릅니다");
  const start = toc.match(/<start-date>([^<]+)<\/start-date>/)?.[1];
  const end = toc.match(/<end-date>([^<]+)<\/end-date>/)?.[1];
  const startMs = Date.parse(start ?? "");
  const endMs = Date.parse(end ?? "");
  requireCondition(Number.isFinite(startMs) && Number.isFinite(endMs) && endMs > startMs, "TOC 실제 기록 구간이 올바르지 않습니다");
  requireCondition(sampleInfo.every((sample) => sample.epochMs >= startMs && sample.epochMs <= endMs), "32개 app sample 전체가 Time Profiler 기록 구간에 들어오지 않습니다");
  requireCondition(/schema="time-sample"/.test(toc), "Time Profiler CPU sample table이 없습니다");
} else {
  requireCondition(existsSync(resolve(runDir, "trace-disabled.txt")), "trace-off 기록이 없습니다");
  requireCondition(!existsSync(resolve(runDir, "time-profiler.trace")), "trace-off 실행에 profiler 원본이 있습니다");
}

console.log(`검증 통과 · iOS ${mode} · 32/32 status=0 · PID ${metadata.app_pid}`);
