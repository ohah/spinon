# C11.4 `fit-content(40px)` + `auto` runtime PR diff 적대적 검토 · 30개 관점

검토일: 2026-10-27
대상: `/tmp/spinon-tailnet-c11-4-fit-content-sync-20261026`의 최종 staged diff
기준 HEAD: `5ef1234b8e1884434540b55df7de760e5eca88a3`
최종 실행: Android API 37 ARM64 AVD pass6, iPhone 17 Pro/iOS 26.2 Simulator pass6
방법: staged 소스·문서·manifest·로그·화면을 대조하고 SHA-256 및 `git diff --cached --check`를 확인했다. 새 빌드·테스트·브라우저 실행은 하지 않았다.

## 판정

**PASS 29 / PARTIAL 1 / FAIL 0.** Swift·Rust·Android matcher는 stretch 단계 중복을 거부하고, 유일한 `applied:true` stretch가 마지막인 trace만 허용한다. pass6 실행 ID, 결과, 현재 tracked source snapshot, 실행 앱/설치 APK 및 screenshot 해시가 각각 연결된다. precomparison `SHA256SUMS` 전 항목도 통과했다. 알려진 PARTIAL은 mutation별 변형 앱 입력을 Android/iOS launch와 draw까지 연결한 전체 negative matrix가 없는 범위 한계다.

pass6 `trackedDiffSha256`는 `e44880c589101dbf5bf595698bc87c351acd21750b8b2dc4f60a65e30cad6dfb`, 재계산한 `sourceSnapshotSha256`는 `524c99fbf2fc227a5d0f8c6e653a66cefc11f5e2fcef325715e4b2b7dce350c0`이며 manifest 값과 일치한다. pass5에서 pass6으로 추가된 tracked path는 precomparison의 `SHA256SUMS` 하나로, 실행 코드 path는 동일하다. 따라서 README가 지정한 pass5 build와 pass6 실행의 결합은 코드 변경 없는 재사용으로 설명된다.

## 적대적 검토 30개

