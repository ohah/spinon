# C11.4 auto-repeat + template areas 계획 PR diff 적대 검토 30개

**검토 범위:** 집중 계획, 사전 비교 README/JSON/PNG/SHA manifest, 13-scene fixture, 계획 실패 관점 20개 기록, `spec/STATUS.md`, 상위 계획, 공개 roadmap 카드.

**판정:** 아래의 서로 다른 30개 실패 경로를 확인했다. 확인 과정에서 발견한 `explicit_tracks` 의미 오류는 수정했다. 계획 PR 검토이며 구현·runtime 검증을 뜻하지 않는다.

| # | 실패 관점 | diff와 근거에서 확인한 방어선 | 판정 |
| --- | --- | --- | --- |
| 1 | 미완료 C11.4 checkbox가 실수로 완료된다. | `spec/STATUS.md`의 C11.4 checkbox는 미완료로 남는다. | 통과 |
| 2 | 구현 전 계획을 완료 수에 더한다. | 상태 대장·roadmap 모두 `77/213`을 유지한다. | 통과 |
| 3 | plan-only 결과가 제품 기능으로 소개된다. | 계획, 상태 대장, roadmap에서 구현/runtime이 시작되지 않았다고 분리한다. | 통과 |
| 4 | 이전 20개 검토가 틀린 계획에 묶여 남는다. | 큰 area 처리 경계를 고친 뒤 계획 실패 관점 20개를 다시 검토했다. | 통과 |
| 5 | 규범 line alias 계약에 근거가 없다. | CSS Grid 2025 CRD §7.2.3.2, §7.3.2, §8.3–§8.4를 연결한다. | 통과 |
| 6 | Browser 결과가 어느 실행인지 재현할 수 없다. | Chrome for Testing 156.0.8078.48, Agent Browser 0.31.1, macOS ARM64를 기록한다. | 통과 |
| 7 | device scale이나 viewport 차이가 frame 해석을 바꾼다. | `1280×633`, DPR 1, `ltr`, `horizontal-tb`를 고정한다. | 통과 |
| 8 | README의 scene 수가 fixture/JSON과 다르다. | fixture ID, 결과 JSON, 문서에서 13개 scene을 대조했다. | 통과 |
| 9 | row-axis alias가 column 결과만 따라 잘못 계산된다. | fill/fit/shorter/larger/empty 및 numeric control을 양 축에 둔다. | 통과 |
| 10 | `auto-fill` 반복 수가 gap을 제외해 1개 어긋난다. | 116px scene의 반복 수 식 `20 + 3×20 + 20 + 4×4`와 frame을 둔다. | 통과 |
| 11 | 다른 container 폭의 반복 수가 같은 산식으로 검증되지 않는다. | 140px scene에서 4개 반복을 별도로 고정한다. | 통과 |
| 12 | 100px 대조에서 auto-repeat을 3개로 계산한다. | 2개는 92px에 맞고 3개는 116px가 필요하다는 산술을 기록한다. | 통과 |
| 13 | auto-fit collapse 뒤 gutter 4px가 남는 Chrome 결과를 규범값으로 복사한다. | Chrome 48/24px와 규범 44/20px를 fixture/계획에서 분리한다. | 통과 |
| 14 | gutter 차이 원인을 area alias로 잘못 단정한다. | 같은 auto-fit geometry의 areas 없는 numeric controls에서 동일 차이를 확인했다. | 통과 |
| 15 | area dimension이 template보다 짧을 때 alias를 repeat 끝으로 이동한다. | shorter scene 뒤의 numeric tail이 area alias와 마지막 fixed track을 분리한다. | 통과 |
| 16 | area가 template보다 클 때 확장된 explicit grid에서 track이 빠진다. | 양 축 area-larger 장면의 5번째 explicit track과 suffix frame을 고정한다. | 통과 |
| 17 | area-added track이 잘못된 크기로 조용히 생성된다. | area-larger 입력은 `grid-auto-columns/rows: 20px`로 크기를 고정한다. | 통과 |
| 18 | root 바깥 frame이 overflow 범위와 맞지 않는다. | root 100px, suffix start 96px, suffix width 20px로 16px overflow를 계산한다. | 통과 |
| 19 | 고정 1280px screenshot에서 area-larger scene이 viewport 밖으로 잘린다. | 가장 오른쪽 column scene의 끝은 x=1176px이고 viewport 안에 있다. | 통과 |
| 20 | `auto-fit` area name만으로 repeated track이 occupied 처리된다. | hero child가 없는 양 축 empty-area 장면에서 반복 track 모두의 collapse를 기대한다. | 통과 |
| 21 | area alias와 authored `hero-start`/`hero-end`가 같은 occurrence로 합쳐진다. | duplicate authored names와 area edges를 함께 둔 collision scene을 기록한다. | 통과 |
| 22 | bare `grid-area: hero`와 named-line placement가 구분되지 않는다. | bare area frame과 `hero-start 2 / hero-end 1` probe frame을 따로 보존한다. | 통과 |
| 23 | shorter-area scene에서 negative numeric line 대조가 빠진다. | tail item이 `-2 / -1`로 마지막 explicit track을 측정한다. | 통과 |
| 24 | CSSOM 20px track list가 각 track의 출처까지 증명한다고 오해한다. | README는 CSSOM 관측과 독립 반복 산술을 구분하고 runtime Taffy trace를 요구한다. | 통과 |
| 25 | `DetailedGridInfo.explicit_tracks`를 area 크기 거부 신호로 사용한다. | Taffy 0.15.0의 count가 `max(template, area)`임을 적고 최종 count 대조에만 쓴다. | 통과 |
| 26 | Taffy가 최대 track 수를 clamp해 area tail을 잘라도 성공으로 게시한다. | grid-coordinate limit을 넘는 입력은 layout publish 전 fail-closed하도록 계획한다. | 통과 |
| 27 | Spinon이 Taffy의 auto-repeat 수 계산을 따로 재구현한다. | fill-count 산식은 진단 산술로만 쓰고 구현은 pinned Taffy 경로를 재사용한다. | 통과 |
| 28 | 새 area 전달 경계가 no-repeat C11.3 동작을 바꾼다. | 기존 C11.3 projection과 no-repeat dimension equality를 별도 회귀한다. | 통과 |
| 29 | String/vector allocation 실패 후 부분 LayoutOutput이 노출된다. | 계획에 checked conversion, fallible reservation, error/no-layout 회귀를 넣었다. | 통과 |
| 30 | Chrome screenshot을 simulator·WPT·제품 renderer 결과로 확대 해석한다. | README와 roadmap은 Chrome oracle only이며 app draw, border, simulator, WPT를 검증하지 않는다고 쓴다. manifest의 세 SHA는 실제 파일과 일치한다. | 통과 |

## 수정한 발견

초기 계획은 `explicit_tracks`를 template count로 오인해 template보다 큰 area를 거부하려 했다. pinned Taffy source에서 count는 `max(expanded template count, area dimension)`이고 추가 area track에는 `grid-auto-*` sizing이 적용되는 것을 확인했다. 계획을 area-larger 양성 지원으로 고쳤고, 100px root의 column/row scene 두 개를 Chrome 156에서 재관측해 20px 추가 track과 16px overflow를 확인했다. 최대 grid coordinate 초과만 별도의 fail-closed 경계로 남겼다.
