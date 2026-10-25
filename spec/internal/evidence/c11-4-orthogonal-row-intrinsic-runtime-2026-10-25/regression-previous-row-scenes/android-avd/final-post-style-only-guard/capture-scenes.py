import hashlib
import json
import pathlib
import re
import struct
import subprocess
import sys
import time

BASE = pathlib.Path(__file__).resolve().parent
ROOT = pathlib.Path.cwd()
SERIAL = 'emulator-5570'
EXPECTED_HEAD = 'd37ee6ea8c985ab27eb32770491c8998dcbf756c'
EXPECTED_INDEX_TREE = '45bb3313f5c126fd724eb54e877f86746107a715'
EXPECTED_SOURCE_SHA = '2cae262c3b1cc8bfcb420b84313f53540909c711d834f001d69058dde249ce11'
EXPECTED_APK_SHA = 'cf978a383a3abceeaa99fe37d2f5c202f859193f20a352d4eea67dd0626371af'

SCENES = [
    ('direct-orthogonal-row-min-content-forward', 'c114-final-orth-min-f', 'orth', 'min', 'forward'),
    ('direct-orthogonal-row-min-content-reversed', 'c114-final-orth-min-r', 'orth', 'min', 'reversed'),
    ('direct-orthogonal-row-max-content-forward', 'c114-final-orth-max-f', 'orth', 'max', 'forward'),
    ('direct-orthogonal-row-max-content-reversed', 'c114-final-orth-max-r', 'orth', 'max', 'reversed'),
    ('direct-min-content-forward', 'c114-final-col-min-f', 'col', 'min', 'forward'),
    ('direct-min-content-reversed', 'c114-final-col-min-r', 'col', 'min', 'reversed'),
    ('direct-max-content-forward', 'c114-final-col-max-f', 'col', 'max', 'forward'),
    ('direct-max-content-reversed', 'c114-final-col-max-r', 'col', 'max', 'reversed'),
    ('direct-row-min-content-forward', 'c114-final-row-min-f', 'row', 'min', 'forward'),
    ('direct-row-min-content-reversed', 'c114-final-row-min-r', 'row', 'min', 'reversed'),
    ('direct-row-max-content-forward', 'c114-final-row-max-f', 'row', 'max', 'forward'),
    ('direct-row-max-content-reversed', 'c114-final-row-max-r', 'row', 'max', 'reversed'),
]


def run(*args, check=True, text=True):
    result = subprocess.run(['adb', '-s', SERIAL, *args], stdout=subprocess.PIPE,
                            stderr=subprocess.STDOUT, text=text)
    if check and result.returncode:
        raise RuntimeError(f'adb -s {SERIAL} {args!r} failed: {result.stdout!r}')
    return result


def host(*args):
    return subprocess.check_output(args)


def sha(data):
    return hashlib.sha256(data).hexdigest()


def assert_snapshot():
    snap = json.loads((BASE / 'source-before-build.json').read_text())
    head = host('git', 'rev-parse', 'HEAD').decode().strip()
    tree = host('git', 'write-tree').decode().strip()
    diff = host('git', 'diff', '--cached', '--binary')
    if head != EXPECTED_HEAD or tree != EXPECTED_INDEX_TREE:
        raise RuntimeError(f'HEAD/index changed: {head}/{tree}')
    if sha(diff) != snap['stagedDiffSha256'] or snap['sourceSnapshotSha256'] != EXPECTED_SOURCE_SHA:
        raise RuntimeError('staged source snapshot changed')
    for item in snap['sourceFiles']:
        p = ROOT / item['path']
        actual = sha(p.read_bytes()) if p.is_file() else None
        if actual != item['worktreeSha256'] or actual != item['indexSha256']:
            raise RuntimeError(f"source changed: {item['path']}")
    if sha((ROOT / 'platforms/android/app/build/outputs/apk/debug/app-debug.apk').read_bytes()) != EXPECTED_APK_SHA:
        raise RuntimeError('local APK changed')
    if not snap['sourceFiles']:
        raise RuntimeError('empty source manifest')


def frame(scene_kind, size_mode, direction):
    if scene_kind == 'orth':
        root_h = 50 if size_mode == 'min' else 80
        text_w, text_h, image_x, image_y = 50, root_h, 50, 0
        root_w = 74
    elif scene_kind == 'col':
        root_h = 48
        text_w, text_h, image_x, image_y = (50, 20, 0, 24) if size_mode == 'min' else (80, 10, 0, 24)
        root_w = 120
    else:
        root_h = 20
        text_w, text_h, image_x, image_y, root_w = 50, 20, 50, 0, 74
    text_node, image_node = (2, 4) if direction == 'forward' else (3, 2)
    return {
        'root': {'node': 1, 'x': 0, 'y': 0, 'width': root_w, 'height': root_h},
        'text': {'node': text_node, 'x': 0, 'y': 0, 'width': text_w, 'height': text_h},
        'image': {'node': image_node, 'x': image_x, 'y': image_y, 'width': 24, 'height': 16},
        'columns': [50.0, 24.0] if scene_kind != 'col' else [50.0 if size_mode == 'min' else 80.0],
        'rows': [root_h] if scene_kind != 'col' else [24.0, 24.0],
        'contribution_axis': 'Rows' if scene_kind != 'col' else 'Columns',
        'text_min': 50.0 if scene_kind == 'orth' else (50.0 if scene_kind == 'col' else 20.0),
        'text_max': 80.0 if scene_kind in ('orth', 'col') else 20.0,
        'image_contribution': 16.0 if scene_kind == 'orth' else (24.0 if scene_kind == 'col' else 16.0),
    }


