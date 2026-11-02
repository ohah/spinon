# C11.1 Root HTML percentage `grid-auto-rows` PR 병합 기록

- 구현 PR [#1047](https://github.com/ohah/spinon/pull/1047)은 2026-11-02T09:10:09Z `Rebase and merge`로 `main`에 병합됐다. GitHub가 보고한 merge SHA는 `bc3430ac664e76a2f246bcbfc387f61121017bb6`이다.
- PR head는 `70133f22c9e9a699a9de42777ad9b0f7719e4dd4`였고, Rebase and merge 결과 기능 commit은 `21842512ea2d3a8538252958832f809046f07898`이다. 최종 main tip에는 실행 evidence revision 고정 문서와 병합 전 로드맵 동기화도 포함된다.
- 제한 구현은 기본 parsed Root HTML 경로의 definite-height direct percentage implicit-row scalar/list만 다룬다. focused Rust tests, scoped Clippy/rustfmt, Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator의 `layout=ready` 및 13-box draw/present 결과는 [실행 evidence](./README.md)에 기록했다.
- Android 및 iOS screenshot은 시각 자료다. PixelCopy/Metal surface readback, 실기기, hardware GPU, WPT 및 전체 CSS 적합성 결과로 해석하지 않는다.
- PR status-check rollup은 비어 있었고 `main` required status checks도 설정되어 있지 않았다. CI 통과로 주장하지 않는다.
- 병합 후에도 C11.1과 C11 공식 체크는 미완료이며 전체 완료 수 `72/213`은 바뀌지 않았다.
- 병합 후 계획·상태 대장 동기화는 [별도 30개 관점 검토](./roadmap-postmerge-docs-review-30.md)에 기록했다.
