#!/usr/bin/env python3
import collections
import gzip
import hashlib
import itertools
import json
import math
from pathlib import Path

HERE = Path(__file__).resolve().parent
FIXTURE = Path('tests/fixtures/css/c11/root-html-grid-repeated-text-row-span-mixed-auto-minimum-precomparison.html')
OBSERVATION = HERE / 'chrome157-mixed-minimum-observation.ndjson.gz'
OUTPUT = HERE / 'verification.json'
PREVIOUS = Path('spec/internal/evidence/c11-4-root-html-repeated-text-row-span-six-fit-auto-minimum-precomparison-2026-11-12/chrome157-observation.ndjson.gz')
CAPS = ('0px', '10px', '20%', '30px', '40px', '50px')
ALIGNMENTS = ('start', 'stretch')
TOPOLOGIES = {
    'span2Six': ('A', 'B', 'C', 'D', 'E'),
    'span2Span3Six': ('A', 'B', 'C', 'D'),
}
PLACEMENTS = {
    'span2Six': [('A', 2, 2), ('B', 3, 2), ('C', 4, 2), ('D', 5, 2), ('E', 6, 2)],
    'span2Span3Six': [('A', 2, 2), ('B', 3, 2), ('C', 4, 3), ('D', 5, 3)],
}
CAP_ORDER_SET = set(itertools.permutations(CAPS))
DOM_ORDERS = ('forward', 'reverse')
TOLERANCE = 0.01


def require(condition, message):
    if not condition:
        raise AssertionError(message)


def close(a, b):
    return abs(a - b) <= TOLERANCE


def geometry(row):
    items = []
    for item in sorted(row['items'], key=lambda value: value['id']):
        items.append((
            item['id'], tuple(item['frame']),
            tuple(item['payloadFrame']) if item['payloadFrame'] is not None else None,
            tuple(tuple(rect) for rect in item['lineRects']),
        ))
    return tuple(row['rowValues']), tuple(items)


def geometry_error(left, right):
    require(len(left[0]) == len(right[0]), 'row-count differs across paired scenes')
    error = max((abs(a - b) for a, b in zip(left[0], right[0])), default=0.0)
    left_items = {item[0]: item[1:] for item in left[1]}
    right_items = {item[0]: item[1:] for item in right[1]}
    require(left_items.keys() == right_items.keys(), 'item IDs differ across paired scenes')
    for identifier in left_items:
        for a, b in zip(left_items[identifier], right_items[identifier]):
            if a is None or b is None:
                require(a == b, f'{identifier}: optional geometry shape differs')
                continue
            if len(a) != len(b):
                raise AssertionError(f'{identifier}: geometry vector length differs')
            for av, bv in zip(a, b):
                if isinstance(av, tuple):
                    require(len(av) == len(bv), f'{identifier}: line rect count differs')
                    error = max(error, max((abs(x - y) for x, y in zip(av, bv)), default=0.0))
                else:
                    error = max(error, abs(av - bv))
    return error


def scene_family(scene):
    suffix = scene['topology'].rsplit('-', 1)[1]
    family = scene['topology'].rsplit('-', 1)[0]
    require(suffix in DOM_ORDERS, f"bad DOM order {scene['topology']}")
    require(family in TOPOLOGIES, f"bad topology {scene['topology']}")
    return family, suffix


def previous_control_geometries():
    controls = {}
    with gzip.open(PREVIOUS, 'rt') as handle:
        metadata = json.loads(next(handle))
        require(metadata['sceneCount'] == 11520, 'previous uniform matrix metadata changed')
        for line in handle:
            if not line.strip():
                continue
            row = json.loads(line)
            if row.get('record') != 'scene':
                continue
            scene = row['scene']
            controls[(scene['trackList'], scene['alignContent'], scene['topology'], scene['minimum'])] = geometry(row)
    require(len(controls) == 11520, 'previous uniform control matrix is incomplete')
    return controls


