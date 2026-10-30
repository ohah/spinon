# C11.4 auto-ratio 병합 후 문서 PR diff 검토 30개

검토 대상은 PR #847 병합 뒤의 공식 상태 대장, C11.4 계획·내부 계약·runtime evidence, 공개 정적 로드맵 변경이다. 각 항목은 서로 다른 잘못된 기록·경로·범위 확장 가능성을 확인했다. 이 문서는 아직 Tailnet 배포 결과를 주장하지 않는다.

| # | 실패 관점 | 확인 | 판정 |
|---:|---|---|---|
| 1 | merge 시각을 잘못 기록 | PR #847의 `mergedAt` `2026-10-30T07:03:22Z`와 대조했다. | 통과 |
| 2 | rebase merge SHA를 PR head SHA로 혼동 | 상태·계획·계약의 merge SHA `04ef250c7009da5b81579ea1ef14e45981e56fd4`를 `gh pr view` 결과와 대조했다. | 통과 |
| 3 | 로컬 main이 merge commit을 포함하지 않음 | 문서 branch 시작점은 merge 뒤의 `origin/main`이며 main을 fast-forward했다. | 통과 |
| 4 | C11.4 상위 체크를 자동 완료로 바꿈 | `spec/STATUS.md`의 C11.4 체크는 unchecked 그대로다. | 통과 |
| 5 | C11 상위 체크를 자동 완료로 바꿈 | `spec/STATUS.md`의 C11 체크는 unchecked 그대로다. | 통과 |
| 6 | 공식 완료 수를 잘못 올림 | 상태 대장과 로드맵 카드 모두 `72/213`을 유지한다. | 통과 |
| 7 | 구현 후보를 완전한 C11.4 기능으로 표현 | 모든 변경은 제한 direct-text fixture 범위를 말하고 전체 C11.4/C11을 미완료로 남긴다. | 통과 |
| 8 | 내부 계약 숫자 버전을 올림 | 계약 0059 숫자 버전 `0.1.0`이 유지된다. | 통과 |
| 9 | numeric ratio의 sizing basis가 content-box로 기록 | 문서는 numeric ratio의 `BoxSizing` basis를 구분한다. | 통과 |
| 10 | auto ratio의 content-box basis 누락 | typed snapshot과 projection 계약은 `ContentBox`를 명시한다. | 통과 |
| 11 | owner gate를 profile 전체에 적용 | 단일 collector `NodeId`로 한정한 projection 설명을 확인했다. | 통과 |
| 12 | CSS computed `box-sizing` 값 자체를 수정한다고 기록 | 계약은 Taffy projection만 override하고 computed CSS property는 보존한다고 한다. | 통과 |
| 13 | padding/border transfer를 반대로 설명 | content box에서 ratio 적용 후 padding/border를 복원한다고 적었다. | 통과 |
| 14 | Android scene 수가 과장됨 | 실행 근거의 최종 matrix와 맞춰 Android 23/23을 기록한다. | 통과 |
| 15 | iOS scene 수가 과장됨 | 실행 근거의 최종 matrix와 맞춰 iOS 23/23을 기록한다. | 통과 |
| 16 | 표면 표본 수를 scene 수와 혼동 | 각 플랫폼 72/72 표면 표본을 별도 수치로 기록한다. | 통과 |
| 17 | simulator 결과를 실기기 성능 보증으로 확대 | AVD software renderer와 iOS compositor 이전 readback 한계를 보존한다. | 통과 |
| 18 | screenshot이 다른 scene/run으로 연결 | Android와 iOS link는 PR #847 runtime evidence의 지정 run ID를 가리킨다. | 통과 |
| 19 | RSPress 정적 로드맵 링크가 잘못된 URL | 새 카드의 계획·evidence·STATUS 경로가 `/spinon/` route 형식을 쓴다. | 통과 |
| 20 | 정적 HTML 문단 태그가 닫히지 않음 | 새 card는 독립 `<p>`로 삽입되고 기존 finite-ratio card의 `</p>`를 보존한다. | 통과 |
| 21 | 새 progress card가 이전 카드 중간에 들어감 | finite-positive PR #842 카드 뒤, C11.3.3 진행 카드 앞에 삽입했다. | 통과 |
| 22 | 계획 파일의 예전 “미구현” 문구가 최신 결과와 모순 | auto-ratio 집중 계획 첫 상태 문단과 상위 계획 항목을 병합 결과로 갱신했다. | 통과 |
| 23 | 내부 계약의 “후보”가 merge 상태와 모순 | auto-ratio 계약 섹션을 제한 opt-in fixture 구현으로 표시했다. | 통과 |
| 24 | runtime README에 merge 증거 누락 | PR, UTC merge 시각, merge SHA를 넣었다. | 통과 |
| 25 | 인접 C07.3 실패를 통과로 포장 | `c07_3_aspect_ratio_tests` 4/5와 양쪽 문자열을 그대로 미해결로 적었다. | 통과 |
| 26 | C07.3 문자열 serializer를 이번 변경이 수정했다고 오인 | 구현 변경에서 serializer를 수정하지 않았다는 기존 범위를 유지한다. | 통과 |
| 27 | 빈 PR check rollup을 CI green으로 주장 | 로드맵·계획·상태·계약은 빈 check와 필수 status check 부재를 구분한다. | 통과 |
| 28 | static archive 근거를 APK/IPA 전체 exclusion으로 확장 | FFI static archive 범위에만 해당한다는 문구를 유지한다. | 통과 |
| 29 | 공개 로드맵 변경을 이미 배포했다고 오인 | PR diff에는 source 변경만 포함하며 Tailnet 배포 완료 문구는 없다. | 통과 |
| 30 | 오래된 수치를 두고 이번 PR 기록과 연결 | 신규 merge 카드와 상태 대장의 PR #847, scene/sample 수, 72/213을 교차 대조했다. | 통과 |

이 검토는 문서 diff 검토이며 feature 구현/runtime 검토 20개나 PR #847 코드 diff 검토 30개를 대체하지 않는다.
