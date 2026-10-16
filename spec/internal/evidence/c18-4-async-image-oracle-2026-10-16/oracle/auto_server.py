from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
import struct, zlib, time

def png(w,h,color):
    def chunk(kind,data):
        return struct.pack('>I',len(data))+kind+data+struct.pack('>I',zlib.crc32(kind+data)&0xffffffff)
    raw=b''.join(b'\x00'+bytes(color)*w for _ in range(h))
    return b'\x89PNG\r\n\x1a\n'+chunk(b'IHDR',struct.pack('>IIBBBBB',w,h,8,6,0,0,0))+chunk(b'IDAT',zlib.compress(raw))+chunk(b'IEND',b'')
A=png(2,2,(240,24,32,255)); C=png(4,2,(32,220,80,255))
PAGE=r'''<!doctype html><meta charset="utf-8"><title>C18.4 auto intrinsic oracle</title><style>html,body{margin:0;padding:0}img{display:block;width:auto;height:auto;max-width:none;max-height:none}</style><img id="target" alt=""><script>
const img=document.querySelector('#target'); img.crossOrigin='anonymous'; const wait=ms=>new Promise(r=>setTimeout(r,ms));
function state(label){const r=img.getBoundingClientRect();return {label,src:new URL(img.src).pathname,complete:img.complete,naturalWidth:img.naturalWidth,naturalHeight:img.naturalHeight,frame:[r.x,r.y,r.width,r.height],time:performance.now()}}
window.done=(async()=>{img.src='/a.png';await wait(40);const pendingA=state('A-pending');await img.decode();const afterA=state('A-decoded');img.src='/c.png';await wait(40);const pendingC=state('C-pending');await img.decode();const afterC=state('C-decoded');return {userAgent:navigator.userAgent,pendingA,afterA,pendingC,afterC}})();
</script>'''
class Handler(BaseHTTPRequestHandler):
    def do_GET(self):
        if self.path=='/':
            b=PAGE.encode();self.send_response(200);self.send_header('Content-Type','text/html; charset=utf-8');self.send_header('Content-Length',str(len(b)));self.end_headers();self.wfile.write(b);return
        if self.path in ['/a.png','/c.png']:
            time.sleep(.30);b=A if self.path=='/a.png' else C;self.send_response(200);self.send_header('Content-Type','image/png');self.send_header('Access-Control-Allow-Origin','*');self.send_header('Content-Length',str(len(b)));self.end_headers();self.wfile.write(b);return
        self.send_error(404)
    def log_message(self,*args): pass
ThreadingHTTPServer(('127.0.0.1',41785),Handler).serve_forever()
