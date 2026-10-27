# C11.4 generic mixed text/image · PR diff review 30

검토 대상: 구현 source diff, Android/iOS scene matcher, simulator capture driver, archive-boundary script, accepted run evidence, and implementation/runtime review. 구현/runtime 20관점 검토와 다른 변경·통합 경계를 확인했다. 처음 실행에서 trace revision 가정을 발견해 수정하고, 현재 최종 source의 두 simulator 행렬을 다시 통과시킨 뒤 검토했다.

| # | PR 변경 실패 관점 | 확인 및 판정 |
|---:|---|---|
| 1 | fixture adapter 파일이 제품 runtime module로 컴파일됨 | C11.4 `fixtures` feature 경계 안의 기존 fixture module에서만 연결된다. 기본/fixture archive negative·positive control 통과. PASS |
| 2 | `c11_4_mixed::prepare` 고정 ID 검증이 삭제되거나 느슨해짐 | ID map만 `Option<Scene>`로 바꿨고 기존 fixed-scene validation body는 그대로다. static controls Android/iOS 각 4/4. PASS |
| 3 | generic adapter가 호출 순서상 direct-text collector 뒤에서 실행됨 | mixed prepare는 generic collector보다 앞선 기존 dispatch 자리에 있으며 ID lookup miss에서 호출된다. 네 generic scene 앱 실행 통과. PASS |
| 4 | source signature 없는 일반 root마다 오류를 반환함 | meaningful text와 HTML image가 모두 있어야 후보로 취급하고, 그 외는 `Ok(None)`이다. text-only/image-only tests 통과. PASS |
| 5 | 다른 작업 ID 이름이 FFI route와 앱 matcher 사이에서 어긋남 | 네 scene ID를 Rust FFI, Android scene list/predicate/switch, iOS scene 및 mixed-ID set에서 대조했다. 양 플랫폼 8/8. PASS |
| 6 | 새 root/item ID가 기존 fixed prefix를 재사용함 | FFI source는 네 root와 네 유형의 item ID에 `novel-generic-mixed-*`를 사용한다. route test가 식별자 prefix와 ID 독립성을 검사한다. PASS |
| 7 | generated HTML source에 script injection 또는 길이 증가가 생김 | ID와 scene 문자열은 Rust 정적 문자열이며 source가 4 KB 미만임을 FFI test에서 확인한다. PASS |
| 8 | source order와 `nodeIds` 순서가 DOM과 달라짐 | FFI source 생성기는 item order대로 append하고 해당 ID를 바로 push한다. Android/iOS reversed scenes에서 source-order trace 통과. PASS |
| 9 | CSS 입력이 기존 Chrome/Ahem/PNG 기준과 달라짐 | fixed 120×48 root, two 24px rows, Ahem `XX XXXXX`, 24×16 PNG와 zero edges를 비교했다. app traces/frame expectations와 일치. PASS |
| 10 | root minimum/maximum column 외 track 형태가 섞임 | adapter는 intrinsic column 단 하나만 선택하고 기존 style validator에 나머지 root constraints를 위임한다. bad track negative 통과. PASS |
| 11 | malformed source가 generic direct-text path로 부분 처리됨 | mixed signature가 있는 오류는 `Err`; exact topology/content/style을 벗어나면 partial input을 반환하지 않는다. malformed tests 통과. PASS |
| 12 | CSS validator가 unsupported root alignment/direction을 허용함 | `justify-items`, `direction`, `writing-mode`와 writing-style snapshot을 별도로 확인한다. Runtime matrix에서도 exact profile 통과. PASS |
| 13 | item placement 또는 `img` natural dimensions가 덮어써짐 | item validator, leaf check, exact src, decode, 24×16 dimensions를 모두 요구한다. trace가 replaced natural size를 보고했다. PASS |
| 14 | document/style revision이 서로 다른 snapshot에서 결합됨 | generation, document, render-tree revisions와 profile을 동일 snapshot에 대조한다. stale/profile negatives 통과. PASS |
| 15 | intrinsic revision mismatch 회귀 수정이 모든 validation을 완화함 | measurements와 item inputs는 전체 revision equality를 유지한다. text layout만 source/style/environment tuple을 비교한다. trace test 6/6. PASS |
| 16 | text-layout revision fix가 화면/레이아웃 결과를 바꿈 | helper는 trace serialization validation에만 있고 compute/layout input에는 손대지 않았다. final source simulator frames/tracks 통과. PASS |
| 17 | stale text layout source/style/environment mismatch가 통과함 | 새 helper test는 intrinsic-only 차이는 허용하고 style/environment 변경은 거부한다. trace report의 source-vs-runtime 검사는 별도로 유지된다. PASS |
| 18 | text layout과 grid item frame 수의 불일치를 숨김 | reporter의 item-input/frame set, text-layout owner validation은 수정하지 않았다. 두 플랫폼 matcher가 frame/node identity 검증을 통과했다. PASS |
| 19 | FFI route가 unrelated generic direct-text scene을 가로챔 | exact scene ID lookup이고 고정 mixed route 다음 위치에 있다. 기존 direct mixed static controls와 generic direct-text legacy fallback 근거가 유지된다. PASS |
| 20 | Android가 새 generic scene을 grid fixture 아닌 mixed image/text로 분류하지 못함 | `C114_GRID_TRACK_SIZING_SCENE_IDS`, direct mixed predicate, source-order expected intrinsic switch를 모두 추가했고 앱 실행 8/8. PASS |
| 21 | Android node number mapping에서 reversed scene이 바뀜 | forward는 text 2/image 4, reversed는 image 2/text 3 mapping으로 고정했다. 두 reversed runtime traces 통과. PASS |
| 22 | iOS helper가 generic suffix 때문에 min/max/reversed를 잘못 판정함 | direct mixed ID set에 신규 scene을 넣어 기존 min/max prefix와 reversed suffix helpers를 재사용한다. 네 scene 모두 통과. PASS |
| 23 | simulator runner가 ADB에 보이는 실기기를 선택함 | Android invocation은 `-s emulator-5554`를 직접 사용하고 manifest는 API 37, ARM64, page size 16 KB, qemu를 검증한다. physical Samsung은 대상으로 지정하지 않았다. PASS |
| 24 | iOS runner가 다른 booted simulator를 캡처함 | UDID를 고정하고 iPhone-17-Pro device type 및 iOS-26.2 runtime을 검사한다. final matrix manifest도 동일 UDID/name/runtime. PASS |
| 25 | 캡처 뒤 실행한 앱 프로세스가 남음 | Android `finally` force-stop과 iOS `finally` terminate를 실행한다. PID/launchctl 확인에서 해당 Spinon 앱은 없고 simulator만 booted였다. PASS |
| 26 | evidence output 경로 변경이 iOS screenshot verifier를 잃게 함 | verifier 경로는 capture script 디렉터리를 기준으로 한다. initial path issue는 보존했고 final iOS build/run 8/8을 다시 통과했다. PASS |
| 27 | scene filter가 기존 capture-script 사용법과 default coverage를 깨뜨림 | 단일/comma-separated filter 모두 유효성 확인 후 사용하고, filter 미지정이면 기존 static four scenes가 기본이다. Android/iOS scene matrix 통과. PASS |
| 28 | 대표 screenshot 선택이 미선택 scene에 캡처되거나 검증되지 않음 | screenshot filter는 현재 scene subset에 포함돼야 한다. 모든 scene은 runtime/frame/trace/draw 검증, 각 platform의 static+generic representative만 PNG/OCR 검증한다. PASS |
| 29 | evidence manifest가 app build와 실행 source를 묶지 못함 | 각 accepted run에 source input hashes, capture driver hash, app artifact hash, scene run IDs, logs, screenshot hashes가 있다. Android/iOS source, diff, capture-script hashes가 서로 일치한다. PASS |
| 30 | PR 화면·계획 상태가 C11.4 전체 완료를 암시함 | README와 갱신한 집중 계획은 fixture-only, no glyph/image texel proof, no WPT/general admission을 명시하고 `72/213` 및 C11.4 미완료를 유지한다. PASS |

검토 후 blocker는 남지 않았다. Failed attempts are preserved separately and excluded from the accepted matrix. GitHub PR check rollup/merge state는 PR 생성 후 다시 확인한다.
