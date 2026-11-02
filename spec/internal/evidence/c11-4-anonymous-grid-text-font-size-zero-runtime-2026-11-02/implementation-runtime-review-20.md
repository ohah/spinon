# C11.4 anonymous Grid text `font-size: 0` implementation/runtime review 20

대상은 이 실행 evidence 디렉터리의 구현 candidate, 두 native harness, simulator capture runner, raw trace와 화면이다. 계획 리뷰 20개는 재사용하지 않았다. 확인 범위는 고정 Ahem fixture이며 일반 CSS admission 또는 전체 C11.4를 판정하지 않는다.

1. **Scene admission 경계:** positive font-size scene 10개와 zero-size scene 4개가 별도 목록이고, unknown zero-size id가 `None`으로 거부되는지 확인했다. FFI generator test는 네 ID와 unknown id를 각각 확인한다. **통과.**
2. **CSS serialization:** zero-size allowlist가 정확히 `0px`로 직렬화되는지, `-0.0`도 canonical `0px`인지 확인했다. **통과.**
3. **Negative CSS size:** computed `-1px`와 `-1.0` typed size가 zero scene 검증을 통과하지 못하는지 확인했다. **통과.**
4. **Non-finite typed size:** NaN과 양·음 infinity가 finite 검사 전에 또는 intrinsic layout 중에 노출되지 않고 fixture validation에서 거부되는지 확인했다. **통과.**
5. **Wrong computed value:** scene id만으로 0을 승인하지 않고 property string과 typed value가 모두 기대값 `0px`/`0.0`인지 묶는지 확인했다. **통과.**
6. **Positive regression:** 기존 5px, 7.5px, 10px, 12.5px, 20px scene이 계속 자기 기대값과 일치해야 하며, 0이 positive scene에 들어가지 않는지 확인했다. 구현 validator와 기존 positive suite는 별도 유지된다. **통과.**
7. **Paragraph validation scope:** `ParagraphStyle`은 finite zero를 수용하지만 negative/non-finite를 계속 거부하는지 확인했다. **통과.**
8. **Inline-run boundary:** `InlineRunStyle` validation과 C15 raster path는 변경되지 않아 zero font size가 이 C11.4 profile 밖으로 전파되지 않는지 diff로 확인했다. **통과.**
9. **Text source identity:** V8 fixture가 root direct Text node를 만들고 고정 `XX XX`, `XX`, 또는 collapsible spaces를 사용하는지 확인했다. HostDocument source를 재작성하지 않는다. **통과.**
10. **Computed typography:** Android/iOS summary trace에서 Ahem, font-size 0, explicit line-height 10, weight 400, normal whitespace/wrap, LTR style과 HostDocument revision이 확인됐다. **통과.**
11. **Zero intrinsic contribution:** min/max-content 두 장면에서 inline min/max contribution 모두 0이며 min/max block contribution이 모두 10인지 플랫폼 JSON trace에서 검사했다. **통과.**
12. **Zero text line geometry:** Ahem zero-size paragraph가 한 줄, advance 0, line-height 10과 finite baseline/top을 반환하는지 양 플랫폼 capture runner가 검사한다. **통과.**
13. **Grid item frame와 text layout 구분:** text-before-box에서 stretched item frame은 5×10이지만 paragraph layout width/advance는 0이고 available width는 5인지 검사한다. 초기 검증기는 두 폭을 하나로 취급했다. 실제 trace를 분리해 validator를 고쳤다. **발견 후 수정, 재실행 통과.**
14. **Source-order placement:** zero text item이 첫 row를 차지해 fixed 5×3 block이 `(0,10)`으로 이동하고 rows가 `[10,3]`인지 네이티브 frame checker와 trace validator가 확인한다. **통과.**
15. **Whitespace-only control:** 공백만 있는 source가 item frame, intrinsic input, paragraph layout을 만들지 않고 no-column/zero-auto-row 상태에 머무는지 검사한다. **통과.**
16. **Empty explicit-column line groups:** `grid-template-columns:none`에서 runtime typed snapshot이 empty columns와 empty line-name groups를 보존한다. 기존 validator는 항상 `columns.len()+1` line-name groups를 요구했다. 실제 런타임 계약에 맞춰 empty-list case를 분리하고 unit test를 추가했다. **발견 후 수정, 재실행 통과.**
17. **Runtime frame-set accounting:** `runtime_frames`가 element와 anonymous text item을 모두 frame 목록에 넣은 뒤 anonymous count를 다시 더해 `frame_set_mismatch`를 내던 경로를 AVD에서 재현했다. 비교를 실제 collected frame 수와 output map 수로 바꿨다. **발견 후 수정, Android/iOS 전 장면 통과.**
18. **Native expected frame order:** Android/iOS checker의 zero text-before-box 예상값에 anonymous text item frame을 추가하고, runtime의 element-first 출력 순서(root, block, anonymous text)에 맞췄다. 초기에 Android checker가 익명 item frame을 빠뜨려 실패했으며 수정 후 통과했다. **발견 후 수정.**
19. **Capture provenance:** Android API 37 ARM64 AVD의 QEMU/API/ABI와 APK SHA, iPhone 17 Pro/iOS 26.2 UDID/model/runtime 및 executable SHA를 기록하고 설치된 binary와 build output hash가 같은지 확인했다. **통과.**
20. **Process and evidence lifecycle:** 각 scene마다 고유 run ID와 complete trace chunk 수를 검사하고 raw logs/matrix/PNG를 저장했다. Android app은 force-stop, iOS app은 terminate됐고 Agent Browser/terminal-browser process는 실행되지 않았다. Simulator screen capture는 render smoke이며 surface readback/glyph pixel proof가 아니다. **통과, 한계 기록.**

