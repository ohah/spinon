# C11.4 상위 계획의 PR #811 상태 동기화

## 변경 이유

`spec/STATUS.md`와 공개 로드맵은 C11.4 `fit-content()` 열 축 제한 runtime slice PR #811의 병합 결과를 기록했지만, [C11.4 상위 계획](../../../plan/c11-4-grid-track-sizing.md)은 같은 항목을 아직 "구현/runtime 시작 전"이라고 표시하고 있었다. 해당 문구를 병합된 결과와 검증 한계에 맞췄다.

## 동기화 내용

- CSS Grid 2025 CRD §11.2의 `fit-content()` 계산을 implicit columns의 고정 scene 네 개에 연결한 slice로 기록한다. 세 main scene은 `min-width:0`과 30/40/200px limit을 사용하고, 별도 automatic-minimum control은 `min-width:auto`와 40px limit에서 100px floor를 확인한다.
- Android API 37 ARM64 software AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 네 scene을 확인했고 PixelCopy 및 compositor 제출 전 Metal 표본은 각 14/14 exact였다. 하드웨어 GPU·실기기·compositor 이후 픽셀을 의미하지 않는다고 제한한다.
- PR #811의 병합 시각과 SHA를 연결하고, check rollup 및 required checks가 비어 있던 사실을 CI 통과로 표현하지 않는다.
- C11.4 checkbox와 공식 완료 `72/213`은 변경하지 않았다. 상태 대장과 공개 roadmap의 기존 기록은 이미 이 결과와 일치한다.

## 화면 근거

Tailnet 공개 로드맵의 PR #811 항목을 Agent Browser로 열어 표시 내용을 확인했다. 이 캡처는 `72/213`, C11.4 미완료, PR merge SHA, AVD/Simulator 표본과 남은 범위를 보여준다. [화면 PNG](./tailnet-roadmap-pr811-visible.png) · [브라우저 관찰](./agent-browser-verification.json) · [브라우저 종료](./browser-close.json).

이 화면은 이번 계획 문서 변경이 `main`에 병합되기 전의 공개 roadmap route다. 병합 후에는 변경된 C11.4 상위 계획 페이지도 Tailnet preview에 반영하고 별도 확인한다.

## 판정 경계

이 변경은 기록 동기화만 한다. 제품 코드, 내부 계약 버전, 공식 상태 checkbox와 완료 수는 바꾸지 않는다. 이 제한 slice는 일반 author CSS 지원, 전체 WPT, 전체 C11.4 또는 C11 완료가 아니다.

[PR 변경 실패 관점 검토 30개](./pr-diff-review-30.md).
