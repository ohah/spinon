#!/usr/bin/env python3
import collections
import gzip
import hashlib
import itertools
import json
import math
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
FIXTURE = Path('tests/fixtures/css/c11/root-html-grid-repeated-text-row-span-six-fit-auto-minimum-precomparison.html')
OBSERVATION = HERE / 'chrome157-observation.ndjson.gz'
OUTPUT = HERE / 'verification.json'
CAPS = ('0px', '10px', '20%', '30px', '40px', '50px')
ALIGNMENTS = ('start', 'stretch')
TOPOLOGIES = ('span2Six', 'span2Span3Six')
DOM_ORDERS = ('forward', 'reverse')
MINIMUMS = ('zero', 'auto')
TOLERANCE = 0.01


def close(a, b, tolerance=TOLERANCE):
    return abs(a - b) <= tolerance


def require(condition, message):
    if not condition:
        raise AssertionError(message)


def scene_base(scene):
    return scene['topology'].removesuffix('-forward').removesuffix('-reverse')


def scene_order(scene):
    return scene['topology'].rsplit('-', 1)[1]


def pair_key(record):
    scene = record['scene']
    return (scene['trackList'], scene_base(scene), scene['alignContent'], scene_order(scene))


def read_records():
    with gzip.open(OBSERVATION, 'rt') as handle:
        metadata = json.loads(next(handle))
        rows = [json.loads(line) for line in handle if line.strip()]
    return metadata, rows