## 추가 artifact 경계 검토

default `aarch64-apple-darwin` release `spinon-ffi` archive의 `strings` scan에는 네 `font-size-0-*` scene ID가 없고, `c04-runtime-gpu-fixture-test` positive-control archive에는 네 ID 모두 있다. Archive SHA-256과 scan 범위는 [runtime evidence](./README.md)에 기록했다. 이는 static archive 문자열 경계이며 APK/IPA 전체 제외 또는 linker stripping을 주장하지 않는다.

## 실행 요약

- Android API 37 ARM64 AVD: 4/4 scene pass.
- iPhone 17 Pro / iOS 26.2 Simulator: 4/4 scene pass.
- `cargo test --locked -p spinon-text-layout --lib c11_4_font_size_zero -- --nocapture`: 2/2 pass.
- `cargo test --locked -p spinon-runtime --features c11-4-grid-track-sizing-fixture-test --lib zero_font_size -- --nocapture`: 2/2 pass.
- `cargo test --locked -p spinon-ffi --features c04-runtime-gpu-fixture-test --lib zero_font_size_matrix -- --nocapture`: 1/1 pass.
- `cargo fmt --all -- --check`와 `git diff --check` pass.
- Full `spinon-text-layout --lib`: 99/99 pass. Fixture-enabled `spinon-ffi --lib`: 189 pass, 2 ignored.
- Default-vs-fixture-test static archive scene-ID boundary: 0/4 vs 4/4.
- Full feature-enabled runtime library suite: 425 passed, 4 failed, 3 ignored. The four failures are unrelated Root HTML grid/rejection and anonymous-text tests; a focused anonymous-text rerun with the previous frame-count expression restored reproduced its `root_html_default_render_owner_frame_missing` failure. Default-feature runtime suite: 219 passed, 2 failed, 3 ignored. These broader failures are disclosed and are not counted as passing.
- `cargo clippy --locked -p spinon-runtime --features c11-4-grid-track-sizing-fixture-test --all-targets -- -D warnings` hits the existing `too_many_arguments` lint in `anonymous_text_hyphens.rs`; scoped `--lib` Clippy passes.
- [Raw runtime evidence](./README.md) · [capture script](../../../../tools/css-reference/capture-c11-4-font-size-zero-simulators.mjs).

이 검토는 별도 PR diff 검토 30개를 대체하지 않는다. CI check, 실기기/hardware GPU, CSS WPT, 일반 author stylesheet, glyph rasterization 및 C11.4/C11 공식 완료를 주장하지 않는다.
