# C11.4 anonymous Grid text `white-space:pre-wrap` 구현/runtime 실패 관점 20개

계획 검토와 분리해 final-source implementation, adapter 경계, 고정 fixture, Android/iOS 실행과 artifact 근거를 확인했다. 각 항목은 별도의 실패 경로나 계약 경계를 다룬다.

1. **일반 CSS 경로로의 우발적 admission** — `WhiteSpaceMode::PreWrap`은 C11.4 anonymous paragraph collector의 명시 allowlist에만 추가했다. 일반 root HTML/CSS admission으로 등록하지 않았고 `STATUS.md`도 후보로 남아 있다. 통과.
2. **정렬 순서 우회** — collector는 기존 `order != 0` 거부를 유지한다. `anonymous_grid_pre_wrap_still_rejects_nonzero_order`가 해당 입력에서 `UnsupportedTextParagraph`를 확인한다. 통과.
3. **Normal/NoWrap/PreLine 정규화 회귀** — 새 `PreWrap` branch를 추가했으며 기존 collapse·nowrap·preserve-breaks 처리는 별도 분기로 남겼다. layout/runtime 기존 scene 행렬과 전체 opt-in runtime suite를 다시 통과했다. 통과.
4. **반복 공백 collapse** — PreWrap에서는 pending-space collapse 경로를 타지 않는다. Ahem fixture의 `XX  XX`는 max-content 60px, Normal 대조는 50px이며 Rust text/layout 및 양 플랫폼 scene에서 일치한다. 통과.
5. **앞·뒤 공백 trimming** — normalization에서 PreWrap 공백은 source 순서대로 직접 출력한다. Rust text test는 leading/trailing 각 2 spaces를 보존해 40px 결과를 확인한다. 통과.
6. **min-content에서 보존 공백의 잘못된 폭 기여** — Ahem `XX  XX` min-content는 20px, max-content는 60px이며 2줄 높이는 20px이다. text/layout tests와 Android/iOS frame/track 결과가 맞는다. 통과.
7. **soft-wrap 줄 끝 공백이 track을 늘리는 오류** — 25px available width의 첫 줄은 visible advance 20px와 hanging advance 10px를 분리하고, 두 번째 줄은 20px이며 layout width는 20px이다. layout test는 Columns contribution이 hanging space를 포함하지 않는지도 검사한다. 통과.
8. **강제 줄바꿈 손실** — LF는 PreWrap에서 normalized LF로 남고 두 줄을 만든다. layout test와 고정 runtime scene은 `[20]` column 및 `[20]` row를 확인한다. 통과.
9. **CRLF의 두 source code unit provenance 손실** — CRLF는 한 output LF range에 연결되며 두 개의 source UTF-16 range가 같은 paragraph range를 가리킨다. 같은 source와 CR/LF가 서로 다른 Text node에 걸친 입력도 단위 테스트한다. 통과.
10. **source range의 UTF-16/UTF-8 좌표 혼용** — source 쪽 범위는 각 Rust `char`의 `len_utf16()`로 만들고 normalized text 쪽은 UTF-8 byte offsets를 기록한다. CRLF 두 좌표계 경계 테스트가 확인됐으며 임의 비-BMP source는 이 bounded slice에서 별도 새 검증하지 않았다고 범위를 제한한다. 코드 경계 통과, 비-BMP 확장은 미검증.
11. **탭의 임의 profile 과대 지원** — tab이 포함되면 정확한 Ahem 10px LTR `XX\tXX` profile 외에는 `UnsupportedPreWrapTabProfile`로 닫는다. 다른 font, 위치, 뒤 추가 문자, inline ancestor 입력에 대한 실패 테스트가 있다. 통과.
12. **default tab stop의 잘못된 확장 폭** — 고정 profile은 두 `X` 뒤의 column 2에서 8-column tab stop까지 6 spaces로 확장해 100px max-content를 만든다. source tab은 paragraph bytes `2..8`로 연결된다. text unit test 통과.
13. **`tab-size`의 일반 지원으로 오인** — runtime style projection은 authored `tab-size`를 검사하지 않고 앱 행렬에도 tab이 없다. 계획과 evidence에 고정 profile 단위 검사만 적어 일반 authored `tab-size` 지원으로 표기하지 않았다. 통과.
14. **HTML comment 동작을 앱 fixture가 직접 검증했다고 오인** — 앱 fixture는 Comment node 대신 두 direct Text source를 사용한다. Chrome precomparison의 comment split과 앱 source-boundary 대응만 주장하고 이 차이를 명시했다. 통과.
15. **whitespace-only sequence가 익명 item/cell 생성** — block 앞 공백-only paragraph는 item을 만들지 않고 marker를 `(0,0)`에 배치한다. Columns `[20,20]`, Rows `[10]`, marker frame `[0,0,10,10]`를 layout/runtime에서 확인했다. 통과.
16. **fixture ID 확대 또는 중복 dispatch** — FFI source는 고정 16 ID allowlist를 먼저 확인하고 test가 ID 중복과 unknown ID의 `None`을 확인한다. 플랫폼의 16 ID 행렬도 같은 목록을 사용한다. 통과.
17. **runtime aspect-ratio adapter가 빈 비텍스트 Grid를 가로채는 회귀** — opt-in runtime 전체 suite에서 이 실패를 실제 발견했다. adapter가 explicit placement만 보고 계속 처리하던 조건을 수정해 의미 있는 text가 없는 경우 `None`을 반환한다. 빈 explicit item regression test를 추가했고 변경 후 runtime suite 327 passed, 0 failed, 3 ignored다. 수정 후 통과.
18. **플랫폼 harness의 다른 기대값 또는 장면 누락** — Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 각 16/16 scene을 실행했다. 신규 7 scene frame/track 결과, draw/present, zero superseded retries가 모두 확인되고 설치 앱 해시와 빌드 앱 해시가 같다. 통과.
19. **fixture payload의 기본 제품 archive 유입** — release static archive 제품/fixture positive-control 비교에서 제품은 marker 0/7 및 host export 0개, fixture는 7/7 및 export 2개다. 이는 macOS ARM64 static archive 경계로 한정하며 APK/IPA 전체를 증명하지 않는다. archive 범위 통과.
20. **부분 검증을 전체 C11 완료 또는 전체 테스트 통과로 승격** — 공식 `72/213`, C11/C11.4 미완료 상태를 유지한다. `cargo test --workspace`는 변경 crate 바깥 `spinon-style-to-layout` C07.3 pinned-Chromium 문자열 비교에서 기대 `1.77778 / 1`, 실제 `1.7777778 / 1`로 실패했다. 해당 결과를 전체 통과나 변경이 유발한 결함으로 단정하지 않고 evidence에 남겼다. 상태 경계 통과.

**판정:** 이 검토에서 발견된 runtime adapter 오진은 구현 중 수정했고 영향받는 opt-in runtime suite 및 최종 시뮬레이터 행렬을 다시 통과시켰다. 새로 확인한 blocker는 없다. 현재 source 후보 diff의 별도 30관점 검토도 완료했다. 남은 범위는 전체 workspace C07.3 불일치의 별도 원인 판정, 구현 PR 최종 head 대조, PR checks/병합 상태이며 이 후보를 완료로 판정하지 않는다.
