# C11.4 cross-axis fixed-repeat + auto-repeat + template areas 구현/runtime 검토 20개

## 범위와 결론

계획 문서 검토와 별도로, 현재 branch의 cascade·projection·FFI·Android/iOS adapter·scene/capture route·runtime/archive evidence를 서로 다른 20개 실패 관점에서 다시 확인했다. 아래 PASS는 해당 제한 fixture/profile에서 확인한 경로를 뜻한다. 일반 CSS admission, 전체 C11.4, WPT, Android 실기기/hardware GPU, iOS compositor 이후 출력은 이 검토의 판정 범위가 아니다.

검토는 읽기 전용으로 진행했다. 새 테스트, 빌드, simulator, 브라우저는 실행하지 않았고 저장된 최종 결과와 소스만 확인했다. 기록된 Rust/Node 검사와 최종 11-scene 실행 결과는 [runtime README](./README.md)에 연결되어 있다.

## 실패 관점

1. **두 cross-axis 방향이 서로 다르게 cascade 되는 경우 — PASS.** Columns fixed + rows auto-repeat와 columns auto-repeat + rows fixed 모두 같은 bounded 조건을 요구한다. `crates/spinon-style/src/stylo_dom/cascade/incremental/grid.rs:129–145`; 양 방향 track 기대값은 `crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4_template_area_auto_repeat.rs:91–157`에 있다.

2. **fixed repeat group 수를 boolean으로만 보존해 여러 group을 허용하는 경우 — PASS.** private 결과에 group count를 보존하고 cross-axis 조건에서 fixed 축은 정확히 1, 반대 auto 축은 0을 요구한다. `crates/spinon-style/src/stylo_dom/cascade/incremental/grid.rs:27–34, 132–145`.

3. **fixed pattern의 무제한 track 수·비정상 breadth — PASS.** repeat list 전체가 fixed group 하나여야 하며 pattern은 1–2개의 finite nonnegative computed `LengthPx`만 허용한다. `crates/spinon-style/src/stylo_dom/cascade/incremental/grid.rs:509–528`; unsupported breadth negative는 `crates/spinon-style/src/stylo_dom/cascade/runtime_grid_tests/fixed_repeat_auto_repeat.rs:545–719`에 있다.

4. **auto-repeat 축에 ordinary prefix/suffix, fixed group, 여러 inner track이 섞이는 경우 — PASS.** list가 auto-repeat group 하나이고 fixed group이 없으며 inner track도 하나여야 한다. `crates/spinon-style/src/stylo_dom/cascade/incremental/grid/auto_repeat.rs:42–88, 101–151, 258–271`.

5. **template areas 없이 cross-axis 조합을 허용하는 경우 — PASS.** cross-axis bounded 예외는 `template_areas.is_some()` 없이는 열리지 않는다. 거부 조건은 `crates/spinon-style/src/stylo_dom/cascade/incremental/grid.rs:132–153`에 있다.

6. **두 축 모두 auto-repeat인 입력에 fixed-repeat이 혼합되는 경우 — 수정됨.** 새 focused negative test가 row/column 두 방향을 각각 거부하고 root node와 반대 축 property를 확인한다. `crates/spinon-style/src/stylo_dom/cascade/runtime_grid_tests/fixed_repeat_auto_repeat.rs:1115–1155`; 저장된 focused 실행 결과는 1/1 통과다.

7. **다른 runtime profile 또는 일반 CSS 경로에 기능이 노출되는 경우 — PASS.** auto-repeat 및 intrinsic 관련 허용 값은 `RuntimeBlockFlexGridTrackSizingV1`에서만 cascade로 전달된다. `crates/spinon-style/src/stylo_dom/cascade/incremental/element.rs:191–202`; Android fixture route도 BuildConfig opt-in을 확인한다(`platforms/android/app/src/main/java/dev/spinon/bootstrap/C0410RuntimeGpuDemo.java:620–630`).

8. **fixed 축 area dimension이 expanded fixed track 수보다 작거나 큰 경우 — 수정됨.** projection은 auto-repeat 없는 축의 area dimension을 track 수와 비교한다. `crates/spinon-style-to-layout/src/projection/style_values/grid_values.rs:60–78`; 두 방향·상하 mismatch 테스트는 node와 `grid-template-areas` property를 검사한다(`crates/spinon-style-to-layout/src/c11_4_track_sizing_areas_tests.rs:477–509`). 이전 wildcard node assertion은 실제 root ID 비교로 바뀌었다.

