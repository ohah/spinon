import fs from 'node:fs';
import http from 'node:http';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const evidence = path.dirname(fileURLToPath(import.meta.url));
const publicRoot = path.resolve(evidence, '../../../..', 'spec/public');
const files = new Map([
  ['/spinon/roadmap.html', 'roadmap.html'],
  ['/spinon/roadmap.svg', 'roadmap.svg'],
  ['/spinon/icon.svg', 'icon.svg'],
  ['/spinon/logo.svg', 'logo.svg'],
]);
const types = new Map([['.html', 'text/html; charset=utf-8'], ['.svg', 'image/svg+xml']]);
const server = http.createServer((request, response) => {
  const route = new URL(request.url ?? '/', 'http://127.0.0.1').pathname;
  const relative = files.get(route);
  if (!relative || (request.method !== 'GET' && request.method !== 'HEAD')) {
    response.writeHead(404).end('not found');
    return;
  }
  const file = path.join(publicRoot, relative);
  const bytes = fs.readFileSync(file);
  response.writeHead(200, { 'content-type': types.get(path.extname(file)), 'content-length': bytes.length, 'cache-control': 'no-cache' });
  if (request.method === 'HEAD') response.end();
  else response.end(bytes);
});
server.listen(8791, '127.0.0.1', () => process.stdout.write('local roadmap preview listening on 127.0.0.1:8791\n'));
