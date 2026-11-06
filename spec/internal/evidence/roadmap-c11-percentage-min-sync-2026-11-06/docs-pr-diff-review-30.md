# C11 #1220 상태·로드맵 동기화 PR diff 실패 관점 검토 30개

대상은 C11 plan 4개, `spec/STATUS.md`, 내부 계약 0059, 공개 `roadmap.html`, 이 evidence 디렉터리의 변경이다. 각 관점은 별도 실패 경로 또는 상태 경계를 확인했다.

| # | 실패 관점 | 결과 |
|---:|---|---|
| 1 | merge timestamp가 PR metadata와 다른가 | 통과 — `2026-11-06T12:13:13Z`로 통일했다. |
| 2 | merge SHA가 로컬 `HEAD` 및 GitHub PR과 다른가 | 통과 — `2e022c9589eacb49ec2d04c5ea0dd1cb3ca575af`를 확인했다. |
| 3 | 구현 PR을 draft/open 상태로 남기는가 | 통과 — #1220 Rebase and merge 완료로 기록했다. |
| 4 | PR checks가 없는 상태를 CI green으로 과장하는가 | 통과 — check rollup과 required checks가 비어 있다고 기록했다. |
| 5 | 기본 Root HTML profile과 C11.4 opt-in fixture를 혼동하는가 | 통과 — 두 경로를 구분했다. |
| 6 | auto-fill만 적고 auto-fit을 빠뜨리는가 | 통과 — 둘 다 제한 지원 범위에 포함했다. |
| 7 | columns만 지원한다고 과소/과대 표기하는가 | 통과 — definite columns와 rows를 적었다. |
| 8 | pattern이 1–2 track이라는 상한을 잃는가 | 통과 — 단일/2-track 제한을 반복했다. |
| 9 | percentage maximum까지 지원한다고 읽히는가 | 통과 — percentage minimum만 허용하고 maximum 제한을 적었다. |
| 10 | 0% count floor가 actual track sizing에 새는가 | 통과 — 1px floor는 count 전용이라고 기록했다. |
| 11 | typed percentage가 px로 대체되는가 | 통과 — Taffy sizing 입력에 typed value를 보존한다고 기록했다. |
| 12 | 100% 초과 값이 clamp되는가 | 통과 — authored minimum을 clamp하지 않는다고 적었다. |
| 13 | indefinite percentage basis를 지원으로 오해하는가 | 통과 — fail-closed 경계를 유지했다. |
| 14 | calc, auto/max-content minimum이 지원으로 보이는가 | 통과 — 지원 밖 grammar로 적었다. |
| 15 | named line, neighbor, area, two-axis repeat가 열리는가 | 통과 — 모두 기존 거부 범위로 남겼다. |
| 16 | 10,000-track quota가 조용히 clamp되는가 | 통과 — quota 초과 거부를 보존했다. |
| 17 | broad `spinon-style` suite를 green으로 표시하는가 | 통과 — 27/29와 두 실패를 정확히 기록했다. |
| 18 | focused tests의 package/filter 범위가 흐려지는가 | 통과 — layout 11, projection 15, cascade 1, runtime 1을 구분했다. |
| 19 | 기존 C11.4 regressions 누락 또는 green 오표기가 있는가 | 통과 — layout 13/13 및 cascade 6/6으로 구분했다. |
| 20 | Android AVD를 실기기로 표기하는가 | 통과 — API 37 ARM64 AVD/ANGLE SwiftShader라고 명시했다. |
| 21 | iOS drawable을 compositor 이후 pixel이라고 부르는가 | 통과 — compositor 제출 전이라고 명시했다. |
| 22 | simulator 화면을 pixel parity 근거로 확대하는가 | 통과 — visual smoke 한계를 기록했다. |
| 23 | WPT 적합성 또는 일반 CSS admission을 암시하는가 | 통과 — 검증하지 않았다고 명시했다. |
| 24 | C11.1 또는 C11을 완료로 체크하는가 | 통과 — 공식 체크박스를 바꾸지 않았다. |
| 25 | 공식 전체 완료 집계를 증가시키는가 | 통과 — `78/214` 유지다. |
| 26 | 전체 집계를 C11 진행률로 표현하는가 | 통과 — repo-wide count라고 명시했다. |
| 27 | 내부 계약 numeric version을 올리는가 | 통과 — `0.1.0` 유지다. |
| 28 | PR #1216을 #1220과 뒤섞거나 누락하는가 | 수정 후 통과 — #1217 Tailnet sync evidence가 #1216 하위에 남도록 들여쓰기 계층을 확인했다. |
| 29 | root auto-repeat index가 현재 범위보다 오래된 설명을 유지하는가 | 수정 후 통과 — #1216/#1220을 후속으로 연결했다. |
| 30 | public roadmap 최신 카드가 순서·링크·모바일 폭에서 잘못 표시되는가 | 통과 — #1220을 최신 카드로 배치하고 Agent Browser desktop/mobile을 확인했다. |

문서 검토 결과 C11 completion/count 승격, CI 과장, 플랫폼 증거 확대 해석은 없었다. 변경 문서의 상태와 로컬 공개 roadmap 렌더링이 merge metadata와 일치한다.
