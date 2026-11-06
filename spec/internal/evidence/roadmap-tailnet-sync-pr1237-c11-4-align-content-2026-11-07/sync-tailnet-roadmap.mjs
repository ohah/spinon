import { execFileSync } from "node:child_process";
import { createHash } from "node:crypto";
import fs from "node:fs";
import path from "node:path";

const root = process.cwd();
const evidence = path.join(
  root,
  "spec/internal/evidence/roadmap-tailnet-sync-pr1237-c11-4-align-content-2026-11-07",
);
const pass = process.argv[2] ?? "pass01";
if (!/^pass\d{2}$/.test(pass)) throw new Error(`invalid backup pass: ${pass}`);
const source = path.join(root, "spec/public/roadmap.html");
const target =
  "/Users/yoonhb/Library/Application Support/SpinonPreview/roadmap.html";
const backup = path.join(
  "/Users/yoonhb/Library/Application Support/SpinonPreviewBackups/roadmap-tailnet-sync-pr1237-c11-4-align-content-2026-11-07",
  pass,
  "roadmap.html",
);
const backupDir = path.dirname(backup);
const hash = (bytes) => createHash("sha256").update(bytes).digest("hex");
const run = (command, args) =>
  execFileSync(command, args, { encoding: "utf8", stdio: ["ignore", "pipe", "pipe"] });
const sourceBytes = fs.readFileSync(source);
const beforeBytes = fs.readFileSync(target);
const sourceHash = hash(sourceBytes);
const beforeHash = hash(beforeBytes);
const sourceText = sourceBytes.toString("utf8");
for (const marker of [
  "PR #1237 Rebase and merge",
  "fddf66295a1f66134a061c0c221410964cb3077a",
  "78/214",
  "C11.4/C11",
]) {
  if (!sourceText.includes(marker)) throw new Error(`roadmap source lacks expected marker: ${marker}`);
}
const passSuffix = pass === "pass01" ? "" : `-${pass}`;
const targetStat = fs.statSync(target);
if (!targetStat.isFile()) throw new Error("preview target is not a regular file");
if (fs.existsSync(backupDir) || fs.existsSync(backup)) {
  throw new Error(`backup path already exists: ${backupDir}`);
}
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
const listener = run("lsof", ["-nP", "-iTCP:8768", "-sTCP:LISTEN"])
  .trim()
  .split("\n");
if (listener.length < 2 || !listener[1].includes("28040")) {
  throw new Error(`unexpected preview listener: ${listener.join(" | ")}`);
}

fs.writeFileSync(
  path.join(evidence, `preflight${passSuffix}.json`),
  `${JSON.stringify(
    {
      recordedAt: new Date().toISOString(),
      repositoryHead: run("git", ["rev-parse", "HEAD"]).trim(),
      source: path.relative(root, source),
      sourceBytes: sourceBytes.length,
      sourceSha256: sourceHash,
      previewTarget: target,
      oldPreviewBytes: beforeBytes.length,
      oldPreviewSha256: beforeHash,
      backupPath: backup,
      tailscaleBackend: tailscale.BackendState,
      previewListener: listener[1],
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
  throw new Error("backup does not match previous preview bytes");
}
const temp = path.join(
  path.dirname(target),
  `.roadmap.html.c11-align-content-${process.pid}.tmp`,
);
const fd = fs.openSync(temp, "wx", targetStat.mode & 0o777);
try {
  fs.writeFileSync(fd, sourceBytes);
  fs.fsyncSync(fd);
} finally {
  fs.closeSync(fd);
}
fs.renameSync(temp, target);
const afterBytes = fs.readFileSync(target);
const afterHash = hash(afterBytes);
if (afterHash !== sourceHash) {
  const restoreTemp = `${target}.restore-${process.pid}.tmp`;
  fs.copyFileSync(backup, restoreTemp, fs.constants.COPYFILE_EXCL);
  fs.renameSync(restoreTemp, target);
  throw new Error("preview verification failed; previous preview restored");
}
fs.writeFileSync(
  path.join(evidence, `copy-and-backup${passSuffix}.json`),
  `${JSON.stringify(
    {
      recordedAt: new Date().toISOString(),
      source: path.relative(root, source),
      sourceBytes: sourceBytes.length,
      sourceSha256: sourceHash,
      previewTarget: target,
      previewBytesAfter: afterBytes.length,
      previewSha256After: afterHash,
      previousPreviewBytes: beforeBytes.length,
      previousPreviewSha256: beforeHash,
      backupPath: backup,
      backupBytes: fs.statSync(backup).size,
      backupSha256: hash(fs.readFileSync(backup)),
      replacement: "same-directory temp file and atomic rename",
      serverRestarted: false,
      filesChanged: ["/spinon/roadmap.html"],
    },
    null,
    2,
  )}\n`,
);
process.stdout.write(`${JSON.stringify({ sourceHash, afterHash, backup })}\n`);
