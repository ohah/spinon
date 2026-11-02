# C11.4 `full-size-kana` 병합 상태 동기화 검토 30개

검토 대상은 main에 병합된 PR [#1082](https://github.com/ohah/spinon/pull/1082)의 공식 상태 대장 및 runtime evidence 병합 상태 갱신이다. PR API의 확인값은 `mergedAt=2026-11-02T20:18:40Z`, `mergeCommit=3c34f3b8c71eb9a960414942cecab4179d62dbb8`다. 검토는 구현 diff 검토와 별도로 상태 집계, 링크, 증거 범위 및 날짜/SHA 정합성을 살핀다.

| # | 실패 관점 | 확인 결과 |
| ---: | --- | --- |
| 1 | 상태 변경이 공식 원본 `spec/STATUS.md` 밖의 미리보기 파일에만 기록되는가 | 공식 상태 대장의 C11.4 진행 기록을 갱신했다. |
| 2 | 상위 C11 CSS Grid가 제한 후보 병합만으로 완료 처리되는가 | C11 상위 체크박스는 `[ ]`로 유지된다. |
| 3 | 하위 C11.4 checkbox가 이 slice로 완료 처리되는가 | C11.4는 미완료 상태로 둔다. |
| 4 | 완료 숫자 `72/213`이 임의로 올라가는가 | 공식 완료 수 `72/213`을 그대로 유지한다. |
| 5 | 계획 PR과 구현 PR 번호가 뒤섞이는가 | 계획 PR #1081과 구현 PR #1082를 별도 링크한다. |
| 6 | PR #1082가 열린 상태로 잘못 남는가 | GitHub API가 반환한 merged state와 UTC merge time을 기록했다. |
| 7 | rebase merge SHA를 원래 feature head SHA로 잘못 적는가 | `mergeCommit.oid`의 `3c34f3b8…`를 기록하며 source/test head와 구분한다. |
| 8 | rebase merge가 merge commit으로 표시되는가 | PR의 merge method `Rebase and merge`와 결과 commit SHA만 기록한다. |
| 9 | 로컬 날짜와 GitHub UTC 시간이 혼동되는가 | 2026-11-03 진행 기록 아래에 GitHub timestamp를 `2026-11-02T20:18:40Z`로 그대로 쓴다. |
| 10 | PR checks가 빈 상태인데 CI 통과로 기록되는가 | check rollup이 비었고 CI 통과로 세지 않는다고 명시했다. |
| 11 | main에 required status check이 없는 사실을 누락하는가 | branch protection 확인에서 required checks 미설정 응답을 확인해 상태에 반영했다. |
| 12 | Android AVD 결과가 실기기 결과로 표현되는가 | Android API 37 ARM64 AVD 표기를 유지하고 물리 장치 주장을 하지 않는다. |
| 13 | iOS Simulator가 실기기로 표현되는가 | iPhone 17 Pro / iOS 26.2 Simulator라고 명시한다. |
| 14 | 두 플랫폼 행렬 중 하나의 성공만 전체 성공처럼 합산되는가 | Android 10/10과 iOS 10/10을 각자 별도 표기한다. |
| 15 | source head `e6729ff`가 rebase merge SHA와 같다고 오인되는가 | runtime README는 simulator cohort의 source head를, 상태 대장은 merge SHA를 구분해 기록한다. |
| 16 | 고정 mapping 범위를 Unicode 전체 지원으로 확대하는가 | 58개 Appendix G mapping과 fixed source allowlist만 기술한다. |
| 17 | 모든 `lang=ja` input에서 동작한다고 오해되는가 | direct owner language와 exact source allowlist 제한을 유지한다. |
| 18 | 일반 CSS cascade/admission을 구현한 것처럼 읽히는가 | opt-in fixture 경로를 명시하고 공개 CSS 지원을 주장하지 않는다. |
| 19 | Chrome이 positive transform oracle로 표현되는가 | Chrome 156 property 미지원과 literal geometry 비교 한계를 유지한다. |
| 20 | screenshot이 kana glyph 렌더 결과로 오인되는가 | `layout=ready` 및 glyph paint disabled 한계를 명시한다. |
| 21 | Katakana min-content line advance 차이가 숨겨지는가 | runtime README에 `9.5/10px` 대 `10/10px` 차이를 유지한다. |
| 22 | ICU4X 일본어 segmentation warning이 지원 주장에 섞이는가 | 일본어 일반 word segmentation을 다루지 않는 한계를 유지한다. |
| 23 | static archive 조사 결과를 APK/IPA 전체 제외로 일반화하는가 | static archive feature boundary만 증명하며 완성 bundle 제외는 입증하지 않는다고 쓴다. |
| 24 | 기본 archive에서 fixture marker가 없는 결과가 제품 기능의 완성을 의미하는가 | positive/negative archive 조사 범위를 후보의 artifact 경계로 한정한다. |
| 25 | PR 이미지가 검은 surface를 동작 화면처럼 보여주는가 | PR 본문에 시각 행동 screenshot을 첨부하지 않은 이유를 기록한다. |
| 26 | 20개 runtime review나 30개 PR diff review 링크가 끊기는가 | 두 검토 파일을 `spec/STATUS.md`에 각각 상대 경로로 연결한다. |
| 27 | plan PR의 사전 비교/계획과 runtime 실행 기록이 분리되지 않는가 | 계획, 사전 비교, runtime evidence 경로를 별도로 유지한다. |
| 28 | 내부 계약 숫자 버전이 구현 병합 때문에 바뀌는가 | 계약 `0.1.0` 유지 원칙을 PR 본문과 0059에 보존한다. |
| 29 | Android/iOS 성공 수와 failed attempts가 함께 성공 수로 계산되는가 | runtime README는 최종 pass05/pass02를 고정하고 이전 실패 cohort를 별도로 둔다. |
| 30 | evidence README를 갱신하면서 manifest가 stale해지는가 | README 수정 뒤 SHA-256 manifest를 재생성하고 전체 entry digest를 검산한다. |

## 판정

상태 갱신에서 잘못된 merge metadata, 완료 승격 또는 증거 범위 확대를 발견하지 못했다. 구현 후보가 main에 병합된 사실과 C11/C11.4 공식 완료 여부는 별개이며, 공식 완료 수는 여전히 `72/213`이다.
