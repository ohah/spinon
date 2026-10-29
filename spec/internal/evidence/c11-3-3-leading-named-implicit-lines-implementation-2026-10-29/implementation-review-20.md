# C11.3.3 leading named implicit lines 구현 실패 경계 검토 20개

구현 검토는 계획 20개 및 계획 PR diff 30개와 분리했다. 실제 resolver, signed track normalizer, Stylo cascade, FFI/app fixture route 및 최종 Android/iOS 결과를 기준으로 각각 다른 실패 경로를 확인했다.

| # | 실패 관점 | 확인 및 결과 |
| --- | --- | --- |
| 1 | implicit named-line resolver가 기존 C11.3.3 route 전체에서 활성화됨 | 전용 implicit named-line wrapper에서만 flag가 켜지고 기존 wrapper는 disabled 상태다. C11.3.3 기존 26 scene 회귀와 새 route allowlist가 통과했다. |
| 2 | parser 문자열을 재해석해 authored name·occurrence의 Stylo typed value를 잃음 | resolver 입력은 typed placement와 explicit `LineMetadata`다. named-line unit 및 style-to-layout matrix 7 scene이 통과했다. |
| 3 | 같은 physical line의 중복 name 토큰을 여러 occurrence로 셈 | 이름은 physical line group 단위로 조회한다. 기존 같은-line named occurrence 회귀와 새 부분-authored-name scene이 통과했다. |
| 4 | 양의 occurrence 부족분을 implicit grid 앞쪽에 생성 | positive occurrence는 explicit track 끝 뒤의 부족 occurrence만 만든다. `positive-missing-occurrences` Android/iOS frame과 pixel이 모두 일치했다. |
| 5 | 음의 occurrence 부족분을 implicit grid 뒤쪽에 잘못 생성 | negative occurrence는 start-side signed boundary를 생성한다. `negative-missing-occurrences`의 앞쪽 item frame이 두 플랫폼 oracle과 일치했다. |
| 6 | authored line name이 부족한 named fallback보다 우선하지 않음 | explicit `ghost`가 있는 부분-authored fixture에서 존재하는 occurrence를 먼저 소비한다. partial-explicit와 authored-name scene의 Rust geometry가 통과했다. |
| 7 | implicit template-area alias와 authored name precedence를 혼동 | area alias lookup이 resolver fallback보다 앞선다. 기존 26-scene area regressions와 unknown-area 후속 scene이 통과했다. |
| 8 | forward span 검색이 반대편 line 또는 explicit grid 경계부터 잘못 count | forward lookup은 반대편 definite boundary 다음부터 순회한다. forward span-two frame 및 resolver tests가 통과했다. |
| 9 | backward span이 trailing implicit line을 먼저 소비 | backward lookup은 opposite line 앞에서 explicit boundary까지만 검색한 뒤 leading 방향 부족분을 만든다. span-one/two 및 partial-explicit tests와 양 플랫폼 matrix가 통과했다. |
| 10 | count 0, `i32::MIN`, `u32::MAX`에서 산술 overflow·무한 순회 | zero를 거부하고 signed 연산에 checked conversion을 쓴다. occurrence/span 경계 unit tests와 Clippy가 통과했다. |
| 11 | named span의 opposite definite numeric endpoint를 독립 numeric implicit placement로 확장 | projection은 named span이 요청한 definite 반대 endpoint 외의 lone numeric extent를 거부한다. negative tests 및 limited-profile wrapper가 통과했다. |
| 12 | signed origin이 explicit track index로 되돌아오지 않거나 leading offset을 두 번 적용 | extent, origin shift, `coordinate_index`가 checked signed offset을 한 번 적용한다. layout tests의 leading/trailing item coordinates가 통과했다. |
| 13 | leading track list가 explicit grid를 기준으로 잘못된 순서로 반복 | leading pattern은 explicit edge에서 역순, trailing pattern은 정방향으로 반복한다. normalized vector `[15,7,30,40,15,7,15]` 및 row/column assertions가 통과했다. |
| 14 | auto/intrinsic/percentage track이 fixed-pixel profile에 들어옴 | style cascade와 layout validator 모두 non-empty px list만 허용하며 Auto/비-px member를 fail closed한다. focused cascade test와 layout profile checks가 통과했다. |
| 15 | track count limit 검사 전 대형 allocation | signed extent와 expansion은 checked count를 `GRID_MAX_TRACKS`에 대조한 뒤 reserve한다. 9,999/10,000/10,001 경계와 reservation tests가 통과했다. |
| 16 | auto-placed item이 definite signed mode에서 조용히 생략됨 | visible item row/column start가 definite가 아니면 `GridImplicitTrackRequired`를 반환한다. signed-mode auto-placement negative test가 통과했다. |
| 17 | hidden subtree child를 배치해 DOM node/draw order가 달라짐 | hidden map에서 숨긴 child를 제외하고 기존 DOM 순서를 보존한다. nested-hidden old C11.3.3 scene이 두 플랫폼 matrix에서 통과했다. |
| 18 | 새 FFI 장면이 임의 script/scene ID를 허용하거나 route limit을 넘김 | 7 scene allowlist만 source를 반환하고 route-safe run ID에 80자 상한을 둔다. Rust FFI allowlist 1/1, CSS structural 14/14, Android final matrix가 통과했다. |
| 19 | Android/iOS 시각 검증이 단순 geometry 또는 잘못된 surface sample로 대체됨 | 각 scene의 root/item identity와 frame을 확인하고 Android PixelCopy 및 iOS Metal sample을 독립 oracle과 exact 비교한다. 새 장면 각 14 node frame·7/7 sample, max geometry error 0. |
| 20 | fixture가 기본 제품 artifact에 포함 | 기본 static archive C11.3.3 FFI symbol 0개·payload 없음, opt-in feature positive control symbol 2개·payload 존재를 확인했다. 이는 app release bundle 검증까지 확장하지 않는다. |

## 수정 후 재확인한 실제 발견

- 첫 Android run ID가 81자여서 app의 80자 제한에 걸렸다. run ID를 짧게 만들고 length assertion을 추가했다.
- fixture root `max-content`가 제한 profile에서 intrinsic sizing 거부를 발생시켰다. Chrome oracle와 비교하는 fixed-width app input으로 바꾸고 scene 폭과 track 합이 같음을 structural test로 고정했다.
- Android expected RGB가 신규 `named-implicit-item` ID를 blue로 분류하지 않아 배경색 비교가 됐다. 명시 color mapping을 넣고 structural test 및 최종 AVD matrix로 다시 확인했다.

## 판정

20개 관점에서 해결되지 않은 C11.3.3 구현 blocker는 찾지 못했다. 전체 `spinon-style` 테스트는 247개 중 246개 통과하며 TrackSizing auto-repeat percentage minimum 관련 기존 실패 1개가 남았다. 이 결과를 전체 suite green으로 기록하지 않았고, 전체 CSS Grid·WPT·실기기/hardware GPU·iOS compositor 완료를 주장하지 않는다.