def parse_frames(log):
    out = []
    pat = re.compile(r'SPINON_C114_GRID_NODE_FRAME \d+:node=(\d+),x=(-?\d+),y=(-?\d+),width=(\d+),height=(\d+)')
    for line in log.splitlines():
        m = pat.search(line)
        if m:
            out.append({'node': int(m.group(1)), 'x': int(m.group(2)), 'y': int(m.group(3)),
                        'width': int(m.group(4)), 'height': int(m.group(5))})
    return out


def assemble_trace(log, run_id):
    parts = {}
    total = None
    marker = 'SPINON_C114_GRID_TRACE_CHUNK '
    header = re.compile(rf'run_id={re.escape(run_id)} part=(\d+)/(\d+) (.*)$')
    for line in log.splitlines():
        if marker not in line:
            continue
        payload = line.split(marker, 1)[1]
        m = header.match(payload)
        if not m:
            continue
        part, count = int(m.group(1)), int(m.group(2))
        total = count
        # The Android writer emits the JSON prefix only in part 1, then continuation bytes.
        fragment = m.group(3)
        if part == 1 and fragment.startswith('grid_track_sizing_diagnostics='):
            fragment = fragment[len('grid_track_sizing_diagnostics='):]
        parts[part] = fragment
    if total is None or sorted(parts) != list(range(1, total + 1)):
        raise RuntimeError(f'trace chunks incomplete: total={total} parts={sorted(parts)}')
    raw = ''.join(parts[i] for i in range(1, total + 1))
    return json.loads(raw), total


def last_axis_tracks(trace, axis):
    matches = []
    for grid in trace.get('trackSizingTraces', []):
        for p in grid.get('passes', []):
            if p.get('axis') == axis and p.get('stages'):
                tracks = p['stages'][-1].get('tracks', [])
                matches.append([float(t['baseSize']) for t in tracks
                                if not t.get('gutter') and not t.get('collapsed')])
    if not matches:
        raise RuntimeError(f'missing {axis} track pass')
    return matches[-1]


def close_list(actual, expected):
    return len(actual) == len(expected) and all(abs(float(a)-float(e)) < 1e-4 for a,e in zip(actual,expected))


def check_contributions(trace, expected, scene_kind, mode, direction):
    text_node, image_node = expected['text']['node'], expected['image']['node']
    rows = []
    for grid in trace.get('trackSizingTraces', []):
        for p in grid.get('passes', []):
            if p.get('axis') == expected['contribution_axis']:
                rows.extend(p.get('contributions', []))
    if not rows:
        raise RuntimeError(f"no {expected['contribution_axis']} contributions")
    seen_text = set(); seen_image = set()
    for c in rows:
        node = int(c.get('item', -1)); kind = c.get('kind'); value = float(c.get('value', -1))
        if int(c.get('span', -1)) != 1 or c.get('crossesFlexibleTrack') is not False:
            raise RuntimeError(f'invalid contribution shape: {c}')
        if kind not in ('MinContent', 'MaxContent'):
            raise RuntimeError(f'invalid contribution kind: {c}')
        if node == text_node:
            want = expected['text_min'] if kind == 'MinContent' else expected['text_max']
            if abs(value - want) > 1e-4:
                raise RuntimeError(f'text contribution mismatch: {c} expected {want}')
            seen_text.add(kind)
        elif node == image_node:
            if abs(value - expected['image_contribution']) > 1e-4:
                raise RuntimeError(f'image contribution mismatch: {c}')
            seen_image.add(kind)
        else:
            raise RuntimeError(f'unexpected contribution item: {c}')
    needed = {'MinContent'} if mode == 'min' else {'MinContent','MaxContent'}
    if not needed.issubset(seen_text) or not needed.issubset(seen_image):
        raise RuntimeError(f'missing contribution kinds: text={seen_text}, image={seen_image}, need={needed}')
    return rows


