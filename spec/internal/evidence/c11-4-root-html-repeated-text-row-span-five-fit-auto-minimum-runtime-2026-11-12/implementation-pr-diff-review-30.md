# PR #1360 실제 변경 실패 관점 검토 30개

검토일: 2026-11-12

검토 기준은 GitHub PR #1360의 원격 diff다. 원격 diff는 136,562 bytes였고, `origin/main...origin/feat/c11-4-five-fit-row-span-20261111`의 local diff와 byte-for-byte 같았다. 아래 검토는 계획 검토 20개와 구현/runtime 검토 20개를 반복하지 않고, PR에 실제 포함된 코드·테스트·문서·화면 증거의 변경 경계와 실패 경로를 다시 확인했다.

| # | 별도 실패 관점 | 검토 결과 |
|---:|---|---|
| 1 | root HTML 외 일반 Grid가 새 predicate에 우연히 들어오는 경우 | style projection의 기존 Root HTML profile gate 안에서만 predicate를 호출한다. 공개 일반 Grid admission은 바뀌지 않는다. |
| 2 | 5개 fit cap만 맞고 list 길이가 달라도 허용하는 경우 | style/layout 양쪽이 정확히 여섯 member를 요구한다. 5·7 member rejection이 있다. |
| 3 | px cap의 단위가 다른 숫자와 섞이는 경우 | `[0,10,30,40]px`는 `LengthPx`로 비교하고 `20%`만 `Percentage`로 비교한다. 숫자만 같은 잘못된 typed cap을 거부한다. |
| 4 | 유효 cap 5개 안에 중복이 포함되는 경우 | 두 admission predicate가 `caps_seen`을 각각 유지한다. 중복 cap은 row-span 지원에서 거부된다. |
| 5 | min/max/auto tail의 중복 또는 부재 | singleton counter와 list 길이를 함께 검사하며 positive/negative tests가 각 tail을 분리한다. |
| 6 | 승인하지 않은 50px cap이 row-span 지원으로 승격되는 경우 | row-span predicate는 40px까지만 허용한다. 기존 auto-placement의 50px 허용은 별도 predicate로 남고 회귀 test가 두 결과를 구분한다. |
| 7 | 다섯 fit row position의 line 범위가 여섯 implicit row 경계 밖으로 나가는 경우 | layout/style의 5-position branch는 각 위치를 1..=6으로 제한하고 strict ascending order를 확인한다. |
| 8 | 기존 1·3·4-fit 또는 auto-placement 동작을 대체하는 경우 | 기존 분기와 함수를 보존하고 5-fit row-span predicate를 앞에 추가한다. 영향받는 prior regression이 계속 빌드된다. |
| 9 | row-span layout contract가 5 fit을 허용하면서 child minimum 조건을 약화하는 경우 | layout 계약의 fit count만 확장하고 나머지 children은 기존 `min-height:auto` 조건을 유지한다. |
| 10 | style validation이 row-span만 확장하고 item eligibility를 넓히는 경우 | `span_auto_minimum_items_candidate`의 허용 위치 수를 추가하고 모든 직접 text item의 기존 조건을 유지한다. |
| 11 | 차트·matrix fixture의 expected 결과를 후보 구현에서 다시 생성하는 경우 | Chrome observation gzip을 고정 입력으로 include하고 독립 `row_values`·frame observation과 대조한다. |
| 12 | gzip test dependency가 product runtime dependency로 들어가는 경우 | `flate2`는 runtime crate의 dev-dependencies에만 추가된다. lockfile diff도 해당 항목 하나다. |
| 13 | track-list matrix가 34,560이라는 수만 선언하고 실제 조합을 빠뜨리는 경우 | metadata test가 6 fit positions × 120 cap assignments × 3 tails, 두 alignment, 네 topology, 두 minimum과 scene 수를 확인한다. |
| 14 | baseline/candidate 쌍 사이에서 track list·items·alignment가 달라지는 경우 | matrix가 쌍별 모든 scene metadata를 동일 비교하고 minimum 값만 `zero`/`auto`인지 검사한다. |
| 15 | 4개 matrix chunk가 겹치거나 scene을 건너뛰는 경우 | 각 청크 경계를 540씩 나누고 전체 실행에서 네 청크가 0..2160 범위를 한 번씩 덮는다. test result는 4/4다. |
| 16 | item box만 맞고 text payload가 잘려도 성공으로 처리하는 경우 | eligible candidate 각각에서 payload frame 높이와 item frame 높이의 floor를 세고 전체 candidate violation이 0인지 단언한다. |
| 17 | empty explicit-row control이 자동 최소 후보 분모에 들어가는 경우 | `empty`와 CSS eligibility 밖 item을 제외한다. baseline/candidate별 분모와 floor 결과를 분리한다. |
| 18 | row-span 결과가 DOM source order에 따라 달라지는 경우 | paired topology의 forward/reverse scene에서 rows, item frames, payload frames를 서로 비교한다. |
| 19 | `align-content:stretch`가 잘못된 tail이나 fit row를 성장시키는 경우 | 같은 key의 start/stretch pair에서 `auto` tail 외 row가 변하지 않는지 검사한다. `min-content`와 `max-content`는 모두 고정된다. |
| 20 | auto tail의 stretch가 작은 free-space 분수 때문에 payload floor를 깨는 경우 | Chrome observation의 tail-specific row와 payload floor를 둘 다 확인한다. 네 matrix chunk에서 candidate floor 오류가 0건이다. |
| 21 | item trace id가 없거나 다른 Grid owner의 contribution이 합쳐지는 경우 | simulator는 고유 Grid trace, item map 소속, line/span 범위를 확인하고 불일치 시 결과를 거부한다. |
| 22 | implicit contribution batch가 column axis의 flex 조건을 잘못 재사용하는 경우 | 선택된 pass의 axis를 rows로 제한하고 `crosses_flexible_track` batch를 거부한다. |
| 23 | growth-limit phase에서 0 증가가 무시되어 max-content tail이 잘리는 경우 | 0 증가 commit 회귀와 세 `max-content` tail placement test를 포함했고 runtime filter에서 통과했다. |
| 24 | maximum size function이 다른 intrinsic contribution 단계에 적용되는 경우 | 다섯 contribution phase를 순서대로 나누고 단계별 Taffy trace에 대조한다. |
| 25 | fit cap이 automatic-minimum base contribution에도 잘못 제한되는 경우 | base-size와 growth-limit potential을 분리하고 Chrome payload-floor matrix로 candidate 동작을 확인한다. |
| 26 | percentage basis 또는 indefinite track value를 억지로 정수화하는 경우 | percentage는 definite grid height로 해석하고 unresolved `Calc`, 음수·비유한 값은 sizing helper에서 거부한다. |
| 27 | rounding simulator가 track trace와 다르게 계산해도 layout을 반환하는 경우 | 각 span batch 및 pre-alignment/final track의 base·growth limit을 비교하며 불일치 시 `None`을 반환한다. |
| 28 | 실패한 dynamic update가 이전 revision trace/frame과 새 값을 섞는 경우 | runtime observer가 layout/cascade key를 비교하고 rejection tests는 atomic 실패 뒤 같은 session recovery를 검증한다. |
| 29 | PR 화면이 feature matrix나 실제 surface pixel parity를 입증하는 것처럼 쓰이는 경우 | README·PR 본문이 네 화면을 default route smoke로 한정하고 row-span fixture, PixelCopy/Metal readback, compositor parity가 아님을 명시한다. |
| 30 | 작업 PR의 문서 변경이 공식 완료 수나 C11.4 전체 완료를 조용히 바꾸는 경우 | PR은 plan/evidence만 갱신하고 `spec/STATUS.md`, C11.4/C11 checkbox, 공식 `78/214` 및 internal `0.1.0`은 유지한다. Tailnet/public roadmap 동기화는 merge 뒤 별도 단계다. |

## 판정

30개 관점에서 merge blocker를 발견하지 못했다. PR diff에는 35개 파일이 있으며, GitHub Actions workflow run과 commit status check는 0개다. 따라서 이 검토와 로컬 테스트를 CI 통과로 표현하지 않는다. C11.4/C11 완료 체크와 공식 `78/214`는 미완료 상태로 남긴다.
