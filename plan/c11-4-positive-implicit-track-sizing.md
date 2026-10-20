# C11.4 · 양의 implicit track의 intrinsic sizing과 목록 순환 계획

**상위 원본:** [C11 상태](../spec/STATUS.md#c11-css-grid) · [C11.4 track sizing 계획](./c11-4-grid-track-sizing.md) · [CSS Grid Level 1 2025 CRD §7.5–§7.6, §8.5, §11.3–§11.8](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#auto-tracks) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md)

**상태:** 계획과 Chrome 155 20-case 사전 비교를 준비했다. 구현 전 계획 자체를 독립 실패 관점 20개로 검토한다. 구현 전용 브랜치와 후속 PR은 계획 PR이 main에 병합된 뒤 최신 `origin/main`에서 시작한다. 이 계획만으로 C11.4 체크나 완료 수를 바꾸지 않는다.

## 목적

`RuntimeBlockFlexGridTrackSizingV1`은 explicit grid 뒤에 생기는 양의 implicit track을 현재 intrinsic track sizing 계산에 연결하지 못한다. CSS의 `grid-auto-rows`와 `grid-auto-columns`는 하나 이상의 track-size 목록이며, positive implicit track에 목록을 순서대로 반복 적용한다. 현재 cascade는 단일 고정 px 외의 목록을 보존하지 않고, layout normalizer는 implicit track이 필요할 때 `auto`를 거부한다. 이 계획은 기존 C11.4 track-size 계산기를 양 축의 positive implicit tracks에 연결하고 property의 authored list 순서를 유지한다.

## 확인된 경계

- `ComputedCssGridAutoRows/Columns`는 현재 `Auto`, 단일 `LengthPx`, `Unsupported`만 보존한다. CSS computed list가 1개가 아니거나 고정 px가 아니면 거부한다.
- `GridAutoTrackSize`는 `Auto` 또는 `LengthPx`뿐이다. dense normalizer의 `expand_tracks`는 px implicit track을 확장하지만 intrinsic track-size 목록을 입력받지 않는다. implicit row/column이 필요한 `auto`도 TrackSizing 경로에서 sizing 후보로 확장하지 않는다.
- C11.4의 template track projection은 이미 `LengthPx`, percentage, `fr`, `auto`, `min-content`, `max-content`, `minmax()`, `fit-content()`와 지원되는 CSS math를 typed `GridTrackSize`로 보존한다. 이를 implicit track 목록에서 재사용할 수 있는지 구현 전에 확인한다.
- 내부 계약 0059는 C11.2.2의 제한 `grid-auto-rows: <single fixed px>`와 C11.2.3의 양 축 고정 px subset을 별도 경계로 유지한다. C11.4 전용 확장이 이들 profile의 지원을 넓히지 않아야 한다.

## 범위

1. C11.4 TrackSizing profile의 `grid-auto-rows`·`grid-auto-columns`에 비어 있지 않은 `<track-size>+` 목록을 보존한다. 각 목록 항목은 현재 `grid-template-rows/columns` projection이 지원하는 typed `GridTrackSize` 문법을 사용한다. 값의 파싱·계산 결과를 새로 허용하지 않고, 기존 converter의 오류 경계를 유지한다.
2. explicit grid 뒤에 추가되는 positive implicit track의 index `i`에 `auto_tracks[(i - explicit_track_count) % auto_tracks.len()]`을 적용한다. Track list가 하나여도 유지하고, 목록의 순서와 축별 독립성을 보존한다.
3. explicit placement가 grid 끝을 넘겨 생성한 track과 row/column auto-placement가 생성한 track을 모두 sizing한다. 한 item이 explicit/implicit track 경계를 span하는 경우에도 track list 시작 위치를 바꾸지 않는다.
4. `grid-auto-rows`와 `grid-auto-columns`에 서로 다른 목록과 크기를 동시에 사용한다. `auto`, fixed length, percentage, `fr`, `min-content`, `max-content`, `minmax()`, `fit-content()` 및 현재 CSS math projector가 지원하는 값을 positive implicit track에서 확인한다.
5. 각 축의 10,000 track 상한과 Spinon 소유 list/input/output의 fallible reservation을 유지한다. 계산 오류·지원하지 않는 문법·상한 초과·검증 가능한 reservation 실패에서 부분 layout을 성공으로 게시하지 않는다. vendored Taffy 내부 working vector의 일반 할당은 `LayoutError`로 변환하거나 전역 OOM 복구를 보장하지 않으므로 완료 주장에서 제외한다.
6. `RuntimeBlockFlexGridTrackSizingV1`에만 적용한다. C11.2.2, C11.2.3, TemplateGrid 및 기존 profile의 auto-track 동작·오류를 회귀한다.
7. 새 사용자용 API는 추가하지 않는다. 내부 계약 0059는 `0.1.0`을 유지하며 typed input·오류·profile 제한만 기술한다.

## 범위 밖

- explicit grid 앞쪽의 negative implicit tracks와 규범의 역방향 목록 순환은 현재 placement model의 표현 범위 밖이며 구체적인 오류로 거부한다.
- `grid-auto-*`의 line names, `repeat()` 함수, template areas, out-of-flow item, 일반적인 item order, arbitrary CSS math 및 현재 `GridTrackSize` projection이 거부하는 computed value는 지원하지 않는다.
- `grid-template-*`에 auto-repeat이 있는 container와 positive implicit track의 결합은 이 slice에서 검증하지 않는다. auto-repeat expansion을 거치는 별도 placement path와의 교차 행렬은 C11.4 후속으로 남긴다. auto-repeat container와 C11.4 `grid-auto-rows`·`grid-auto-columns` typed 목록을 함께 주면 지원하지 않는 조합으로 명시 거부한다. `grid-auto-*` 자체에는 `repeat()` 문법이 없고, 해당 입력은 거부한다. `grid-template-*`의 auto-repeat·line placement·placement/spanning 전체 matrix는 계속 별도 미완료다.
- vertical writing mode, WPT full-suite, 실기기, hardware GPU 및 iOS compositor 이후 surface equality는 이 계획의 완료 기준이 아니다.

## 사전 oracle과 판정

[사전 비교 자료](../spec/internal/evidence/c11-4-implicit-intrinsic-auto-tracks-precomparison-2026-10-20/README.md)는 W3C 규범과 Chrome 관찰을 별도 파일로 고정한다. 20개 case는 direct intrinsic sizing, row/column auto-placement, positive implicit row·column에 한 축만 고정된 배치, list cycling, `auto`·fixed·percentage·fraction·`minmax()`·`fit-content()`·`calc()` 및 border 밖 overflow를 포함한다.

- 규범 원본: CSS Grid Level 1 2025 CRD §7.5–§7.6, §8.5, §11.3–§11.8.
- Chrome oracle: Agent Browser 0.31.1의 HeadlessChrome 155.0.8059.85, macOS 26.5.1, viewport 1280×633 CSS px, DPR 1, Ahem fixture font loaded.
- fixture와 binary/source 입력 hash는 evidence의 `browser-environment.json`, 원 관찰은 `chrome155-observation.json`, screenshot은 `chrome155-agent-browser.png`에 둔다. `spec-derived-expectations.json`은 구현 출력에서 역산하지 않은 규범 입력과 계산값이다.
- Track order, integer used size와 integer frame은 정확히 비교한다. fractional track/frame은 개별 오차 1/64 CSS px 이내로 고정한다. 평균으로 최대 오차를 숨기지 않는다.
- positive implicit pattern은 첫 implicit index부터 시작하며 각 축의 list cycle, explicit track prefix, occupied span, final track, item/root frame을 각각 판정한다. no-implicit control은 explicit grid가 바뀌지 않아야 한다.
- overflow case에서는 grid border 안·경계·밖 item surface color를 별도 좌표로 비교한다. Geometry success만으로 border 바깥 paint를 통과 처리하지 않는다.

## 구현·검증 순서

1. 이 계획과 Chrome input/expected values를 독립 실패 관점 20개로 검토한다. 발견한 누락은 구현 전에 계획·fixture·사전 비교를 함께 고친다.
2. 계획 PR을 `Rebase and merge`한 뒤 latest `origin/main`에서 구현 branch를 만든다.
3. Cascade snapshot에서 auto-track list의 순서·길이·축을 보존하고 typed projection은 기존 track converter와 fallible reservation을 재사용한다. unsupported values가 property/node 진단을 유지하는지 확인한다.
4. Layout container input이 track-size 목록을 소유하게 하고 positive implicit track expansion에서 pattern을 축별로 반복한다. explicit prefix를 건너뛰거나 columns/rows list를 섞지 않는다. allocation과 track cap에서 partial output을 금지한다.
5. Rust/style projection matrix에서 20개 oracle 및 zero/invalid/negative implicit, hidden item, cross-boundary span, pattern boundary, track cap과 allocation failure controls를 검증한다. unsupported grammar가 여전히 fail-closed인지 확인한다.
6. C11.4 opt-in app fixture에 oracle 20개 route를 연결한다. 기존 PR #372 기준 53-scene matrix와 신규 route를 합쳐 Android API 37 ARM64 AVD·iPhone 17 Pro/iOS 26.2 Simulator에서 각각 73/73 scene을 실행한다. root/item frame, used track list, axis trace, revision, draw/present를 검사한다. overflow case에서는 border 내부·stroke·바깥 PixelCopy/Metal 좌표를 고정한다. 실기기를 연결하지 않는다.
7. `cargo test --locked --workspace`, workspace all-target Clippy `-D warnings`, `cargo fmt --all -- --check`, focused CSS reference/runtime tests, Android/iOS build gates와 기본/fixture artifact 경계를 실행한다. 변경이 fixture-only라면 기본 artifact negative와 opt-in positive control을 둘 다 남긴다.
8. 구현 코드와 실제 runtime 동작을 새 실패 관점 20개로 적대 검토하고, PR diff는 저장소 규칙에 따라 별도 30개 관점으로 검토한다. 결함을 수정하면 영향받은 관점을 새 revision에서 재검토한다. 계획 표를 재사용하지 않는다.
9. PR 본문에 Android/iOS 화면을 가능한 경우 raw image link로 첨부한다. 현재 required checks와 merge state를 새로 확인해 `Rebase and merge`한다. required checks가 없으면 이를 적고 CI 통과로 말하지 않는다.
10. 병합 후 `spec/STATUS.md`, `spec/internal/0059-c11-grid-layout.md`, `spec/public/roadmap.html`을 실제 결과·미해결 범위와 맞춘다. C11.4 완료 체크와 완료 수는 전체 B/C/D 관문이 닫히기 전까지 유지한다. Tailnet 정적 미리보기는 최신 main 산출물 sync·served SHA·Agent Browser 화면을 검증하고 별도 PR로 기록한다. Tailnet sync PR은 새 독립 diff 실패 관점 30개를 수행한다.

## 완료 판정

이 slice의 완료는 계획된 positive implicit track list profile만 의미한다. Chrome 20개, Rust/profile/artifact 검사, Android와 iOS의 73/73 행렬, border 안팎 surface sample, trace·draw/present, 서로 분리한 code/runtime·PR diff 적대 검토를 모두 통과해야 한다. C11.4 intrinsic contribution matrix, negative implicit track, nested/anonymous item, full auto-repeat placement, WPT 및 C11 상위 전체 완료를 주장하지 않는다.
