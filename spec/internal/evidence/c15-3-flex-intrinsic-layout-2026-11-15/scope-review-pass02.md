# C15.3 Flex intrinsic test-only 독립성 검토 · pass02 · 2026-11-16

## 경계

두 reviewer가 서로 다른 세 실패 관점으로 현재 Flex intrinsic WIP를 읽기 전용 검토했다. 파일 수정·빌드·테스트·브라우저 실행은 하지 않았다. 이 기록은 기존 구현 20관점 및 PR 준비 30관점 기록을 대체하지 않고, 실제 PR diff 검토로 세지 않는다.

## 검토 결과

1. **Oracle와 입력 차이:** Chrome 155 사전 관찰의 `#flex-sizing` DOM rect는 `56.8125×24px`이고 고정 Noto Sans KR fixture가 로드됐다. Rust 테스트는 같은 텍스트·폰트·size·line-height·nowrap을 쓰지만, HTML의 `inline-flex`/`span` 대신 Taffy `Flex`/Paragraph 입력을 직접 만든다. 따라서 CSS projection이나 HostDocument/runtime 동등성으로 확대하지 않는다. Chrome binary hash와 CDP revision은 기록되지 않았다.
2. **Layout·measurement 경로:** 테스트는 `TaffyLayoutEngine.compute_with_text_layout_engine`에 max-content Flex parent와 auto-sized item을 전달하고 parent/item frame을 각각 0.05 CSS px 이내로 비교한다. 최종 text snapshot의 owner, source/style, revision, line count, line height와 available width도 확인한다. Flex intrinsic callback의 종류·요청은 trace하지 않고, no-wrap 입력은 min-content와 max-content 폭을 같게 만들 수 있으므로 특정 callback 경로 자체를 입증하지 않는다. 이 한계는 bounded Rust layout consumer 범위와 양립한다.
3. **독립성·공유 입력:** 변경 대상은 `c15_text_layout_tests.rs`의 module 등록과 `flex_intrinsic.rs` 한 테스트다. `#[cfg(test)]` 아래 있으며 C11.4 제품 Grid 파일과 분리돼 선택 staging할 수 있다. Cargo target은 공유하므로 장시간 C11.4 matrix와 병렬 빌드는 피하고, `Cargo.lock`의 C11.4 전용 `sha2`, `mise.toml` toolchain update, C11.4 파일은 이 후보에서 제외한다.

## 결론과 다음 게이트

이 bounded test-only 후보는 C11.4와 독립적으로 PR packaging을 진행할 수 있다. 기존 pass02의 1/1 실행과 현재 source hash가 일치하며, 공식 C15.3/C15 상태는 미완료로 유지한다. PR 생성 뒤에는 실제 GitHub diff에 대해 새 30관점 검토와 CI/merge 상태 확인이 남아 있다.
