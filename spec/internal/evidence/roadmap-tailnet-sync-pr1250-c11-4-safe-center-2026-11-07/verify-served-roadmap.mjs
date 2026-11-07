import { createHash } from "node:crypto";
import fs from "node:fs";
import path from "node:path";

const root = process.cwd();
const evidence = path.join(root, "spec/internal/evidence/roadmap-tailnet-sync-pr1250-c11-4-safe-center-2026-11-07");
const source = fs.readFileSync(path.join(root, "spec/public/roadmap.html"));
const targetPath = "/Users/yoonhb/Library/Application Support/SpinonPreview/roadmap.html";
const target = fs.readFileSync(targetPath);
const hash = (bytes) => createHash("sha256").update(bytes).digest("hex");
const sourceHash = hash(source);
if (hash(target) !== sourceHash) throw new Error("preview target differs from source bytes");
const text = source.toString("utf8");
const markers = [
  "PR #1249 Rebase and merge",
  "a69030722d82a97e6ce40e276c0aae2b8447550d",
  "78/214",
  "C11.4/C11은 미완료",
  "align-content:safe|unsafe center",
  "align-content:safe|unsafe end",
];
for (const marker of markers) if (!text.includes(marker)) throw new Error(`source lacks marker: ${marker}`);
const urls = ["http://127.0.0.1:8768/spinon/roadmap.html", "https://macstudio.tailed42f2.ts.net/spinon/roadmap.html"];
const served = [];
for (const url of urls) {
  const response = await fetch(url, { cache: "no-store" });
  const bytes = Buffer.from(await response.arrayBuffer());
  const item = { url, status: response.status, bytes: bytes.length, sha256: hash(bytes) };
  served.push(item);
  if (response.status !== 200 || item.sha256 !== sourceHash) throw new Error(`served content mismatch: ${JSON.stringify(item)}`);
}
const result = { recordedAt: new Date().toISOString(), source: "spec/public/roadmap.html", sourceBytes: source.length, sourceSha256: sourceHash, target: targetPath, targetBytes: target.length, targetSha256: hash(target), served, requiredMarkers: markers, allMarkersPresent: true, githubPagesDeployed: false };
fs.writeFileSync(path.join(evidence, "served-verification.json"), `${JSON.stringify(result, null, 2)}\n`);
process.stdout.write(`${JSON.stringify(result)}\n`);
