import { createHash } from "node:crypto";
import fs from "node:fs";
import path from "node:path";

const expected = fs.readFileSync("spec/public/roadmap.html");
const expectedHash = createHash("sha256").update(expected).digest("hex");
const pass = process.argv[2] ?? "pass01";
if (!/^pass\d{2}$/.test(pass)) throw new Error(`invalid verification pass: ${pass}`);
const urls = [
  "http://127.0.0.1:8768/spinon/roadmap.html",
  "https://macstudio.tailed42f2.ts.net/spinon/roadmap.html",
];
const responses = [];
for (const url of urls) {
  const response = await fetch(url, { cache: "no-store" });
  const bytes = Buffer.from(await response.arrayBuffer());
  const sha256 = createHash("sha256").update(bytes).digest("hex");
  const text = bytes.toString("utf8");
  const result = {
    url,
    status: response.status,
    bytes: bytes.length,
    sha256,
    hasPr1237: text.includes("PR #1237 Rebase and merge"),
    hasMergeSha: text.includes("fddf66295a1f66134a061c0c221410964cb3077a"),
    hasOfficialCount: text.includes("78/214"),
    keepsC11Open: text.includes("C11.4/C11은 미완료"),
  };
  responses.push(result);
  if (
    response.status !== 200 ||
    sha256 !== expectedHash ||
    !result.hasPr1237 ||
    !result.hasMergeSha ||
    !result.hasOfficialCount ||
    !result.keepsC11Open
  ) {
    throw new Error(`served roadmap check failed: ${JSON.stringify(result)}`);
  }
}
const output = {
  recordedAt: new Date().toISOString(),
  source: "spec/public/roadmap.html",
  sourceBytes: expected.length,
  sourceSha256: expectedHash,
  responses,
};
const outputPath = path.join(
  process.cwd(),
  `spec/internal/evidence/roadmap-tailnet-sync-pr1237-c11-4-align-content-2026-11-07/served-verification${pass === "pass01" ? "" : `-${pass}`}.json`,
);
fs.writeFileSync(outputPath, `${JSON.stringify(output, null, 2)}\n`);
process.stdout.write(`${JSON.stringify(output)}\n`);
