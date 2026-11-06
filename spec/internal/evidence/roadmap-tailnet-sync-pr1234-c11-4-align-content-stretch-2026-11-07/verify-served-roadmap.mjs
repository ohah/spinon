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
    hasPr1234: text.includes("PR #1234"),
    hasMergeSha: text.includes("0983a94cc80424b0f00f0e401286982125573e48"),
    hasOfficialCount: text.includes("78/214"),
  };
  responses.push(result);
  if (
    response.status !== 200 ||
    sha256 !== expectedHash ||
    !result.hasPr1234 ||
    !result.hasMergeSha ||
    !result.hasOfficialCount
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
  `spec/internal/evidence/roadmap-tailnet-sync-pr1234-c11-4-align-content-stretch-2026-11-07/served-verification${pass === "pass01" ? "" : `-${pass}`}.json`,
);
fs.writeFileSync(outputPath, `${JSON.stringify(output, null, 2)}\n`);
process.stdout.write(`${JSON.stringify(output)}\n`);