def main():
    metadata, rows = read_records()
    fixture_bytes = FIXTURE.read_bytes()
    fixture_sha = hashlib.sha256(fixture_bytes).hexdigest()
    require(metadata['record'] == 'metadata', 'metadata record missing')
    require(metadata['fixtureSha256'] == fixture_sha, 'fixture hash mismatch')
    require(metadata['sceneCount'] == 11520, f"unexpected metadata scene count {metadata['sceneCount']}")
    require(metadata['browserCli'] == 'agent-browser 0.31.1', f"unexpected browser CLI {metadata['browserCli']}")
    require('HeadlessChrome/157.' in metadata['userAgent'], f"unexpected user agent {metadata['userAgent']}")
    require(metadata['devicePixelRatio'] == 1, 'DPR must be 1')
    require(metadata['viewportCssPx'] == [1280, 900], 'viewport mismatch')
    require(len(rows) == 11520, f'expected 11520 rows, got {len(rows)}')
    require(all(row['record'] == 'scene' for row in rows), 'non-scene observation row')
    require([row['index'] for row in rows] == list(range(11520)), 'scene indices are not contiguous')

    cap_orders = set(itertools.permutations(CAPS))
    require(len(cap_orders) == 720, 'independent permutation count mismatch')
    tracks = {row['scene']['trackList'] for row in rows}
    require(len(tracks) == 720, f'expected 720 track lists, got {len(tracks)}')

    indexed = {}
    floor = collections.Counter()
    grid_end_overflow = collections.Counter()
    alignment_max_error = 0.0
    source_order_max_error = 0.0
    span_sum_max_error = 0.0
    row_count_histogram = collections.Counter()
    cap_order_seen = set()
    topology_seen = collections.Counter()
    align_seen = collections.Counter()
    minimum_seen = collections.Counter()
    floor_breakdown = collections.Counter()
    geometry_changed_pair_keys = set()

    for row in rows:
        scene = row['scene']
        require(scene['alignContent'] in ALIGNMENTS, f"bad alignContent in scene {row['index']}")
        require(scene['minimum'] in MINIMUMS, f"bad minimum in scene {row['index']}")
        require(scene['fitPositions'] == [1, 2, 3, 4, 5, 6], f"fit position mismatch in scene {row['index']}")
        require(tuple(scene['fitCaps']) in cap_orders, f"cap order is not a permutation in scene {row['index']}")
        require(scene['trackMembers'] == [f'fit-content({cap})' for cap in scene['fitCaps']], f"track member mismatch in scene {row['index']}")
        require(scene['trackList'] == ' '.join(scene['trackMembers']), f"track list mismatch in scene {row['index']}")
        require(row['computedAutoRows'] == scene['trackList'], f"computed auto rows mismatch in scene {row['index']}")
        require(row['computedAlignContent'] == scene['alignContent'], f"computed alignment mismatch in scene {row['index']}")
        require(len(row['gridFrame']) == 4 and row['gridFrame'] == [0, 0, 40, 220], f"grid frame mismatch in scene {row['index']}")
        values = row['rowValues']
        require(len(values) == 7, f"expected explicit row plus 6 implicit rows in scene {row['index']}, got {len(values)}")
        require(all(math.isfinite(v) and v >= 0 for v in values), f"non-finite/negative track in scene {row['index']}")
        row_count_histogram[len(values)] += 1
        require(values[0] == 10, f"explicit row changed in scene {row['index']}")

        item_map = {item['id']: item for item in row['items']}
        require(len(item_map) == len(row['items']), f"duplicate item id in scene {row['index']}")
        require(set(item_map) == {'empty', *[item[0] for item in scene['items']]}, f"item set mismatch in scene {row['index']}")
        require(item_map['empty']['minHeight'] == '0px', f"empty control min-height changed in scene {row['index']}")
        require(item_map['empty']['frame'] == [0, 0, 40, 10], f"empty control frame changed in scene {row['index']}")
        expected_min = 'auto' if scene['minimum'] == 'auto' else '0px'
        for identifier, start, span in scene['items']:
            item = item_map[identifier]
            require(item['minHeight'] == expected_min, f"item min-height mismatch in scene {row['index']}/{identifier}")
            require(len(item['frame']) == 4 and len(item['payloadFrame']) == 4, f"missing frame in scene {row['index']}/{identifier}")
            row_start = start - 1
            expected_y = sum(values[:row_start])
            expected_height = sum(values[row_start:row_start + span])
            require(close(item['frame'][1], expected_y), f"span top mismatch in scene {row['index']}/{identifier}")
            require(close(item['frame'][3], expected_height), f"span height does not equal covered rows in scene {row['index']}/{identifier}")
            span_sum_max_error = max(span_sum_max_error, abs(item['frame'][3] - expected_height))
            require(item['payloadFrame'][0] == item['frame'][0], f"payload x mismatch in scene {row['index']}/{identifier}")
            require(close(item['payloadFrame'][1], item['frame'][1]), f"payload top mismatch in scene {row['index']}/{identifier}")
            payload_floor_shortfall = max(0.0, item['payloadFrame'][3] - item['frame'][3])
            if payload_floor_shortfall > TOLERANCE:
                floor[scene['minimum']] += 1
                floor_breakdown[(scene['minimum'], scene['fitCaps'][0])] += 1
            overflow = max(0.0, item['payloadFrame'][1] + item['payloadFrame'][3] - 220.0)
            if overflow > TOLERANCE:
                grid_end_overflow[scene['minimum']] += 1
            require(len(item['lineRects']) > 0, f"text line boxes missing in scene {row['index']}/{identifier}")

        key = (scene['trackList'], scene['topology'], scene['alignContent'], scene['minimum'])
        require(key not in indexed, f'duplicate scene key {key}')
        indexed[key] = row
        topology_seen[scene_base(scene)] += 1
        align_seen[scene['alignContent']] += 1
        minimum_seen[scene['minimum']] += 1
        cap_order_seen.add(tuple(scene['fitCaps']))

    require(len(indexed) == 11520, 'unique scene key count mismatch')
    require(len(cap_order_seen) == 720, 'not all 720 cap orders observed')
    require(topology_seen == {'span2Six': 5760, 'span2Span3Six': 5760}, f'topology counts mismatch: {topology_seen}')
    require(align_seen == {'start': 5760, 'stretch': 5760}, f'alignment counts mismatch: {align_seen}')
    require(minimum_seen == {'zero': 5760, 'auto': 5760}, f'minimum counts mismatch: {minimum_seen}')

    paired_keys = set()
    for row in rows:
        scene = row['scene']
        key = pair_key(row)
        if scene['minimum'] == 'zero':
            candidate = indexed[(scene['trackList'], scene['topology'], scene['alignContent'], 'auto')]
            paired_keys.add(key)
            base_items = {item['id']: item for item in row['items']}
            candidate_items = {item['id']: item for item in candidate['items']}
            for identifier in base_items:
                a = base_items[identifier]
                b = candidate_items[identifier]
                require(a['text'] == b['text'], f"paired text changed for {key}/{identifier}")
                require(len(a['lineRects']) == len(b['lineRects']), f"paired line count changed for {key}/{identifier}")
                if a['frame'] != b['frame'] or a['payloadFrame'] != b['payloadFrame']:
                    geometry_changed_pair_keys.add(key)
    require(len(paired_keys) == 5760, f'expected 5760 baseline/candidate pair keys, got {len(paired_keys)}')

    for track_list in tracks:
        for topology in TOPOLOGIES:
            for alignment in ALIGNMENTS:
                for minimum in MINIMUMS:
                    forward = indexed[(track_list, f'{topology}-forward', alignment, minimum)]
                    reverse = indexed[(track_list, f'{topology}-reverse', alignment, minimum)]
                    forward_items = {item['id']: item for item in forward['items']}
                    reverse_items = {item['id']: item for item in reverse['items']}
                    for identifier in forward_items:
                        a = forward_items[identifier]
                        b = reverse_items[identifier]
                        for field in ('frame', 'payloadFrame', 'lineRects'):
                            av, bv = a[field], b[field]
                            if av is None or bv is None:
                                require(av == bv, f"source-order {field} shape differs for {track_list}/{topology}/{alignment}/{minimum}/{identifier}")
                                continue
                            require(len(av) == len(bv), f"source-order {field} count differs for {track_list}/{topology}/{alignment}/{minimum}/{identifier}")
                            for va, vb in zip((x for entry in av for x in entry) if field == 'lineRects' else av,
                                              (x for entry in bv for x in entry) if field == 'lineRects' else bv):
                                source_order_max_error = max(source_order_max_error, abs(va - vb))
                                require(close(va, vb), f"source-order geometry differs for {track_list}/{topology}/{alignment}/{minimum}/{identifier}")
                    arow = forward['rowValues']
                    brow = reverse['rowValues']
                    require(len(arow) == len(brow), 'source order changed row count')
                    for va, vb in zip(arow, brow):
                        source_order_max_error = max(source_order_max_error, abs(va - vb))
                        require(close(va, vb), f"source-order row sizes differ for {track_list}/{topology}/{alignment}/{minimum}")

    for track_list in tracks:
        for topology in TOPOLOGIES:
            for order in DOM_ORDERS:
                for minimum in MINIMUMS:
                    start = indexed[(track_list, f'{topology}-{order}', 'start', minimum)]
                    stretch = indexed[(track_list, f'{topology}-{order}', 'stretch', minimum)]
                    for a, b in zip(start['rowValues'], stretch['rowValues']):
                        alignment_max_error = max(alignment_max_error, abs(a - b))
                        require(close(a, b), f"start/stretch row geometry differs for {track_list}/{topology}/{order}/{minimum}")
                    start_items = {item['id']: item for item in start['items']}
                    stretch_items = {item['id']: item for item in stretch['items']}
                    for identifier in start_items:
                        for field in ('frame', 'payloadFrame', 'lineRects'):
                            a, b = start_items[identifier][field], stretch_items[identifier][field]
                            if a is None or b is None:
                                require(a == b, f"start/stretch {field} shape differs")
                                continue
                            require(len(a) == len(b), f"start/stretch {field} count differs")
                            af = [v for entry in a for v in entry] if field == 'lineRects' else a
                            bf = [v for entry in b for v in entry] if field == 'lineRects' else b
                            for va, vb in zip(af, bf):
                                alignment_max_error = max(alignment_max_error, abs(va - vb))
                                require(close(va, vb), f"start/stretch geometry differs for {track_list}/{topology}/{order}/{minimum}/{identifier}")

    require(floor['auto'] == 0, f"candidate payload-floor violations: {floor['auto']}")
    require(floor['zero'] > 0, 'baseline did not reproduce any payload-floor shortfall')
    require(grid_end_overflow['auto'] == 0, f"candidate grid-end overflow: {grid_end_overflow['auto']}")
    require(grid_end_overflow['zero'] == 0, f"baseline grid-end overflow: {grid_end_overflow['zero']}")

    result = {
        'schema': 'spinon.c11-4.repeated-text-row-span-six-fit-auto-min-precomparison-verification/v1',
        'capturedAtUtc': metadata['capturedAtUtc'],
        'source': {
            'browserCli': metadata['browserCli'], 'userAgent': metadata['userAgent'], 'platform': metadata['platform'],
            'devicePixelRatio': metadata['devicePixelRatio'], 'viewportCssPx': metadata['viewportCssPx'],
            'fixturePath': str(FIXTURE), 'fixtureBytes': len(fixture_bytes), 'fixtureSha256': fixture_sha,
            'observationPath': OBSERVATION.name, 'observationBytes': OBSERVATION.stat().st_size,
            'observationSha256': hashlib.sha256(OBSERVATION.read_bytes()).hexdigest(),
        },
        'sceneCount': len(rows), 'uniqueTrackLists': len(tracks), 'orderedCapAssignments': len(cap_order_seen),
        'pairedBaselineCandidateKeys': len(paired_keys), 'baselineCandidateGeometryChangedPairKeys': len(geometry_changed_pair_keys), 'topologyCounts': dict(topology_seen),
        'alignmentCounts': dict(align_seen), 'minimumModeCounts': dict(minimum_seen),
        'rowCountHistogram': {str(k): v for k, v in row_count_histogram.items()},
        'payloadFloorViolations': {'minHeightZero': floor['zero'], 'minHeightAuto': floor['auto']},
        'gridEndOverflowOccurrences': {'minHeightZero': grid_end_overflow['zero'], 'minHeightAuto': grid_end_overflow['auto']},
        'maxErrorsCssPx': {
            'spanningItemHeightVsCoveredRows': span_sum_max_error,
            'sourceOrderReversalGeometry': source_order_max_error,
            'alignContentStartVsStretch': alignment_max_error,
        },
        'acceptance': {
            'allSixCapOrdersPresent': len(cap_order_seen) == 720,
            'baselineReproducesPayloadFloorShortfall': floor['zero'] > 0,
            'candidateHasNoPayloadFloorShortfall': floor['auto'] == 0,
            'bothModesStayWithinDefiniteGridEnd': grid_end_overflow['zero'] == 0 and grid_end_overflow['auto'] == 0,
            'spanFramesMatchCoveredTracks': span_sum_max_error <= TOLERANCE,
            'sourceOrderInvariant': source_order_max_error <= TOLERANCE,
            'startStretchGeometryInvariant': alignment_max_error <= TOLERANCE,
        },
        'floorBreakdownByFirstCap': {f'{mode}|{cap}': count for (mode, cap), count in sorted(floor_breakdown.items())},
        'limits': [
            'Chrome CSS observation only; no Spinon runtime result.',
            'Fixed direct-text topology, fixed six-cap multiset, definite 40x220px one-column Grid.',
            'No public CSS support, WPT, Android/iOS app geometry, device pixels, or C11.4 completion claim.',
        ],
    }
    OUTPUT.write_text(json.dumps(result, ensure_ascii=False, indent=2) + '\n')
    print(json.dumps(result, ensure_ascii=False, indent=2))


if __name__ == '__main__':
    try:
        main()
    except Exception as error:
        print(f'VERIFICATION_FAILED: {error}', file=sys.stderr)
        raise
