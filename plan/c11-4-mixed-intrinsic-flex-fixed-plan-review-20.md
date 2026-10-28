# C11.4 mixed intrinsic/flexible/fixed track 계획 실패 관점 검토

**대상:** [집중 계획](./c11-4-mixed-intrinsic-flex-fixed.md)와 [사전 비교](../spec/internal/evidence/c11-4-mixed-intrinsic-flex-fixed-precomparison-2026-10-28/README.md)

| # | 독립 실패 가설 | 결과 | 근거와 방어 |
|---:|---|---|---|
| 1 | CSS Grid 규범과 Chrome 결과를 같은 oracle처럼 취급해 Chrome serialization 차이를 정답으로 고정한다. | PASS | §11.5.1/§11.7/§11.7.1 rational expected와 Chrome 156 CSSOM/DOMRect를 분리했다. 앱은 independent 값을 판정 기준으로 삼는다. |
| 2 | 잘못된 specification revision이나 flexible expansion section을 인용한다. | PASS | 계획과 README가 동일한 CSS Grid 2025 CRD §§11.5.1, 11.7, 11.7.1 deep link를 고정한다. |
| 3 | Ahem font가 늦게 로드되거나 text metrics가 바뀌어 100/80/40 입력이 다른 값으로 비교된다. | PASS | Agent Browser에서 `document.fonts.ready`, `document.fonts.check("10px Ahem")`, font status와 파일 SHA-256을 확인했다. 관측 JSON과 fixture hash를 함께 기록했다. |
| 4 | nowrap/문자 폭의 min-content 해석이 기대값의 100/80/40을 뒷받침하지 않는다. | PASS | 고정 Ahem 10px의 `X` 수를 10/8/4로 나눈다. Chrome에서 대응 first track이 100/80/40px로 나타나며 independent input은 browser used value에서 역산하지 않는다. |
| 5 | vertical row scenes가 row-axis intrinsic input을 만들지 못하거나 writing mode를 다르게 해석한다. | PASS | Rows는 `vertical-rl` Ahem text를 쓰고 Chrome에서 freeze/equal/no-freeze가 각각 100/80/40px로 나타난다. CSS/OS direction은 관측 화면과 expected JSON에 고정했다. |
| 6 | definite axis 크기를 border/padding이나 viewport 크기와 혼동한다. | PASS | Root는 zero border/padding의 고정 300px target axis, 120px cross axis다. 기대 계산은 content axis를 직접 사용한다. |
| 7 | gutter가 두 번 또는 한 번만 제외되어 initial hypothetical flex fraction이 80px이 아니다. | PASS | 한 축의 내부 gutter 두 개를 각각 10px로 고정하고 `300 - 40 - 20 = 240`, `(1fr+2fr)=3`, 초기 fraction 80px를 기록한다. |
| 8 | fixed 40px neighbor가 flex track으로 취급되거나 두 flex factor 합산에 포함된다. | PASS | 가운데 track은 authored 40px fixed track이고 flex factors는 outer tracks 1과 2뿐이다. Chrome used tracks와 independent track tuple이 40px 가운데 값을 분리한다. |
| 9 | 1fr base 100px의 freeze/recompute가 남은 공간 계산에서 fixed track/gutter를 다시 빼서 결과를 틀리게 한다. | PASS | 240px flexible space에서 frozen 100px를 한 번 제외하면 140px이 남고, 2fr track은 140px이다. track+gutter 합은 300px이며 remaining free space는 0이다. |
| 10 | equality case가 `<`와 `<=` freeze 조건을 구별하지 못한다. | PASS | 80px min-content base와 80px hypothetical size를 Columns와 Rows 각각 고정했다. strict-less 조건에서는 freeze하지 않고 `[80,40,160]`을 만든다. |
| 11 | no-freeze case가 freeze threshold와 너무 가까워 float rounding 경계에 걸린다. | PASS | 별도 40px base case를 두어 80px hypothetical과 40px 차이를 둔다. 임계값 판정은 equality case가 맡는다. |
| 12 | 2fr track의 40px text contribution이 `minmax(0,2fr)`의 zero minimum을 무시하고 base를 강제로 40px로 만든다. | PASS | Independent expected는 min track function `0px`를 유지하고 `c`를 해당 track에 둔다. Runtime trace에서 base/final used sizes를 별도로 검사하고 Chrome expectation은 140/160px이다. |
| 13 | image file이 decode되지 않았거나 CSS width/height가 natural size를 가려 mixed source control이 무효다. | PASS | Image는 CSS width/height `auto`, self-alignment start, 24×16px decoded natural source를 사용한다. Chrome observation은 scene의 eight image decode 완료를 기록한다. |
| 14 | item source order 변화가 CSS placement를 바꾸거나 identity별 frame 검증과 섞인다. | PASS | Freeze control을 Columns/Rows 각각 `[a,b,c]`와 `[c,b,a]`로 비교하며 explicit line placement를 유지한다. Browser observation과 expected frames는 item identity로 연결한다. |
| 15 | page-global y/x 좌표에 의존해 viewport나 card spacing 변화가 runtime expected를 오염시킨다. | PASS | Browser global frames는 원본 관찰에만 남기고 앱 acceptance는 root-relative `expected.json`을 사용한다. |
| 16 | Chrome의 1/64px serialization 또는 app `f32` tolerance가 규범 rational expected를 덮는다. | PASS | Independent scene은 정수 rational이다. App tolerance `1e-4 CSS px`와 Chrome cross-check `1/64 CSS px`를 따로 쓴다. |
| 17 | `ExpandFlexibleTracks` 이전/이후 stage를 잘못 골라 최종 used sizes를 stage base로 오인한다. | PASS | Axis pass 수, `MaximizeTracks → ExpandFlexibleTracks → StretchAutoTracks` 순서와 해당 stage snapshot을 함께 요구한다. Stage API 내부 vector index는 고정하지 않는다. |
| 18 | known scene의 잘못된 style/source가 `None` fallback이나 partial output으로 성공한다. | PASS | Plan은 known scene identity와 content/style/revision mismatch를 fail-closed negative controls로 지정하고 partial intrinsic input/layout/draw 부재를 요구한다. |
| 19 | simulator screenshot을 surface pixel equality, real-device GPU 또는 root 밖 overflow 증거로 확대한다. | PASS | 이 장면은 expected overflow가 없다. Android API 37 ARM64 16 KB AVD와 iPhone 17 Pro/iOS 26.2 Simulator의 screenshot은 geometry/presentation만 뒷받침하며 surface equality/실기기 주장은 제외한다. |
| 20 | opt-in fixture가 default product artifact에 들어가거나 8개 제한 scene을 C11.4 전체/72→73으로 과장한다. | PASS | Default/product negative 및 fixture-test positive archive controls를 요구한다. 공식 C11.4 미완료와 `72/213`을 명시하고 CSS admission/WPT 전체/전체 Grid 완료를 제외한다. |

**결과:** PASS 20 / PARTIAL 0 / FAIL 0. 계획은 precomparison observation, source/font/image hash와 independent `derive_expected.py`로 구현 시작 전 판정 기준을 고정한다. 이 review는 계획 PR diff review나 구현/runtime review를 대신하지 않는다.
