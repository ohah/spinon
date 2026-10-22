# 구현 PR diff 실패 관점 검토 30개

이 검토는 계획 검토와 구현/runtime 검토를 반복하지 않고 현재 PR diff의 계약 경계, 기존 경로 영향, fixture 증거 연결을 별도로 점검한다. 변경 코드의 실행 근거는 [runtime README](./README.md)와 [구현/runtime 검토](./implementation-runtime-review-20.md)에 있다.

| # | diff 관점 | 결과 |
| ---: | --- | --- |
| 1 | CSS Grid 2025 CRD §8.3의 missing/insufficient fallback과 구현 의미가 맞는가 | 통과 — 독립 계산과 고정 Chrome 156 관측을 precomparison에 분리 기록 |
| 2 | 변경이 모든 named-line projection에 적용되는가 | 통과 — `Some(auto_repeat)`의 auto-repeat axis projector에만 적용 |
| 3 | 일반 explicit grid의 authored-name 검사도 제거되는가 | 통과 — `project_axis_with_areas_and_implicit_named_lines`는 유지 |
| 4 | layout-core가 auto-repeat 밖 named placement를 새로 허용하는가 | 통과 — validator 호출은 auto-repeat normalization 내부로 한정 |
| 5 | projection이 repeat count를 재계산하는가 | 통과 — symbolic placement 전달만 하고 Taffy resolver를 재사용 |
| 6 | 이름 없는 단어를 별도 parser 토큰으로 재해석하는가 | 통과 — existing computed placement decode 결과를 그대로 소비 |
| 7 | bare occurrence의 기본값이 바뀌는가 | 통과 — 기존 `None → 0` 변환 유지 |
| 8 | explicit 0 occurrence와 bare occurrence를 합치는가 | 통과 — `occurrence_was_explicit` 구분 후 explicit 0 거부 |
| 9 | signed occurrence의 범위를 확장하는가 | 통과 — style/layout 양쪽의 Taffy integer guard 유지 |
| 10 | integer line의 0 rejection을 변경하는가 | 통과 — `GridPlacementInput::Line` 분기는 그대로 |
| 11 | named span count 0을 통과시키는가 | 통과 — decode 및 layout validator가 0 거부 |
| 12 | named span 큰 값을 truncation하는가 | 통과 — `u16::try_from` 경계와 negative test 유지 |
| 13 | 두 span endpoint를 자동 배치로 우회시키는가 | 통과 — paired-span conflict guard가 validator 앞에서 유지 |
| 14 | definite-axis classification을 metadata 존재에 의존시키는가 | 통과 — line/named-line placement만 사용하며 metadata 검색 제거 |
| 15 | 두 축 모두 auto-repeat일 때 고정축 분류가 반복 전 길이로 좌표를 계산하는가 | 통과 — definite classification만 하고 named occurrence lookup은 하지 않음 |
| 16 | fixed secondary axis가 non-auto-repeat일 때 기존 `resolve_axis` 결과를 바꾸는가 | 통과 — 해당 branch는 기존 resolver 호출을 유지 |
| 17 | column과 row implementation이 비대칭인가 | 통과 — 두 axis의 같은 projection/validator path; row positive/negative scene 추가 |
| 18 | line-name seam/merged occurrence counting이 이 변경에서 다시 구현되는가 | 통과 — 기존 expansion metadata와 resolver를 사용 |
| 19 | `auto-fit` collapse interaction을 검증한 것처럼 쓰는가 | 통과 — 문서에서 `auto-fill` subset으로 제한하고 `auto-fit` 제외 |
| 20 | implicit track size를 default `auto`로 두고 픽셀 기대값을 과장하는가 | 통과 — fixture가 고정 `grid-auto-columns/rows:20px`를 선언 |
| 21 | fixture adapter가 사용자가 넘기는 임의 scene 문자열을 실행하는가 | 통과 — JSON allowlist에서 단일 scene을 찾고 선택 scene만 serialization |
| 22 | fixture JavaScript가 `align-self` 같은 미지원 property를 요구하는가 | 통과 — 해당 속성은 제거됐고 고정 높이로 item sizing을 고정 |
| 23 | fixture scene ID 중복 또는 inventory count mismatch가 가능한가 | 통과 — 18 named-line inventory, global 297 count 및 uniqueness assertions |
| 24 | Android allowlist/expected frames/pixel IDs가 scene inventory와 다른가 | 통과 — 10개 신규 ID, frame·sample mapping 동기화 및 10/10 run |
| 25 | iOS allowlist/expected frames/pixel IDs가 Android와 다른가 | 통과 — 동일 10개 scene과 expected frame, Metal 30/30 exact |
| 26 | root 밖 표본을 root interior 또는 root edge라고 잘못 부르는가 | 통과 — item-end sample ID를 `outside-root`로 분리하고 frame 기반 판단 |
| 27 | sample 좌표가 renderer scale에 맞춰져 있는가 | 통과 — Android PixelCopy matrix와 iOS drawable scale 3.0 로그 모두 각 3 samples/scene |
| 28 | screenshot에서 보이지 않는 border stroke 통과를 주장하는가 | 통과 — plan/spec/evidence에서 border stroke 미검증 명시 |
| 29 | 기본 제품 artifact에 fixture가 섞이는가 | 통과 — product archive scene/source/error marker와 fixture host symbols 부재; fixture-test archive positive control에 모두 존재 |
| 30 | 제한 fixture 결과가 공식 CSS/Grid completion으로 승격되는가 | 통과 — C11.4 checkbox와 공식 `77/213` 유지, internal contract `0.1.0` 유지 |

`nm` evidence는 Rust 1.96 toolchain의 LLVM reader로 최종 스캔했다. Xcode 기본 `nm`이 LLVM 22 attribute를 읽지 못한 초기 시도는 완료 근거로 취급하지 않았다. PR merge는 GitHub merge state와 current checks를 별도로 조회한 뒤 기록한다.
