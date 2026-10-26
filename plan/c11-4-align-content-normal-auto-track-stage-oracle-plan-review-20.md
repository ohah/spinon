# C11.4 `align-content: normal` §11.8 계획·사전 비교 적대 검토 20개

**대상:** [계획](./c11-4-align-content-normal-auto-track-stage-oracle.md), [사전 비교 README](../spec/internal/evidence/c11-4-align-content-normal-auto-track-stage-precomparison-2026-10-26/README.md), [`input.json`](../spec/internal/evidence/c11-4-align-content-normal-auto-track-stage-precomparison-2026-10-26/input.json), [`expected.json`](../spec/internal/evidence/c11-4-align-content-normal-auto-track-stage-precomparison-2026-10-26/expected.json), [상위 계획 색인](./c11-4-grid-track-sizing.md).

**기준:** CSS Grid Layout Module Level 1, 2025 CRD [§11.8](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-stretch), `docs/project-rules.md`, 현재 `origin/main` `77fb2144d72b5bab2f12e7e61c33be54aa0f31dd`. §11.8 원문은 Agent Browser로 확인했다. 계획·사전 비교·코드는 수정하지 않았고 테스트는 실행하지 않았다.

1. **규범 버전·섹션 식별** — 링크는 CSS Grid 2025 CRD의 `#algo-stretch` §11.8을 가리킨다. 계획이 §11.6 Maximize Tracks를 auto-track stretch 단계로 혼동하거나 오래된 규칙을 섞지 않았다. **PASS**
2. **§11.8 적용 조건** — CRD는 해당 축의 grid-container content-distribution 값이 `normal` 또는 `stretch`일 때 `auto` max track sizing function을 가진 track에 남은 양의 definite free space를 균등 분배한다고 정한다. 계획의 조건과 대상이 원문에 맞다. definite minimum-size fallback은 별도 경로로 남겼다. **PASS**
3. **정렬 속성과 축 선택** — 행 축에는 `align-content:normal`, 열 축에는 `justify-content:start`를 둔다. inline-axis 정렬이나 item의 `align-items`를 content distribution 조건으로 오인하지 않는다. **PASS**
4. **정상값과 적용 의미** — 새 입력은 `LayoutAlignContent::Normal`로 구성하고 기대 stage는 `StretchAutoTracks { applied: true }`다. 고정된 `taffy_style.rs` mapping도 `Normal`을 Taffy `STRETCH`로 보낸다. **PASS**
5. **Definite free-space 전제** — root block size와 viewport height가 모두 100px로 고정되고 `RootSizingPolicy::Match`를 사용한다. 45px의 양의 여유 공간을 indefinite basis나 추정 크기로 계산하지 않는다. **PASS**
6. **content-box basis** — root border/padding은 0이므로 definite 100px height가 이 입력의 content size와 일치한다. border-box/content-box 변환이나 세로 여백을 산식에서 빠뜨릴 여지가 없다. **PASS**
7. **trace 활성화 입력** — `GridPlacementMode::TrackSizing`을 명시해 stage trace가 있는 입력을 구성한다. 기본 placement mode의 빈 trace를 성공으로 오인하지 않는다. **PASS**
8. **children과 행 배치** — direct block child 세 개의 source order와 고정 높이 `[10,20,5]px`를 입력에 고정한다. 각 child가 한 행씩 차지하는 topology 및 fixed third track의 연결이 분명하다. **PASS**
9. **stretch 전 기초 크기** — 독립 기대값은 `IntrinsicSizingComplete`, `MaximizeTracks`, `ExpandFlexibleTracks` 뒤 rows `[10,20,5]px`다. 고정 높이 아이템과 고정 5px 행을 입력에서 분리했으며, Taffy 결과로 기대값을 역산하지 않는다. **PASS**
10. **gutter 개수·위치** — 세 행에는 내부 row gap이 두 개뿐이며 각각 10px이다. 선행·후행 trace sentinel은 0px이고 CSS row track 크기 합계와 혼동되지 않는다. **PASS**
11. **free-space 산술** — `100 - (10 + 20 + 5) - (10 + 10) = 45px`로 규범 단계 직전의 양의 definite free space를 정확히 계산했다. gap 누락·중복이면 식이 달라진다. **PASS**
12. **eligible track 집합** — auto max function을 가진 행은 indices 0, 1뿐이고 third track은 fixed 5px다. 모든 행을 분배 대상으로 넣지 않는다. **PASS**
13. **균등 증가량** — 45px을 두 eligible 행에 나눠 각 22.5px 증가시킨다. 기대 rows `[32.5,42.5,5]px`는 증가량 산술과 일치한다. **PASS**
14. **최종 합계·fixed-track 불변식** — `32.5 + 42.5 + 5 + 10 + 10 = 100px`이며 third row는 5px 그대로다. 최종 남은 row space가 0px이라는 JSON 기대도 일관된다. **PASS**
15. **stage snapshot 경계** — 계획은 `IntrinsicSizingComplete → MaximizeTracks → ExpandFlexibleTracks → StretchAutoTracks`의 고유성과 순서를 검사한다. pre-stretch 값을 stretch 뒤 값과 뒤섞어 비교하지 않는다. **PASS**
16. **trace slot 구조** — CSS tracks와 Taffy raw gutter entries를 분리하고 두 내부 gutter와 양 끝 sentinel을 검사한다. 단순히 첫 세 raw entry를 CSS track으로 읽는 실수를 막는다. **PASS**
17. **frame 기대값 유도** — root `(0,0,20,100)`과 child frames `(0,0,20,10)`, `(0,42.5,20,20)`, `(0,95,20,5)`는 track 시작 위치와 고정 child 크기에서 나온다. start control의 y `[0,20,50]`도 기존 크기·gap 합과 맞다. **PASS**
18. **독립 oracle과 대조군** — 새 `normal` 결과는 기존 Taffy `stretch` 출력에서 유도하지 않고 spec 산술·고정 입력으로 직접 비교한다. 기존 `stretch` positive와 `start` 대조군은 비교 확인용으로만 쓰며 각각의 expected values도 명시했다. **PASS**
19. **비교 허용치·유한성** — `1e-4 CSS px` 절대 오차와 finite 값 조건을 기존 helper에 맞춘다. track·frame 모두 같은 comparator를 사용하며 half-pixel 기대값은 f32에서 정확히 표현 가능하다. **PASS**
20. **검증 명령·범위·완료 상태** — 보완된 계획은 정확한 focused `cargo test --locked -p spinon-layout css_grid_2025_align_content_normal_stretches_auto_tracks_equally`, `cargo fmt --all -- --check`, 프로젝트 기본 Clippy `-D warnings`, `git diff --check`를 적었다. `#[cfg(test)]` 모듈의 테스트만 추가하므로 앱/FFI 경로와 Android/iOS Simulator 검증은 적용되지 않는다는 경계도 타당하다. 계획 전 20관점·계획 PR diff 30관점, 구현 후 새 20관점·PR diff 30관점을 각각 분리하며 C11.4와 `72/213`을 그대로 둔다. **PASS**

## 판정

**PASS 20 / PARTIAL 0 / FAIL 0. 차단 항목 0개.** 처음 확인한 검증 명령 누락은 계획에 focused test, formatter, Clippy 및 diff-check 명령을 추가해 해소됐다. §11.8 조건, 독립 free-space 산술, track/frame 기대값, 상위 계획 링크 및 test-only 범위가 서로 일치한다.

Agent Browser 검토 세션 `spinon-c114-align-normal-review-20261026`은 닫았다. 종료 뒤 활성 session 목록은 비었고 daemon PID는 실행 중이지 않았다. 이 리뷰에서는 Cargo test, formatter, Clippy, simulator 또는 기타 테스트를 실행하지 않았다.
