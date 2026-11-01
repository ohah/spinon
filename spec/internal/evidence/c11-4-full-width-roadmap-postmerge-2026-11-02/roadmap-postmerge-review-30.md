# C11.4 `full-width` 병합 후 상태·로드맵 diff 검토 30개

검토 대상은 PR #995 merge SHA `772df994a5fba4aa8d405b3f42792c281150f88c` 뒤의 `spec/STATUS.md`, C11 계획, 내부 계약 0059 및 공개 `roadmap.html` 변경이다. 구현 PR의 30개 diff 검토와 구분해 상태 일관성, 출처, 링크, 공개 문구를 확인했다.

| # | 실패 관점 | 확인 결과 |
|---:|---|---|
| 1 | PR #995 merge commit을 source implementation commit으로 잘못 표기함 | merge SHA `772df994…`와 source `2b6fec25…`를 구분해 기록한다. |
| 2 | merge timestamp를 local date와 섞어 부정확하게 씀 | PR 기록의 UTC `2026-11-01T18:03:31Z`를 그대로 유지한다. |
| 3 | 제한 slice 병합만으로 C11.4 checkbox를 완료 처리함 | `spec/STATUS.md`와 내부 계약의 parent checkbox는 그대로 미완료다. |
| 4 | subtask merge를 C11 전체 완료로 확대함 | C11.3.3 전체·C11.4 전체·C11 상위 미완료 표기를 유지한다. |
| 5 | 공식 완료 집계가 세부 slice 수만큼 증가함 | 공식 `72/213`을 유지하고 roadmap counter script는 수정하지 않는다. |
| 6 | roadmap execution 상태에서 C11.4가 빠짐 | `inProgress:['C11.4']`를 유지한다. |
| 7 | C11.4 full-width로 일반 CSS 지원을 암시함 | direct `lang=ja`, exact source 및 fixed typography 제한을 명시한다. |
| 8 | 테스트 플랫폼을 실기기로 확대 표기함 | Android API 37 ARM64 AVD와 iOS Simulator로만 쓴다. |
| 9 | AVD renderer가 hardware GPU로 보임 | SwiftShader/software GPU 경계를 runtime evidence에 남긴다. |
| 10 | iOS drawable 결과가 compositor 이후 pixel로 보임 | 캡처는 `layout=ready`이며 glyph/pixel equality가 아니라고 적는다. |
| 11 | Chrome 156이 full-width 변환 oracle처럼 보임 | Chrome의 computed `none` 결과를 unsupported-value 관측으로 분리한다. |
| 12 | CSS Text CRD 상태 변화를 확정 사실로 처리함 | `full-width`의 at-risk 상태와 추후 명세 변경 미검증을 남긴다. |
| 13 | full-width mapping을 임의 Unicode까지 일반화함 | fixed ASCII source subset만 설명한다. |
| 14 | screenshot 증거가 glyph raster를 입증한다고 과장함 | 두 캡처 모두 harness status만 보여 준다는 제한을 유지한다. |
| 15 | app binary에서 fixture가 완전히 제외됐다고 주장함 | macOS static archive 경계로 한정하고 APK/IPA 전체 exclusion을 부인한다. |
| 16 | PR checks가 비었는데 CI green으로 적음 | GitHub check run 미보고 상태를 명시하고 CI pass를 주장하지 않는다. |
| 17 | 내부 interface version을 slice 하나 때문에 올림 | 내부 계약 `0059`의 `0.1.0`을 유지한다. |
| 18 | full-width section이 아직 구현 전 candidate로 남음 | 계약 heading과 merge metadata를 PR #995 상태로 갱신한다. |
| 19 | capitalize ASCII slice가 PR #986 merge 전으로 남음 | PR #986 merge SHA와 제한 profile 상태를 0059에 연결한다. |
| 20 | Unicode titlecase PR #991이 계획만으로 남음 | 0059에 PR #991 merge SHA와 제한 runtime profile을 반영한다. |
| 21 | hyphens:auto PR #982가 candidate로만 남음 | 0059의 merged heading, merge SHA 및 runtime evidence를 동기화한다. |
| 22 | hyphens:auto 제한을 공개 CSS admission으로 확대함 | 고정 English fixture feature 밖의 거부 경계를 보존한다. |
| 23 | C11 plan의 최신 full-width 작업이 이전 titlecase 후보와 충돌함 | full-width PR #995 병합 결과로 최신 기록을 교체한다. |
| 24 | C11 plan 상단의 C11.4 merge 요약에서 #995 누락 | 현재 상태 line에 #995 slice와 parent 미완료를 추가한다. |
| 25 | 공개 roadmap 카드가 PR과 merge SHA를 혼동함 | `PR #995 Rebase and merge`, UTC merge time과 full SHA를 함께 쓴다. |
| 26 | roadmap 카드 local plan link가 잘못된 base route를 가리킴 | `/spinon/plan/c11-4-anonymous-grid-text-transform-full-width.html`를 지정한다. |
| 27 | precomparison/runtime evidence 링크가 없는 페이지를 가리킴 | Markdown source route에서 생성되는 `.html` route를 각각 연결한다. |
| 28 | diff review 또는 공식 상태 링크가 빠짐 | PR diff review 30개와 `/spinon/spec/STATUS.html` route를 카드에 연결한다. |
| 29 | roadmap source page만 바뀌고 진행 모델도 완료로 바뀜 | 표시용 카드만 추가하고 완료 counter/checkbox와 parent state는 그대로 둔다. |
| 30 | Tailnet server를 문서 반영 과정에서 재시작·정리함 | 실행 중인 PID 28040과 local root를 보존하고 preview overlay 절차로 갱신한다. |

이 PR은 병합 상태의 문서 동기화다. 전체 docs dead-link 검사를 수행하거나 C11/C11.4 완료를 주장하지 않는다.
