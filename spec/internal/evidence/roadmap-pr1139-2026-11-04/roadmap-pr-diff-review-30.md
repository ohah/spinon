# PR #1139 C11 numeric placement roadmap card 검토 30개 · 2026-11-04

대상 diff는 공개 `spec/public/roadmap.html`의 C11.1 numeric explicit placement 카드다. 기준은 병합된 구현 PR [#1137](https://github.com/ohah/spinon/pull/1137), merge SHA `6af14f69512c0c73d890efc728751e0ca342f9c0`와 [runtime evidence](../c11-root-html-grid-explicit-placement-runtime-2026-11-04/README.md)다.

| # | 독립 실패 관점 | 확인 결과 |
|---:|---|---|
| 1 | 아직 구현 전이라는 과거 문구가 남는가 | precomparison 전용 문단을 병합된 제한 구현 기록으로 교체했다. |
| 2 | 계획·구현 카드가 중복되는가 | 이전 위치에 있던 중복 구현 문단을 제거했다. |
| 3 | 다른 C11 기록이 함께 제거되는가 | diff는 대상 카드 한 곳과 duplicate만 바꾼다. |
| 4 | 구현 PR 번호가 다른가 | #1137을 가리킨다. |
| 5 | PR link가 화면 링크 대신 잘못된 branch URL인가 | GitHub PR canonical URL을 사용한다. |
| 6 | merge SHA가 branch commit과 혼동되는가 | rebase merge commit `6af14f69512c0c73d890efc728751e0ca342f9c0`을 쓴다. |
| 7 | 공식 완료 수가 증가하는가 | `72/213` 유지 문구가 들어 있다. |
| 8 | C11.1이 공식 완료로 표시되는가 | C11.1/C11 미완료라고 명시한다. |
| 9 | 일반 parsed CSS admission으로 과장되는가 | no-area row-flow Root HTML 경계로 한정한다. |
| 10 | 모든 visible item 조건이 빠지는가 | 모든 visible item이 조건을 만족해야 한다고 명시한다. |
| 11 | 단일 축 numeric placement도 포함되는가 | 두 축 모두 definite numeric line/span이라고 쓴다. |
| 12 | named-area 지원으로 읽히는가 | no-area 조건이 전면에 남는다. |
| 13 | column-flow나 dense가 포함되는가 | row-flow만 기록한다. |
| 14 | implicit track 생성이 포함되는가 | 기존 C11.2.1 연결만 쓰고 새 implicit track을 주장하지 않는다. |
| 15 | 신규 shorthand parser를 추가했다고 읽히는가 | 새 parser 주장은 없고 기존 경로 재사용을 명시한다. |
| 16 | Android 실행을 실기기로 오인하는가 | API 37 ARM64 AVD라고 구분한다. |
| 17 | iOS Simulator를 iPhone 실기기로 오인하는가 | iOS 26.2 Simulator라고 명시한다. |
| 18 | 22-box가 readback 비교 횟수로 과장되는가 | draw/present 장면 수로만 표현한다. |
| 19 | screenshot PNG를 renderer surface readback으로 주장하는가 | PixelCopy/Metal drawable readback이 아니라고 적는다. |
| 20 | screenshot 표본이 overflow claim을 뒷받침하는가 | simulator PNG 표본이 테두리 밖 visible overflow를 보인다고 제한적으로 쓴다. |
| 21 | GitHub check 공백을 CI green으로 표현하는가 | 빈 check rollup과 required check 미설정을 별도로 적는다. |
| 22 | main 보호 상태와 PR check state가 합쳐지는가 | 두 상태를 다른 사실로 기록한다. |
| 23 | Chrome 비교가 전체 CSS conformance를 증명하는가 | 기대값 근거로만 표현한다. |
| 24 | runtime focused gate가 전체 suite green으로 확대되는가 | scoped gate와 simulator facts만 담는다. |
| 25 | plan route가 Tailnet base를 빠뜨리는가 | `/spinon/plan/...` 경로를 유지한다. |
| 26 | precomparison evidence link가 빠지는가 | 기존 Chrome 사전 비교 route를 보존한다. |
| 27 | runtime evidence route가 빠지는가 | 구현·플랫폼 근거 링크를 연결한다. |
| 28 | 공식 status anchor가 사라지는가 | `/spinon/STATUS.html#c11-css-grid` link를 보존한다. |
| 29 | 최신 PR 순서가 과거 PR과 역전되는가 | #1132 이후, 더 이른 #1128 기록 전 위치에 둔다. |
| 30 | 좁은 화면에서 새 구조가 가로 폭을 강제하는가 | 기존 `kanban-note hero-progress` class만 재사용하며 새 width/CSS를 추가하지 않는다. |

검토 결과: 30개 실패 관점에서 카드의 상태·범위·근거 링크·표현에 모순을 찾지 못했다.
