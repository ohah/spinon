# C11.4 fixed-repeat 후보 로드맵 검토 링크 보정 · PR diff 실패 관점 30개

**검토 범위:** 병합 PR #589 뒤 Agent Browser에서 발견한 공개 로드맵·상태 대장의 최신 수정 PR diff 검토 링크 누락을 고친다. 변경은 `spec/public/roadmap.html`과 `spec/STATUS.md`의 문서 링크 및 이 검토 기록뿐이다. 기능 코드, 지원 범위, C11.4 완료 상태는 바꾸지 않는다.

| # | 독립 실패 관점 | 보정·검증 기준 |
| --- | --- | --- |
| 1 | 공개 로드맵이 수정 PR diff 검토 30개 페이지를 가리키지 않을 수 있음 | fixed-repeat 계획 항목에 새 review route를 추가하고 Agent Browser에서 확인한다. |
| 2 | 상태 대장도 같은 링크를 놓칠 수 있음 | C11.4 후보 paragraph에 저장소 내 markdown 링크를 추가한다. |
| 3 | 링크 filename이 실제 파일명과 다를 수 있음 | `revision-pr-diff-review-30.md` 경로를 실제 repository 파일과 대조한다. |
| 4 | RSPress route에서 `.html` suffix가 빠지거나 틀릴 수 있음 | public roadmap route에 `/spinon/plan/...revision-pr-diff-review-30.html`을 사용한다. |
| 5 | `revision-review-20` page로 잘못 연결될 수 있음 | browser에서 page title이 PR diff 30개인지 확인한다. |
| 6 | 원래 계획 PR diff 검토 링크를 새 링크로 덮어쓸 수 있음 | 기존 `...pr-diff-review-30`와 새 `...revision-pr-diff-review-30` 둘 다 유지한다. |
| 7 | 계획 자체의 수정 검토 20개 링크를 잃을 수 있음 | 수정 계획 review-20 링크도 각각 상태 대장과 roadmap에 보존한다. |
| 8 | Chrome oracle evidence 링크가 제거될 수 있음 | Chrome README와 screenshot 링크를 그대로 둔다. |
| 9 | 링크 누락 수정이 기능을 지원한다고 보일 수 있음 | 후보는 계획/oracle 단계이고 기존 adapter는 혼합 repeat을 거부한다고 유지한다. |
| 10 | C11.4 checkbox가 실수로 체크될 수 있음 | `spec/STATUS.md`의 체크 상태를 비교 전후 동일하게 둔다. |
| 11 | 공식 완료 집계가 바뀔 수 있음 | public roadmap과 status의 공식 `72/213`을 그대로 둔다. |
| 12 | 내부 계약 버전이 범위 수정 때문에 바뀔 수 있음 | 0059 `0.1.0`을 변경하지 않는다. |
| 13 | 새로운 API 지원을 주장할 수 있음 | diff는 링크만 추가하고 API/interpreter 문서는 변경하지 않는다. |
| 14 | status와 roadmap의 C11.4 계획 서술이 달라질 수 있음 | 두 곳의 계산형 경계, 9-scene 수, 계획-only 상태를 유지한다. |
| 15 | Korean label이 기존 문장 흐름과 혼재해 구분되지 않을 수 있음 | `수정 PR diff 검토 30개`로 명시해 기존 계획 PR review와 구별한다. |
| 16 | public URL에 내부 상대경로를 잘못 사용할 수 있음 | public HTML은 `/spinon/plan/...html` absolute route를 사용한다. |
| 17 | status markdown 링크가 문서 루트 기준에서 깨질 수 있음 | `../plan/...md` 상대 링크를 실제 `spec/STATUS.md` 위치에서 확인한다. |
| 18 | page title은 맞지만 링크가 stale cache route로 갈 수 있음 | 고유 query parameter 없이 canonical route로 직접 열어 제목과 30행을 확인한다. |
| 19 | 새 link가 중복되어 roadmap 문장이 반복될 수 있음 | fixed-repeat plan bullet 안에 link를 1회만 추가한다. |
| 20 | 외부 W3C 규범 링크를 덮어쓸 수 있음 | CSS Grid 2025 CRD 링크를 그대로 보존한다. |
| 21 | 수정 PR review의 역할을 구현 review로 혼동할 수 있음 | 페이지 제목과 body는 PR diff 검토임을 분명히 하고 simulator 근거를 주장하지 않는다. |
| 22 | `LengthPx` normalization scope가 authored-unit whitelist로 넓어질 수 있음 | 기존 computed-type 경계와 constant absolute calc 설명을 보존한다. |
| 23 | percentage/unresolved Calc 제외 조건을 지울 수 있음 | roadmap 문구를 변경하지 않아 원래 제외 범위를 그대로 둔다. |
| 24 | 9-scene Chrome evidence 수가 다를 수 있음 | roadmap의 9-scene 표기를 수정하지 않고 그대로 유지한다. |
| 25 | screenshot이 새 review page처럼 보일 수 있음 | 현재 Chrome screenshot과 README route를 변경하지 않는다. |
| 26 | 계획 수정·계획 변경 PR review page의 경로 관계를 깨뜨릴 수 있음 | 계획에서 연결된 revision-pr-diff route와 실제 generated RSPress page를 대조한다. |
| 27 | `spec/STATUS.md`의 긴 C11.4 row에서 다른 항목을 덮어쓸 수 있음 | 정확히 하나의 링크 anchor만 삽입하고 `git diff --check` 및 변경 범위를 검토한다. |
| 28 | 전체 public roadmap에 불필요한 unrelated changes가 들어올 수 있음 | 고정-repeat bullet의 한 링크만 추가했는지 staged diff로 확인한다. |
| 29 | 문서 build는 통과해도 실제 화면에 새 route가 보이지 않을 수 있음 | RSPress 2.0.22 선택 build 후 Agent Browser에서 공개 roadmap route의 새 링크, 상태 대장 링크와 30행 review 페이지를 확인했다. |
| 30 | 수정 후 status·roadmap hash가 이전 값으로 남을 수 있음 | status·roadmap·이번 review 문서를 포함해 evidence SHA256SUMS를 갱신했고 15개 전체 `shasum -c`가 통과했다. |

## 결론

이 후속 PR은 #589 뒤 확인한 링크 누락만 해결한다. 선택 build, Agent Browser route/30행, 15개 SHA 검증을 마쳤다. 기능 구현, simulator, WPT와 C11.4 완료 상태에는 영향을 주지 않는다.