def validate_observation(row, mask, family, metadata):
    scene = row['scene']
    scene_family_name, dom_order = scene_family(scene)
    require(scene_family_name == family, f"topology family mismatch at scene {row['index']}")
    ids = TOPOLOGIES[family]
    expected_auto = [identifier for bit, identifier in enumerate(ids) if mask & (1 << bit)]
    require(scene['minimumMask'] == mask, f"mask mismatch at scene {row['index']}")
    require(scene['autoItems'] == expected_auto, f"auto-item mapping mismatch at scene {row['index']}")
    require(scene['fitPositions'] == [1, 2, 3, 4, 5, 6], f"fit positions mismatch at scene {row['index']}")
    require(tuple(scene['fitCaps']) in CAP_ORDER_SET, f"bad cap permutation at scene {row['index']}")
    require(scene['trackMembers'] == [f'fit-content({cap})' for cap in scene['fitCaps']], f"track members mismatch at scene {row['index']}")
    require(scene['trackList'] == ' '.join(scene['trackMembers']), f"track list mismatch at scene {row['index']}")
    require(row['computedAutoRows'] == scene['trackList'], f"computed rows mismatch at scene {row['index']}")
    require(row['computedAlignContent'] == scene['alignContent'], f"computed alignment mismatch at scene {row['index']}")
    require(row['devicePixelRatio'] == 1, 'DPR changed within observations')
    require('HeadlessChrome/157.' in row['userAgent'], 'Chrome version changed within observations')
    require(row['gridFrame'] == [0, 0, 40, 220], f"grid frame mismatch at scene {row['index']}")
    rows = row['rowValues']
    require(len(rows) == 7 and rows[0] == 10, f"expected explicit 10px row plus six implicit rows at scene {row['index']}")
    require(all(math.isfinite(value) and value >= 0 for value in rows), f"invalid track value at scene {row['index']}")
    item_map = {item['id']: item for item in row['items']}
    require(len(item_map) == len(row['items']), f"duplicate item at scene {row['index']}")
    require(set(item_map) == {'empty', *ids}, f"item set mismatch at scene {row['index']}")
    require(item_map['empty']['minHeight'] == '0px', 'explicit-row control minimum changed')
    require(item_map['empty']['frame'] == [0, 0, 40, 10], 'explicit-row control frame changed')
    expected_placements = PLACEMENTS[family]
    if dom_order == 'reverse':
        expected_placements = list(reversed(expected_placements))
    require(scene['items'] == [list(item) for item in expected_placements], f"placement order mismatch at scene {row['index']}")
    placement = {identifier: (start, span) for identifier, start, span in scene['items']}
    require(set(placement) == set(ids), f"placement set mismatch at scene {row['index']}")
    floor_shortfalls = {'auto': 0, 'zero': 0}
    grid_end_overflow = {'auto': 0, 'zero': 0}
    max_span_error = 0.0
    for identifier in ids:
        item = item_map[identifier]
        expected_minimum = 'auto' if identifier in expected_auto else '0px'
        require(item['minHeight'] == expected_minimum, f"computed min-height mismatch at scene {row['index']}/{identifier}")
        start, span = placement[identifier]
        y = sum(rows[:start - 1])
        height = sum(rows[start - 1:start - 1 + span])
        require(close(item['frame'][1], y), f"span top mismatch at scene {row['index']}/{identifier}")
        span_error = abs(item['frame'][3] - height)
        max_span_error = max(max_span_error, span_error)
        require(span_error <= TOLERANCE, f"span frame does not equal covered rows at scene {row['index']}/{identifier}")
        require(close(item['payloadFrame'][1], item['frame'][1]), f"payload top mismatch at scene {row['index']}/{identifier}")
        require(len(item['lineRects']) > 0, f"text line boxes missing at scene {row['index']}/{identifier}")
        shortfall = max(0.0, item['payloadFrame'][3] - item['frame'][3])
        if shortfall > TOLERANCE:
            floor_shortfalls['auto' if identifier in expected_auto else 'zero'] += 1
        overflow = max(0.0, item['payloadFrame'][1] + item['payloadFrame'][3] - 220.0)
        if overflow > TOLERANCE:
            grid_end_overflow['auto' if identifier in expected_auto else 'zero'] += 1
    return floor_shortfalls, grid_end_overflow, max_span_error


