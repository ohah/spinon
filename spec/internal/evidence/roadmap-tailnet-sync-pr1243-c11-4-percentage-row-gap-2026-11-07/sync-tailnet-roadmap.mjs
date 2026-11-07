import { execFileSync } from "node:child_process";
import { createHash } from "node:crypto";
import fs from "node:fs";
import path from "node:path";

const root = process.cwd();
const evidence = path.join(
  root,
  "spec/internal/evidence/roadmap-tailnet-sync-pr1243-c11-4-percentage-row-gap-2026-11-07",
);
const source = path.join(root, "spec/public/roadmap.html");
const target =
  "/Users/yoonhb/Library/Application Support/SpinonPreview/roadmap.html";
const backup = path.join(
  "/Users/yoonhb/Library/Application Support/SpinonPreviewBackups/roadmap-tailnet-sync-pr1243-c11-4-percentage-row-gap-2026-11-07",
  "pass01",
  "roadmap.html",
);
const backupDir = path.dirname(backup);
const hash = (bytes) => createHash("sha256").update(bytes).digest("hex");
const run = (command, args) =>
  execFileSync(command, args, {
    encoding: "utf8",
    stdio: ["ignore", "pipe", "pipe"],
  });
const urls = [
  "http://127.0.0.1:8768/spinon/roadmap.html",
  "https://macstudio.tailed42f2.ts.net/spinon/roadmap.html",
];
const sourceBytes = fs.readFileSync(source);
const sourceText = sourceBytes.toString("utf8");
const sourceHash = hash(sourceBytes);
for (const marker of [
  "PR #1243 Rebase and merge",
  "50d171d69b730af821076faf4ffcb02a4a5e44a8",
  "78/214",
  "C11.4/C11은 미완료",
  "roadmap-tailnet-sync-pr1243-c11-4-percentage-row-gap-2026-11-07/README.md",
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
let openHandleText = "";
try {
  openHandleText = run("lsof", ["-t", target]);
} catch (error) {
  if (error.status !== 1) throw error;
}
const openHandles = openHandleText.split(/\s+/).filter(Boolean);
if (openHandles.length !== 0) {
  throw new Error(`preview target has open handles: ${openHandles.join(",")}`);
}
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
const beforeBytes = fs.readFileSync(target);
const beforeHash = hash(beforeBytes);
const beforeServed = [];
for (const url of urls) {
  const response = await fetch(url, { cache: "no-store" });
  const bytes = Buffer.from(await response.arrayBuffer());
  const item = { url, status: response.status, bytes: bytes.length, sha256: hash(bytes) };
  beforeServed.push(item);
  if (response.status !== 200 || item.sha256 !== beforeHash) {
    throw new Error(`pre-copy served bytes do not match target: ${JSON.stringify(item)}`);
  }
}
fs.mkdirSync(evidence, { recursive: true });
fs.writeFileSync(
  path.join(evidence, "preflight.json"),
  `${JSON.stringify(
    {
      recordedAt: new Date().toISOString(),
      repositoryHead: run("git", ["rev-parse", "HEAD"]).trim(),
      source: path.relative(root, source),
      sourceBytes: sourceBytes.length,
      sourceSha256: sourceHash,
      previewTarget: target,
      previousPreviewBytes: beforeBytes.length,
      previousPreviewSha256: beforeHash,
      previousServedResponses: beforeServed,
      backupPath: backup,
      tailscaleBackend: tailscale.BackendState,
      previewListener: listenerLines[1],
      previewServerCommand: serverCommand,
      previewTargetOpenHandles: openHandles,
      pagesDeployment: "not requested; unchanged",
    },
    null,
    2,
  )}\n`,
);
fs.mkdirSync(backupDir, { recursive: true });
fs.copyFileSync(target, backup, fs.constants.COPYFILE_EXCL);
if (hash(fs.readFileSync(backup)) !== beforeHash) {
  throw new Error("backup does not match old preview bytes");
}
const temp = path.join(path.dirname(target), `.roadmap.html.c11-pr1243-${process.pid}.tmp`);
const fd = fs.openSync(temp, "wx", targetStat.mode & 0o777);
try {
  fs.writeFileSync(fd, sourceBytes);
  fs.fsyncSync(fd);
} finally {
  fs.closeSync(fd);
}
fs.renameSync(temp, target);
const afterHash = hash(fs.readFileSync(target));
if (afterHash !== sourceHash) {
  const restoreTemp = `${target}.restore-${process.pid}.tmp`;
  fs.copyFileSync(backup, restoreTemp, fs.constants.COPYFILE_EXCL);
  fs.renameSync(restoreTemp, target);
  throw new Error("preview target hash mismatch; previous bytes restored");
}
const afterServed = [];
for (const url of urls) {
  const response = await fetch(url, { cache: "no-store" });
  const bytes = Buffer.from(await response.arrayBuffer());
  const item = { url, status: response.status, bytes: bytes.length, sha256: hash(bytes) };
  afterServed.push(item);
  if (response.status !== 200 || item.sha256 !== sourceHash) {
    const restoreTemp = `${target}.restore-${process.pid}.tmp`;
    fs.copyFileSync(backup, restoreTemp, fs.constants.COPYFILE_EXCL);
    fs.renameSync(restoreTemp, target);
    throw new Error(`served preview mismatch; old bytes restored: ${JSON.stringify(item)}`);
  }
}
fs.writeFileSync(
  path.join(evidence, "copy-and-backup.json"),
  `${JSON.stringify(
    {
      recordedAt: new Date().toISOString(),
      source: path.relative(root, source),
      sourceBytes: sourceBytes.length,
      sourceSha256: sourceHash,
      previewTarget: target,
      previewBytesAfter: fs.statSync(target).size,
      previewSha256After: afterHash,
      servedResponsesAfter: afterServed,
      previousPreviewBytes: beforeBytes.length,
      previousPreviewSha256: beforeHash,
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
process.stdout.write(`${JSON.stringify({ sourceHash, afterHash, backup, serverPid: listenerPid })}\n`);
