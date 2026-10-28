# C11.4 hidden item 구현·런타임 적대 검토 20개

검토 대상은 `display:none` 두 scene의 FFI route, runtime adapter, computed-style projection, Android/iOS matcher, capture와 verifier다. 관점마다 다른 실패 조건을 확인했다. `코드`는 실제 분기·불변식 대조이고 `실행`은 해당 test 또는 두 simulator의 현재 run 결과다.

| # | 별도 실패 가설 | 확인과 결과 |
| ---: | --- | --- |
| 1 | 임의 root나 scene ID가 hidden adapter로 들어간다 | FFI는 기존 여덟 ID 뒤에 정확히 두 hidden ID만 위임한다. hidden FFI unit test에서 unknown ID 세 가지가 `None`이다. |
| 2 | 기존 8개 mixed scene이 hidden 경로에 흡수되어 회귀한다 | 기존 scene route가 먼저 유지되고 unknown만 hidden module로 넘어간다. Android/iOS capture에서 기존 장면 8/8이 각자 다시 통과했다. |
| 3 | hidden scene의 stale style/document revision이 유효 입력으로 통과한다 | `validate_snapshot_revision`을 root shape 검사 앞에서 호출한다. hidden adapter test의 stale revision 변조가 거부된다. |
| 4 | hidden root가 정확한 document-root HTML `div` 및 네 direct child 계약을 벗어나도 처리된다 | root identity와 parent를 확인하고 child 배열 길이·순서·tag·ID·parent를 고정한다. 그 뒤 visible sibling과 image topology도 다시 검사한다. |
| 5 | `visibility:hidden`이나 `display:contents`를 `display:none`과 동등하게 본다 | computed `display:none`, `visibility:visible`을 동시에 요구하고 두 값의 변형을 hidden runtime negative test에서 거부한다. |
| 6 | Grid line 및 computed `visibility` projection이 default runtime에서 노출되거나 fixture style에 빠진다 | 해당 longhand와 author property는 C11.4 fixture feature projection 안에 추가했다. Android/iOS 앱 빌드가 이 source를 사용했고 10 scene matcher가 결과를 통과했다. |
| 7 | 15-character hidden text가 paragraph 또는 intrinsic input 목록에 포함된다 | hidden handle은 `paragraph_styles`에서 제외된다. runtime test는 두 축에서 hidden node가 paragraph map과 `grid_item_intrinsic_inputs`에 없음을 확인한다. |
| 8 | 큰 hidden text가 첫 track을 150px로 밀어도 verifier가 허용한다 | hidden scene expected first track은 100px이며 trace는 `[100,40,140]`이다. Chrome visible control은 별도 사전 oracle에서 `[150,40,90]`을 보인다. hidden sizing 결과를 control 입력으로 대체하지 않았다. |
| 9 | 보이는 A text까지 hidden 처리로 intrinsic sizing에서 사라진다 | runtime test가 Ahem `XXXXXXXXXX`의 100px first track item 및 track base를 양 축에서 검사한다. |
| 10 | image URL·decode·natural size가 바뀌어도 hidden route가 유효하다 | exact URL, leaf image, decode, intrinsic resource snapshot과 24×16 natural size를 확인한다. 공통 negative tests에서 source·bytes·크기 오류가 두 축 모두 거부된다. |
| 11 | 보이는 C text 또는 fixed image의 위치가 hidden bookkeeping 때문에 변한다 | runtime test는 image 24×16 frame과 C의 마지막 flexible track 시작점·크기를 검사한다. 양 플랫폼 verifier는 모든 root-relative frame을 재비교했다. |
| 12 | Columns와 Rows가 뒤바뀌거나 opposite-axis root dimension이 잘못된다 | adapter와 app matchers는 axis별 300×120/120×300 root를 고정한다. runtime 및 simulator 2×10 scene matrix가 각각 통과했다. |
| 13 | hidden node가 첫 explicit cell 밖에 있거나 track placement 변조가 허용된다 | hidden style은 row와 column 모두 1/2 line을 요구하고 root track topology도 기존 혼합 track validator로 확인한다. hidden style/track mutation test가 거부된다. |
| 14 | hidden orthogonal text가 projection에서 지원 paragraph와 충돌하거나 모든 vertical profile 검사가 느슨해진다 | projection은 snapshot에서 `display:none` ancestor까지 추적해 hidden writing node만 비교에서 빼고, root 및 visible paragraph 검증은 유지한다. Rows test와 simulator scene이 통과했다. |
| 15 | hidden node의 내부 0×0 frame을 실제 CSS box로 해석하거나 non-zero frame을 받아들인다 | runtime test와 기대 frame은 hidden 0×0만 허용한다. manifest는 hidden bookkeeping frame을 다섯 번째 record로 분리한다. |
| 16 | hidden frame record가 Android/iOS box 수나 draw 수를 늘린다 | 양 앱 matcher는 4 visible boxes, 5 frame records, 4 drawn boxes를 따로 기대한다. 두 플랫폼의 screenshot/presentation verifier가 10/10 scene을 통과했다. |
| 17 | hidden node 제거 후 visible items의 source order를 0부터 다시 매겨 순서 정보를 잃는다 | runtime intrinsic list는 기존 DOM index `[1,2,3]`을 고정한다. Java/Swift matcher와 Python/JS verifier가 hidden scene에서 같은 순서를 확인한다. |
| 18 | hidden item이 다른 axis track contribution 또는 stage trace에 남는다 | verifier는 두 axis pass의 contribution 목록을 모두 검사하고 hidden node ID가 있으면 거부한다. 자체 wrong-source-order, hidden-input, hidden-contribution negative control도 모두 거부됐다. |
| 19 | visible control, 미등록 scene, C11.2.1 충돌 profile이 일부 결과를 만들어 통과한다 | 양 simulator run manifest에서 세 route rejection과 launch/runtime log를 확인했다. visible control은 hidden runtime fixture의 supported route가 아니다. |
| 20 | 현재 Android/iOS 화면 근거가 stale source, 준비된 report, 또는 과장된 pixel 동일성으로 대체된다 | current run manifest의 source/V8/app hash, 20 scene log, 10×2 screenshot hash·size와 draw/presentation을 독립 verifier가 확인했다. 실기기, hardware GPU와 플랫폼 간 pixel equality는 검증하지 않았다. |

feature Clippy는 최초 실행에서 기존 validator의 반환형 복잡도와 새 unit test의 불필요한 borrow를 찾았다. 의미를 바꾸지 않는 `ObservedFrames` type alias와 borrow 수정 뒤 fixture Clippy 및 기본 workspace Clippy가 모두 `-D warnings`로 통과했다. 별도 기본/opt-in release archive symbol·payload 비교는 [artifact 경계 기록](artifact-boundary.md)을 참조한다.
