# C11.4 one-fit row-span mixed automatic minimum · runtime evidence

이 디렉터리는 PR [#1381](https://github.com/ohah/spinon/pull/1381)의 제한 구현 결과, 실행 로그와 플랫폼 화면을 모은다. 범위는 parsed Root HTML의 기존 one-fit row-span profile에서 span text item마다 computed `min-height:0px|auto`를 적용하는 것이다. 일반 CSS Grid author admission, 전체 C11.4/C11 완료 또는 official `78/214` 집계 승격을 뜻하지 않는다.

## RuntimeSession 결과

- Chrome 157 oracle의 고정 120,960 scene을 기본 Root HTML RuntimeSession에서 모두 확인했다: `120,960/120,960` scenes, `900/900` six-member track-list batches, eligible automatic-minimum item occurrence `169,344`.
- 기존 endpoint와 회귀: one-fit positions 1–3 `2,880` pairs, positions 4–6 `4,320` pairs, two-fit row-span `3,840` scenes, one-fit auto-placement `1,440` scenes, two-fit auto-placement와 repeated-text `810` corpus.
- Atomic rejection/recovery, malformed trace controls, layout 관련 `56/56`, style projection 관련 `5/5`, 모듈 분리 후 focused RuntimeSession `1/1`, `cargo fmt --all -- --check`와 변경 crate scoped Clippy `-D warnings`를 기록했다.

자세한 분모와 실행 요약은 [runtime matrix 결과](./runtime-matrix-result.md), 일시적인 timeout 재현은 [timeout regression](./runtime-matrix-timeout-regression.md), 개별 실행 로그는 디렉터리의 `*.log`에 있다. 구현/runtime 관점 20개는 [implementation review](./implementation-runtime-review-20.md), 실제 구현 PR diff 실패 관점 30개는 [PR diff review](./implementation-pr-diff-review-30.md)다.

## Android / iOS simulator smoke

동일 HTML source SHA-256 `a944c85929ac16065cd950d4edc58788f8eb98ab2759644aa6ceb23874b57676`로 Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 light/dark 고정 C04.12 Root HTML route를 실행했다. 네 실행 모두 최종 `layout=ready`, 12 boxes, draw/present 성공이다. Android renderer는 Vulkan Goldfish GFXStream llvmpipe software다.

이 결과는 visual smoke다. mixed-mask 전체 앱 matrix, physical device, hardware GPU, PixelCopy/Metal readback, compositor parity 또는 surface pixel parity를 검증하지 않는다. [플랫폼 검증 범위와 정리 기록](./platform-visual-smoke.md)와 [Android light](./android-light.png), [Android dark](./android-dark.png), [iOS light](./ios-light.png), [iOS dark](./ios-dark.png) 화면을 참고한다.

## 병합 및 리소스 정리

PR [#1381 병합 기록](./merge-PR1381.md)에 source/base/hash, GitHub check 상태와 post-run 정리 내용을 기록했다. [사후 상태·로드맵 동기화 PR diff 검토 30개](./post-merge-docs-pr-review-30.md)도 별도 파일로 둔다. Simulator와 AVD는 종료했고 Agent Browser 검증 세션도 닫았다. 재사용 가능한 캐시와 simulator profile은 보존했다.
