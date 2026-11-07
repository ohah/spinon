import { execFileSync } from "node:child_process";
import { createHash } from "node:crypto";
import fs from "node:fs";
import path from "node:path";

const root = process.cwd();
const evidence = path.join(
  root,
  "spec/internal/evidence/roadmap-tailnet-c11-safe-flex-end-in-progress-2026-11-07",
);
const syncPass = process.argv[2] ?? "pass01";
if (!/^pass[0-9]{2}$/.test(syncPass)) {
  throw new Error("sync pass must use passNN format");
}
const source = path.join(root, "spec/public/roadmap.html");
const previewRoot = "/Users/yoonhb/Library/Application Support/SpinonPreview";
const target = path.join(previewRoot, "roadmap.html");
const backup = path.join(
  "/Users/yoonhb/Library/Application Support/SpinonPreviewBackups/roadmap-tailnet-c11-safe-flex-end-in-progress-2026-11-07",
  syncPass,
  "roadmap.html",
);
const preflightPath = path.join(
  evidence,
  syncPass === "pass01" ? "preflight.json" : `preflight-${syncPass}.json`,
);
const copyPath = path.join(
  evidence,
  syncPass === "pass01" ? "copy-and-backup.json" : `copy-and-backup-${syncPass}.json`,
);
const backupDir = path.dirname(backup);
const urls = [
  "http://127.0.0.1:8768/spinon/roadmap.html",
  "https://macstudio.tailed42f2.ts.net/spinon/roadmap.html",
];
const hash = (bytes) => createHash("sha256").update(bytes).digest("hex");
const run = (command, args) =>
  execFileSync(command, args, {
    encoding: "utf8",
    stdio: ["ignore", "pipe", "pipe"],
  });
const served = async (expectedHash) => {
  const records = [];
  for (const url of urls) {
    const response = await fetch(url, { cache: "no-store" });
    const bytes = Buffer.from(await response.arrayBuffer());
    const record = {
      url,
      status: response.status,
      bytes: bytes.length,
      sha256: hash(bytes),
    };
    records.push(record);
    if (response.status !== 200 || record.sha256 !== expectedHash) {
      throw new Error(`served roadmap mismatch: ${JSON.stringify(record)}`);
    }
  }
  return records;
};

const sourceBytes = fs.readFileSync(source);
const sourceText = sourceBytes.toString("utf8");
const sourceHash = hash(sourceBytes);
for (const marker of [
  "flex-end 구현 중",
  "78/214",
  "C11.4/C11은 미완료",
]) {
  if (!sourceText.includes(marker)) {
    throw new Error(`roadmap source lacks expected marker: ${marker}`);
  }
}
if (fs.existsSync(backupDir) || fs.existsSync(backup)) {
  throw new Error(`backup path already exists: ${backupDir}`);
}
const targetStat = fs.statSync(target);
if (!targetStat.isFile()) throw new Error("preview target is not a regular file");
const previousBytes = fs.readFileSync(target);
const previousHash = hash(previousBytes);
const previousServed = await served(previousHash);
const tailscale = JSON.parse(run("tailscale", ["status", "--json"]));
if (tailscale.BackendState !== "Running") {
  throw new Error(`Tailscale backend is ${tailscale.BackendState}`);
}
const listenerLines = run("lsof", ["-nP", "-iTCP:8768", "-sTCP:LISTEN"])
  .trim()
  .split("\n");
if (listenerLines.length < 2 || !listenerLines[1].includes("127.0.0.1:8768")) {
  throw new Error(`unexpected preview listener: ${listenerLines.join(" | ")}`);
}
const listenerPid = listenerLines[1].trim().split(/\s+/)[1];
const serverCommand = run("ps", ["-p", listenerPid, "-o", "command="]).trim();
if (!serverCommand.includes("tailnet-preview-server.mjs")) {
  throw new Error(`unexpected preview server command: ${serverCommand}`);
}
let openHandles = [];
try {
  openHandles = run("lsof", ["-t", target]).split(/\s+/).filter(Boolean);
} catch (error) {
  if (error.status !== 1) throw error;
}
fs.writeFileSync(
  preflightPath,
  `${JSON.stringify(
    {
      recordedAt: new Date().toISOString(),
      repositoryHead: run("git", ["rev-parse", "HEAD"]).trim(),
      source: path.relative(root, source),
      sourceBytes: sourceBytes.length,
      sourceSha256: sourceHash,
      previousPreviewBytes: previousBytes.length,
      previousPreviewSha256: previousHash,
      previousServedResponses: previousServed,
      backupPath: backup,
      tailscaleBackend: tailscale.BackendState,
      previewListener: listenerLines[1],
      previewServerCommand: serverCommand,
      previewTargetOpenHandles: openHandles,
      previewServerRestarted: false,
      pagesDeployment: "not requested; unchanged",
    },
    null,
    2,
  )}\n`,
);

fs.mkdirSync(backupDir, { recursive: true });
fs.copyFileSync(target, backup, fs.constants.COPYFILE_EXCL);
if (hash(fs.readFileSync(backup)) !== previousHash) {
  throw new Error("backup does not match old preview bytes");
}
const temp = path.join(
  previewRoot,
  `.roadmap.html.c11-safe-flex-end-${process.pid}.tmp`,
);
const fd = fs.openSync(temp, "wx", targetStat.mode & 0o777);
try {
  fs.writeFileSync(fd, sourceBytes);
  fs.fsyncSync(fd);
} finally {
  fs.closeSync(fd);
}
fs.renameSync(temp, target);
try {
  const targetHash = hash(fs.readFileSync(target));
  if (targetHash !== sourceHash) {
    throw new Error(`target hash mismatch: ${targetHash}`);
  }
  const servedResponses = await served(sourceHash);
  fs.writeFileSync(
    copyPath,
    `${JSON.stringify(
      {
        recordedAt: new Date().toISOString(),
        source: path.relative(root, source),
        sourceBytes: sourceBytes.length,
        sourceSha256: sourceHash,
        previewTarget: target,
        previewBytesAfter: fs.statSync(target).size,
        previewSha256After: targetHash,
        servedResponsesAfter: servedResponses,
        previousPreviewBytes: previousBytes.length,
        previousPreviewSha256: previousHash,
        backupPath: backup,
        backupBytes: fs.statSync(backup).size,
        backupSha256: hash(fs.readFileSync(backup)),
        replacement: "same-directory temp file, fsync, atomic rename",
        serverPid: listenerPid,
        serverRestarted: false,
        filesChanged: ["/spinon/roadmap.html"],
      },
      null,
      2,
    )}\n`,
  );
  process.stdout.write(
    `${JSON.stringify({ sourceHash, targetHash, backup, serverPid: listenerPid })}\n`,
  );
} catch (error) {
  const restoreTemp = `${target}.restore-${process.pid}.tmp`;
  fs.copyFileSync(backup, restoreTemp, fs.constants.COPYFILE_EXCL);
  fs.renameSync(restoreTemp, target);
  throw error;
}