9. **area-added track이 auto-repeat 반복 수를 잘못 늘리는 경우 — PASS.** 예상 track vector는 area expansion을 repeat tracks와 구분한다. 예를 들어 rows `[20,20,20,12,12]` 및 `[20,20,12]`을 고정하며 Rust registry가 track 및 item 수를 대조한다. `crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4_template_area_auto_repeat.rs:123–190`; 계약 경계는 `spec/internal/0059-c11-grid-layout.md:466`이다.

10. **기존 #545 양축 auto-repeat + areas positive가 새 cross-axis predicate에 걸리는 경우 — PASS.** 양축 auto-repeat scene과 TemplateV1 negative control이 유지되고, 새 predicate는 fixed repeat이 있을 때만 적용된다. `crates/spinon-style/src/stylo_dom/cascade/runtime_grid_tests/template_repeat.rs:491–520`, `crates/spinon-style/src/stylo_dom/cascade/incremental/grid.rs:129–165`.

11. **기존 #604 동일축 fixed+auto+areas와 shorthand 회귀 — PASS.** 동일축 track·line·area 검사가 보존되고 `grid-template` 및 `grid` shorthand가 longhand와 비교된다. `crates/spinon-style/src/stylo_dom/cascade/runtime_grid_tests/fixed_repeat_auto_repeat.rs:722–911`.

12. **track 수·line metadata budget 또는 산술 overflow — PASS, 경계 스트레스는 제한적.** cascade 한도는 10,000 computed tracks와 1 MiB line metadata이며 확장 산술에 checked operations를 사용한다. `crates/spinon-style/src/stylo_dom/cascade/incremental/grid.rs:24–25, 640–675`; auto-repeat budget은 `crates/spinon-style/src/stylo_dom/cascade/incremental/grid/auto_repeat.rs:135–180`에서 적용된다.

13. **반복 확장 또는 area projection reservation 실패로 부분 결과를 내는 경우 — PASS, Taffy 전역 OOM 복구는 보장하지 않음.** cascade의 vector/string 확장은 fallible reservation을 사용한다(`crates/spinon-style/src/stylo_dom/cascade/incremental/grid.rs:807–832`). area/name projection도 `try_reserve` 오류를 context가 있는 layout error로 전파한다(`crates/spinon-style-to-layout/src/projection/style_values/grid_values.rs:530–569`). 기존 failpoint/recovery 테스트는 `crates/spinon-style/src/stylo_dom/cascade/runtime_grid_tests/fixed_repeat_auto_repeat.rs:1068–1113`에 있다.

14. **area projection이 coordinate·rectangle·metadata 검증을 우회하거나 public 타입을 확장하는 경우 — PASS.** projection에서 `u16`/track 한도, area metadata, 유효 rectangle 및 reservation을 검사한다. `crates/spinon-style-to-layout/src/projection/style_values/grid_values.rs:478–583`; 지원 subset은 private cascade metadata로 전달되고 계약은 `spec/internal/0059-c11-grid-layout.md:466–468`에 제한되어 있다.

15. **fixture host C 선언이 기본 native build에 노출되는 경우 — 수정됨.** `crates/spinon-ffi/include/spinon_ffi.h` 선언은 C04 core와 fixture-test define을 모두 요구한다(`:455–461`). Cargo module/feature gate도 연결되어 있으며(`crates/spinon-ffi/Cargo.toml:26–38`, `crates/spinon-ffi/src/runtime_gpu/ffi.rs:11–12`), Node boundary test는 header 조건과 Android/iOS route gate를 고정한다(`tools/css-reference/fixture-module-boundary.test.mjs:568–596`). README는 네 define 조합의 header 전처리 대조를 기록한다(`README.md:32–35`).

16. **FFI scene ID·sample pointer/count 경계 — PASS.** C11.4 scene ID는 128 byte·UTF-8·registry lookup을 확인한다(`crates/spinon-ffi/src/runtime_gpu/ffi/fixtures/c11_4.rs:47–78`). Surface sample API는 곱셈 overflow, 1–64 count, null pointer, reservation을 검사한다(`crates/spinon-ffi/src/runtime_gpu/ffi/fixtures/c11_4.rs:97–138`).

