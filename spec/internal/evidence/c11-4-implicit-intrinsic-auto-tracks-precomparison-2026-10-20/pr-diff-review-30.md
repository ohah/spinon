# C11.4 positive implicit track sizing 계획 PR 변경 적대 검토 30개

**대상:** 이 PR의 계획, C11.4 상태·공개 로드맵 기록, Chrome 사전 비교 fixture와 증거 파일 전체.

각 회차는 별개의 실패 경로, 근거 경계 또는 사용자 계약을 확인했다. 계획 자체의 20개 실패 관점 검토와는 별도다. 이 검토는 구현·앱 동작·Simulator·WPT 적합성을 검증했다고 주장하지 않는다.

| # | 적대 관점 | 확인 결과와 처리 |
| ---: | --- | --- |
| 1 | 계획 PR이 C11.4를 완료로 표시하는가 | `spec/STATUS.md`의 기존 unchecked C11.4 상태가 유지된다. diff에는 계획 링크만 추가했다. |
| 2 | 미완료 항목 수가 계획 PR에서 늘어나는가 | 상태 대장의 체크·집계 행을 바꾸지 않았고 공개 로드맵도 기존 완료 집계를 유지한다고 명시한다. |
| 3 | 공개 로드맵이 계획을 구현 완료처럼 소개하는가 | 새 문단에 계획·사전 비교 단계이며 앱 구현과 Android/iOS 실행 전이라고 썼다. |
| 4 | 새 링크가 계획·evidence·상태 대장의 다른 slice를 가리키는가 | public route와 markdown 상대 경로 모두 `c11-4-positive-implicit-track-sizing` 및 동일한 evidence 날짜 디렉터리를 가리킨다. |
| 5 | 계획이 다른 C11.4 계획을 대체하는 것으로 읽히는가 | 기존 intrinsic contribution·auto-repeat·placement 미완료를 유지하고 이 slice를 양의 implicit track 목록으로 한정한다. |
| 6 | 적용 명세가 모호한 최신 초안 링크인가 | CSS Grid Level 1의 2025 CRD 날짜와 §7.5/§7.6/§8.5/§11.3–§11.8 범위를 고정했다. |
| 7 | 규범의 track-size 목록이 단일 값으로 축소되는가 | non-empty `<track-size>+`를 보존하고 지원 문법은 기존 C11.4 typed projection에 한정했다. |
| 8 | implicit list 순서나 첫 positive index가 한 칸 이동하는가 | 첫 implicit track에 index 0을 쓰고 authored order로 modulo 순환한다고 명시했다. |
| 9 | rows와 columns가 공유 cursor/list로 섞이는가 | 두 축의 순서·목록·placement 흐름을 독립 입력과 독립 결과로 검증하게 했다. |
| 10 | 기존 converter가 지원하지 않는 문법을 새로 허용하는가 | 구현 계획은 현재 `GridTrackSize` projector가 거부하는 computed value를 계속 fail-closed로 둔다. |
| 11 | Stylo가 거부한 값이 `auto`로 조용히 복구되는가 | Unsupported 값을 diagnostic/error로 보존하고 silent fallback 금지를 적었다. |
| 12 | explicit positive line placement만 sizing되는가 | explicit placement가 만든 positive implicit row/column을 runtime matrix에 포함했다. |
| 13 | auto-placement만 sizing되고 explicit placement 경로가 빠지는가 | row/column auto-placement 별 장면과 경로별 Rust 테스트도 요구한다. |
| 14 | item이 explicit/implicit 경계를 span할 때 pattern 또는 occupancy가 틀리는가 | cross-boundary span, occupied range와 list 시작 위치를 별도 acceptance 항목으로 둔다. |
| 15 | negative implicit track을 forward cycle로 잘못 처리하는가 | negative track은 이번 adapter 표현 범위 밖이며 구체적 오류와 partial-output 금지를 요구한다. |
| 16 | auto-repeat container가 검증 없이 implicit sizing 지원에 포함되는가 | template auto-repeat과 implicit sizing의 조합은 명시적으로 후속 미완료 범위에 남겼다. |
| 17 | 구현 계획의 10,000 상한이 저장소 상수와 다른가 | 현재 layout/style의 `GRID_MAX_TRACKS` 및 `MAX_COMPUTED_GRID_TRACKS`가 모두 10,000이다. |
| 18 | size 합산 또는 index 계산 overflow가 truncated success가 되는가 | checked arithmetic, cap 경계와 cap+1 거부를 계획의 실패 검증에 넣었다. |
| 19 | allocation 오류 범위가 Spinon 코드의 보장보다 넓게 읽히는가 | 계획을 수정해 fallible reservation 범위를 Spinon 소유 입력·목록·출력으로 한정했다. |
| 20 | vendored Taffy 내부 allocation까지 복구한다고 오인하는가 | Taffy 내부 일반 allocation은 `LayoutError` 변환·전역 OOM 복구 보장에서 제외한다고 명시했다. |
| 21 | 실패 뒤 부분 layout이 성공 결과로 게시되는가 | 계산/지원/상한/검증 가능한 reservation 오류에서 성공 output을 게시하지 않는 조건을 넣었다. |
| 22 | C11.2.2/2.3 또는 TemplateGrid가 의도치 않게 확장되는가 | 변경 대상은 `RuntimeBlockFlexGridTrackSizingV1`으로 제한하고 기존 profile 오류 회귀를 요구한다. |
| 23 | 공개 API 추가 또는 내부 계약 버전 상승을 암시하는가 | 새 공개 API를 만들지 않고 계약 0059의 `0.1.0`을 유지하도록 계획했다. |
| 24 | fixture의 18개 case ID가 누락·중복되거나 JSON이 불일치하는가 | 관찰·기대값 모두 18개이며 ID는 각각 18개 고유, 같은 ID의 기대 track 문자열은 18/18 일치한다. |
| 25 | Chrome fixture에서 Ahem 상대 경로가 틀려 다른 폰트로 측정되는가 | `tests/fixtures/css/c11/../../resources/c15/Ahem.ttf`가 저장소 fixture 위치와 일치하고 환경 자료가 font loaded를 기록한다. |
| 26 | Ahem 파일 또는 입력 fixture가 바뀌었는데 해시가 낡았는가 | SHA256SUMS의 fixture·font·환경·관찰·화면·기대값·규범 capture가 모두 `OK`다. |
| 27 | 브라우저/OS/DPR 차이가 기대값에 섞여 재현이 안 되는가 | Chrome 155.0.8059.85, Agent Browser 0.31.1, macOS 26.5.1, viewport 1280×633, DPR 1을 보존한다. |
| 28 | 기대값이 Spinon 구현 결과에서 역산되어 circular oracle이 되는가 | 기대값은 별도 JSON과 CSS Grid 규범 reasoning에 두고 Chrome 관찰 JSON을 독립 대조했다. |
| 29 | overflow 경로를 track geometry만 맞춘 뒤 paint 검증까지 통과했다고 확대 해석하는가 | 44px border-box와 48px outside geometry만 관찰했고 Chrome pixel sample 미측정을 명시했다. 앱의 border 안·밖 표면 검증은 구현 acceptance에 남겼다. |
| 30 | 계획 PR을 기능·Simulator·WPT 또는 Tailnet 완료로 보고하는가 | 문서·Chrome precomparison 결과에만 한정하고 앱 구현, 71-scene Simulator, artifact, CI, Tailnet sync는 후속 검증으로 분리했다. |

## 추가 검증과 미실행 항목

- `git diff --cached --check` 통과.
- Chrome 관찰과 규범-derived 기대값의 unique ID/track-list 대조 18/18 통과.
- `SHA256SUMS` 파일 7개 모두 통과.
- 이 worktree에는 `node_modules`가 없어 Rspress 문서 빌드를 실행하지 않았다. PR의 최신 check와 실제 GitHub 렌더링을 확인한다.
- 계획 PR 변경 검토에서 project rule의 PR 30회 요구를 확인해 계획의 PR-diff 검토 수를 20에서 30으로 수정했다.
