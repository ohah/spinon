from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
from pathlib import Path
import struct, zlib, time


def png(width, height, rgba):
    def chunk(kind, data):
        return struct.pack('>I', len(data)) + kind + data + struct.pack('>I', zlib.crc32(kind + data) & 0xffffffff)
    rows = b''.join(b'\x00' + bytes(rgba) * width for _ in range(height))
    return b'\x89PNG\r\n\x1a\n' + chunk(b'IHDR', struct.pack('>IIBBBBB', width, height, 8, 6, 0, 0, 0)) + chunk(b'IDAT', zlib.compress(rows)) + chunk(b'IEND', b'')

IMAGES = {
    '/a-slow.png': (0.50, png(2, 2, (240, 24, 32, 255))),
    '/b-fast.png': (0.02, png(1, 4, (24, 64, 240, 255))),
    '/c-slow.png': (0.50, png(4, 2, (32, 220, 80, 255))),
    '/d-fast.png': (0.02, png(2, 1, (200, 40, 220, 255))),
}
PAGE = r'''<!doctype html><meta charset="utf-8"><title>C18.4 image request oracle</title>
<style>html,body{margin:0;width:80px;height:48px;background:rgb(17,34,51)}img{display:block;width:48px;height:32px;object-fit:fill}</style>
<img id="target" alt=""><script>
const img=document.querySelector('#target'); img.crossOrigin='anonymous';
const events=[]; const results={userAgent:navigator.userAgent,events};
function pixels(){try{const c=document.createElement('canvas');c.width=1;c.height=1;const x=c.getContext('2d',{willReadFrequently:true});x.drawImage(img,0,0,1,1);return [...x.getImageData(0,0,1,1).data]}catch(e){return `${e.name}:${e.message}`}}
function state(label){return {label,src:new URL(img.currentSrc||img.src,location.href).pathname,complete:img.complete,naturalWidth:img.naturalWidth,naturalHeight:img.naturalHeight,rect:(()=>{const r=img.getBoundingClientRect();return [r.x,r.y,r.width,r.height]})(),pixel:img.naturalWidth?pixels():null}}
for(const name of ['load','error'])img.addEventListener(name,()=>events.push({event:name,...state('event'),time:performance.now()}));
const wait=ms=>new Promise(resolve=>setTimeout(resolve,ms));
async function load(path){img.src=path;try{await img.decode();return 'resolved'}catch(e){return `rejected:${e.name}`}}
window.done=(async()=>{
  const raceStart=performance.now(); img.src='/a-slow.png'; await wait(30); results.raceBeforeFast=state('slow-A-pending'); img.src='/b-fast.png'; results.bDecode=await (async()=>{try{await img.decode();return 'resolved'}catch(e){return `rejected:${e.name}`}})(); results.afterFast=state('fast-B-decoded'); await wait(550); results.afterSlowResponse=state('after-slow-A-response'); results.raceDurationMs=performance.now()-raceStart;
  img.src='/c-slow.png'; await wait(40); results.duringReplacement=state('slow-C-pending'); results.cDecode=await (async()=>{try{await img.decode();return 'resolved'}catch(e){return `rejected:${e.name}`}})(); results.afterC=state('slow-C-decoded');
  img.src='/broken.png'; results.brokenDecode=await (async()=>{try{await img.decode();return 'resolved'}catch(e){return `rejected:${e.name}`}})(); await wait(30); results.afterBroken=state('broken-404');
  results.dDecode=await load('/d-fast.png'); results.afterRecovery=state('recovery-D-decoded'); return results;
})();
</script>'''

class Handler(BaseHTTPRequestHandler):
    def do_GET(self):
        if self.path == '/':
            payload = PAGE.encode()
            self.send_response(200); self.send_header('Content-Type', 'text/html; charset=utf-8'); self.send_header('Content-Length', str(len(payload))); self.end_headers(); self.wfile.write(payload); return
        if self.path in IMAGES:
            delay, payload = IMAGES[self.path]; time.sleep(delay)
            self.send_response(200); self.send_header('Content-Type', 'image/png'); self.send_header('Access-Control-Allow-Origin', '*'); self.send_header('Content-Length', str(len(payload))); self.end_headers(); self.wfile.write(payload); return
        if self.path == '/broken.png':
            payload=b'not an image'
            self.send_response(200); self.send_header('Content-Type', 'image/png'); self.send_header('Access-Control-Allow-Origin', '*'); self.send_header('Content-Length', str(len(payload))); self.end_headers(); self.wfile.write(payload); return
        self.send_error(404)
    def log_message(self, format, *args):
        Path('/private/tmp/spinon-c18-4-oracle-20261016/server.log').open('a').write((format%args)+'\n')

ThreadingHTTPServer(('127.0.0.1', 41784), Handler).serve_forever()