17. **scene ID 중복·누락, 0/1/2-item schema 및 기대 track 배열 오류 — PASS.** Rust registry는 template-area 총 40개, cross-axis 11개, unique ID, item 수와 columns/rows 배열을 확인한다. `crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4_template_area_auto_repeat.rs:91–190`.

18. **전체 corpus 변경이 기존 41-scene 경로나 11-scene 선택을 오염시키는 경우 — PASS.** Node boundary는 template-area 29→40, corpus 473→484, 기존 41-scene ID 보존, 신규 11개만 `--scene-id`로 선택되는지 확인한다. `tools/css-reference/fixture-module-boundary.test.mjs:283–345`; capture parser는 인자를 comma-separated ID로 분리한다(`tools/css-reference/capture-c11-4-auto-repeat-neighbors-simulator.mjs:12–13`). 기록된 Node 결과는 12/12다.

19. **Android/iOS adapter의 frame·track·trace·sample mismatch 또는 collapsed gutter 오판 — 수정됨.** 두 adapter는 명시적인 11-scene 기대값과 두 축 trace를 검사한다. Empty-area의 collapsed track 주변 gutter는 0이며, 가운데 한 track만 접히는 scene은 shared gap을 앞 4px·track 0px·뒤 0px로 확인한다(Android `platforms/android/app/src/main/java/dev/spinon/bootstrap/C0410RuntimeGpuDemo.java:3825–3948`, iOS `platforms/ios/Sources/C0410RuntimeGpuDemo.swift:2428–2454`). 최신 AVD와 Simulator run은 각 11/11 draw/present 및 35/35 exact sample을 기록한다. `README.md:9–16`; Android는 SwiftShader이고 iOS 표본은 compositor 제출 전이다.

20. **fixture payload가 기본 artifact에 새어 나오거나 후보 범위가 공식 완료로 과장되는 경우 — PASS, 최종 번들 검증은 별도 한계.** archive verifier가 11 scene marker 전부, 대표 payload, 두 host symbol을 확인한다(`spec/internal/evidence/c11-4-cross-axis-fixed-auto-repeat-areas-runtime-2026-10-25/staticlib-archive-boundary-20261025T0319KST/verify-boundary.py:10–112`). 저장 결과는 기본 archive 0/11·symbol 부재, opt-in positive control 11/11·payload/symbol 존재다. 이는 Rust static archive 경계만 증명한다. `spec/STATUS.md:362`, `spec/internal/0059-c11-grid-layout.md:470–472`, `spec/0001-conformance.md:74`는 제한 candidate와 C11.4 미완료·공식 72/213을 유지한다. 공개 roadmap과 PR/merge 기록은 PR 단계에서 갱신할 예정이며 이 후보를 현재 공개 CSS 지원으로 취급하지 않는다.

## 이전 finding 재평가

- C header native feature guard 누락: **수정됨** — 두 native define을 요구하며 header 경계 검사와 preprocessor 대조가 기록됐다.
- 두 auto 축에 fixed repeat 혼입을 거부하는 직접 회귀 부재: **수정됨** — 새 양방향 focused negative test가 exact node/property를 확인한다.
- fixed-axis area mismatch에서 node identity를 확인하지 않음: **수정됨** — projection test가 실제 root node를 비교한다.
- 가운데 collapsed track 앞뒤 gutter를 모두 0으로 요구: **기준 정정** — CSS Grid shared-gap과 trace에 맞춰 앞 4px·뒤 0px를 검사한다.
- runtime source hash 불일치: **해소됨** — capture 시작 hash `a333…`, Android의 사후 test-only hash `8813…`, iOS의 시작-only metadata와 각 app hash를 README가 구분한다. projection test assertion 외 product/native/app input은 capture 후 변경되지 않았다.

남은 명시적 제한은 최종 APK/IPA 전체 fixture exclusion, Android hardware GPU, 실기기, iOS compositor 이후 pixels, 일반 CSS admission, WPT 및 전체 C11.4다. 계획 검토의 항목은 이 구현/runtime 검토에 재사용하지 않았다.