| # | 실패 경로·불변식 | 결과와 근거 |
|---:|---|---|
| 1 | scene ID 뒤에 인자나 JS를 붙여 허용 장면을 우회한다. | **PASS** — `crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/fit_content_auto_neighbor.rs`는 ID 완전 일치만 source를 반환한다. FFI test는 `fit-content-auto-neighbor-wpt;throw 1`와 미등록 ID를 거부한다. |
| 2 | scene 생성 중 item/child가 빠지거나 순서가 바뀐다. | **PASS** — 고정 JS는 div 5개, root의 direct item 2개, item별 child 1개를 만든다. `prepare`는 child 수·순서·소유 관계를 검사한다 (`.../fit_content_auto_neighbor.rs:1-59`, runtime fixture `:30-87`). |
| 3 | 다른 element/tag/namespace가 같은 node ID를 재사용한다. | **PASS** — adapter가 root와 각 item/child의 HTML namespace, `div` tag, exact ID 및 root-parent 관계를 검사한다 (`crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_fit_content_auto_neighbor.rs:20-103`). |
| 4 | fixture가 의도한 Grid profile 밖의 계산 경로로 admission된다. | **PASS** — `RuntimeBlockFlexGridTrackSizingV1` 외 profile은 `prepare`에서 거부된다. 일반 CSS admission을 켜는 경로는 없다. |
| 5 | root dimensions, gap, border, alignment 또는 auto-flow가 바뀌어도 positive slice로 처리된다. | **PASS** — root computed style 검사는 `60×300`, `align-content:normal`, row flow, zero spacing/border, explicit column/row 및 repeat/area 부재를 allowlist와 대조한다 (`.../c11_4_fit_content_auto_neighbor.rs:262-323`). |
| 6 | grid item 또는 nested child의 크기·margin·border·placement 변형이 예상 frame을 우연히 만든다. | **PASS** — item auto-size와 auto placement, child `60×10`, zero spacing/border를 각각 확인하고 최종 frame도 별도 검사한다 (`.../c11_4_fit_content_auto_neighbor.rs:325-393`). |
| 7 | CSS computed `grid-auto-rows`가 `[fit-content(40px), auto]`가 아니거나 순서가 바뀐다. | **PASS** — computed property와 typed style을 함께 검사해 `FitContent(LengthPx(40))` 다음 `Auto`를 고정한다. FFI scene test도 declaration을 확인한다. |
| 8 | report에 적은 typed Grid input이 Taffy 직전 실제 입력과 다르다. | **PASS** — `StyleLayoutOutput`에서 `LayoutInput::grid_containers()`를 복사하고, runtime observation은 실제 `grid_container_inputs`를 받아 root identity와 `TrackSizing` input을 검증한다 (`crates/spinon-style-to-layout/src/projection.rs`, `crates/spinon-runtime/src/session/ua_cascade/runtime_layout.rs`). |
| 9 | diagnostics를 cache key에 재결합하며 computed-style snapshot revision을 새 revision으로 덮는다. | **PASS** — `with_revision`은 snapshot을 복사 보존한다. `rekeying_grid_diagnostics_preserves_computed_style_provenance`가 다른 key로 재결합한 뒤 원본 snapshot을 비교한다 (`focused-revision-provenance-test.log`, 1/1). |
| 10 | generation/document/render-tree/style/environment 중 하나가 layout output과 달라도 검증을 통과한다. | **PASS** — `revisions_match`는 HostDocument source와 모든 key revision을 비교하고, stale-style 및 document key 변형을 단위 테스트에서 거부한다 (`.../c11_4_fit_content_auto_neighbor.rs:352-401`, `focused-runtime-tests.log`). |
| 11 | 누락되거나 중복된 fixture node frame을 FFI report가 완성된 결과로 내보낸다. | **PASS** — runtime은 대상 5 NodeId의 중복·누락 frame을 오류로 거부하고, JSON report도 다섯 frame map의 완전성을 확인한다 (`.../c11_4_fit_content_auto_neighbor.rs:132-173`, FFI report `fit_content_auto_neighbor_report.rs:20-45`). |
| 12 | frame 순서·node mapping 또는 CSS px 위치가 달라도 총 크기만 맞아 통과한다. | **PASS** — 각 node tuple을 CSS px에서 exact 비교한다: root `[0,0,60,300]`, fit item/child `[0,0,60,10]`, auto item `[0,10,60,290]`, auto child `[0,10,60,10]`. Android/iOS pass6 `result.json`도 같은 순서·값을 기록한다. |
| 13 | 다른 Grid trace가 대상 style/frame과 연결된다. | **PASS** — trace가 정확히 하나이고 grid NodeId가 관측 root와 같아야 한다 (`.../c11_4_fit_content_auto_neighbor.rs:176-205`). |
| 14 | Rows/Columns pass가 빠지거나 중복되고 axis 순서가 달라도 summary만 맞는다. | **PASS** — trace는 Columns 1개, Rows 1개 및 2-pass 순서를 요구한다. 양 플랫폼 app matcher와 pass6 trace JSON도 같은 축을 기록한다. |
| 15 | required row sizing stage 누락·중복·재정렬을 positive trace로 허용한다. | **PASS** — IntrinsicSizingComplete, MaximizeTracks, ExpandFlexibleTracks, StretchAutoTracks가 각 한 번씩 지정 순서여야 한다 (Rust `.../c11_4_fit_content_auto_neighbor.rs:420-473`, Android Java `:984-1019`, iOS Swift `:2777-2817`). |
| 16 | `applied:false` stretch 또는 중복 stretch가 마지막 true 단계만 남겨 matcher를 우회한다. | **PASS** — Rust는 enum의 모든 `StretchAutoTracks { .. }`, Android는 모든 `StretchAutoTracks` prefix, iOS는 모든 같은 prefix를 세어 정확히 하나만 허용한다. 앞선 false와 뒤의 true를 함께 둔 변형은 양 앱 matcher에서 거부된다. |
| 17 | 마지막 stretch가 `applied:false`이거나 그 뒤 다른 stage가 와도 통과한다. | **PASS** — 세 matcher가 exact `applied:true`와 최종 stage index를 요구한다. Rust `rejects_row_stages_after_auto_track_stretch`는 정상 trace 뒤 `FixedTrackEarlyReturn`을 붙인 변형을 실패시킨다 (`focused-runtime-tests.log`, 4/4). |
| 18 | 최종 `[10,290]`만 맞고 앞 단계가 이미 늘었거나 fit-content row가 늘어난다. | **PASS** — 세 matcher가 앞 단계 `[10,10]`, 최종 `[10,290]`을 확인한다. pass6 Android/iOS 결과의 tracks도 `[10,290]`이다. |
| 19 | gutter/collapsed sentinel이 실제 CSS row 수에 들어가 비교를 밀어낸다. | **PASS** — Rust는 gutter/collapsed track을 제외하고, Android/iOS는 gutter를 건너뛰며 collapsed non-gutter를 거부한 뒤 실제 base 개수를 검사한다. |
| 20 | 앱 negative matrix가 각 adapter mutation을 두 플랫폼의 launch/draw에서 모두 거부한다고 과장된다. | **PARTIAL** — README는 mutation-by-mutation app matrix를 검증하지 않았다고 명시한다. Rust focused tests는 typed row 값·순서, extra explicit row, child count, stale revision, late stage 변형을 확인하지만 각 변형을 Android/iOS 앱 draw까지 연결하지 않았다. 이 제한은 통과 실행을 무효화하지 않는다. |
| 21 | report serializer가 다른 C11.4 scene의 진단을 덮거나 잘못된 scene을 null positive로 만든다. | **PASS** — fixture observation이 있을 때만 `fitContentAutoNeighbor` JSON object를 붙이고, 없는 경우 `null`을 낸다. scene-specific app matcher는 exact scene ID에서만 엄격 검사를 수행하며 나머지 diagnostics 필드는 유지한다 (`trace_report.rs`, FFI report file). |
| 22 | fixture module/scene ID/FFI export가 기본 또는 core product archive로 유출된다. | **PASS** — macOS ARM64 기본 및 `c04-runtime-gpu` `.rlib`/`.a` scan은 marker·prefix·export가 absent, fixture-test positive control은 present다 (`artifact-boundary/*-symbol-string-scan.txt`). 이 결과를 APK/IPA 전체 제외로 확대하지 않는다는 범위도 README에 있다. |
| 23 | Android verifier가 report field 부족·scene 오류·revision 불일치를 무시한다. | **PASS** — JSON matcher는 profile/style/input, 5 NodeId, HostDocument source, revisions, frame map, trace axes/stages를 검증하고 parse/type 오류에는 false를 반환한다 (`C0410RuntimeGpuDemo.java:809-925`). |
| 24 | iOS verifier가 Android와 다른 style/input/frame 또는 revision을 통과시킨다. | **PASS** — Swift matcher는 profile, computed properties, typed Grid string/input, unique node IDs, revisions, 다섯 frames와 trace를 guard로 요구한다 (`C0410RuntimeGpuDemo.swift:2672-2755`). |
| 25 | 긴 trace log가 일부 chunk만 있어도 parse/summary가 성공한다. | **PASS** — capture verifier는 run ID별 trace chunk 집합의 완성을 확인한 뒤 JSON을 파싱한다 (`capture-c11-4-auto-repeat-neighbors-simulator.mjs:6176-6197`). pass6 Android는 5/5, iOS는 10/10 chunks를 `result.json`이 기록한다. |
| 26 | Surface 환경 재계산이 다른 source identity인데 통과한다. | **PASS** — pass6 Android log는 generation 2/sequence 4, iOS는 generation 1/sequence 6에서 각각 strict match와 identity `1/25/3/0/1`을 기록한다. |
| 27 | draw가 검증된 Surface generation/sequence와 다른 revision인데 성공 보고된다. | **PASS** — pass6 양 플랫폼 `PRESENTATION_MATCH`는 `matched=true`이며 expected/actual identity가 `1/25/3/0/1`로 일치한다. result JSON은 `drawPresented:true`, `supersededRetries:0`이다. |
| 28 | Android PixelCopy가 stale·density 변환 오류 또는 root 밖 영역 색을 잘못 확인한다. | **PASS** — pass6 AVD log는 5개 CSS 좌표를 디바이스 좌표/RGBA와 함께 기록하고 5/5 exact다. API 37 ARM64 QEMU AVD라는 제한을 유지하며 hardware GPU나 실기기 근거로 해석하지 않는다. |
| 29 | iOS 표본이 track fill이 아니라 오래된 drawable이나 compositor 이후 출력으로 잘못 주장된다. | **PASS** — pass6 Metal drawable log의 child 2개, auto-row fill 2개, root 우측 바깥 표본 1개가 5/5 exact다. README와 internal contract는 readback이 compositor 제출 전임을 명시한다. |
| 30 | pass6 evidence가 이전 실행·다른 source/app/image와 섞이거나 stale 자료가 최종 화면으로 제시된다. | **PASS** — manifest의 run ID/result 경로는 pass6 `20261026T190540661Z-27b72972-0` 및 `20261026T190530681Z-85dd0569-0`과 일치한다. tracked diff와 source snapshot은 재계산 값과 같고, Android local/installed APK SHA는 `9b37c7480fdb7c590c0e242434f3bf627a24aaa065e5cd1312feeeeee107a7ff`, iOS app executable SHA는 `10657b3358b29ce8e69802f3fb12e4d5881be9960bdad1fe9da4dd12617bec20`이다. screenshot SHA `35b3d269a54e01f7019215f74c4aa6ccbec6f62c6afaf83527c30e1366f5ad7c`/`d4d94e106634755cabd3dd204de3e241ec47a336f96127cf15ef36dee19fb51f`가 실제 PNG와 일치한다. `SHA256SUMS`는 8/8 통과한다. stage된 run directory는 pass6 Android/iOS 각 7개 파일뿐이며 이전 pass5 capture folders는 untracked다. 재사용한 pass5 build log는 README에 명시됐고 pass5→pass6 `trackedDirtyPaths` 차이는 checksum 문서 1개뿐이다. |

## 결론과 한계

- staged diff에서 병합을 막을 수정 결함을 찾지 못했다. C11.4는 고정 scene의 opt-in runtime slice이며 전체 CSS admission이나 Spinon WPT pass가 아니다.
- Android는 PixelCopy AVD evidence이고 iOS는 compositor 이전 Metal drawable evidence다. 실기기/hardware GPU, iOS compositor 이후 pixel, release APK/IPA fixture exclusion은 주장하지 않는다.
- mutation별 Android/iOS negative app matrix가 없는 점은 #20 PARTIAL로 유지한다. 공식 C11.4 checkbox와 완료 `72/213`도 미완료로 유지한다.
- 현재 검토는 staged diff의 소스·저장 evidence 대조다. 테스트·빌드·브라우저를 새로 실행하지 않았다.
