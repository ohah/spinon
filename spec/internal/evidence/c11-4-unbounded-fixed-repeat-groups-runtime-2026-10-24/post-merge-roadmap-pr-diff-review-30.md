# C11.4 fixed-repeat 그룹 수 제한 제거 · 병합 후 문서 동기화 PR diff 검토 30개

PR #597 병합 뒤 official status, 작업 계획, 실행 보고서와 공개 roadmap에 같은 merge 사실을 반영하는 변경을 검토했다. 코드 동작이나 C11.4 지원 범위는 바꾸지 않는다.

| # | 실패 관점 | 판정 |
| ---: | --- | --- |
| 1 | 병합 기록이 잘못된 PR에 연결되는가 | #597 링크와 저장소 소유자·경로가 일치한다 |
| 2 | Rebase and merge와 squash/merge-commit을 혼동하는가 | 실제 PR 병합 방식과 기록이 일치한다 |
| 3 | merge timestamp가 현지 시간으로 UTC 표기되는가 | `2026-10-24T11:34:18Z`를 사용한다 |
| 4 | merge SHA가 head commit을 가리키는가 | `ddea615bf488628bb77059fbe9e46551dfa36400`로 기록했다 |
| 5 | PR이 아직 open인데 merged로 주장하는가 | GitHub PR state가 `MERGED`이고 main에 SHA가 존재한다 |
| 6 | PR check rollup을 성공으로 과장하는가 | 비어 있었음을 그대로 기록한다 |
| 7 | required status checks 미설정을 CI green으로 바꾸어 말하는가 | branch protection 조회 결과 그대로 분리해 적는다 |
| 8 | main status check과 다른 branch의 checks를 혼동하는가 | main의 required status 설정으로 범위를 한정한다 |
| 9 | merge metadata가 다른 C11.4 slice에 붙는가 | fixed-repeat 그룹 상한 제거 항목만 갱신했다 |
| 10 | 이전 두-group 구현 PR #594와 혼동되는가 | #597에 한정된 제목과 링크를 붙였다 |
| 11 | historical plan-only 기록을 지워 당시 상태를 왜곡하는가 | 계획 당시 기록은 보존하고 runtime 최신 상태를 추가했다 |
| 12 | 계획-only 문구가 현재 상태로 읽히는가 | 공개 roadmap에서 “계획 당시 기록”으로 표시한다 |
| 13 | 공식 STATUS checkbox가 실수로 체크되는가 | 새 기록은 설명 행이며 체크 항목을 바꾸지 않았다 |
| 14 | 공식 완료 집계가 72에서 변하는가 | `72/213`을 유지한다고 세 문서에서 명시한다 |
| 15 | C11 전체가 완료로 승격되는가 | C11 및 C11.4 전체가 미완료라고 유지한다 |
| 16 | 제한 runtime slice를 공개 CSS admission으로 오해하는가 | opt-in 제한 범위와 일반 CSS 미지원 문구를 보존한다 |
| 17 | Android AVD 결과를 실기기 결과로 확대하는가 | software-rendered emulator 한계를 유지한다 |
| 18 | iOS Metal sample을 compositor 이후 화면으로 확대하는가 | compositor 제출 전 표본임을 유지한다 |
| 19 | PR merge row와 runtime evidence 사이 SHA가 다른가 | README·plan·STATUS·roadmap 모두 같은 SHA를 쓴다 |
| 20 | README의 merge section이 남은 범위를 삭제하는가 | section은 별도로 추가되고 unsupported scope 뒤에 남는다 |
| 21 | 계획의 merge metadata가 test count를 바꾸는가 | test 결과와 판정값은 그대로다 |
| 22 | public roadmap이 이전 plan-only 시점을 현재 진행 상태로 남기는가 | 기존 시점은 historical, 신규 항목은 PR #597 병합으로 표시한다 |
| 23 | public roadmap에서 screenshot asset 경로가 깨지는가 | 두 PNG 링크는 main에 병합된 파일명을 가리킨다 |
| 24 | HTML paragraph 중첩이나 닫힘 태그가 손상되는가 | 추가 링크가 기존 paragraph 내부에서 정상 닫힌다 |
| 25 | public route의 `.html` 링크가 Markdown 원본 경로를 노출하는가 | 기존 RSPress static route 규칙을 따른다 |
| 26 | review 링크가 아직 없는 파일을 가리키는가 | 본 PR에 추가되는 이 검토 파일과 일치한다 |
| 27 | 문서가 아직 수행 전인 Tailnet sync를 완료로 주장하는가 | Tailnet 제공 여부는 이 문서에서 완료 주장하지 않는다 |
| 28 | unrelated roadmap lane이나 C13 상태를 바꾸는가 | 변경은 해당 C11.4 기록에만 한정했다 |
| 29 | 내부 계약 version이 구현 추가로 오르는가 | 계약 0059의 `0.1.0`을 그대로 유지한다 |
| 30 | merge 이후 기록에서 남은 review·검증 한계가 빠지는가 | simulator·archive 근거와 제품 scope 한계를 계속 연결한다 |

검토에서 발견된 문서 동기화 결함은 남아 있지 않다. 변경은 PR #597 merge metadata를 source-of-truth와 미리보기에 전파하며 구현 지원 범위나 공식 완료 상태를 변경하지 않는다.
