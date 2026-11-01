# C11.4 `line-break: loose` 병합 후 상태·계획 diff 검토 30개

## 검토 범위

PR #962 병합 뒤 `spec/STATUS.md`, 내부 계약 0059, focused plan과 parent track-sizing plan에 merge 결과를 동기화하는 변경만 검토했다. 기능 implementation 20개 및 PR #962 diff 30개와 별도로 문서 날짜·상태·완료 경계·링크 오기 가능성을 확인했다.

| # | 상태/로드맵 실패 관점 | 확인과 판정 |
| ---: | --- | --- |
| 1 | 다른 PR 번호를 기록하는가 | 구현 PR 번호는 GitHub 확인값 #962다. |
| 2 | plan PR #961과 구현 PR을 혼동하는가 | 계획과 구현 PR을 별도 링크/상태로 적었다. |
| 3 | merge 시각이 로컬 시간으로 오기되는가 | GitHub의 UTC `2026-11-01T02:41:05Z`를 그대로 사용했다. |
| 4 | merge SHA를 원본 head commit과 혼동하는가 | Rebase merge 결과 `5831014125e26079748f0b99cb99d3c21613358c`를 기록했다. |
| 5 | merge 방식을 잘못 적는가 | GitHub PR 결과와 프로젝트 규칙에 맞춰 Rebase and merge라고 적었다. |
| 6 | report되지 않은 CI를 성공으로 표시하는가 | PR checks가 `no checks reported`였다고 명시한다. |
| 7 | `MERGEABLE`을 테스트 성공으로 해석하는가 | Mergeability와 CI를 별도 사실로 기록한다. |
| 8 | official `72/213` 수를 올리는가 | 모든 수정 문서가 공식 집계 `72/213`을 유지한다. |
| 9 | C11 checkbox를 완료 처리하는가 | C11/C11.4 전체 완료로 표시하지 않는다. |
| 10 | limited fixture를 일반 CSS support로 승격하는가 | `limited opt-in` 범위와 author CSS admission 제외가 유지된다. |
| 11 | 내부 계약 숫자 버전을 부당하게 올리는가 | `0.1.0`을 유지한다. |
| 12 | 기존 normal/strict section을 새 feature로 덮는가 | 기존 5-scene section과 evidence 링크는 별도 상태로 보존된다. |
| 13 | loose slice의 실 구현 상태가 계속 pending으로 보이는가 | PR #962 merge details를 status, plan, parent plan, contract에 반영했다. |
| 14 | focused plan이 parent roadmap과 다른 PR 상태를 보이는가 | 두 plan의 구현 PR 번호·시간·SHA가 일치한다. |
| 15 | parent track-sizing 문서가 candidate-only로 뒤처지는가 | heading과 본문을 PR #962 병합 기록으로 바꿨다. |
| 16 | precomparison 링크가 다른 line-break plan을 가리키는가 | loose 전용 Chrome 156 precomparison 경로를 사용한다. |
| 17 | 실행 README가 baseline 5개만 설명하는가 | 신규 4개와 회귀 5개가 합쳐진 9-scene 행렬을 정확히 설명한다. |
| 18 | Android/iOS evidence 경로가 바뀌는가 | `android-avd/`와 `ios-simulator/` matrix/log 경로가 README와 일치한다. |
| 19 | Android app 기대값 오류 수정 이력이 사라지는가 | 최초 실패 log와 수정 뒤 전체 재실행 사실을 보존한다. |
| 20 | iOS retry가 무제한 혹은 무조건 재실행처럼 적히는가 | `status=-12` 한정, 한 번 retry와 attempt log를 기록한다. |
| 21 | screenshot이 glyph paint 증거로 승격되는가 | route/status 화면이며 text paint를 입증하지 않는다고 명시한다. |
| 22 | iOS readback이 compositor 이후 pixels로 확대되는가 | drawable/compositor 이후 pixels 미검증 한계가 유지된다. |
| 23 | 실제 기기나 hardware GPU 검증으로 적는가 | AVD와 Simulator로만 구분하고 실기기/hardware GPU는 제외한다. |
| 24 | browser 확인을 simulator 동작 proof로 혼동하는가 | PR screenshot 표시 확인과 runtime matrix를 각각 설명한다. |
| 25 | static archive 결과가 APK/IPA 제외 claim으로 확대되는가 | artifact boundary 문서가 archive 한정이라고 설명한다. |
| 26 | failure review 링크가 옛 baseline review로 잘못 연결되는가 | loose runtime evidence 안의 별도 20/30 리뷰를 참조한다. |
| 27 | post-merge 문서가 implementation evidence 자체를 수정/삭제하는가 | matrix, original failures, attempts와 screenshots는 그대로 둔다. |
| 28 | RSPress 또는 docs deploy 상태를 확인 없이 주장하는가 | 이 diff는 로컬 문서 상태만 말하고 Tailnet publication 성공을 주장하지 않는다. |
| 29 | status entry가 Korean style과 list hierarchy를 깨는가 | C11 entry의 기존 들여쓰기·한글 기록 방식을 유지한다. |
| 30 | 다른 untracked evidence/caches가 문서 PR에 들어가는가 | PR scope는 지정한 상태/계획 4개 문서, evidence README와 이 review file에 한정한다. |

## 결론

위 30개 관점에서 문서 상태와 코드 merge 결과의 불일치를 찾지 못했다. Tailnet preview는 별도 publication 작업이며 여기서는 아직 완료로 기록하지 않는다.
