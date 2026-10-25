# C11.4 orthogonal row intrinsic runtime PR 변경 검토 30개

검토 기준: staged index tree 7d3caa2f6587107b568147677f7fabafcb57f7b2 (2026-10-25). 판정: 제품 blocker 없음, P2 미래 방어성 관찰 1건. 이전에 확인한 두 Rustdoc/provenance 문구 문제는 최신 index에서 해결됐다.

## 범위와 근거 경계

- 502개 staged 파일: crates 87, Android/iOS source 2, 이 slice evidence 409, plan 1, STATUS/0059/public roadmap 3. 총 +68,408/-76. unstaged tracked 변경 없음.
- FFI scene/JSON, Runtime 준비·검증·테스트, Style→Layout, Stylo cascade/preflight, text intrinsic 계산, logical→physical conversion, Taffy adapter/vendor 의미, Android/iOS matcher를 검토했다. C15.3 변경은 ParagraphStyle 및 fallible contribution API migration 영향으로만 다뤘다.
- Simulator 최종 run은 test-time tree 45bb3313f5c126fd724eb54e877f86746107a715에서 수행됐다. 현재 tree까지의 후속 변경은 Rustdoc과 evidence timing 설명이며 executable code, fixture, input, expected output은 바뀌지 않았다. 검토자는 테스트/build/simulator를 재실행하지 않았다.
- 최종 matrix 앞 Android harness의 readonly status 변수와 trace parser, iOS PID 정규식 문제가 있었으나 제품 assertion failure는 아니었다. 수정 harness로 Android/iOS 각 12/12가 기록됐다.
- 이미지/text/glyph draw telemetry가 0이므로 screenshots는 geometry/surface 제출 근거이며 실제 glyph/image pixel painting 근거가 아니다. Android는 software AVD, iOS는 compositor 이후가 아니다.
- git diff --cached --check는 raw captures 34개에서 175개 공백 진단을 냈다. 로그 및 simulator device JSON 원본 공백이며, *.log와 simulator-devices-at-run.json을 제외하면 clean이다. source/prose 진단은 없다. raw captures는 보존했다.
- Rustdoc 추가 뒤 cargo fmt --all -- --check 통과는 부모 에이전트가 실행한 결과로만 기록한다.

## 30개 관점

