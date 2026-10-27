# PR #699 `fit-content()` min-height slice · diff failure 관점 30개

PR #699 draft commit `2eb8b36caff303eba528ef65d0a83c865c945042`을 대상으로 서로 다른 실패 가설 30개를 검토했다. 각 행의 정상 대조군과 근거를 구분한다. “실행”은 실제 focused test, archive probe 또는 simulator run이고, “코드 경계”는 rejection/validation 경로 검토다. 코드 경계 검토를 fault injection 통과로 확대하지 않는다.

| # | 실패 가설 | 정상 대조군 | 확인과 판정 |
|---:|---|---|---|
| 1 | 기대 row 10px이 Taffy 출력에서 역산되어 틀린 기준을 고정한다 | WPT source revision과 CSS Grid 2025 CRD 산술에서 독립 계산한 `[10]` 및 root 300px | 사전 비교 `expected.json`, layout oracle, Android/iOS capture 결과를 대조했다. **통과 · 실행** |
| 2 | root `height:300px`를 `auto`로 잘못 취급해 이 fixture가 다른 사례를 받는다 | canonical style의 `height:auto; min-height:300px` | runtime adapter는 computed property와 typed dimension 둘 다 `auto`여야 받는다. **통과 · 코드 경계** |
| 3 | `min-height`를 빼거나 `auto`로 바꿔도 fit-content row가 제한 height처럼 통과한다 | fixed `min-height:300px` canonical oracle | min-height 없음은 `IndefiniteGridBasis`를 기대하는 layout test로 확인했다. **통과 · 실행** |
| 4 | 음수·NaN·±Infinity min-height가 유한 definite basis로 인정된다 | `min-height:300px` canonical scene | 네 잘못된 값이 `InvalidStyle { field: "min_height" }`로 거부되는 layout test를 확인했다. **통과 · 실행** |
| 5 | percentage fit-content limit가 indefinite height basis에서 계산된다 | fixed length `fit-content(40px)` canonical row | `fit-content(50%)`는 `IndefiniteGridBasis`로 거부된다. **통과 · 실행** |
| 6 | 음수·NaN·±Infinity fit-content limit가 사용된다 | 유한·비음수 `40px` limit | 네 값이 `InvalidGridTrack { property: "grid-auto-rows" }`로 거부되는 layout test를 확인했다. **통과 · 실행** |
| 7 | `fit-content(41px)` fixture 입력이 40px WPT case의 성공으로 오인된다 | runtime adapter unit test의 40px canonical `GridContainerInput` | 41px mutation을 adapter가 거부한다. **통과 · 실행** |
| 8 | explicit row를 넣어 implicit-row assertion을 우회한다 | `rows=[]`, auto row list에만 `fit-content(40px)` | explicit row mutation을 adapter가 거부한다. **통과 · 실행** |
| 9 | scene ID 뒤에 `;throw 1`을 붙여 FFI source allowlist를 우회한다 | 정확히 일치하는 `fit-content-min-height-wpt` ID | FFI allowlist test가 접미사·명령문 ID를 거부한다. **통과 · 실행** |
| 10 | 등록되지 않은 scene ID가 이 source나 다른 fixture source를 받는다 | canonical ID만 `Some(source)` | canonical FFI route test는 이웃의 미등록 ID를 오류로 닫고 Android/iOS capture list는 요청 장면 하나만 선택한다. **통과 · 실행** |
| 11 | opt-in fixture marker/symbol이 product FFI archive에 포함된다 | fixture-test archive의 positive control | 별도 target directory에서 product archive에는 symbol·scene marker가 없고 fixture archive에는 둘 다 있음을 확인했다. APK/IPA 전체 검증은 아니다. **통과 · 실행** |
| 12 | 다른 root ID의 비슷한 DOM이 이 adapter를 잘못 선택한다 | 정확한 `c114-fit-content-min-height-grid` ID | root ID가 다르면 adapter는 fixture observation을 만들지 않는다. **통과 · 코드 경계** |
| 13 | 잘못된 namespace/tag 또는 document root 밖의 node를 그대로 평가한다 | HTML namespace `div`, `HostParent::Root` | root와 descendants의 namespace/tag/parent 조건이 일치하지 않으면 unsupported 경로로 닫힌다. **통과 · 코드 경계** |
| 14 | Grid direct item을 추가·누락해 implicit row 수를 바꾼다 | root의 direct item 한 개 | root child count를 정확히 1로 요구한다. **통과 · 코드 경계** |
| 15 | item ID, tag, namespace 또는 root parent를 바꿔 다른 item을 재사용한다 | 고정 item identity와 root 소유 관계 | item identity와 `HostParent::Node(root)`를 각각 확인한다. **통과 · 코드 경계** |
| 16 | nested child를 빼거나 추가하거나 parent/ID를 바꿔 contribution을 바꾼다 | 고정 child 하나와 `HostParent::Node(item)` | child count와 element identity/ownership을 확인한다. **통과 · 코드 경계** |
| 17 | computed style을 누락·중복하거나 unrelated element style을 끼워 넣는다 | fixture node 세 개와 style 세 개의 1:1 집합 | node ID set의 cardinality 및 중복을 거부한다. **통과 · 코드 경계** |
| 18 | 다른 runtime profile의 computed style을 C11.4 Grid 입력으로 오인한다 | `RuntimeBlockFlexGridTrackSizingV1` | prepare와 output validation에서 profile을 확인한다. **통과 · 코드 경계** |
| 19 | root width/min-height/rows/align/gap/border를 바꿔 비슷한 frame만 만든다 | typed style과 computed property가 일치하는 canonical root | key dimensions, Grid property/typed layout, gaps, edges, borders와 input track shape를 검증한다. **통과 · 코드 경계** |
| 20 | item margin/padding/border/min-height/placement가 intrinsic contribution을 바꾼다 | auto sizing, zero spacing/border, auto placement | computed item dimensions·spacing·border·placement를 고정한다. **통과 · 코드 경계** |
| 21 | child width/height/spacing/border를 바꿔 10px contribution을 위장한다 | child `60×10px`, zero spacing/border | child computed style을 고정하고 최종 frame도 별도로 확인한다. **통과 · 코드 경계** |
| 22 | typed Grid input에 extra container, column/row, auto-column, repeat, area, line name을 넣는다 | root 하나, `[60px]` column, empty explicit rows, pinned auto row | Grid container 개수와 모든 허용/비허용 typed fields를 재검증한다. **통과 · 코드 경계** |
| 23 | target frame이 누락되거나 중복되어 partial frame set이 성공한다 | 세 고정 node frame 각 하나 | adapter는 관련 frame 수와 node별 중복 수를 비교한다. **통과 · 코드 경계** |
| 24 | root/item/child frame이 `[60×300,60×10,60×10]`과 다르다 | 독립 oracle의 CSS-px frame 기대값 | oracle 및 양 simulator 결과가 세 frame 모두 exact다. **통과 · 실행** |
| 25 | row trace가 다른 Grid 소유이거나 Columns/Rows pass가 빠지거나 추가된다 | root trace 한 개, 두 축 pass 각 한 개 | adapter는 trace owner, trace 수, pass 수와 각 축 cardinality를 확인한다. **통과 · 코드 경계** |
| 26 | sizing stage가 중복·역순이거나 임의 stage가 끼어든다 | canonical runtime trace의 여섯 stage | oracle·adapter·양 앱 matcher가 전체 목록과 순서를 고정하고 capture가 같은 정상 trace를 통과했다. 변형 trace 주입은 하지 않았다. **통과 · 코드 경계 + 정상 control 실행** |
| 27 | initial 0px을 final row로 보거나 stretch에서 fit-content row가 300px까지 늘어난다 | Initialize 0px, 이후 각 stage 10px | oracle와 양 simulator의 row stage를 비교했다. 임의 stage/size fault injection은 하지 않았다. **통과 · 정상 control 실행** |
| 28 | host document/tree/style/environment revision 중 하나가 다른 layout key와 섞인다 | 한 computed style snapshot에서 생성한 `LayoutInputRevision` | output validation은 HostDocument generation/document/render-tree, style revision, environment revision을 runtime key와 대조한다. **통과 · 코드 경계** |
| 29 | 환경 결과 뒤 다른 generation/sequence draw가 성공 처리된다 | 현재 surface generation 및 presentation sequence에 묶인 draw | Android/iOS가 동일 environment/draw identity로 통과했고 코드가 stored identity/generation/sequence를 비교한다. stale draw fault injection은 하지 않았다. **통과 · 코드 경계 + 정상 control 실행** |
| 30 | child pixel 또는 root 밖 paint가 잘못돼도 geometry만으로 성공 처리한다 | child `#ffd166`, root 배경 `#17212b`, root 밖 검정, boxes 3개 | API 37 AVD PixelCopy와 iOS Metal 표본이 각 3/3 exact다. iOS는 compositor 제출 전이며 outside-root sample은 `(60,15)`다. **통과 · 실행** |

30개 관점 모두에서 이 PR diff의 수정 필요 항목은 남지 않았다. 실제 negative input을 실행한 항목과 코드 경계만 검토한 항목은 각 행에 구분했다. 이는 전체 WPT, 일반 CSS admission, 실기기/hardware GPU 또는 release APK/IPA의 fixture exclusion 검증으로 확대하지 않는다. 정상 simulator 화면과 전체 제한은 [runtime evidence](./README.md)에 있다.
