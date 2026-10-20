# Tailnet roadmap 동기화 문서 PR diff 실패 관점 검토 30개

| # | 실패 관점 | 확인 및 판정 |
| ---: | --- | --- |
| 1 | C04.12 parent가 하위 작업 때문에 완료 처리되는가 | STATUS parent checkbox는 unchecked로 유지한다. |
| 2 | 공식 완료 집계가 부풀려지는가 | 상위 집계 대상은 수정하지 않는다. |
| 3 | 구현 subtask만 완료 체크됐는가 | STATUS에서 page-scale runtime mapping 하위 행만 checked다. |
| 4 | 구현 동작이 merge 사실보다 과장되는가 | 결과는 기본 root HTML GPU host의 초기 scene mapping으로 한정한다. |
| 5 | scale 값이 DPR과 혼동되는가 | 동일 environment revision의 DPR과 `page_scale`을 별도 입력으로 적는다. |
| 6 | 변환이 모든 runtime GPU host에 적용된다고 읽히는가 | root HTML default GPU host에만 적용한다고 명시한다. |
| 7 | CSS layout viewport가 변환되는가 | CSS layout frame과 hit-test 좌표가 유지됨을 적는다. |
| 8 | Canvas clear가 page scale을 따라 축소되는가 | clear와 physical surface 경계가 유지됨을 적는다. |
| 9 | transform 적용 시점이 모호한가 | scene publication 전에 한 번 적용한다고 기록한다. |
| 10 | stale revision 또는 invalid mapping의 거부 조건이 빠졌는가 | 누락, revision mismatch, overflow/underflow fail-closed 조건을 유지한다. |
| 11 | 지원 viewport case가 달라지는가 | default, scale 0.5, scale 2, width 500을 동일하게 기록한다. |
| 12 | platform evidence를 physical device로 과장하는가 | Android API 37 AVD와 iOS Simulator라고 명시한다. |
| 13 | iOS compositor 이후 픽셀을 주장하는가 | compositor 이후 readback은 미포함으로 유지한다. |
| 14 | Android hardware GPU 검증으로 표현되는가 | AVD/SwiftShader 범위로 한정한다. |
| 15 | pinch, pan, scroll 기능을 포함한다고 읽히는가 | 해당 visual viewport 동작은 남은 범위로 적는다. |
| 16 | native pointer/touch 입력 변환이 지원된다고 읽히는가 | native input remapping 미지원 경계를 유지한다. |
| 17 | glyph가 확대 후 재래스터된다고 읽히는가 | text rerasterization은 미지원으로 둔다. |
| 18 | 전체 HTML/CSS 호환을 주장하는가 | 일반 HTML/CSS 미지원 경계를 유지한다. |
| 19 | 내부 계약 version을 바꾸는가 | 0077/0081은 기존 0.1.0을 유지한다. |
| 20 | 새 공개 API 추가로 오해되는가 | 새 공개 API를 선언하지 않는다. |
| 21 | PR merge SHA가 live GitHub 값과 다른가 | `6c70bdda6270721f06c0374722211480a948878b`로 대조한다. |
| 22 | merge 시간이 UTC로 오기되는가 | PR metadata의 `2026-10-20T11:17:38Z`를 사용한다. |
| 23 | PR이 squash/merge commit 방식으로 기록되는가 | GitHub event는 Rebase and merge다. |
| 24 | review 30회 근거가 빠지는가 | STATUS와 roadmap에서 review 파일로 연결한다. |
| 25 | implementation review와 plan review 링크가 혼동되는가 | 둘을 기존 별도 evidence 경로로 연결한다. |
| 26 | PR checks가 실행 또는 통과했다고 과장되는가 | rollup empty와 required status check 부재를 기록한다. |
| 27 | branch protection의 PR 요구가 누락되는가 | post-merge review 기록에 CLEAN/PR·linear-history 조건을 보존한다. |
| 28 | Tailnet roadmap과 STATUS가 서로 다른 merge state를 보이는가 | 두 preview route에서 #388과 parent 미완료를 대조한다. |
| 29 | Markdown 상대 링크가 빌드 이후 깨지는가 | 선택 Rspress route와 실제 Agent Browser navigation으로 확인한다. |
| 30 | 문서 변경이 시뮬레이터 재검증까지 했다고 오해되는가 | sync report가 문서 preview 갱신과 앱 검증을 분리한다. |

## 판정

30개 관점에서 문서·route·범위 변경을 검토하고, PR metadata 및 실제 Tailnet 제공 바이트와 함께 대조한다. 이 diff 검토는 기존 PR #388 구현 review를 대신하거나 재사용하지 않는다.
