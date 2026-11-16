# C15.3 Flex intrinsic text layout · 2026-11-15

## 결과

`spinon-layout` 테스트에서 Noto Sans KR 20px, line-height 24px, `한글字`를 no-wrap Flex item으로 배치했다. `TextLayoutEngine::new_without_system_fonts()`에 저장소의 고정 Noto Sans KR fixture만 등록하고, `width:auto`·`height:auto`·`flex-basis:auto`인 item을 Taffy Flex max-content 경로에 전달했다. Flex box와 item frame은 Chrome 155 사전 기준 `56.8125×24px`에서 각각 0.05 CSS px 이내였고, 최종 text snapshot의 owner, source, font/style, revision, 한 줄 높이와 available width도 확인했다.

Chrome의 [C15.3 관찰 JSON](../c15-3-unicode-shaping-precomparison-2026-10-17/chrome155-observation.json)은 `#flex-sizing` rect가 `56.8125×24px`이라고 기록한다. 고정 폰트의 파일 hash와 license는 [fixture README](../../../../tests/fixtures/resources/c15/c15-3/README.md)에 있다.

## 실행

- 실행: `mise exec -- cargo test --locked -p spinon-layout c15_3_flex_max_content_item_matches_noto_intrinsic_text_oracle -- --test-threads=1 --nocapture`
- toolchain: `rustc 1.100.0 (0ef1bc386 2026-11-09)`, `cargo 1.100.0 (3acc79c0c 2026-11-05)`
- pass02 결과: 1 passed, 0 failed, 557 filtered out
- pass02 원본 로그: [focused-layout-pass02.log](./focused-layout-pass02.log)
- pass02 로그 SHA-256: `fc9dffe81d4afed51d900419deb6e32849a0fba084a3858bc76da4cbeb02672b`
- pass02 실행 전·후 C15.3 테스트와 fixture SHA-256: [실행 전](./source-hashes-pass02-before.sha256), [실행 후](./source-hashes-pass02-after.sha256); 해시가 일치한다. 기존 pass01 로그는 보존했다.

## 범위

세 번의 초기 독립성 적대 검토에서 이 test-only 범위가 C11.4와 파일이 겹치지 않는다고 판정했다. 2026-11-16에 별도로 진행한 두 번째 읽기 전용 검토도 test-only 범위의 독립성을 확인했다. oracle 차이, measurement-path 한계와 공유 Cargo 경계는 [scope review pass02](./scope-review-pass02.md)에 기록했다. Cargo `target`은 공유하므로 C11.4 장시간 매트릭스가 끝난 뒤 테스트를 빌드·실행했다. 별도 구현 20관점 검토도 GO이며 제한 범위와 관찰 공백은 [검토 기록](./implementation-review-20.md)에 정리했다.

이것은 Rust `LayoutInput`/Taffy Flex intrinsic consumer 검사다. Chrome fixture 전체 파싱, CSS `inline-flex` 투영, HostDocument/runtime 경로, 앱 simulator/device 동작이나 text paint를 증명하지 않는다. C15.3/C15 완료 상태와 공식 완료 수 `78/214`는 바꾸지 않는다.

이 bounded test-only 후보의 별도 PR packaging 검토는 [PR 전 적대 검토 30관점](./pre-pr-candidate-review-30-pass01.md)에 있다. C11.4 sibling pass07은 43개 선택 test 중 36 passed·7 failed였으므로 green matrix로 해석하지 않는다. 다만 입력 1,920/1,920 hash mismatch는 0이고 두 C15 source가 pass07 사전·사후, C15 focused pass02 사전·사후 및 현재 checkout에서 일치한다([source hash closure](./source-hash-closure-pass01.json), [재검토](./source-hash-closure-recheck-pass01.md)). hash gate는 닫혔으므로 두 Rust source와 C15 근거만 선택 staging할 수 있다. C11.4 변경, Cargo.lock sha2, Rust toolchain 변경은 이 패키지에서 제외한다. 이 기록은 최종 PR diff 검토가 아니며 현재 PR도 없다.
