# C11.1 `grid-auto-rows: fr` 계획 실패 관점 검토 20개

검토 대상은 [전용 구현 계획](./c11-root-html-grid-auto-row-fr.md), Chrome 156 precomparison 및 expected JSON이다. 이 목록은 구현/runtime 검토나 계획 PR diff 검토를 대신하지 않는다.

1. **Profile 탈출:** 새 값을 일반 CSS 경로 전체에 허용할 위험. 기본 `RuntimeRootHtmlTextPaintV1`와 `RootHtmlImplicitRowsV1`만 명시했다.
2. **px scalar 회귀:** 기존 단일 px representation이 track list로 바뀔 위험. `GridAutoRows::LengthPx`를 유지한다고 규정했다.
3. **scalar fr cascade:** `1fr` computed value가 TrackList에 들어오는지 놓칠 위험. projection 관문에서 scalar와 multi-track 입력을 따로 확인한다.
4. **목록 종류 혼합:** px/%/fr 혼합 중 한 종류를 후속 projection에서 버릴 위험. expected values와 수락 범위가 각 단위 및 순서를 보존하도록 고정한다.
5. **0fr:** 유효한 zero factor를 음수·0 금지 검사와 혼동할 위험. Chrome scene에 `0fr`과 nonzero automatic minimum을 넣었다.
6. **음수·비유한 factor:** layout 내부 typed input이 invalid 값을 통과할 위험. 구현 관문에 negative, NaN, infinity rejection을 요구한다.
7. **indefinite height:** fr track이 content sizing으로 조용히 넘어갈 위험. `%`/`fr` 목록에 finite definite root height를 요구한다.
8. **height override:** min/max-height 또는 aspect-ratio가 basis/available space를 바꿀 위험. 해당 값을 unsupported로 유지했다.
9. **percentage basis:** row gap을 percentage basis에서 차감할 위험. basis는 definite content height로 고정하고 gap을 분리했다.
10. **flex available space:** fixed rows와 gutters를 남는 공간에서 빼지 않을 위험. mixed scene에서 20px flex track을 사전 계산했다.
11. **explicit/implicit 경계:** 목록 순환을 explicit row부터 시작하거나 explicit row를 덮을 위험. 첫 implicit row와 explicit row 비순환을 수락/검증 기준에 적었다.
12. **pattern cycle:** 필요한 implicit 수가 목록 길이를 넘을 때 값 순서가 바뀔 위험. 4개 implicit row 및 3-entry cycle scene을 고정했다.
13. **subunit factors:** factor 합이 1 미만일 때 남는 공간을 잘못 채울 위험. `0.25fr` 반복 scene이 35px trailing space를 기대한다.
14. **automatic minimum:** fr track이 fixed item 최소 크기보다 작아질 위험. `0fr`과 별도 `1fr` overflow scene에서 이를 관찰한다.
15. **zero basis:** definite `0px` height를 indefinite로 오인할 위험. zero scene의 used rows `[0,0]`을 기록했다.
16. **percentage overflow:** 120% implicit row를 100%나 root height로 clamp할 위험. 120px used row와 50px root overflow를 oracle에 명시했다.
17. **aggregate geometry:** gap/px/% extent가 f32 범위를 넘은 뒤 partial frame이 나올 위험. implementation 관문에서 extent, quota 및 publication 전 rejection을 요구한다.
18. **quota/allocation:** 큰 pattern이나 expanded rows가 allocation 전에 제한되지 않을 위험. `GRID_MAX_TRACKS`, reservation 실패, no-partial-output 회귀를 포함했다.
19. **cascade fallback:** syntax-invalid CSS fallback과 typed-but-unsupported value를 같은 진단으로 취급할 위험. 둘을 별도 검증하도록 했다.
20. **runtime/platform 과장:** Rust unit test나 screenshot을 제품 route/표면 pixel parity로 부를 위험. 기본 app route, Android/iOS Simulator, draw/present, readback 한계를 분리해 기록한다.

## 판정

각 위험에 대응하는 범위 또는 실행 관문이 계획에 있다. 사전 Chrome fixture의 7개 scene은 같은 source hash에서 computed auto rows, used track rows와 root-relative item frame을 독립 계산과 일치시켰다. 계획 PR을 올린 뒤 실제 PR diff를 새 30관점 목록으로 다시 검토한다. 구현 단계에서 드러나는 numeric range 또는 Taffy flex algorithm 불일치는 구현 전에 해결하고 계획·oracle을 갱신한다.