| # | 등급 | 실패 관점 | 근거와 판정 |
|---:|:---:|---|---|
| 1 | 통과 | scene admission 범위 | orthogonal_rows/scene.rs:29–50,124–129는 min/max × forward/reversed 네 ID만 선택한다. |
| 2 | 통과 | unknown orthogonal ID 오수용 | prepare.rs:31–36은 unknown orthogonal grid를 거부하고 무관한 root는 기존 경로로 넘긴다. |
| 3 | 통과 | root owner/profile/tag 오염 | prepare.rs:38–53은 profile, div, root parent, data-node, direct child 수를 확인한다. |
| 4 | 통과 | child source order/node identity 오염 | prepare.rs:59–71은 순서별 ID, parent, data-node와 item style을 대조한다. |
| 5 | 통과 | text leaf/text 값 및 image decode 경로 | prepare.rs:73–120은 단일 Ahem text leaf, image leaf/source/decode를 확인한다. |
| 6 | 통과 | decoded natural image metadata 변경 | prepare.rs:122–146은 PNG metadata가 24×16과 다르면 실패한다. |
| 7 | 통과 | root Style→Taffy 외형/track 범위 확대 | orthogonal_rows/style.rs:13–78은 74px root, fixed columns, 단일 row, gaps/edges/repeat/areas 등 범위를 고정한다. |
| 8 | 통과 | text CSS/writing/typography/placement mismatch | orthogonal_rows/style.rs:81–149는 vertical-rl/mixed, Ahem, typography, placement, auto dimensions를 검증한다. |
| 9 | 통과 | image inherited mode와 replaced sizing | orthogonal_rows/style.rs:151–167 및 prepare.rs:91–143는 inherited horizontal-tb/mixed와 natural image를 확인한다. |
| 10 | 통과 | author stylesheet의 text-orientation scanner bypass | runtime_grid.rs:273–295는 declaration 발견 및 incomplete scan을 cascade 전에 거부한다. |
| 11 | 통과 | inline unsupported declaration 또는 incomplete parse | runtime_grid.rs:299–315,378–409는 inline allowlist와 전용 scanner에서 오류를 fail-closed 처리한다. |
| 12 | 통과 | global Stylo preference leak/race | device.rs:51–73,94–108은 lock, 이전 값 보존, RAII restore를 적용하고 runtime_grid.rs:272–317가 제한 scope를 둔다. |
| 13 | 통과 | unsupported computed writing mode/orientation 허용 | incremental/compute.rs:623–652는 제한 enum 외 값을 명시적 오류로 반환한다. TextOrientation은 Mixed만 있다(writing.rs:13–21). |
| 14 | 통과 | stale incremental writing-style table 사용 | incremental/compute.rs:60–80은 track-sizing reuse를 거부하고 full cascade fallback을 요구한다. |
| 15 | P2 | style-only guard에서 orientation tuple 미검사 | projection.rs:554–575는 writing mode만 검사한다. 현재 computed conversion은 Mixed 외를 거부하고 enum도 Mixed only라 현행 우회는 없다; 미래 enum 확장 시 orientation 검사 추가 권고. |
| 16 | 통과 | computed vertical nodes와 paragraph profile 불일치 | projection.rs:578–637은 computed/expected vertical-node set을 대조하고 제한된 단일 text profile을 확인한다. |
| 17 | 통과 | public intrinsic method가 일반 vertical text를 광고 | engine/intrinsic.rs:27–53 및 242–260은 고정 Ahem profile만 허용한다고 Rustdoc과 predicate에 명시한다. 이전 문구 관찰은 최신 index에서 해결됐다. |
| 18 | 통과 | logical sizes→physical width/height 축 오류 | intrinsic_measurement.rs:226–256은 vertical child block→width, inline→height로 변환한다. Runtime test assertions.rs:106–127은 수치를 고정한다. |
| 19 | 통과 | 잘못된 mode, NaN 또는 음수 logical size fallback | intrinsic_measurement.rs:226–250은 unsupported mode와 invalid size를 LayoutError로 거부한다. |
| 20 | 통과 | OrthogonalText에서 registry-token identity를 잃는 provenance | intrinsic_measurement.rs:89–96은 Text의 identity와 OrthogonalText의 numeric revision 차이를 명시한다. 한 CalcLayoutTree engine 사용 및 현재 숫자 소비 경로상 의미 영향 없음; 이전 문구 관찰은 최신 index에서 해결됐다. |
| 21 | 통과 | min/max callback에서 available-space 축 선택 오류 | calc_tree/text.rs:150–221은 definite width와 height MinContent/MaxContent를 검사하고 필요한 metadata/값 누락을 오류 처리한다. |
| 22 | 통과 | callback observation/source/revision/allocation failure 누락 | calc_tree.rs:719–760, intrinsic_measurements.rs:150–213은 measurements/revision을 기록하며 예약 오류를 보존한다. |
| 23 | 통과 | normal Text measurement가 Orthogonal 요구를 막음 | runtime test assertions.rs:140–184와 Runtime validate.rs:210–245는 Orthogonal record 최소 1개와 해당 record 전체 일치를 요구하고 normal Text 기록은 허용한다. |
| 24 | 통과 | root/item frame, output revision, reverse source order 혼선 | Runtime validate.rs:31–107,109–208 및 test assertions.rs:50–88은 revision, frame, node, source order와 logical provenance를 검증한다. |
| 25 | 통과 | Normal alignment를 Stretch로 사전 변환 | taffy_style.rs:395–410은 Normal/Stretch를 분리하고 taffy_style/tests.rs:7–17가 adapter mapping을 확인한다. |
| 26 | 통과 | Taffy Grid/Flex의 Normal 및 replaced image 의미 | vendored grid_item.rs:357–365는 non-replaced만 stretch, flexbox.rs:647–655는 Flex Normal→Stretch를 구현한다. 이 profile의 text/image 50×row 및 24×16 기대와 일치한다. |
| 27 | 통과 | trace axis/stage/track shape 불일치 | Runtime validate.rs:250–291,353–381은 단일 root trace, Columns-before-Rows, stages 및 [50,24]/[50 또는 80] tracks를 요구한다. |
| 28 | 통과 | row contribution 값/kind/span/flexible/query 중복 오류 | Runtime validate.rs:292–349는 text 50/80, image 16, span 1, non-flex를 전부 검증한다. 선택 kind는 양 item에서 필요하고 유효 duplicate/다른 min-max contribution은 허용한다. |
| 29 | 통과 | API migration, FFI JSON 및 feature/artifact 경계 | model.rs:126–141과 intrinsic_measurement.rs:216–256의 변경 caller가 Result를 처리한다. trace_report.rs:63–120,121–175,211–234 source를 직렬화한다. feature gates Cargo.toml 및 artifact-boundary/README.md:1–12는 default 0/4·0/2, positive 4/4·2/2이며 앱 전체 제외로 확대하지 않는다. |
| 30 | 통과 | Android/iOS native matcher와 evidence/공식상태 과장 | Android Java:4670–4979와 iOS Swift:413–417,1682–1758,3102–3198는 scene/source/frame/trace를 검사한다. 최종 README는 test-time tree/hash, 12/12, draw 한계를 연결한다. 0059:498–504, plan, STATUS:276, roadmap:180은 제한 slice와 72/213 미완료 상태를 유지한다. |

## 판정

- P0/P1 및 merge blocker는 발견하지 못했다.
- 유일한 남은 finding은 row 15의 P2 미래 방어성 문제다. 현재 CSS/Stylo/type 경계가 Mixed 외 orientation을 fail-closed로 막아 현재 bypass는 아니다.
- simulator 및 Rust/Clippy/archive 결과는 검토자 실행 결과가 아니다. staged final evidence를 참조했고, simulator 결과는 test-time tree 45bb3313…에 귀속된다. 현재 tree 7d3caa2…의 후속 변경은 Rustdoc/evidence timing 설명이다.

표의 상대 file stem은 다음 경로 기준이다. Runtime fixture files: crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_mixed/orthogonal_rows/; Runtime tests: crates/spinon-runtime/src/session/ua_cascade/runtime_layout_tests/c11_grid_tests/orthogonal_rows/; Style cascade: crates/spinon-style/src/stylo_dom/cascade/; Style→Layout: crates/spinon-style-to-layout/src/; layout: crates/spinon-layout/src/; text layout: crates/spinon-text-layout/src/; FFI trace: crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/trace_report.rs. Android adapter: platforms/android/app/src/main/java/dev/spinon/bootstrap/C0410RuntimeGpuDemo.java. iOS adapter: platforms/ios/Sources/C0410RuntimeGpuDemo.swift. Evidence files are under spec/internal/evidence/c11-4-orthogonal-row-intrinsic-runtime-2026-10-25/.
