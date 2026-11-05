# C11.1 Root HTML intrinsic Grid percentage gap runtime evidence

## 범위와 입력

이 evidence는 기존 `RootHtmlIntrinsicTrackSizingV1`에서 definite Root HTML Grid의 direct percentage gutter를 다룬다. CSS parser나 public CSS API, 전체 Grid 지원의 근거가 아니다. 동일한 9-scene source를 default Root HTML runtime route에 넣어 Android와 iOS에서 실행했다. 별도 [구현/runtime 실패 관점 20개 검토](./implementation-runtime-review-20.md) 및 실제 implementation PR diff [적대 검토 30개](./pr-diff-review-30.md)를 수행했다.

- Runtime fixture: [root-html-grid-intrinsic-gap-percentage-runtime.html](../../../../tests/fixtures/css/c11/root-html-grid-intrinsic-gap-percentage-runtime.html)
- Fixture SHA-256: `f0150d202f93c94662f14655608c08ba54ab6c8c2b14ef21da0bd357bfdf1f12`
- Reference: [Chrome 156 Agent Browser 사전 비교 및 독립 expected](../c11-root-html-intrinsic-gap-percentage-precomparison-2026-11-05/README.md)
- Route: Android `--ez spinon_c0412_root_html_default_render true`; iOS `--spinon-c0412-root-html-default-render`
- 앱 자산은 빌드 중에만 fixture로 교체했다. `asset-restoration.txt`와 각 installed/built asset hash가 source 복구를 기록한다. 저장소 원본 Android/iOS 자산 SHA-256은 양쪽 모두 `38813ca802618bd20bdf3bbb9ed9bc7525c289f48c0928813c855567f9e7df85`이다.

9 scenes는 no-gap control, column/row/both 10%, 서로 다른 120×80px 축 basis, column/row 120% visible overflow, 0%, `grid-gap` alias를 포함한다. 예상 geometry는 Chrome 관찰과 별도로 계산했다.

## 실행 결과

| 플랫폼 | 실행 대상 | 결과 | 화면 |
|---|---|---|---|
| Android | Android API 37 ARM64 AVD `emulator-5554`, ANGLE/SwiftShader | `layout=ready boxes=48`; root `320×150` CSS px; draw/present `boxes=48`; surface `840×630` | [simulator-screen.png](./android/simulator-screen.png) |
| iOS | iPhone 17 Pro / iOS 26.2 Simulator | `layout=ready boxes=48`; root `320×150` CSS px; draw/present `boxes=48`; surface `960×720` | [simulator-screen.png](./ios/simulator-screen.png) |

양 플랫폼 앱 번들에 담긴 fixture의 SHA-256은 source와 동일하다. 전체 build/launch output은 `android/` 및 `ios/` 아래 있고, `run-simulators.sh`는 기존 자산을 백업하고 exit trap으로 복구한다. 실행 로그에서 source fixture의 byte 수·SHA-256, `layout=ready boxes=48`, draw/present 48개와 비어 있지 않은 스크린샷을 강제 확인한다. 첫 PR diff 검토에서 로그 추출이 실패해도 통과할 수 있고 앱 종료 정리의 실패 창이 있음을 발견해 스크립트를 보강한 뒤 두 플랫폼 캡처를 다시 실행했다. 재실행이 모든 marker를 통과했고 fixture asset을 원래 hash로 복구했으며 앱만 종료했다. 두 번째 실행의 정규화 전후 로그 hash는 [manifest](./log-normalization.sha256.txt)의 `Second simulator capture` 항목이다. 캡처 시뮬레이터는 종료하지 않았다.

## Rust와 경계 검증

- `cargo test -p spinon-layout root_html_intrinsic_tracks --lib`: 8 passed.
- `cargo test -p spinon-runtime --features c11-1-grid-intrinsic-tracks-fixture-test root_html_grid_intrinsic_tracks --lib`: 11 passed.
- Fixture-module boundary test: 33 passed.
- Scoped all-target Clippy와 `cargo check --locked -p spinon-ffi --features c04-runtime-gpu` 통과.
- Typed layout tests는 axis basis, 0%/10%/120%, distinct fixed row/column bases, invalid typed percentage, resolved used-gap 및 aggregate f32 overflow를 검사한다. Runtime tests는 shorthand와 alias frame, overflow visibility, render scene 포함, projection이 표현할 수 없는 매우 큰 computed percentage의 원자 거부를 검사한다.
- 전체 `spinon-layout --lib` suite는 450개 중 449개 통과했다. 유일한 실패는 변경하지 않은 C15 `host_document_collects_direct_text_and_inherited_spans_in_source_order` assertion이다. 같은 assertion은 percentage-gap 변경 전 fixed-px baseline에서 448개 중 447개 통과로 기록돼 있다([baseline log](../c11-root-html-intrinsic-gap-runtime-2026-11-05/layout-full-suite.log)).
- fixture-test feature를 켠 전체 `spinon-runtime --lib`는 후보 276 passed/8 failed/3 ignored, 기준 `5d47b06cc` 275 passed/8 failed/3 ignored였다. 실패 테스트 8개의 이름이 동일해 후보에서 새 실패가 추가되지 않았지만 전체 suite는 green이 아니다([candidate log](./runtime-suite.log), [baseline log](./baseline-runtime-suite.log)). 이 기준 비교는 같은 checkout에서 변경된 Rust 파일 5개만 임시 보관·HEAD 복원한 뒤 실행했고, 실행 후 모두 보관 hash와 일치하게 복구했다.
- 최종 출력은 [layout focused](./layout-focused.log), [layout full suite](./layout-suite.log), [runtime focused](./runtime-focused.log), [runtime full suite](./runtime-suite.log), [runtime baseline](./baseline-runtime-suite.log), [Clippy](./clippy.log), [FFI check](./ffi-check.log), [fixture boundary](./fixture-boundary.log), [rustfmt](./rustfmt.log)에 저장한다.

## 판정 한계

Screenshots는 simulator 화면을 눈으로 확인하는 visual smoke다. Android PixelCopy, iOS Metal drawable readback, 실기기·hardware GPU, iOS compositor 이후 pixel parity와 WPT 적합성을 검증하지 않는다. Chrome은 reference oracle이고 Spinon WPT 통과로 세지 않는다. CSS projection이 표현할 수 없는 매우 큰 computed percentage는 layout adapter가 아니라 `unsupported_computed_value` projection diagnostic으로 거부한다. `calc()` gap, auto/indefinite axes, cyclic intrinsic percentage, implicit intrinsic tracks, 일반 Root HTML CSS admission은 이 slice 바깥이다. C11.1/C11/C11.4는 미완료이고 공식 집계 `78/214`는 바꾸지 않는다.

시뮬레이터 캡처 뒤 Android/iOS 앱 프로세스는 종료했다. 이번 runtime capture에서는 Agent Browser와 terminal-browser를 띄우지 않았고, process 확인에서 둘 다 실행 중이지 않았다. Chrome 156 사전 비교에 사용한 Agent Browser session도 capture 뒤 종료된 상태다.

기계 생성 로그 중 Android full log, iOS build log, 두 focused test log의 줄 끝 공백과 반복 EOF blank line만 정규화했다. 정규화 전·후 SHA-256은 [manifest](./log-normalization.sha256.txt)에 기록한다.
