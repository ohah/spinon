import { createHash } from "node:crypto";
import { readFile, writeFile } from "node:fs/promises";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const here = dirname(fileURLToPath(import.meta.url));
const source = JSON.parse(
  await readFile(join(here, "tailnet-final-verification-pass02.json"), "utf8"),
);
const origin = process.env.SPINON_LOCAL_PREVIEW_ORIGIN ?? "http://127.0.0.1:8768";
const prefix = source.remote_path_prefix;

const results = await Promise.all(
  source.targets.map(async ({ path, bytes: expectedBytes, sha256: expectedSha256 }) => {
    const response = await fetch(new URL(`${prefix}${path}`, origin));
    const body = Buffer.from(await response.arrayBuffer());
    const sha256 = createHash("sha256").update(body).digest("hex");
    return {
      path,
      http_status: response.status,
      bytes: body.length,
      sha256,
      verified:
        response.status === 200 &&
        body.length === expectedBytes &&
        sha256 === expectedSha256,
    };
  }),
);

const output = {
  local_origin: origin,
  route_prefix: prefix,
  route_count: results.length,
  local_http_200_and_hash_verified: results.filter((result) => result.verified).length,
  all_targets_verified: results.every((result) => result.verified),
  targets: results,
};
await writeFile(
  join(here, "local-http-verification-pass02.json"),
  `${JSON.stringify(output, null, 2)}\n`,
);
console.log(JSON.stringify(output, null, 2));
if (!output.all_targets_verified) process.exitCode = 1;
