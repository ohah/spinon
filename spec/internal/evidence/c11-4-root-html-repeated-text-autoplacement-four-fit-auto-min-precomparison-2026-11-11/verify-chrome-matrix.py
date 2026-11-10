import json
from collections import defaultdict, Counter
from itertools import combinations, permutations
from pathlib import Path

observation = Path(__file__).with_name('chrome157-observation.ndjson')
expected_lines = [2, 2, 2, 3, 2, 2, 3, 1]
fit_position_sets = {tuple(value) for value in combinations(range(1, 7), 4)}
tail_orders = {('min-content', 'max-content'), ('max-content', 'min-content')}
cap_orders = {tuple(value) for value in permutations(('0px', '10px', '20%', '30px'))}

def close(a, b, tolerance=1e-5):
    return abs(a - b) <= tolerance

with observation.open() as stream:
    metadata = json.loads(next(stream))
    records = [json.loads(line) for line in stream]
assert metadata['sceneCount'] == 5760
assert metadata['uniqueTrackListCount'] == 720
assert 'HeadlessChrome/157.' in metadata['userAgent']
assert metadata['platform'] == 'MacIntel' and metadata['devicePixelRatio'] == 1
assert len(records) == 5760
assert [record['index'] for record in records] == list(range(5760))
assert {tuple(record['scene']['fitPositions']) for record in records} == fit_position_sets
assert {tuple(record['scene']['tail']) for record in records} == tail_orders
assert {tuple(record['scene']['fitCaps']) for record in records} == cap_orders
assert {record['scene']['alignContent'] for record in records} == {'start', 'stretch'}
assert {record['scene']['domOrder'] for record in records} == {'forward', 'reverse'}
assert {record['scene']['minHeight'] for record in records} == {'0px', 'auto'}
assert len({record['scene']['trackList'] for record in records}) == 720

pairs = defaultdict(dict)
fit_row_occurrences = 0
baseline_below_floor = 0
candidate_floor_violations = []
non_fit_floor_violations = []
rows_mismatch = []
frame_mismatch = []
line_mismatch = []
boundary_overflow = []
payload_row_overflow = Counter()
payload_end_overflow = []
maximum_payload_end = Counter()
for record in records:
    scene = record['scene']
    key = (scene['trackList'], scene['alignContent'], scene['domOrder'])
    assert record['computedAutoRows'] == scene['trackList']
    assert close(record['gridFrame'][0], 40) and close(record['gridFrame'][1], 220)
    assert len(record['rows']) == 9 and close(record['rows'][0], 10)
    assert len(record['items']) == 9
    assert record['items'][0]['id'] == 'empty-0'
    direct_items = record['items'][1:]
    expected_ids = [str(i) for i in range(1, 9)]
    if scene['domOrder'] == 'reverse':
        expected_ids.reverse()
    assert [item['id'] for item in direct_items] == expected_ids
    for item in direct_items:
        source_id = int(item['id'])
        assert item['lineCount'] == expected_lines[source_id - 1]
        assert item['lineCount'] == len(item['lineTops'])
        assert item['lineTops'] and all(close(item['lineTops'][i] - item['lineTops'][i - 1], 10) for i in range(1, len(item['lineTops'])))
    for position, item in enumerate(direct_items, start=1):
        expected_track_size = record['rows'][position]
        if not close(item['frame'][3], expected_track_size):
            frame_mismatch.append((record['index'], item['id'], item['frame'][3], expected_track_size))
        track_position = (position - 1) % 6 + 1
        if track_position in scene['fitPositions']:
            if scene['minHeight'] == 'auto':
                fit_row_occurrences += 1
                if item['frame'][3] + 1e-5 < item['payload'][3]:
                    candidate_floor_violations.append((record['index'], item['id'], item['frame'][3], item['payload'][3]))
            elif item['frame'][3] + 1e-5 < item['payload'][3]:
                baseline_below_floor += 1
        elif item['frame'][3] + 1e-5 < item['payload'][3]:
            non_fit_floor_violations.append((record['index'], item['id'], scene['minHeight'], item['frame'][3], item['payload'][3]))
        if item['frame'][1] + item['frame'][3] > record['gridFrame'][1] + 1e-5:
            boundary_overflow.append((record['index'], item['id'], item['frame'][1] + item['frame'][3]))
        payload_end = item['payload'][1] + item['payload'][3]
        maximum_payload_end[scene['minHeight']] = max(maximum_payload_end[scene['minHeight']], payload_end)
        if item['payload'][1] + item['payload'][3] > item['frame'][1] + item['frame'][3] + 1e-5:
            payload_row_overflow[scene['minHeight']] += 1
        if payload_end > record['gridFrame'][1] + 1e-5:
            payload_end_overflow.append((record['index'], item['id'], payload_end))
    pairs[key][scene['minHeight']] = record
assert not frame_mismatch, frame_mismatch[:3]
assert not line_mismatch, line_mismatch[:3]
assert not candidate_floor_violations, candidate_floor_violations[:3]
assert not non_fit_floor_violations, non_fit_floor_violations[:3]
assert not boundary_overflow, boundary_overflow[:3]
assert not payload_end_overflow, payload_end_overflow[:3]
assert payload_row_overflow == Counter({'0px': 7200})
assert len(pairs) == 2880 and all(set(modes) == {'0px', 'auto'} for modes in pairs.values())
paired_track_changes = 0
stretch_track_changes = 0
eligible_candidate = 0
for (track_list, alignment, dom_order), modes in pairs.items():
    zero, auto = modes['0px'], modes['auto']
    eligible_candidate += sum(1 for position in range(1, 9) if ((position - 1) % 6 + 1) in auto['scene']['fitPositions'])
    if zero['rows'] != auto['rows']:
        paired_track_changes += 1
    if alignment == 'stretch':
        start = pairs[(track_list, 'start', dom_order)]
        if start['0px']['rows'] != zero['rows'] or start['auto']['rows'] != auto['rows']:
            stretch_track_changes += 1
assert fit_row_occurrences == eligible_candidate == 15360
assert baseline_below_floor == 7200
assert paired_track_changes == 2880
assert stretch_track_changes == 0

summary = {
    'schema': 'spinon.c11-4.repeated-text-autoplacement-four-fit-auto-min-verification/v1',
    'browser': metadata['userAgent'], 'platform': metadata['platform'], 'dpr': metadata['devicePixelRatio'],
    'sceneCount': len(records), 'pairedKeys': len(pairs), 'uniqueTrackLists': 720,
    'fitPositionQuadruples': 15, 'tailOrders': 2, 'capPermutations': 24,
    'candidateFitTrackItemOccurrences': fit_row_occurrences,
    'baselineFitOccurrencesBelowPayloadFloor': baseline_below_floor,
    'candidateFitFloorViolations': len(candidate_floor_violations),
    'nonFitTrackFloorViolations': len(non_fit_floor_violations),
    'visiblePayloadRowOverflowByMinHeight': dict(payload_row_overflow),
    'pairedKeysWithChangedRows': paired_track_changes,
    'alignContentStretchRowsDifferFromStart': stretch_track_changes,
    'itemFrameGridEndOverflows': len(boundary_overflow),
    'payloadGridEndOverflows': len(payload_end_overflow),
    'maximumPayloadEndByMinHeight': dict(maximum_payload_end),
    'sample': {'tracks': records[0]['scene']['trackList'], 'baselineRows': records[0]['rows'], 'candidateRows': records[1]['rows']},
}
print(json.dumps(summary, separators=(',', ':')))
