import http from 'node:http';
import fs from 'node:fs';
import path from 'node:path';

const root = path.resolve(process.argv[2] ?? '');
if (!process.argv[2] || !fs.statSync(root).isDirectory()) {
  throw new Error('pass the selected RSPress output directory as the first argument');
}

const contentTypes = new Map([
  ['.css', 'text/css; charset=utf-8'],
  ['.html', 'text/html; charset=utf-8'],
  ['.js', 'text/javascript; charset=utf-8'],
  ['.json', 'application/json; charset=utf-8'],
  ['.png', 'image/png'],
  ['.svg', 'image/svg+xml'],
]);

const server = http.createServer((request, response) => {
  let pathname;
  try {
    pathname = decodeURIComponent(new URL(request.url, 'http://localhost').pathname);
  } catch {
    response.writeHead(400).end('bad request');
    return;
  }
  if (!pathname.startsWith('/spinon/')) {
    response.writeHead(404).end('not found');
    return;
  }
  let relativePath = pathname.slice('/spinon/'.length) || 'index.html';
  if (relativePath.endsWith('/')) relativePath += 'index.html';
  else if (!path.extname(relativePath)) relativePath += '.html';
  const target = path.resolve(root, relativePath);
  if (!target.startsWith(`${root}${path.sep}`)) {
    response.writeHead(403).end('forbidden');
    return;
  }
  fs.stat(target, (error, stat) => {
    if (error || !stat.isFile()) {
      response.writeHead(404).end('not found');
      return;
    }
    response.writeHead(200, {
      'content-type': contentTypes.get(path.extname(target)) ?? 'application/octet-stream',
      'content-length': stat.size,
      'cache-control': 'no-store',
    });
    fs.createReadStream(target).pipe(response);
  });
});

server.listen(0, '127.0.0.1', () => {
  process.stdout.write(`http://127.0.0.1:${server.address().port}/spinon/\n`);
});
