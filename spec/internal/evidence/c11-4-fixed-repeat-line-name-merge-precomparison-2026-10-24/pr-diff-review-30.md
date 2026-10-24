# C11.4 fixed-repeat line-name merge 계획 PR 변경분 적대 검토 30개 · 2026-10-24

대상은 계획 PR의 전체 변경분이다: 계획·계획 review, Chrome fixture/observation/독립 기대값/validator/screenshot, 부모 계획의 남은 범위 링크와 `spec/STATUS.md`의 plan-only 행. 각 항목은 변경된 산출물을 서로 다른 실패 경로에서 다시 읽었다. 계획 20관점 검토와 겹치지 않도록 diff 정확성·링크·표시·상태 원장을 중심으로 확인했다.

| # | diff 실패 관점 | 확인과 결과 | 판정 |
| ---: | --- | --- | --- |
| 1 | repeat 문법 anchor 오기 | Agent Browser에서 W3C 2025 CRD heading ID `repeat-syntax`를 확인했다. | 링크가 맞다. 통과. |
| 2 | named-line 규범 anchor 오기 | W3C heading ID `common-uses-named-lines`를 확인했다. | 링크가 맞다. 통과. |
| 3 | placement 규범 anchor 오기 | W3C heading ID `line-placement`를 확인했다. | 링크가 맞다. 통과. |
| 4 | auto-repeat 횟수 한도 누락 | 계획의 one-auto-repeat 범위와 fixture CSS를 대조했다. | second auto-repeat은 unsupported 경계로 남는다. 통과. |
| 5 | fixed-repeat count/track 제한 변경 | #597 계획·코드와 계획 scope를 대조했다. | positive integer, 1–2 track breadth profile과 10,000 경계를 유지한다. 통과. |
| 6 | ordinary `TrackSize` boundary 미검증 | `col-ordinary-join` scene의 8px ordinary track과 line groups를 관찰 JSON에서 확인했다. | ordinary track 경계 fixture가 추가돼 있다. 통과. |
| 7 | authored component 순서 | CSS source, CSSOM serialization, expected vector를 순서별로 대조했다. | fixed prefix·ordinary neighbor·auto repeat·suffix의 순서가 보존된다. 통과. |
| 8 | fixed repeat 내부 반복 seam | CSSOM `[join start]` group과 seam list를 대조했다. | end→다음 start 병합을 관찰값으로 고정했다. 통과. |
| 9 | 두 fixed group 사이 collision | `[join join mid]`의 세 source 목록을 fixture와 대조했다. | duplicate token과 authored order가 남는다. 통과. |
| 10 | fixed/auto 시작 경계 | `[mid-end seam seam]`이 fixed end, outer list, auto start의 조합인지 확인했다. | 세 이름 목록의 경계가 분명하다. 통과. |
| 11 | auto-repeat 내부 경계 | `[repeat-end seam]` CSSOM group이 반복마다 생성되는지 확인했다. | auto-repeat의 N+1 metadata와 일치한다. 통과. |
| 12 | auto/fixed 끝 collision | `[repeat-end repeat-end join]`과 suffix 시작을 확인했다. | auto end, outer list, fixed start가 하나의 물리 line이다. 통과. |
| 13 | line token 조기 dedup | observation JSON에 duplicate `seam`과 `repeat-end`가 남는지 확인했다. | source token 목록을 보존한다. 통과. |
| 14 | duplicate token이 occurrence를 소비 | `seam 5/6` raw placement와 geometry를 확인했다. | 중복 token을 별도 occurrence로 계산하지 않는다. 통과. |
| 15 | positive occurrence off-by-one | start/join, seam, mid, repeat-end item rect를 독립 기대값과 대조했다. | 좌표가 모두 exact CSS px로 맞는다. 통과. |
| 16 | negative occurrence implicit-line 오염 | `seam -3/-2`, `tail -2/-1`이 명시 grid 경계 안에서 resolves되는지 확인했다. | expected rectangles와 일치한다. 통과. |
| 17 | named span 방향·기산점 | `span 2 mid / mid-end 2`의 resolved area와 `(56,0,36,40)`을 확인했다. | named span oracle이 포함돼 있다. 통과. |
| 18 | auto-fill sizing 산식 | widths, gaps, ordinary count를 원본 fixture에서 다시 합산했다. | `120+28n`: 6회 288px, 7회 316px이라 300px 축은 6회다. 통과. |
| 19 | auto-fit placement/collapse | `0,0,0,24,0,0px` serialization과 retained item rect를 확인했다. | fourth repeated track만 남는다. 통과. |
| 20 | row-axis projection 누락 | rows scene의 track list와 seam/tail item geometry를 별도 검토했다. | row 16-track order와 `(0,96,40,24)`, `(0,278,40,10)`이 기대와 일치한다. 통과. |
| 21 | ordinary neighbor auto-repeat 산식 | 120px scene의 3 fixed tracks, gap, 12px minimum을 다시 계산했다. | `36+16n`: 5회 116px, 6회 132px이라 5회가 맞다. 통과. |
| 22 | expected rectangles가 Chrome에서 복사됨 | track starts·gaps에서 독립 계산한 기대값과 observation JSON을 validator로 비교했다. | 4 scene 모두 통과; expectation JSON이 oracle output으로 생성되지 않는다. 통과. |
| 23 | validator scene 누락/추가 | validator가 기대 scene ID 집합, axis별 sizes, item IDs를 검사하는지 읽고 실행했다. | scene set·track sequence·placement를 모두 검사한다. 통과. |
| 24 | CSSOM parsing validator 약화 | integer `px` parser가 모든 fixture track을 읽고 duplicate merged groups를 검사하는지 확인했다. | 현재 fixture 수치에 대해 16/8 track vectors와 merged groups가 통과한다. 통과. |
| 25 | metadata vector cardinality 오류 | 계획의 `non-auto tracks + 2`와 현재 auto-repeat layout validator 식을 대조했다. | Taffy component boundary 개수와 맞는다. 통과. |
| 26 | memory/overflow 과장 | Stylo 1 MiB cascade, layout source 1 MiB, expanded 16 MiB, 10,000 track constants를 소스에서 확인했다. | 기존 한도를 유지하고 checked arithmetic을 요구한다. 통과. |
| 27 | OOM 보장 과대 주장 | Spinon-owned fallible allocations와 vendored Taffy infallible working vectors를 대조했다. | plan이 vendor/global OOM 복구를 명시적으로 제외한다. 통과. |
| 28 | screenshot/observation revision mismatch | fixture source hash, JSON UA/viewport, screenshot source, scene 순서를 확인했다. | HeadlessChrome 156, 1280×633 DPR1; screenshot은 scene만 보이고 observation JSON은 별도 보존된다. 통과. |
| 29 | 공식 C11 상태 승격 | `spec/STATUS.md`, top-level C11 checkbox와 aggregate count를 다시 확인했다. | 새 행은 plan-only이며 C11 `[ ]`, `72/213`, contract `0.1.0`을 변경하지 않는다. 통과. |
| 30 | 리뷰/PR 문서 self-consistency | 별도 plan review 20, 이 diff review 30, README/상대 링크와 PR image source 문구를 대조했다. | 서로 다른 증거를 연결하며 Chrome 화면을 앱 화면으로 부르지 않는다. 통과. |

## 결과

30개의 diff 관점에서 변경은 계획·oracle·상태 기록에 한정된다. CSS fixture validator, JSON parse, artifact SHA-256 대조와 `git diff --check`가 통과했다. 제품 구현·Android/iOS app runtime은 이 계획 PR에서 변경하거나 검증하지 않았다. PR screenshot은 Chrome oracle로 명시해 표시한다.
