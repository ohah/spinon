# C11.1 PR #1106 병합 후 상태 동기화 검토 30개

2026-11-03에 병합된 Root HTML `fit-content(percentage)` 제한 구현을 GitHub, `origin/main`, 공식 status ledger, 내부 계약, 계획과 public roadmap 사이에서 대조했다. 일반 CSS 지원과 공식 완료 수 승격 여부를 별도 확인했다.

| # | 확인 관점 | 결과 |
|---:|---|---|
| 1 | GitHub PR #1106 state | PASS — API가 `MERGED`를 반환한다. |
| 2 | GitHub mergedAt | PASS — `2026-11-03T04:59:34Z`다. |
| 3 | GitHub merge commit | PASS — `e631b0e1db352ed0832efb0c67b1fb44e64141ea`다. |
| 4 | GitHub PR head | PASS — 문서 검토 commit `5f0cbb34…`가 PR head였다. |
| 5 | 실제 implementation commit | PASS — `5459a7d48583b366095508c3d143a3e73cecdea0`가 merge commit의 parent다. |
| 6 | 계획 PR #1105 상태 | PASS — 병합 시각과 SHA를 #1106과 분리했다. |
| 7 | main branch update | PASS — `origin/main` top commits에 코드와 문서 두 commit이 연속으로 나타난다. |
| 8 | C11 상위 체크박스 | PASS — 미완료 상태다. |
| 9 | C11.1 전체 체크박스 | PASS — 미완료 상태다. |
| 10 | 공식 완료 분자 | PASS — `72`로 유지된다. |
| 11 | 공식 완료 분모 | PASS — `213`으로 유지된다. |
| 12 | status header 제품 지원 완료 | PASS — `없음`으로 유지된다. |
| 13 | C11.1 feature summary | PASS — named-area 없는 explicit axes의 bounded direct percentage slice로 설명한다. |
| 14 | Profile admission | PASS — `RuntimeRootHtmlTextPaintV1`만 적용하는 제한을 기록한다. |
| 15 | Axis basis | PASS — column width와 row height의 content-box basis를 명시한다. |
| 16 | Opposite-axis gate | PASS — 두 content dimensions 모두 definite해야 하며 반대 축 `auto`는 제외한다. |
| 17 | Percentage edge values | PASS — `0%`, definite zero basis, over-100 argument 보존을 적는다. |
| 18 | Track sizing formula | PASS — auto minimum, resolved limit, max-content contribution의 min/max 경계를 유지한다. |
| 19 | Gap basis | PASS — gap을 percentage basis에서 차감하지 않는다. |
| 20 | Named areas | PASS — named-area + percentage 조합은 제외한다. |
| 21 | `calc()`와 intrinsic limits | PASS — 이번 후속에서 제외한다. |
| 22 | Auto/implicit tracks | PASS — auto-repeat 및 implicit `grid-auto-*` 확장은 선언하지 않는다. |
| 23 | General CSS admission | PASS — 공개 CSS 지원을 주장하지 않는다. |
| 24 | WPT conformance | PASS — WPT 적합성 미검증 범위를 유지한다. |
| 25 | Android evidence | PASS — API 37 ARM64 SwiftShader AVD PixelCopy 4/4로 한정한다. |
| 26 | iOS evidence | PASS — iPhone 17 Pro/iOS 26.2 Simulator drawable 표본 4/4, compositor 제출 전으로 한정한다. |
| 27 | GitHub checks | PASS — PR check rollup empty이며 CI 통과로 표현하지 않는다. |
| 28 | Main branch protection | PASS — `required_status_checks=null`을 확인했다. |
| 29 | Candidate evidence integrity | PASS — 55개 기존 SHA-256 항목을 #1106 merge tree의 파일로 대조해 모두 일치했다. |
| 30 | Worktree and untracked preservation | PASS — 단일 checkout에서 순차 진행했고 기존 untracked 자료를 건드리지 않았다. |

## 판정

병합 정보와 제한 동작 상태가 `spec/STATUS.md`, 내부 계약 0059, 전용 계획 및 public roadmap card에서 일치한다. C11.1/C11 미완료와 공식 `72/213`을 유지한다. 이 검토는 Tailnet 서버 제공 상태를 증명하지 않으며, 해당 확인은 별도 동기화 뒤 Agent Browser와 served-content hash로 수행한다.

## 관련 자료

- [공식 상태 대장](../../../STATUS.md) · [내부 계약 0059](../../0059-c11-grid-layout.md) · [전용 계획](../../../../plan/c11-root-html-grid-fit-content-percentage.md)
- [PR #1106 implementation evidence](../c11-root-html-grid-fit-content-percentage-runtime-2026-11-03-pass02/README.md)
- [PR #1106](https://github.com/ohah/spinon/pull/1106)