def run_scene(scene, run_id, kind, mode, direction):
    if len(run_id) >= 80:
        raise RuntimeError(f'run ID too long: {run_id}')
    expected = frame(kind, mode, direction)
    launch_path = BASE / f'android-launch-{scene}.log'
    runtime_path = BASE / f'android-runtime-{scene}.log'
    trace_path = BASE / f'android-trace-{scene}.json'
    png_path = BASE / f'android-{scene}.png'
    run('logcat', '-c')
    launch = run('shell','am','start','-S','-f','0x10008000','-n','dev.spinon.bootstrap/.MainActivity',
                 '--ez','spinon_c114_grid_track_sizing','true','--es','spinon_c114_grid_scene',scene,
                 '--es','spinon_c114_grid_run_id',run_id)
    launch_path.write_text(launch.stdout)
    deadline = time.monotonic() + 45
    log = ''
    while time.monotonic() < deadline:
        log = run('logcat','-d').stdout
        if 'SPINON_C114_GRID_ASSERTION_ERROR' in log or 'SPINON_C114_GRID_ARGUMENT_ERROR' in log:
            break
        summary = f'SPINON_C114_GRID_SUMMARY run_id={run_id} scene_id={scene}' in log
        draw = 'SPINON_C0410_DRAW status=0 presented boxes=3' in log
        if summary and draw:
            break
        time.sleep(1)
    runtime_path.write_text(log)
    if 'SPINON_C114_GRID_ASSERTION_ERROR' in log or 'SPINON_C114_GRID_ARGUMENT_ERROR' in log:
        raise RuntimeError(f'native matcher rejected {scene}; raw log saved')
    if f'SPINON_C114_GRID_SUMMARY run_id={run_id} scene_id={scene}' not in log:
        raise RuntimeError(f'native summary timed out for {scene}; raw log saved')
    if 'SPINON_C0410_DRAW status=0 presented boxes=3' not in log:
        raise RuntimeError(f'draw status missing for {scene}; raw log saved')
    trace, chunk_count = assemble_trace(log, run_id)
    trace_path.write_text(json.dumps(trace, indent=2, ensure_ascii=False) + '\n')
    frames = parse_frames(log)
    by_node = {f['node']: f for f in frames}
    for key in ('root','text','image'):
        exp = expected[key]
        if by_node.get(exp['node']) != exp:
            raise RuntimeError(f"{scene} {key} frame mismatch: observed={by_node.get(exp['node'])} expected={exp}")
    columns = last_axis_tracks(trace, 'Columns')
    rows = last_axis_tracks(trace, 'Rows')
    if not close_list(columns, expected['columns']) or not close_list(rows, expected['rows']):
        raise RuntimeError(f'{scene} track mismatch columns={columns} rows={rows} expected={expected["columns"]}/{expected["rows"]}')
    contributions = check_contributions(trace, expected, kind, mode, direction)
    screen = run('exec-out','screencap','-p',text=False)
    if not screen.stdout.startswith(b'\x89PNG\r\n\x1a\n'):
        raise RuntimeError(f'screencap failed for {scene}')
    width,height=struct.unpack('>II', screen.stdout[16:24])
    if (width,height)!=(1080,2400):
        raise RuntimeError(f'screenshot size mismatch: {width}x{height}')
    png_path.write_bytes(screen.stdout)
    obs = {
       'sceneId':scene,'runId':run_id,'nativeMatcher':'PASS','draw':'PASS','boxCount':3,
       'expectedFrames':expected,'observedFrames':frames,
       'finalColumnTrackSizes':columns,'finalRowTrackSizes':rows,
       'contributionsAxis':expected['contribution_axis'],'contributions':contributions,
       'traceParts':chunk_count,'traceSha256':sha(trace_path.read_bytes()),
       'runtimeLogSha256':sha(runtime_path.read_bytes()),'launchLogSha256':sha(launch_path.read_bytes()),
       'screenshotSha256':sha(png_path.read_bytes()),'screenshotPixels':{'width':width,'height':height},
    }
    return obs


assert_snapshot()
if run('get-state').stdout.strip() != 'device':
    raise SystemExit('emulator-5570 is not ready')
meta=(BASE/'avd-metadata.txt').read_text()
for expected_meta in ('sdk=37\n','abi=arm64-v8a\n','qemu=1\n','page_size=16384\n','boot_completed=1\n'):
    if expected_meta not in meta: raise SystemExit(f'AVD metadata mismatch: {expected_meta!r}')
installed=(BASE/'installed-apk-sha256.log').read_text().split()[0]
if installed != EXPECTED_APK_SHA: raise SystemExit('installed APK hash mismatch')
results=[]
for scene,run_id,kind,mode,direction in SCENES:
    result=run_scene(scene,run_id,kind,mode,direction)
    results.append(result)
    (BASE/'android-results.partial.json').write_text(json.dumps({'sourceSnapshotSha256':EXPECTED_SOURCE_SHA,'apkSha256':EXPECTED_APK_SHA,'observations':results},indent=2,ensure_ascii=False)+'\n')
    print(f"PASS {scene} ({len(results)}/12) trace_parts={result['traceParts']} screenshot={result['screenshotPixels']['width']}x{result['screenshotPixels']['height']}",flush=True)
assert_snapshot()
(BASE/'android-results.json').write_text(json.dumps({'head':EXPECTED_HEAD,'indexTreeGitHash':EXPECTED_INDEX_TREE,'sourceSnapshotSha256':EXPECTED_SOURCE_SHA,'apkSha256':EXPECTED_APK_SHA,'observations':results},indent=2,ensure_ascii=False)+'\n')
(BASE/'android-results.partial.json').unlink(missing_ok=True)
print('PASS all 12 scenes; source snapshot unchanged',flush=True)
