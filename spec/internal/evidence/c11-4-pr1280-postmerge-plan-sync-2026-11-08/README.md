# C11.4 PR #1280 병합 후 계획 동기화

2026-11-08에 PR [#1280](https://github.com/ohah/spinon/pull/1280)이 병합된 뒤 계획 문서의 상태를 맞췄다. 제한 구현 완료와 C11.4/C11 전체 완료를 구분하고, 다음 순차 후보의 기존 계획·사전 비교·계획 검토 상태를 명시한다.

## 변경 범위

- `plan/c11-css-grid.md`: 네 intrinsic track type 24순서 제한 구현을 PR #1280 병합 상태로 갱신했다. 96/96 Chrome 비교, 12/12 Layout, 96 typed cap cases, 7/7 RuntimeSession, 48 overflow cases 및 플랫폼 캡처의 visual-smoke 경계를 요약했다. CI가 보고되지 않은 점과 content-varying min/max contribution이 미검증인 점을 보존했다.
- `plan/c11-4-grid-track-sizing.md`: 병합된 24순서 slice를 현재 순차 작업에서 완료 기록으로 옮기고 다음 계획 후보를 별도 intrinsic auto-row profile의 singleton `grid-auto-rows:max-content`로 표시했다. 해당 후보는 Chrome 157 사전 비교와 계획 실패 관점 20개 검토가 끝났으며 구현 전이다.
- `plan/c11-4-root-html-intrinsic-auto-rows-max-content.md`: PR #1280이 별도 four-member profile에 max-content를 추가했지만 다음 후보의 singleton profile은 그대로 거부된다는 경계를 명시했다.
- C11.4/C11 상태 체크와 공식 완료 `78/214`는 변경하지 않았다.

## 확인

- PR #1280의 병합 시각·방식·merge SHA는 GitHub PR metadata와 공식 `spec/STATUS.md` 기록에 대조했다.
- 새 계획 및 근거 링크의 대상 파일이 존재한다.
- `git diff --check`가 통과했다. 이 문서 변경을 위해 제품 테스트나 simulator 실행은 하지 않았다.
- 서로 다른 실패 경로를 확인한 문서 PR diff 검토 30개는 [검토 기록](./pr-diff-review-30.md)에 남겼다.

이 문서 PR은 계획 상태 설명만 고친다. C11.3/C11.4의 완료 판정, 제품 API 지원 범위, Tailnet 제공본 배포를 주장하지 않는다.
