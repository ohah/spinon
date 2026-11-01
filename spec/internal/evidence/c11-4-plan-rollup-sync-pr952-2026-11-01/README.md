# C11.4 통합 계획 상태 대조

2026-11-01 코드·공식 대장·공개 roadmap 대조에서 C11.4의 세부 계획 두 곳이 실제 병합 상태보다 뒤처진 것을 확인했다. `spec/STATUS.md`, 내부 계약 0059, `spec/public/roadmap.html`, PR #948/#952 기록은 이미 두 제한 slice의 구현·simulator 결과·미완료 범위를 적고 있었다. 통합 계획 `plan/c11-css-grid.md`와 `plan/c11-4-grid-track-sizing.md`만 overflow-wrap을 미구현으로 표시하고 text-transform의 최신 결과를 요약하지 않았다.

이 변경은 통합 계획의 요약과 링크를 실제 병합 상태에 맞춘다. 구현 코드, API/계약 버전, `spec/STATUS.md`의 체크박스, 공개 roadmap, 공식 완료 수는 바꾸지 않는다. C11과 C11.4는 미완료이고 공식 수는 `72/213`이다.

## 기존 실행 근거

- PR [#948](https://github.com/ohah/spinon/pull/948)은 2026-10-31T22:23:22Z 병합됐다. 고정 Ahem `XXXX` 9-scene slice는 Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 각각 9/9 scene을 통과했다. [계획](../../../../plan/c11-4-anonymous-grid-text-computed-overflow-wrap.md) · [실행 근거](../c11-4-anonymous-grid-text-overflow-wrap-runtime-2026-11-01/README.md).
- PR [#952](https://github.com/ohah/spinon/pull/952)은 2026-10-31T23:48:28Z 병합됐다. Noto Sans KR `IIII ii`, `lang=en`, `none|uppercase|lowercase` 6-scene slice는 두 simulator에서 각각 6/6을 통과했다. [계획](../../../../plan/c11-4-anonymous-grid-text-computed-text-transform.md) · [실행 근거](../c11-4-anonymous-grid-text-transform-runtime-2026-11-01/README.md).
- 두 PR 모두 GitHub check을 보고하지 않았다. 이 기록은 기존 runtime/test/Simulator 결과를 집계하며 검사를 재실행하거나 CI 성공을 주장하지 않는다.

## 변경 검토

- [통합 계획 변경 실패 관점 검토 30개](pr-diff-review-30.md)
