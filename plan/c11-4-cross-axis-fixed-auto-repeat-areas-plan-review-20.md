# C11.4 교차축 fixed-repeat + auto-repeat + areas 계획 검토 20개

2026-10-25 기준 [계획](./c11-4-cross-axis-fixed-auto-repeat-areas.md), [11-scene fixture](../tests/fixtures/css/c11/grid-cross-axis-fixed-auto-repeat-template-areas-c114.html), [독립 CRD 기대값 및 Chrome 관측](../spec/internal/evidence/c11-4-cross-axis-fixed-auto-repeat-areas-precomparison-2026-10-25/README.md)을 구현 전에 서로 다른 실패 관점 20개로 검토했다. 각 검토는 재현 입력 또는 source 경계를 대조했으며, 지적은 계획/fixture/evidence를 수정한 뒤 재검토한다. 이 문서는 구현/runtime 검토나 계획 PR diff 검토를 대신하지 않는다.

| # | 독립 실패 관점 | 검토 결과와 조치 |
|---:|---|---|
| 1 | CSS Grid CRD 문서 수준·규범 인용 | 처음에는 링크 URL의 `css-grid-1`과 제목의 “Level 2” 표기가 일치하지 않았다. 계획·기대값·evidence를 **CSS Grid Layout Module Level 1 2025 CRD**로 맞췄다. 교차축 허용은 각 축의 독립 track list와 per-list auto-repeat 제한을 조합한 규범 해석으로 명시한다. |
| 2 | Fixture CSS 유효성 | `grid-template`의 track-list slash 문법 뒤에서 `grid-template-areas`를 별도 선언한다. 모든 named area는 직사각형이며 11-scene Chrome CSSOM은 모든 `gridTemplateAreas`를 유효한 행렬로 반환한다. |
| 3 | auto-repeat count 산식 | `20n + 4(n−1) ≤ size`에 따라 44px은 2회, 76px은 3회다. 재현 validator가 두 경우 및 각 장면의 count를 확인한다. |
| 4 | auto-fit 가운데 빈 반복 및 gutter | 첫/마지막 row만 점유하고 가운데 named row에는 item이 없는 장면을 추가했다. Chrome track `[20,0,20]`, 마지막 item y=24는 접힌 row 양쪽 gutter도 제거됨을 분리해 확인한다. |
| 5 | fixed-axis area dimension 불일치 | projection은 auto-repeat이 없는 축에서 area dimension과 expanded track 수를 같게 요구한다. 구현 계획에 columns-fixed/rows-fixed 양방향의 더 작은/큰 mismatch 거부를 추가한다. 이 11개 Chrome positive matrix는 fixed-axis mismatch를 통과시키지 않는다. |
| 6 | `grid-template` shorthand 범위 | track-list slash 형식만 다룬다. area 문자열을 포함하는 shorthand 변형과 혼동하지 않도록 scene ID/title에 track-list를 명시하고, 해당 parser branch positive 회귀를 계획한다. |
| 7 | rows/columns 전치 불변식 | 첫 장면과 shorthand 장면을 root 크기·area·item span까지 정확히 전치했다. CSSOM track vector는 `[20,20]`/`[20,20,20]`에서 역전되고 item frame은 `[0,0,20,44]`/`[0,0,44,20]`이 된다. |
| 8 | overflow 산출 범위 | overflow를 `max(0, child.right-root.right)` 및 `max(0, child.bottom-root.bottom)`로 정의했다. 캡처는 frame geometry만 증명하고 PixelCopy/Metal이나 실제 painted pixel을 증명한다고 주장하지 않는다. |
| 9 | 기대값과 Chrome의 독립 비교 | 수동 확인만으로 PASS를 남기면 재현되지 않는다는 점을 반영해 `validate-comparison.py`를 추가했다. 독립 기대값에 named-area matrix 전체, 모든 child key/frame, auto-repeat track indices를 기록하고, resolved columns/rows, fixed-axis area/track dimension, root size/auto-repeat axis, matrix 셀, 모든 child frame, overflow, 반복 수 공식, 정확한 repeat index 구간과 named-area 확장 뒤 explicit track 수를 비교한다. |
| 10 | 캡처 재현 정보 | Agent Browser 버전, Chrome full build, WebKit revision, host OS/build/arch, URL, viewport/DPR, 재실행 명령, cleanup과 파일 SHA-256을 `capture-metadata.json`에 기록한다. 재검토에서 발견한 중복 `sha256.plan` 항목은 제거해 경로당 하나의 hash만 남겼다. |
| 11 | fixed-repeat group 수·pattern 보존 | cascade `ComputedGridTracks`의 boolean만으로 허용 group 수와 track grammar를 구분할 수 없다. public layout 타입 대신 private cascade result에 group count와 bounded-`LengthPx` eligibility를 보존한다. Fixed-axis list는 prefix/suffix 없는 group 하나, finite nonnegative `LengthPx` 1–2개로 명시했고, CSS oracle에 `[16px 20px]` 2-item pattern positive scene을 추가해 authored order 확장을 확인한다. |
| 12 | auto 축 grammar와 fixed-repeat 혼입 | auto axis의 parser도 fixed group을 허용한다. 교차축 예외에서는 auto axis track list가 prefix/suffix 없는 single-breadth auto-repeat 하나이고 fixed group 0개, 반대 axis fixed group 정확히 1개로 조건을 분리했다. |
| 13 | 양축 auto-repeat + fixed 혼합 회귀 | 단순 guard 완화가 두 auto axes 조합을 열지 않도록 `fixed_repeat_mixture_is_limited_to_one_auto_repeat_axis` negative regression을 명시하고 유지한다. |
| 14 | areas가 없는 cross-axis 조합 | 기능 예외의 일부가 `grid-template-areas` 존재에 의존한다. 같은 track CSS의 area 유무 쌍으로 no-area cross-axis가 계속 거부되는 것을 테스트한다. |
| 15 | opt-in profile 유출 | `RuntimeBlockFlexGridTrackSizingV1`에서만 허용하고 `RuntimeBlockFlexGridTemplateV1` 같은 비대상 profile에서 거부한다. 공개 CSS admission은 범위 밖이다. |
| 16 | fixed-axis area row/column 방향 | fixed columns와 fixed rows 각각에서 expanded fixed track 수보다 작거나 큰 area dimension을 node/property 오류로 거부하도록 acceptance에 적었다. Auto-axis의 area 확장만 positive다. |
| 17 | track grammar·metadata 한도와 allocation | Fixed pattern 3+ tracks, `minmax`, percentage, flex, intrinsic 및 normalized-absolute-LengthPx가 아닌 calc breadth는 거부한다. Auto axis의 ordinary prefix/suffix와 multi-track repeat pattern도 거부한다. Cascade 10,000 tracks/1 MiB, projection combined line+area metadata 1 MiB, layout 10,000 tracks/source metadata 1 MiB/expanded metadata 16 MiB도 고정했다. Spinon-owned reservation fault 후 같은 engine 회복을 검사하며 Taffy 내부의 모든 OOM 회복은 보장하지 않는다. |
| 18 | 기존 기능 상호 회귀 | #545 dual-auto positive, #604 same-axis fixed+auto positive, 새 cross-axis positive 및 dual-auto + fixed negative를 분리해서 실행하도록 정리했다. |
| 19 | runtime fixture schema와 기본 artifact 경계 | Rust scene registry/dispatch, Android/iOS allowlist/expected adapters와 fixture boundary 검사뿐 아니라 capture script의 template JSON 29→40 및 전체 corpus 473→484 assertion, Rust fixture test의 최소 2-item/track-array schema를 갱신한다. Android/iOS simulator는 신규 cross-axis ID 11개만 `--scene-id`로 선택한다. Default/product archive 0 대 opt-in positive control만 주장하고 APK/IPA 전체 exclusion으로 확대하지 않는다. |
| 20 | 완료 판정과 문서 동기화 | 계획/구현 PR을 구분하고 implementation에서 `STATUS.md`, 내부 계약 0059, conformance, public roadmap 및 evidence를 갱신하도록 적었다. 계약 `0.1.0`, C11.4 checkbox와 공식 `72/213`은 상위 완료 조건 전까지 유지한다. |

## 검토 상태

초기 독립 검토 20개에서 표준 문서 수준, auto-fit 가운데 gutter, 축 전치, overflow/evidence 재현성, fixed-repeat pattern 범위, runtime capture schema/count를 보강했다. 추가 적대적 검토에서 빠진 2-item fixed pattern oracle, area/child exact validator coverage와 repeat-axis mapping을 찾아 보강하고, 11 scene으로 다시 비교해 재검토했다. 구현에는 아직 들어가지 않았으며, 이 계획 PR diff는 사용자가 지정한 별도의 실패 관점 20개([검토 결과](./c11-4-cross-axis-fixed-auto-repeat-areas-plan-pr-diff-review-20.md))로 확인했다.
