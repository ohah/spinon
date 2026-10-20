# C11.4 · auto-repeat 뒤 negative line과 leading implicit track 계획

**상위 원본:** [C11 상태 대장](../spec/STATUS.md#c11-css-grid) · [C11.4 전체 track sizing 계획](./c11-4-grid-track-sizing.md) · [양축 auto-repeat 계획](./c11-4-two-axis-auto-repeat.md) · [negative implicit track 계획](./c11-4-negative-implicit-track-sizing.md) · [CSS Grid Layout Module Level 1 · 2025 CRD §7.2.3.2, §7.5–§7.6, §8.3–§8.5](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md)

**상태:** 계획과 18-case Chrome 155 사전 비교를 고정했고, 서로 다른 실패 관점 계획 검토 20개를 완료했다([검토표](../spec/internal/evidence/c11-4-auto-repeat-negative-lines-precomparison-2026-10-20/plan-review-20.md)). 구현 코드/runtime과 simulator 검증은 아직 수행하지 않았다. 이 조각만으로 C11.4 상태나 공식 완료 수를 올리지 않는다.

## 목적

`RuntimeBlockFlexGridTrackSizingV1`의 이미 제한 지원된 auto-repeat과 negative implicit placement를 결합한다. numeric negative line은 auto-repeat의 실제 repetition count가 포함된 explicit grid 끝에서 계산되어야 한다. 그 결과 explicit grid 앞에 생기는 track에는 `grid-auto-columns`·`grid-auto-rows` 목록을 역방향으로 반복한다. `auto-fit`은 같은 count를 유지하고 placement 뒤 빈 repeated track만 collapse한다.

## 현재 코드 경계

- Stylo typed projection은 authored track pattern과 축별 `GridAutoRepeat { start, end, mode }`를 함께 보존한다. CSS list의 auto-repeat 영역은 authored track vector에서 한 번 나타난다.
- `taffy_style/grid.rs`는 metadata를 Taffy의 `Repeat(AutoFill|AutoFit)` component로 전달한다. vendored Taffy `compute_explicit_grid_size_in_axis`는 layout 중 definite available size와 gap으로 실제 repetition count를 산출한다.
- 그보다 앞서 `grid_auto_placement/track_sizing.rs::normalize_track_sizing_grid_container`는 `container.columns.len()`·`container.rows.len()`을 explicit track count로 `resolve_axis`에 전달한다. 따라서 현재 normalization은 auto-repeat 확장 전 vector 길이로 numeric negative line을 계산한다. 이 정적 경계 관찰은 Spinon runtime failure 재현을 대신하지 않는다.
- signed placement 구현은 typed `grid-auto-*` list를 leading implicit track에 역방향 적용한다. 이 계획은 해당 로직과 C11.4 mode gate를 재사용하고, repeat count 계산과 placement normalization의 순서를 검증한다.

## 지원 후보

다음 조건을 모두 만족하는 `RuntimeBlockFlexGridTrackSizingV1`만 대상으로 한다.

1. `horizontal-tb`, LTR, fixed definite `width`·`height`, fixed non-negative px gap을 사용한다. fixture grid container에는 border와 padding을 두지 않아 content-box 크기를 고정한다.
2. columns와 rows 각 list에 auto-repeat 하나까지 허용하며 한 축만 쓰는 기존 입력과 두 축 조합을 함께 유지한다. `auto-fill`과 `auto-fit`은 축별로 독립 계산한다.
3. 반복 fragment는 기존 parser가 받아들이는 fixed px minimum의 단일 `minmax(<px>, <px>|1fr)` track이다. repeat 앞뒤 neighbor는 유한한 0 이상 fixed px다. 기존 typed `grid-auto-*` list는 비어 있지 않은 finite non-negative px 값으로 고정한다.
4. numeric positive/negative definite lines, reversed line pair, explicit 경계를 가로지르는 span, leading implicit tracks, trailing implicit control, 두 axis의 조합을 다룬다. zero line, extreme line/span, overlarge extent는 기존 오류와 cap으로 fail closed한다.
5. auto-repeat이 계산한 expanded explicit track count를 negative line resolver와 동일한 axis basis로 전달한다. `auto-fit` occupancy와 collapse는 item placement 뒤 적용한다. 구현 방법은 아래 순서에서 Taffy boundary와 함께 평가하며, 잘못된 size basis를 추측하거나 partial layout을 반환하지 않는다.

이 scope는 새 공개 CSS 지원 완료 선언이 아니다. 다른 profile, 일반 Grid 문법, C11.4 전체, WPT 적합성은 바꾸지 않는다.

## 사전 oracle과 판정 기준

[고정 입력·Chrome 155 관찰·독립 규범 기대·hash](../spec/internal/evidence/c11-4-auto-repeat-negative-lines-precomparison-2026-10-20/README.md)은 Agent Browser 0.31.1의 HeadlessChrome 155.0.0.0, macOS 26.5.1 ARM64, 1280×633 CSS px, DPR 1에서 수집했다. 18개 case는 columns 2/3/4회, rows 3/4/5회 반복, repeat-only axis, `auto-fill`/`auto-fit`, leading list cycle, reversed line, prefix span, 양축 조합을 포함한다.

반복 수는 각 축의 definite content-box size에서 independently 계산한다.

- Columns: `20 + 10 + 40n + 8(n+1) = 38 + 48n`; 134/200/230px에서는 각각 2/3/4회다. repeat-only 200px은 4회다.
- Rows: `14 + 6 + 30n + 6(n+1) = 26 + 36n`; 134/180/206px에서는 각각 3/4/5회다.
- Expanded explicit track 수 `E = prefix + repeatCount + suffix`다. CSS negative line `k < 0`의 zero-based boundary는 `E + 1 + k`; positive line `k > 0`은 `k - 1`이다. line 0은 거부한다.
- Leading implicit track은 explicit grid와 가장 가까운 위치부터 `grid-auto-*` list의 마지막 값을 받는다. 예를 들어 `[31,47]`은 leading 두 track의 왼쪽→오른쪽 크기가 `[31,47]`이다.
- `auto-fit` repetition count는 같은 입력의 `auto-fill`과 같아야 한다. collapse 대상과 gutter는 W3C 규칙을 기준으로 판정하고 Chrome 관찰만으로 규범 기대를 바꾸지 않는다.

정수 track size·repeat count·line boundary는 exact 비교한다. 일반 frame은 CSS px `1/64` 이내다. Chrome/규범 차이가 발견되면 관찰과 기대를 별도 보존하고, 기준을 결과 확인 뒤 바꾸지 않는다.

### 관찰된 Chrome 차이

`columns-fit-third-repeat-only`에서 Chrome 155는 `20px 0px 0px 154px 10px`와 prefix 뒤 8px item offset을 반환했다. W3C §7.2.3.2의 collapsed track/gutter 규칙은 해당 간격도 접도록 한다. 기존 [auto-fit 축별 계획](./c11-4-auto-fit-min-size-axis.md#2026-10-20-gutter와-fixture-경계-정정)의 Blink 차이와 같은 현상이다. validator는 이 auto-fit frame을 규범 기대값으로 사용하지 않는다.

## 구현 및 검증 순서

1. 이 계획을 코드 변경과 분리해 서로 다른 실패 관점 20개로 검토하고, 지적을 계획·fixture·oracle·판정 기준에 반영한다. 계획 PR의 최종 diff도 저장소 PR 규칙에 따라 30개 관점으로 검토한다.
2. 계획 PR을 `Rebase and merge`한 뒤 같은 `spinon` 체크아웃 디렉터리에서 branch만 `main`으로 전환하고 구현 branch를 만든다. 새 worktree는 만들지 않는다. merge SHA와 clean/in-scope status를 확인하고 기존 C12.4 untracked evidence 두 디렉터리는 계속 보존한다.
3. Taffy 0.15.0 source와 current adapter를 따라 definite available size, border/padding/content-box basis, gap, prefix/suffix, min/max fixed component, one-count floor, per-axis count와 track cap을 대조한다. 우선 Taffy가 이미 결정한 expanded count를 placement normalization에 전달할 수 있는 최소 경계를 찾는다. 불가능하면 제한 typed subset의 checked count resolver를 추가하고 Taffy output과 별도로 일치 검증한다. auto-fit collapse는 placement 후 유지한다.
4. Rust layout·style projection regression을 추가한다. 18 reference cases에 더해 repeat boundary의 바로 아래/정확/위 size, negative line inside/from-before explicit grid, reversed/equal/zero lines, spans across prefix, both-axis independent counts, repeat-only list, 0px auto track, hidden item, sparse/dense cursor, duplicate/overlap, placement order, 9,999/10,000/10,001 total-track limits, extreme signed arithmetic, invalid grammar와 unsupported profile을 검증한다. allocation-failure injection이 가능한 Spinon-owned reserve path에서는 partial output 부재와 동일 worker 재사용을 확인한다. Taffy internal allocator가 전역 OOM을 복구한다고 주장하지 않는다.
5. 기존 C11.4 106-case combined regression matrix를 유지하고, 그 안의 two-axis auto-repeat 33 IDs가 그대로 남았는지 exact subset으로 확인한다. 이 fixture의 18 unique IDs를 runtime matrix에 추가한다. 각 ID는 expected count, used tracks, leading origin, node frame, `auto-fit` collapsed range, stage trace, revision, draw/present marker와 1:1 연결한다. Android와 iOS 각각 기존 106-case와 새 18-case를 실행하고 정확한 unique ID union을 검사한다.
6. Android API 37 ARM64 AVD와 iPhone 17 Pro / iOS 26.2 Simulator에서 root/item geometry·trace·draw/present를 확인한다. 실기기는 연결하지 않는다. AVD software backend와 iOS Metal readback 시점은 실행 결과에 구분해 기록한다.
7. `columns-fill-negative-positive-cycle`의 fixed-size overflow fixture에서 Android PixelCopy와 iOS Simulator Metal drawable로 root interior background, explicit root extent와 root 바깥 trailing item/tail surface를 고정 좌표에서 확인한다. CSS border stroke 표본이라고 부르지 않는다. Chrome oracle 화면은 screenshot reference이며 Spinon simulator 동작 화면과 분리한다.
8. 실제 변경과 관련된 `cargo fmt --all -- --check`, focused layout/style tests, workspace tests, all-target Clippy, Android/iOS simulator builds와 fixture artifact boundary를 실행한다. 실행하지 않은 명령·플랫폼은 통과로 기록하지 않는다.
9. 구현 변경은 계획 표와 다른 실패 관점 20개로 code/runtime 검토한다. 최종 implementation PR diff는 독립 30개 관점으로 검토하고 발견 수정 뒤 영향을 받은 관점을 재검토한다. PR 본문은 Android와 iOS simulator 화면을 각각 raw image로 첨부하며 Chrome 이미지에는 oracle label을 붙인다.
10. 병합 직전 현재 required checks와 merge state를 다시 확인한다. 필수 status check가 설정되지 않았으면 그 사실을 기록하고 CI 통과라고 말하지 않는다. 사용자 지시에 따라 준비된 PR을 `Rebase and merge`한다.
11. 실제 지원 경계에 맞춰 내부 계약 0059, C11.4 상세 계획과 `spec/STATUS.md`의 설명·참조를 갱신하되 C11.4/C11 완료 체크와 공식 완료 수는 그대로 둔다. Tailnet roadmap preview는 main 문서 output과 SHA를 sync하는 별도 PR에서 갱신하고 HTTP/hash/Agent Browser 화면을 확인한다. 그 PR도 별도 적대 검토를 마치고 같은 checkout에서 순차 처리한다.

## 제외 범위와 완료 판정

line names/named spans, template areas, integer repeat와 auto-repeat 혼합, 한 axis의 여러 auto-repeat, non-px neighbor, intrinsic/indefinite repeat sizing, cyclic percentage, orthogonal writing mode, subgrid, nested/anonymous/out-of-flow items, 전체 WPT, physical device, hardware GPU 및 iOS compositor-after pixel equality는 이번 slice로 닫지 않는다.

이 조각을 후보 완료로 기록하려면 독립 18-case oracle와 validator, count/line/collapse/projection/layout/cap/error/profile 회귀, 기존 C11.4 106 및 auto-repeat 33 regression, 새 app case 양 플랫폼 실행, overflow surface 표본, trace/draw/present, implementation review 20개, implementation PR diff review 30개와 실제 화면 evidence가 모두 있어야 한다. 전체 C11.4와 C11은 그 뒤에도 미완료다.
