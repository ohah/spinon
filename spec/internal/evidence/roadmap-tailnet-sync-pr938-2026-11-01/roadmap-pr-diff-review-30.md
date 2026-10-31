# PR #938 병합 상태 문서 PR 변경 실패 관점 검토 30개

검토 대상은 PR #938의 병합 상태를 반영하는 `spec/STATUS.md`, C11.4 계획·내부 계약 0059·runtime evidence·공개 `roadmap.html`, 그리고 선택 문서 경로 검증 근거다. 구현 PR #938의 diff 검토와 별개로, 이 문서 변경이 현재 상태·지원 범위·공개 링크를 잘못 표현할 경로를 확인했다.

| # | 실패 관점 | 확인 결과 |
|---:|---|---|
| 1 | 다른 PR의 병합 SHA를 #938에 연결하는가 | GitHub PR #938의 merge SHA `8c5ca0c62ed030642455f53122928b25bba80fda`와 새 기록이 일치한다. 통과. |
| 2 | 병합 시간이 로컬 시간처럼 적히는가 | `2026-10-31T18:34:54Z` UTC를 그대로 사용한다. 통과. |
| 3 | PR check 결과가 없는데 CI 통과라고 쓰는가 | 빈 check rollup을 명시하고 CI 성공을 주장하지 않는다. 통과. |
| 4 | 상태 기준일이 새 기록보다 오래된 채 남는가 | `spec/STATUS.md` 기준일을 2026-11-01로 맞췄다. 통과. |
| 5 | 제한 slice만으로 C11을 완료 처리하는가 | C11 상위 checkbox는 `[ ]`이며 변경되지 않았다. 통과. |
| 6 | C11.3·C11.3.3·C11.4를 완료 처리하는가 | 세 checkbox 모두 `[ ]` 상태다. 통과. |
| 7 | 새 negative-indent 결과를 공식 완료 수에 더하는가 | 현재 `roadmap.html` parser와 같은 ID/child parsing을 재현해 72/213을 얻었다. 이 diff는 공식 checkbox를 바꾸지 않는다. 통과. |
| 8 | C11의 완료 하위 범위를 숨겨 parent 완료처럼 보이는가 | C11.2.2와 C11.2.3만 체크 상태로 남고 C11 parent는 미완료다. 통과. |
| 9 | PR #938 내용을 PR #930에 잘못 귀속하는가 | 두 카드가 서로 다른 PR, SHA, scene 수와 계획을 가리킨다. 통과. |
| 10 | 변경 날짜가 실제 캡처 날짜와 다르게 기록되는가 | 카드 날짜 2026-11-01과 Android/iOS 실행 식별자가 맞는다. 통과. |
| 11 | 12개 신규·18개 회귀를 30개 신규로 읽게 하는가 | 새 12개와 기존 18개를 분리해 30-scene matrix로 쓴다. 통과. |
| 12 | 음수 범위를 임의의 모든 값으로 확대하는가 | `-5px`·`-10px` fixed allowlist만 표현한다. 통과. |
| 13 | `-30px` 관측을 지원 결과로 소개하는가 | `-30px`는 oracle-only 또는 제외 범위로 표시한다. 통과. |
| 14 | percentage·`hanging`·`each-line`을 지원한다고 읽히는가 | 해당 값과 조합은 모두 범위 밖으로 남는다. 통과. |
| 15 | Android AVD를 실기기·hardware GPU 결과라고 쓰는가 | Android API 37 ARM64 AVD라고 쓰고 software-renderer 한계를 남긴다. 통과. |
| 16 | iOS Simulator를 iPhone 실기기라고 쓰는가 | iPhone 17 Pro / iOS 26.2 Simulator라고 정확히 쓴다. 통과. |
| 17 | 검은 renderer screenshot을 glyph paint 증거로 과장하는가 | layout-ready 화면이며 glyph pixel 증거가 아니라고 기록한다. 통과. |
| 18 | 수정 전 Android matrix를 마지막 실행으로 링크하는가 | Android 기대값 수정 후의 `1793471314021-0e2d635a` run을 사용한다. 통과. |
| 19 | Android 수정과 iOS 실행 순서를 혼동하는가 | Android final rerun과 iOS simulator matrix를 각자 별도 근거로 연결한다. 통과. |
| 20 | static archive 검사로 APK/IPA 제외까지 주장하는가 | static archive 범위로 한정하고 APK/IPA 전체 exclusion은 주장하지 않는다. 통과. |
| 21 | 내부 계약 숫자 버전이 바뀌는가 | 계약 0059는 `0.1.0`을 유지한다. 통과. |
| 22 | 공개 카드나 검토 문서가 존재하지 않는 HTML route를 가리키는가 | Rspress 2.0.22 selected build에서 plan, contract, runtime README/reviews, post-merge review, STATUS, 20-point roadmap review와 이 30-point PR review route가 생성됐다. 20-point review의 30-point review 링크도 output HTML에서 확인했다. 통과. |
| 23 | screenshot 링크가 빌드 산출물에 있다고 잘못 전제하는가 | Rspress 출력 밖의 두 PNG를 저장소 source 경로와 SHA-256으로 별도 확인했다. Tailnet 정적 복사·served hash는 후속 배포 관문이다. 통과. |
| 24 | raw `roadmap.html`을 Rspress Markdown route라고 취급하는가 | 원본 정적 HTML source를 직접 검사했고, generated Markdown route와 분리했다. 통과. |
| 25 | 10개 로컬 링크 중 일부만 검사하는가 | 카드의 local link 10개 모두를 HTML 8개와 PNG 2개로 분류해 각각 확인했다. 통과. |
| 26 | selected build를 전체 docs build 성공으로 말하는가 | selected build는 dead-link 검사 비활성 경로 확인이다. 전체 `docs:build` 실패와 구분했다. 통과. |
| 27 | 전체 docs build 실패 원인과 영향을 숨기는가 | 전체 빌드에서 1,171 page-level dead-link reports를 기록했고 C11 negative-indent raw asset·저장소 밖 fixture/plan link 사례도 보존했다. 통과. |
| 28 | PR #938 병합 뒤 Tailnet 미리보기가 갱신됐다고 선행 주장하는가 | 이 변경에는 live Tailnet 결과를 주장하지 않는다. HTTP·served SHA·Agent Browser 확인은 병합 후 남은 일이다. 통과. |
| 29 | helper `.ts`/`.mjs`를 `spec/` 아래 추가해 docs route에 노출하는가 | 선택 build tooling을 `tools/tailnet-preview/`에 보관하고 `spec/internal/evidence/`에는 review·JSON·build logs만 둔다. 통과. |
| 30 | PR diff에 무관한 사용자 untracked evidence가 섞이는가 | 변경 파일은 정확한 경로로 stage하고 다른 untracked 항목은 추가하지 않는다. 통과. |

결론: 문서 diff는 PR #938의 제한 구현·현재 미완료 상태·검증 한계를 정확히 기록한다. 전체 docs dead-link 검사는 실패 상태이며, live Tailnet 갱신은 이 PR의 완료 증거가 아니다.
