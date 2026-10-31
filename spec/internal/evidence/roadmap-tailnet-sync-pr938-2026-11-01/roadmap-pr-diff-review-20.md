# PR #938 병합 상태 공개 로드맵 변경 검토 20개

대상은 PR #938 병합 사실과 C11.4 음수 `text-indent` 제한 slice를 기록한 `spec/STATUS.md`, C11.4 계획·계약·공개 `roadmap.html` 변경이다. 구현 PR의 30관점 검토와 post-merge status review 20개를 재사용하지 않고 공개 요약이 사용자에게 잘못된 전체 지원 인상을 줄 수 있는 별도 오류를 확인한다.

| # | 실패 관점 | 확인 결과 |
|---:|---|---|
| 1 | 다른 PR의 번호나 구현 결과를 #938에 귀속하는가 | 모든 새 공개 카드·상태 entry가 PR #938을 가리킨다. 통과. |
| 2 | PR의 merge SHA가 다른 커밋인가 | GitHub merge commit `8c5ca0c62ed030642455f53122928b25bba80fda`와 일치한다. 통과. |
| 3 | merge 시각을 로컬 시간으로 변환해 혼동하는가 | GitHub UTC `2026-10-31T18:34:54Z`를 그대로 표시한다. 통과. |
| 4 | 보고되지 않은 check를 CI 성공으로 기술하는가 | GitHub check rollup이 비었다고 기록하며 CI pass를 주장하지 않는다. 통과. |
| 5 | 최근 결과 때문에 C11 또는 C11.4를 완료 처리하는가 | 두 상위 checkbox를 미완료로 명시한다. 통과. |
| 6 | 제한 slice 때문에 공식 완료 집계를 올리는가 | 공식 `72/213`을 유지한다고 status와 공개 카드에 기록한다. 통과. |
| 7 | 새 구현이 모든 음수 `text-indent`를 지원한다고 읽히는가 | 값은 고정 `-5px`·`-10px` scene 12개에 한정한다. 통과. |
| 8 | 기존 regression 수가 12인지 18인지 혼동하는가 | 신규 12개와 기존 18개, 총 30개로 기술한다. 통과. |
| 9 | min-content에서 indent를 모든 word에 더한다고 설명하는가 | 공개 카드에서 수식을 과도하게 넓히지 않고 고정 scene 범위를 쓴다. 계획·계약은 first word와 later word floor를 설명한다. 통과. |
| 10 | max-content에 indent를 단어별로 여러 번 더한다고 설명하는가 | 내부 계약과 계획은 첫 formatted line에 한 번 적용한다고 설명한다. 통과. |
| 11 | 수정 전 Android matrix를 최종 run으로 링크하는가 | 공개 카드와 status는 Android 최종 run `1793471314021-0e2d635a`를 링크한다. 통과. |
| 12 | Android 결과를 실기기 또는 hardware GPU로 표시하는가 | Android API 37 ARM64 AVD로 표기한다. 통과. |
| 13 | iOS Simulator 결과를 iPhone 실기기 결과로 오인하는가 | iPhone 17 Pro / iOS 26.2 Simulator로 명시한다. 통과. |
| 14 | 검은 renderer screenshot을 glyph pixel equality로 내세우는가 | layout-ready 상태만 보여주며 glyph pixel 근거가 아님을 명시한다. 통과. |
| 15 | PR에 reported check가 없는 사실이 빠지는가 | check rollup 비어 있음과 CI non-claim이 적혀 있다. 통과. |
| 16 | 제한 fixture 결과가 일반 CSS admission 또는 public CSS support로 승격되는가 | 모든 요약은 opt-in fixture라고 쓰고 일반 admission은 미포함이라고 명시한다. 통과. |
| 17 | WPT·전체 CSS Grid 적합성으로 읽히는가 | WPT 및 전체 C11/C11.4는 범위 밖으로 명시한다. 통과. |
| 18 | 내부 계약 숫자 버전이 우연히 바뀌는가 | 내부 계약 0059는 `0.1.0`을 유지한다. 통과. |
| 19 | 공개 로드맵의 내부 근거 링크가 존재하지 않는 HTML route를 가리키는가 | Rspress 2.0.22 selected build에서 plan, contract, runtime README, implementation/runtime review, PR diff review, post-merge review, roadmap review와 STATUS HTML route가 생성됐다. 별도 route audit은 공개 raw roadmap HTML의 PR #938 링크 10개를 대조해 8개 generated HTML과 Android/iOS 원본 PNG 2개를 각각 SHA-256으로 확인했다. Rspress 출력에는 Markdown에서 링크한 PNG가 복사되지 않으므로 Tailnet 정적 asset 복사·served hash 확인은 배포 후 별도 관문이다. 통과. |
| 20 | C11 progress summary에서 미완료 상위 작업이 가려지는가 | C11, C11.3, C11.3.3, C11.4는 미완료이고 C11.2.2/C11.2.3만 체크 상태로 남는다. 통과. |

공개 변경은 병합된 제한 slice 하나를 최신 카드로 추가한다. 전체 C11 진행률이나 공식 완료 수를 변경하지 않는다. 이 문서 검토와 별도로 수행한 PR 변경 실패 관점 30개는 [PR diff review](roadmap-pr-diff-review-30.md)에서 확인한다.

선택 경로 빌드는 Rspress `2.0.22`로 통과했지만 `checkDeadLinks`를 끈 경로 범위 확인이다. 전체 `docs:build`는 1,171개 문서의 dead-link 오류로 종료됐으며, 그중 새 negative-indent evidence의 raw PNG·JSON·TSV·log 및 저장소 밖 fixture/plan 상대 링크도 보고됐다. 이를 전체 문서 빌드 성공으로 취급하지 않고, 공개 카드가 가리키는 10개 로컬 링크는 generated HTML route 8개와 source PNG 2개로 분리해 검사했다. Tailnet 반영 후 HTTP·served SHA-256·화면 확인은 아직 남아 있다.
