# C11.4 Grid `visibility:collapse` 구현·runtime 검토 20개

검토 대상은 visibility collapse cascade gate, exact scene validator, Grid track diagnostics, render/hit projection, Android/iOS fixture adapters 및 capture runner의 최종 코드다. 계획 단계에서 작성한 [계획 검토 20개](../../../../plan/c11-4-grid-visibility-collapse-plan-review-20.md)와는 별도 관점이다. 최종 실행 근거는 [runtime README](./README.md), [Android AVD](./android-avd/c114-collapse-mvtfme5e-89daa39e/result.json), [iOS Simulator](./ios-simulator/c114-collapse-mvtfn06e-14ef5dc2/result.json), [archive 경계](./artifact-boundary/README.md)에 있다.

| # | 실패 관점 | 검토와 결과 |
|---:|---|---|
| 1 | 기본 `c04-runtime-gpu` artifact에 새 route·scene payload가 섞임 | 기본 static archive의 문자열에서 root prefix, 새 scene ID 및 eval symbol이 0개다. fixture-test positive control에는 10개 문자열 match와 eval symbol이 있다. archive 한정 경계로 통과. |
| 2 | 기존 C11.4 profile이 `visibility:collapse`를 우연히 허용 | `track_sizing_profile_rejects_unsupported_visibility_collapse_in_author_stylesheets`와 fixture cascade 검사가 기본/opt-in 경계를 각각 확인한다. 통과. |
| 3 | `collapse` 허용이 `visible`/`hidden` 같은 기존 visibility 값을 바꿔버림 | visibility opt-in은 별도 boolean gate이며 기존 `supports_runtime_visibility_value` 경로를 유지한다. 정확한 declaration serialization 비교와 기존 renderer suite를 확인했다. 통과. |
| 4 | author stylesheet와 inline style에서 서로 다른 admission 결과 발생 | `stylesheet_registry`의 visibility-collapse 정책과 `runtime_grid`의 inline preflight가 같은 opt-in route flag를 받는다. 고정 scene은 inline input만 사용하므로 외부 stylesheet 적합성은 주장하지 않는다. 경계 통과. |
| 5 | 다른 Grid scene이 root ID prefix 때문에 새 route로 잘못 분기 | route는 10개 exact scene ID만 찾고, suffix 변형·빈 값은 catalog test에서 거부한다. 통과. |
| 6 | 알려진 ID를 붙인 임의 document tree가 validator를 통과 | top-level HTML `div`, root/item/content tag·ID·부모, direct child 수, computed element 수를 exact 검사한다. source inspection과 10개 실행에서 통과. |
| 7 | 다른 root, namespace, parent 또는 node kind가 유효 scene으로 취급 | `scene_spec`와 `validate_element`가 HTML namespace, root parent, element kind, 예상 부모를 각각 확인한다. 통과. |
| 8 | style snapshot이 다른 document나 revision에서 재사용됨 | profile, generation, document revision, render-tree revision이 snapshot과 다르면 `prepare`가 거부한다. 통과. |
| 9 | computed value를 `hidden`으로 덮어써 cascade 결과를 잃음 | opt-in cascade test가 computed `visibility=collapse`를 exact 확인하고 runtime validator도 root/item/descendant 값을 각각 검사한다. 통과. |
| 10 | collapse item을 `display:none`처럼 Grid formatting context에서 제거 | collapse/hidden은 Columns `[50,50]`, Rows `[50,50]`과 item `100×20`/`20×100`을 유지한다. none 대조군은 `[0,0]`과 0×0이다. Rust trace와 두 플랫폼 10 scene에서 통과. |
| 11 | row/column 중 한 축에서 item placement나 track 판정 누락 | 양축 모두 collapse, hidden, none, visible descendant, visible control 5 scene을 갖는다. Android와 iOS 각 10/10에서 통과. |
| 12 | track diagnostic이 계산 후 이동한 위치를 읽지 못해 false failure 또는 검증 생략 | validator는 완료된 runtime의 `grid_track_sizing_diagnostics`에서 root trace를 요구하고 used track list를 exact 비교한다. 최종 AVD 4 trace part, iOS 7/8 part가 장면별 완전하게 기록됐다. 통과. |
| 13 | collapsed item frame이 유지되지 않거나 size가 축별로 틀림 | root/item/content/visible child frame을 exact 비교한다. Android/iOS fixture stage의 expected frame summary와 runtime validator가 일치했고 20 scene 실행에서 통과. |
| 14 | hidden/collapse item이 paint box나 hit target으로 남음 | render snapshot에는 collapse/hidden item box가 없고 root interior hit은 root로, overflow hit은 none으로 검사한다. Rust render unit test와 양 플랫폼 surface 표본 통과. |
| 15 | `visibility:visible` descendant도 collapsed parent와 함께 숨김 | fixed child의 computed value, 8×8 frame, render box, (4,4) hit target과 blue 표면 sample을 확인한다. 두 축·두 플랫폼 통과. |
| 16 | content overflow가 collapse/hidden에서 보이거나 hit-test 가능 | root 바깥 1px 지점의 hit 결과가 none이고 PixelCopy/Metal drawable 표본은 흰색이어야 한다. positive control과 함께 두 플랫폼에서 통과. |
| 17 | overflow 표본이 잘못되면 renderer 결함으로 오진 | 동일 root-local 좌표 `(81,10)`/`(10,81)`의 visible control은 red paint 및 content hit을 요구한다. Android/iOS 두 장면 모두 통과. |
| 18 | `display:none` 보정 때문에 intrinsic contribution 테스트가 가려짐 | none 입력은 computed display, zero item, `[0,0]` track을 별도로 검사하며 pixel sample을 요청하지 않는다. 양축 통과. |
| 19 | simulator가 다른 app binary를 실행하거나 결과 요약만 통과 | runner가 설치 binary hash를 build output과 대조하고, scene ID·frames·draw boxes·trace parts·surface-pixel summary를 모두 요구한다. Android AVD와 iPhone 17 Pro Simulator 각 10/10 통과. |
| 20 | fixture 흰색 clear 설정이 같은 host의 다음 Grid scene에 남음 | 첫 검토에서 sticky state를 발견했다. 각 C11.4 eval 시작 때 flag를 false로 초기화하고, exact visibility scene의 성공 뒤에만 true로 설정했다. 수정 후 최종 Android/iOS 행렬을 다시 실행해 통과했다. |

**결과:** 별도 실패 관점 20개를 검토했다. Clippy가 찾은 비최소 불리언, 인자 과다, 접을 수 있는 match, 불필요 lifetime도 수정한 뒤 최종 관련 crate all-target Clippy가 통과했다. 남은 범위는 일반 author CSS admission, 다른 Grid profile, Flex/Table collapse, 전체 WPT/C11.4이며 이 리뷰는 이를 완료로 취급하지 않는다.
