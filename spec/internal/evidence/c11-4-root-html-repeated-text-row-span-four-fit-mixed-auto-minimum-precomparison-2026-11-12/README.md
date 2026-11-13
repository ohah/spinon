# C11.4 four-fit repeated-text row-span item별 `min-height:0|auto` 사전 비교

## 기준과 범위

- 기준 구현은 Agent Browser `0.31.1`의 HeadlessChrome `157.0.0.0`이다. macOS arm64, DPR 1, viewport `1280×900 CSS px`에서 실행했다. Terminal Browser는 사용하지 않았다.
- [HTML fixture](../../../../tests/fixtures/css/c11/root-html-grid-repeated-text-row-span-four-fit-mixed-auto-minimum-precomparison.html)는 기존 four-fit row-span 입력, text, placement와 `40×220px` definite Grid를 유지하고 span item별 minimum mask만 추가한다.
- `grid-auto-rows` 720개 목록은 15 fit-position 조합 × 24 typed cap 순열 × 두 tail order다. 각 목록에서 span-2 topology 32개 mask와 span-2/span-3 topology 16개 mask를 양쪽 DOM 순서와 `align-content:start|stretch`로 실행했다.
- 전체 Chrome CSS 관측은 138,240 scenes다. 이는 Spinon RuntimeSession, iOS/Android 앱, surface pixels 또는 C11.4 구현 결과가 아니다.

## 독립 검증 결과

[독립 verifier](./verify-chrome-matrix.py)는 연속 scene index와 mask bit order, 정확한 authored six-track list·cap 위치·tail, computed track serialization, definite Grid bounds, item/payload/line geometry, 실제 span track eligibility, source-order pair, alignment pair, 기존 uniform endpoint 회귀와 압축물 hash를 확인한다. item frame과 span row 합의 허용 오차는 `0.01 CSS px`다.

| 측정 | 결과 |
| --- | ---: |
| Chrome scene / 고유 track list | 138,240 / 720 |
| Fit 위치 조합 / cap 순서 / tail order | 15 / 24 / 2 |
| topology별 mask | span2Six 32 · span2Span3Six 16 |
| 기존 uniform four-fit endpoint 재비교 | 11,520 scenes · 최대 차이 0 CSS px |
| automatic-minimum 대상 payload-floor 위반 | 0 |
| zero minimum payload-floor 미달 | span2Six 28,032 · span2Span3Six 9,216 |
| Grid 끝 초과 | 1,392, 전부 `min-height:0` scenes |
| source-order / non-auto stretch geometry 차이 | 각 0 CSS px |
| item frame와 covered rows 최대 차이 | 0.00045 CSS px |

이 결과는 해당 고정 Chrome fixture에서 per-item automatic minimum mask가 기존 uniform endpoint를 보존하며 중간 mask에서 관측 가능한 geometry 변화를 만든다는 뜻이다. 제품 RuntimeSession에서 동일한 결과를 검증하기 전에는 구현 보장으로 해석하지 않는다.

초기 verifier 실행은 scene 0에서 computed `min-height:0px`와 verifier 집계 label `zero`를 혼동한 assertion으로 중단됐다. 비교 구현 문제가 아니라 검증기 기대 문자열 오류였고, assertion을 CSS computed value `0px`로 수정한 뒤 같은 관측 hash에서 전체 138,240 scenes를 다시 검증해 통과했다.

## 산출물과 재현

- 전체 관측: [압축 NDJSON](./chrome157-four-fit-mixed-observation.ndjson.gz)
- 대표 혼합 mask 화면: [PNG](./chrome157-four-fit-mixed-sample.png)
- 수집 환경과 fixture hash: [capture manifest](./capture-manifest.json)
- 독립 요약: [verification](./verification.json)
- 구현 전 plan review 20개: [review](../../../../plan/c11-4-root-html-repeated-text-row-span-four-fit-mixed-auto-minimum-plan-review-20.md)
- 계획 PR 변경분 review 30개: [review](./plan-pr-diff-review-30.md)
- 캡처 후 전용 Agent Browser 세션 `spinon4mix12/4mixpre`를 닫았다. `agent-browser session list`는 `No active sessions`를 반환했다.

```sh
node spec/internal/evidence/c11-4-root-html-repeated-text-row-span-four-fit-mixed-auto-minimum-precomparison-2026-11-12/capture-chrome.mjs
python3 spec/internal/evidence/c11-4-root-html-repeated-text-row-span-four-fit-mixed-auto-minimum-precomparison-2026-11-12/verify-chrome-matrix.py
```

## RuntimeSession 실행 표본

RuntimeSession 검증은 전체 138,240개 Chrome oracle이 아니라 계획에 정한 20개 완전 track-list context의 3,840 scenes를 사용한다. 표본은 15개 fit-position set을 모두 포함하고, 24 cap order 중 결정적으로 선택한 20개와 두 tail order를 포함한다. 각 context는 네 topology와 item별 mask, source-order, alignment 조합 192개를 전부 보존한다.

- 선택 표본: [runtime-matrix-3840.ndjson.gz](./runtime-matrix-3840.ndjson.gz)
- 표본 작성기: [build-runtime-sample.py](./build-runtime-sample.py)
- 독립 표본 verifier: [verify-runtime-sample.py](./verify-runtime-sample.py)
- 원본 Chrome oracle SHA-256: `312497ced752c543ba367b57995c49bf00fe51bb5bcdd22f5302af0d7fc22570`

```sh
python3 spec/internal/evidence/c11-4-root-html-repeated-text-row-span-four-fit-mixed-auto-minimum-precomparison-2026-11-12/build-runtime-sample.py
python3 spec/internal/evidence/c11-4-root-html-repeated-text-row-span-four-fit-mixed-auto-minimum-precomparison-2026-11-12/verify-runtime-sample.py
```

표본 검증기는 원본 oracle hash, 선택 규칙, 모든 3,840 records의 원본 scene 내용·순서, 20개 track list, mask 집합 및 topology 변형을 확인한다. RuntimeSession이 이 표본과 일치했다는 주장은 별도 실행 결과와 함께 기록한다.

## 한계

두 fixed row-span topology, 네 typed fit caps, 두 intrinsic tail order, 한 definite Grid와 고정 text/font/placement만 포함한다. 일반 CSS admission, 다른 fit 개수, auto-placement, WPT 전체, Android/iOS runtime geometry, device pixels, compositor parity, 실기기·hardware GPU와 C11.4/C11 완료는 확인하지 않는다.
