# C12.3 fixed 계획의 구현 경계 후속 검토

이 후속 검토는 구현 코드 탐색에서 확인된 Stylo·Chromium 경계를 `plan/c12-3-fixed-positioning.md`와 대조한다. 첫 계획 검토를 대체하지 않으며, 계획의 완료 조건이나 구현 범위를 넓히지 않는다.

## 확인 기준

- 고정 Chromium `154.0.8037.98`, revision `b859317bf11f6be47f9b7799ec690a0a42a1fb33`의 `LayoutObject::ComputeIsFixedContainer`, `ComputedStyle::HasTransformRelatedProperty`, `HasNonInitialFilter`, containment eligibility.
- 저장소 고정 `stylo 0.22.0`의 생성 Longhand API와 `properties/longhands.toml`.
- 구현 전 고정한 [22개 Chrome case·79개 node 기준](c12-3-fixed-precomparison-2026-10-11.md) 및 현재 Spinon layout owner·revision 경계.

## 계획 대조

| # | 실패 관점 | 확인 결과와 계획의 경계 |
|---:|---|---|
| 1 | CSS `Fixed`를 layout DTO에 남기지 않아 후속 owner 의미를 잃는가 | DTO에 `Fixed`를 추가하고 Taffy 입력 경계에서만 `Absolute`로 투영한다. |
| 2 | fixed box의 absolute 자손이 viewport에 직접 붙는가 | positioned owner 순회에서 fixed box가 absolute containing block이 되게 하고 자손은 fixed box 계산 노드 아래 둔다. |
| 3 | fixed 자손이 fixed 조상을 containing block으로 사용하는가 | fixed owner map은 absolute owner map과 분리하고 보이는 fixed 노드는 viewport를 사용한다. |
| 4 | source parent나 HostDocument parent를 계산 owner에 맞춰 변경하는가 | 재부모화는 Taffy 계산 graph에만 적용하고 source tree와 event/lifetime graph를 보존한다. |
| 5 | fixed percentage inset과 dimension이 source parent 크기를 쓰는가 | fixed의 두 축 basis는 viewport로 고정한다. 일반 자손의 size basis는 여전히 원래 CSS 부모를 사용한다. |
| 6 | 숨겨진 조상 아래 fixed box가 viewport root로 살아나는가 | hidden flag를 원본 tree에서 전파한 뒤 fixed 승격을 건너뛰고 `NoBox` owner를 남긴다. |
| 7 | 한 축의 양 inset이 모두 `auto`일 때 임의 static position을 만드는가 | 해당 축의 static-position 의존 입력은 오류로 닫는다. 한쪽만 auto인 축은 reference로 비교한다. |
| 8 | fixed layout root가 viewport 자식 처리와 충돌하는가 | layout root의 fixed/absolute는 입력 검증 단계에서 명시 거부한다. |
| 9 | transform 등 stylesheet 입력이 cascade까지 도달해 잘못된 viewport owner를 만드는가 | 파싱 가능한 effect declaration은 제한 author property allowlist에서 거부한다. 알 수 없는 declaration의 Stylo 진단도 snapshot에 보존하고 RuntimeBlockPositioning projection에서 거부한다. |
| 10 | inline style이 author stylesheet preflight를 우회하는가 | 파싱 가능한 effect는 같은 runtime inline allowlist에서 거부하고, 알 수 없는 inline declaration은 진단을 layout 경계까지 전달해 거부한다. |
| 11 | effect 표식이 원본 selector 문자열에서 추측되는가 | 계산 가능한 effect는 Stylo `ComputedValues`의 typed/computed longhand에서 추출하고 selector 원문은 사용하지 않는다. |
| 12 | Stylo가 제공하지 않는 `content-visibility`가 computed effect에 있다고 가장하는가 | Stylo 0.22.0은 이 longhand를 Gecko 전용으로 설정했고 생성 ID/accessor가 없다. inline·stylesheet 진단을 RuntimeBlockPositioning projection까지 보존해 거기서 거부한다. |
| 13 | transform 관련 Blink trigger 중 motion path를 놓치는가 | `offset-path` declaration은 현재 Stylo 0.22.0에서 `UnknownProperty` 진단을 내므로 typed marker를 가장하지 않고, 해당 진단이 projection을 통과하지 않는지 확인한다. |
| 14 | `will-change` 쉼표 목록에서 transform 관련 토큰을 놓치는가 | computed `will-change`를 목록 단위로 검사해 transform·translate·rotate·scale·offset-path를 한 effect로 표시한다. |
| 15 | `will-change:opacity`를 fixed containing block으로 잘못 분류하는가 | Chromium 기준은 이를 fixed owner trigger로 보지 않는다. CSS allowlist 밖이라 현재 profile 계산은 거부하지만 effect mapper는 fixed trigger로 표시하지 않는다. |
| 16 | filter 자체는 `none`인데 `will-change:filter`를 놓치는가 | 계산 가능한 direct filter와 will-change filter는 별도 effect로 추적한다. `backdrop-filter` declaration은 현재 Stylo build에서 `UnknownProperty` 진단으로 fail closed한다. |
| 17 | containment shorthand `content`·`strict`에서 layout/paint flag가 누락되는가 | 현재 build의 `contain` declaration은 `UnknownProperty` 진단으로 fail closed한다. computed containment bit 검사만으로 이 parser 경계를 대체하지 않는다. |
| 18 | snapshot에서 조상 computed style이 빠져 효과 검사가 건너뛰어지는가 | fixed 조상 chain의 computed node 부재는 missing-element 오류로 닫는다. |
| 19 | effect가 여러 개일 때 성공 결과 일부가 먼저 게시되는가 | projection 전에 ancestor effect를 검사하고 실패하면 전체 StyleLayout 계산이 중단된다. runtime publication은 기존 render revision gate를 통과해야 한다. |
| 20 | 고정 Chrome 사전 비교가 실제 runtime·mobile 증거로 과장되는가 | 기존 Chrome 기준은 입력 oracle로만 쓴다. Android 실기기·iOS Simulator 및 resize/environment revision 실행 근거를 별도로 요구한다. |

## 계획 수정

- computed effect 목록에서 `content-visibility` computed marker 주장을 제거했다. 현재 Stylo API에 없는 속성이기 때문이다.
- locked Stylo 0.22.0에서 `offset-path`, `backdrop-filter`, `contain`도 current build의 `UnknownProperty` 진단을 낸다. 해당 computed marker를 주장하지 않고 parse 진단이 layout projection을 통과하지 못하도록 요구한다.
- 파싱 가능한 author effect는 stylesheet·inline preflight에서 거부한다. 추출 가능한 computed effect는 style-to-layout 경계에서도 재검사한다.
- `compute_runtime_style_layout`의 RuntimeBlockPositioning 경계에서 cascade 진단을 거부하도록 구현 전 요구를 보강한다. 회귀 시험은 위 unknown property가 stylesheet·inline 양쪽에서 layout frame으로 이어지지 않는지 검사한다.
- Chrome 사전 비교는 [PR #135](https://github.com/ohah/spinon/pull/135)에서 병합되었으므로 기존 계획 검토의 “기준 fixture가 아직 없다”는 stale 문장을 완료된 기준과 미실행 runtime gate로 나눠 정정했다.
