# C11.4 auto-repeat calc 최소값 implementation PR diff 검토 30개

이 검토는 현재 구현 PR의 변경 diff만 본다. 계획 검토·계획 수정 검토·구현/runtime 검토와 다른 변경 경계와 회귀 관점을 사용한다.

| # | 독립 diff 실패 관점 | diff 대조와 판정 |
| --- | --- | --- |
| 1 | fixture module declaration이 기본 product build에서도 컴파일된다. | 신규 module declaration은 c04-runtime-gpu-fixture-test cfg 아래 있다. default/product raw marker scan은 module marker와 scene ID가 없음을 확인했다. |
| 2 | Cargo feature 정의를 불필요하게 바꾸어 app product 조합이 달라진다. | Cargo.toml은 바뀌지 않았다. 기존 opt-in c04-runtime-gpu-fixture-test를 재사용한다. |
| 3 | 기존 C/Swift ABI surface에 task 전용 exported symbol이 추가된다. | extern C/header/Swift export 변경은 없다. 신규 runtime source는 crate 내부 fixture module에 머문다. |
| 4 | task ID를 보존하지 않는 비정규 Rust module 경로를 쓴다. | 새 adapter는 기존 fixtures root 아래 c11_4_auto_repeat_calc_minimum.rs로 추가됐다. |
| 5 | scene payload JSON이 fixture adapter와 다른 경로를 가리킨다. | include_str 경로와 canonical fixture-module-boundary assertion이 일치한다. |
| 6 | fixture source가 product module에 잘못 들어간다. | adapter 등록은 fixture module tree를 통해서만 이뤄지고 scene-list dispatch는 opt-in fixture runtime route 안에 있다. |
| 7 | route가 scene ID 외의 user CSS를 받아 범위를 넓힌다. | adapter는 include된 고정 JSON에서 exact scene ID를 검색하고 미등록 ID는 None을 반환한다. |
| 8 | JSON을 JS source에 붙일 때 quote/escape가 깨진다. | serde_json 직렬화 결과를 JSON.parse 인자로 넘긴다. validator가 source 생성 및 미등록 ID 거부를 검사한다. |
| 9 | root 또는 item node ID가 scene 사이에서 충돌한다. | c114-<scene-id>-grid와 item 이름을 합성한다. six ID unique check가 통과했다. |
| 10 | calc scene에서 다른 길이나 maximum을 적용한다. | CSS fixture 문자열은 calc(20px + 20px), 1fr, definite 184px 한 조합이다. |
| 11 | fixed fill control이 다른 item 수·size·gap을 써 equality comparison을 무효화한다. | control은 같은 root, cross-axis track, gap과 네 item을 쓴다. 바뀐 값은 minmax(40px,1fr)뿐이다. |
| 12 | columns matrix가 unintended rows repeat을 함께 생성한다. | columns scene rows는 48px fixed track이며 result JSON에서 한 row track을 확인했다. |
| 13 | rows matrix가 unintended columns repeat을 함께 생성한다. | rows scene columns는 48px fixed track이며 result JSON에서 한 column track을 확인했다. |
| 14 | rows가 기본 row-major flow를 사용해 예상 vertical sequence를 우연히 얻는다. | rows 장면만 autoFlow=column을 통해 grid-auto-flow:column을 적용한다. 실제 순서는 node frame에서 확인했다. |
| 15 | zero-gap arithmetic에서 다른 gap 값이 payload expectation에 숨어든다. | 모든 scene의 columnGap/rowGap은 0px이며 capture result는 두 축의 gap을 0으로 기록했다. |
| 16 | auto-fill과 auto-fit scene을 scene name과 expectation에서 혼동한다. | auto-fit ID만 -fit suffix를 쓰며 fixture expectations, host count/frame helpers, result JSON이 동일한 ID로 연결된다. |
| 17 | auto-fit expected tracks 수와 collapse mask 길이가 다르다. | adapter validator가 columns/rows의 track과 collapsed 배열 길이를 확인한다. |
| 18 | root expected width/height가 item frame 목록에서 반대로 적용된다. | columns root는 184×48, rows root는 48×184이며 두 platform에서 actual report가 각각 일치한다. |
| 19 | item frame 기대가 node list와 어긋난다. | fixture JSON items와 expect.items 이름·개수가 validator에서 zip 비교되고 runtime report node sequence와 대조된다. |
| 20 | Android root helper가 unknown ID에 기본 dimension을 내어 다른 scene을 가린다. | helper는 식별된 6개 ID만 값이 있고 나머지는 null을 반환한다. 기존 fallback 우선순위도 유지된다. |
| 21 | iOS root helper의 nil handling이 unsupported scene을 calc case로 분류한다. | helper는 6개 정확한 ID에서만 optional size를 돌려준다. |
| 22 | empty item-frame 문자열 sentinel이 calc scene에서 실제 zero-sized item으로 오인된다. | existing host contract에서 empty itemFrame은 node_frames가 각 item을 판정한다. runtime captures는 complete node frames를 요구한다. |
| 23 | Android node loop에서 fit/control suffix를 잘못 판별한다. | fit만 한 item을 생성하고 fill/control은 네 frame을 생성한다. 두 플랫폼 fit/control run의 boxes와 node count가 일치한다. |
| 24 | iOS row/column coordinate formulas가 transposed된다. | rows는 x=0, y=index×46이며 columns는 x=index×46, y=0이다. simulator output six scenes에서 exact match한다. |
| 25 | fixed control이 expected root frame resolver 뒤의 다른 helper에 가로채인다. | 신규 resolver는 area repeat 뒤, 다른 scene-specific fallbacks 앞에 위치한다. calc/fixed scene 모두 정확한 root frame을 반환한다. |
| 26 | expected box count sentinel 0이 실제 valid scene count를 뜻한다. | 신규 6 scene에서 count는 fit=2 또는 fill/control=5이며 unlisted scene은 기존 resolver로 계속 진행한다. |
| 27 | capture catalog JSON에 scene data는 있지만 --scene-id selection이 지원되지 않는다. | loader는 six scenes를 catalog에 추가하고 selected-catalog assertion은 exact six IDs를 확인한다. |
| 28 | boundary test가 stale catalog count나 duplicated ID를 허용한다. | test는 total 342, unique ID, calc six ordered IDs, Android/iOS allowlists와 selected scene set을 검사한다. 19/19 통과했다. |
| 29 | platform helper code가 단위 검증은 통과하지만 앱 경로에서 호출되지 않는다. | Android와 iOS app을 fixture-test flag로 빌드하고 실제 emulator-5558 및 iPhone 17 Pro Simulator에서 6/6 scene씩 실행했다. |
| 30 | 문서가 code diff를 product support, pixel equality, C11.4 completion 또는 오래된 count로 확대한다. | 수정 계획·0059·STATUS·README가 pixel 검사 부재, archive-only 범위, internal version 0.1.0, C11.4 미완료와 최신 74/207을 기록한다. |

## 판정

30개 diff 관점에서 남은 수정은 없다. 정적 archive는 task scene ID/raw markers에 한정되며 Android APK/iOS app의 release fixture exclusion 증거로 사용하지 않는다. 구현/runtime 관점은 [별도 검토 20개](./implementation-review-20.md), 계획 수정 관점은 [계획 검토 20개](../../../../plan/c11-4-auto-repeat-calc-minimum-revision-review-20.md)에서 분리해 기록한다.