def main():
    fixture_bytes = FIXTURE.read_bytes()
    fixture_sha = hashlib.sha256(fixture_bytes).hexdigest()
    controls = previous_control_geometries()
    expected_masks = {family: 1 << len(ids) for family, ids in TOPOLOGIES.items()}
    expected_scenes = 720 * 2 * sum(expected_masks.values()) * 2
    scene_count = 0
    groups = 0
    cap_orders = set()
    mask_counts = collections.Counter()
    mask_floor = collections.Counter()
    mask_overflow = collections.Counter()
    baseline_by_shape = {}
    mask_geometry_changes = collections.Counter()
    max_pair_error = 0.0
    max_span_error = 0.0
    uniform_controls_checked = 0

    with gzip.open(OBSERVATION, 'rt') as handle:
        metadata = json.loads(next(handle))
        require(metadata['record'] == 'metadata', 'metadata record missing')
        require(metadata['fixtureSha256'] == fixture_sha, 'fixture SHA-256 mismatch')
        require(metadata['sceneCount'] == expected_scenes, f"metadata scene count mismatch: {metadata['sceneCount']}")
        require(metadata['browserCli'] == 'agent-browser 0.31.1', f"unexpected browser CLI {metadata['browserCli']}")
        require('HeadlessChrome/157.' in metadata['userAgent'], 'unexpected user agent')
        require(metadata['platform'] == 'MacIntel' and metadata['devicePixelRatio'] == 1, 'browser platform or DPR changed')
        require(metadata['viewportCssPx'] == [1280, 900], 'viewport mismatch')

        records = []
        for line in handle:
            if not line.strip():
                continue
            record = json.loads(line)
            require(record.get('record') == 'scene', 'non-scene record in observation stream')
            require(record['index'] == scene_count, f"scene index gap at {scene_count}")
            records.append(record)
            scene_count += 1
            if len(records) < 4:
                continue

            batch = records
            records = []
            scenes = [record['scene'] for record in batch]
            first = scenes[0]
            family, order = scene_family(first)
            mask = first['minimumMask']
            require(0 <= mask < expected_masks[family], f"mask out of range for {family}: {mask}")
            require(len(batch) == 4, 'alignment/order group must contain four observations')
            expected_modes = [('start', 'forward'), ('start', 'reverse'), ('stretch', 'forward'), ('stretch', 'reverse')]
            actual_modes = [(scene['alignContent'], scene_family(scene)[1]) for scene in scenes]
            require(actual_modes == expected_modes, f"unexpected comparison group ordering: {actual_modes}")
            require(all(scene_family(scene)[0] == family for scene in scenes), 'topology family changed in a group')
            expected_items = PLACEMENTS[family]
            if order == 'reverse':
                expected_items = list(reversed(expected_items))
            require(first['items'] == [list(item) for item in expected_items], f"unexpected placement list for {first['topology']}")
            require(all(scene['trackList'] == first['trackList'] and scene['fitCaps'] == first['fitCaps'] for scene in scenes), 'cap order changed within a group')
            require(all(scene['minimumMask'] == mask and scene['autoItems'] == first['autoItems'] for scene in scenes), 'minimum assignment changed within a group')

            geometries = []
            for record, scene in zip(batch, scenes):
                cap_orders.add(tuple(scene['fitCaps']))
                mask_counts[(family, mask)] += 1
                floor, overflow, span_error = validate_observation(record, mask, family, metadata)
                mask_floor[(family, mask, 'auto')] += floor['auto']
                mask_floor[(family, mask, 'zero')] += floor['zero']
                mask_overflow[(family, mask, 'auto')] += overflow['auto']
                mask_overflow[(family, mask, 'zero')] += overflow['zero']
                max_span_error = max(max_span_error, span_error)
                geometries.append(geometry(record))

                minimum = 'auto' if mask == expected_masks[family] - 1 else 'zero' if mask == 0 else None
                if minimum:
                    control_key = (scene['trackList'], scene['alignContent'], scene['topology'], minimum)
                    require(control_key in controls, f"previous uniform control missing: {control_key}")
                    error = geometry_error(geometries[-1], controls[control_key])
                    uniform_controls_checked += 1
                    max_pair_error = max(max_pair_error, error)
                    require(error <= TOLERANCE, f"new {minimum} control differs from prior matrix: {control_key}, error={error}")

            for candidate_geometry in geometries[1:]:
                error = geometry_error(geometries[0], candidate_geometry)
                max_pair_error = max(max_pair_error, error)
                require(error <= TOLERANCE, f"source-order/alignment geometry changed for {first['trackList']}/{family}/mask={mask}: {error}")

            baseline_key = (first['trackList'], family)
            if mask == 0:
                baseline_by_shape[baseline_key] = geometries
            else:
                require(baseline_key in baseline_by_shape, f"zero-minimum controls must precede mask {mask}")
                if any(geometry_error(base, candidate) > TOLERANCE for base, candidate in zip(baseline_by_shape[baseline_key], geometries)):
                    mask_geometry_changes[(family, mask)] += 1
            groups += 1

    require(not records, 'trailing incomplete four-scene group')
    require(scene_count == expected_scenes, f"expected {expected_scenes} scenes, captured {scene_count}")
    require(len(cap_orders) == 720 and cap_orders == set(itertools.permutations(CAPS)), 'not all 720 cap permutations were captured')
    require(groups == expected_scenes // 4, 'comparison-group count mismatch')
    for family, mask_limit in expected_masks.items():
        for mask in range(mask_limit):
            require(mask_counts[(family, mask)] == 720 * 4, f"incomplete {family} mask {mask}: {mask_counts[(family, mask)]}")
            require(mask_floor[(family, mask, 'auto')] == 0, f"auto item payload-floor violation for {family} mask {mask}")
            require(mask_overflow[(family, mask, 'auto')] == 0 and mask_overflow[(family, mask, 'zero')] == 0,
                    f"Grid-end overflow for {family} mask {mask}")
    require(max_span_error <= TOLERANCE, f"span-frame/covered-row error exceeded tolerance: {max_span_error}")
    require(uniform_controls_checked == 11520, f"expected 11,520 prior uniform controls, checked {uniform_controls_checked}")
    require(sum(mask_geometry_changes.values()) > 0, 'mixed minimum assignments did not change any geometry from all-zero controls')

    result = {
        'schema': 'spinon.c11-4.repeated-text-row-span-six-fit-mixed-auto-min-precomparison-verification/v1',
        'capturedAtUtc': metadata['capturedAtUtc'],
        'source': {
            'browserCli': metadata['browserCli'], 'userAgent': metadata['userAgent'], 'platform': metadata['platform'],
            'devicePixelRatio': metadata['devicePixelRatio'], 'viewportCssPx': metadata['viewportCssPx'],
            'fixturePath': str(FIXTURE), 'fixtureBytes': len(fixture_bytes), 'fixtureSha256': fixture_sha,
            'observationPath': OBSERVATION.name, 'observationBytes': OBSERVATION.stat().st_size,
            'observationSha256': hashlib.sha256(OBSERVATION.read_bytes()).hexdigest(),
            'uniformControlObservationPath': PREVIOUS.as_posix(),
        },
        'sceneCount': scene_count, 'comparisonGroups': groups, 'uniqueCapOrders': len(cap_orders),
        'assignmentCountsPerMask': {family: 720 * 4 for family in TOPOLOGIES},
        'maskCounts': {family: mask_limit for family, mask_limit in expected_masks.items()},
        'uniformControlScenesRechecked': uniform_controls_checked,
        'autoItemPayloadFloorViolations': 0,
        'zeroMinimumPayloadFloorShortfalls': {family: sum(count for (item_family, _, mode), count in mask_floor.items() if item_family == family and mode == 'zero') for family in TOPOLOGIES},
        'gridEndOverflowOccurrencesByMinimum': {family: {
            'auto': sum(count for (item_family, _, mode), count in mask_overflow.items() if item_family == family and mode == 'auto'),
            'zero': sum(count for (item_family, _, mode), count in mask_overflow.items() if item_family == family and mode == 'zero'),
        } for family in TOPOLOGIES},
        'mixedMaskGeometryChangesVsAllZero': {f'{family}|{mask}': count for (family, mask), count in sorted(mask_geometry_changes.items())},
        'maxErrorsCssPx': {
            'spanFrameVsCoveredRows': max_span_error,
            'sourceOrderAndAlignmentInvariance': max_pair_error,
            'uniformControlRegression': max_pair_error,
        },
        'acceptance': {
            'allCapPermutationsPresent': len(cap_orders) == 720,
            'allPerItemMinimumMasksPresent': all(mask_counts[(family, mask)] == 2880 for family, limit in expected_masks.items() for mask in range(limit)),
            'allAutomaticMinimumItemsPreserveTheirPayloadFloor': all(mask_floor[(family, mask, 'auto')] == 0 for family, limit in expected_masks.items() for mask in range(limit)),
            'noPayloadCrossesGridEnd': all(mask_overflow[(family, mask, mode)] == 0 for family, limit in expected_masks.items() for mask in range(limit) for mode in ('auto', 'zero')),
            'sourceOrderAndAlignmentInvariant': max_pair_error <= TOLERANCE,
            'coveredRowFramesMatch': max_span_error <= TOLERANCE,
            'uniformZeroAndAutoControlsMatchPreviousCapture': uniform_controls_checked == 11520 and max_pair_error <= TOLERANCE,
        },
        'limits': [
            'Agent Browser Chrome CSS observation only; no Spinon implementation result.',
            'Fixed six-cap multiset, two row-span topologies, and definite 40x220px one-column Grid.',
            'No public CSS support, WPT, Android/iOS runtime geometry, device pixels, or C11.4 completion claim.',
        ],
    }
    OUTPUT.write_text(json.dumps(result, ensure_ascii=False, indent=2) + '\n')
    print(json.dumps(result, ensure_ascii=False, indent=2))


if __name__ == '__main__':
    main()
