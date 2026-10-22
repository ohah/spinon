# C11.4 mixed cross-axis span groups PR diff 실패 관점 검토 30개

대상 diff는 C11.4 opt-in Rust fixture/test, Android/iOS harness, Clippy가 요구한 adjacent test cleanup, internal contract, official status ledger, execution evidence·screenshots 및 Tailnet 미리보기 상태 기록이다. 구현·runtime 20개 검토와 별도다.

| # | Diff 경계 | 확인 결과 |
| ---: | --- | --- |
| 1 | 새 C11.4 scene이 제품 runtime에 무조건 포함되는가 | `c04-runtime-gpu-fixture-test` 경로 아래에 있고 default release archive에서 marker가 없다. 통과. |
| 2 | fixture module이 정식 `fixtures/` 경계를 벗어나는가 | runtime 및 FFI scene source가 각 `fixtures/c11_4/` 아래에 있다. boundary test 통과. |
| 3 | anonymous text scene이 너무 넓은 prefix matcher에 들어가는가 | `anonymous-`가 pure text scene을 text/image adapter로 보낸 실패를 재현했다. matcher를 `anonymous-text-image-`로 좁혀 회귀 matrix에서 pure text와 text/image 모두 통과했다. |
| 4 | intrinsic/definite profile이 잘못 교차되는가 | scene별 width가 `min-content` 또는 `40px`으로 고정된다. source test 통과. |
| 5 | grid tracks/gap이 다른 계산값으로 변하는가 | 각 축 3/2 track, exact `minmax(calc(10% + 2px),max-content)`, 2px gap을 adapter test가 검사한다. 통과. |
| 6 | item C의 span이 span-2로 잘못 줄어드는가 | A/B/C placement 문자열은 각각 span 2/2/3이고 source test가 세 placement를 확인한다. 통과. |
| 7 | reverse order에서 item identity가 사라지는가 | `data-node`, item key, stable ID 목록을 유지하고 child insertion만 역전한다. 통과. |
| 8 | A/B/C의 fixed geometry style이 다른 scene 간 달라지는가 | B는 40×10px, A/C는 auto size, `min-width/height:auto`로 고정한다. 통과. |
| 9 | HTML text metrics와 CSS layout input이 달라지는가 | fixture test가 exact text, Ahem 10px font 및 text-node 수를 확인한다. 통과. |
| 10 | app harness가 root 외 child frame을 generic single-item 검사로 건너뛰는가 | mixed fixture는 specialized 4-frame verifier를 사용한다. root와 세 node 검사. 통과. |
| 11 | root border box만 맞고 child order/identity는 바뀌는가 | verifier는 `node=2/4/6`과 forward/reverse A/B/C frames를 함께 검사한다. 통과. |
| 12 | regex parse가 fractional x/width를 자르는가 | Android/Swift parsers는 decimal 값을 읽고 `1/64 CSS px` 이내로 비교한다. 통과. |
| 13 | reverse sourceOrder가 고정 A/B/C metrics로 검사되는가 | intrinsic input validator는 reverse metrics `[C,B,A]` 순서와 sourceOrder를 별도 기대한다. 통과. |
| 14 | text diagnostics가 누락되거나 malformed여도 pass되는가 | Android JSON parsing 및 iOS dictionary parsing은 필수 input/세 항목 조건이 없으면 false를 반환한다. 통과. |
| 15 | font revision을 지정하지 않은 값이 통과되는가 | 각 input의 `fontRegistryRevision=1`을 검사한다. 통과. |
| 16 | 부정확한 initial intrinsic revision 검사로 정상 최초 계산을 거부하는가 | revision 1 가정을 제거했고 관측된 최초 revision 0을 유효하게 처리한다. 통과. |
| 17 | root box가 item count에 포함되지 않는가 | Android/iOS 기대 box count는 4다. 통과. |
| 18 | Android harness allowlist가 new IDs를 누락하는가 | 두 definite mixed IDs를 pixel matrix allowlist와 sample dispatch에 포함했다. 빌드 통과. |
| 19 | Android expected sample이 기존 two-item coordinates를 재사용하는가 | mixed definite sample은 item B overflow x=50, clear x=60 CSS px를 확인한다. validator 코드 통과. |
| 20 | Android sample이 item이 아닌 root에 매핑되는가 | overflow sample은 item B ID, clear sample은 root ID로 지정한다. 코드 통과. |
| 21 | iOS Metal validator가 기존 scene switch에서 빠지는가 | 두 definite mixed IDs를 same two-sample Metal case에 추가했다. build 통과. |
| 22 | 기존 C11.4 pixel oracle이 실제 border color와 어긋나는가 | Android/iOS/canonical JS 기대값에서 root green border와 dark negative root edge를 고쳤고 cycle scene의 border 없는 배경 표본은 보존했다. 전체 표본 재실행 exact. |
| 23 | Android frame summary를 Surface presentation으로 오해하는가 | Android API 37 AVD host GPU 전체 matrix에서 263/263 draw/present, PixelCopy 219/219 exact를 확인했다. iOS 전체 matrix도 별도로 확인했다. |
| 24 | iOS Metal readback을 compositor 이후 화면 근거로 과장하는가 | 문서에서 drawable submission 전 readback으로 한정한다. 통과. |
| 25 | screenshot을 glyph paint 근거로 표현하는가 | README와 PR image label은 box scene만 나타내며 glyph raster 미검증으로 한정한다. 통과. |
| 26 | default artifact가 opt-in symbol을 포함하는가 | 현재 default release archive `417f7d…e3b5`에 scene IDs와 C11.4 fixture entry symbols가 없다. 통과. |
| 27 | opt-in artifact가 positive control marker를 잃는가 | iOS Simulator archive `a2b311…176e`에 scene IDs와 fixture FFI symbols가 있다. 통과. |
| 28 | analyzer/toolchain 경고를 fixture 결함으로 잘못 처리하는가 | 기존 overlapping-rows test에서 map key 미사용 lint를 `.values()`로 바꿨고 Clippy가 통과한다. 동작 변경 없음. |
| 29 | roadmap edit가 C11.4 parent/count/API version을 승격하는가 | checkbox, `77/213`, internal version `0.1.0`, 공개 API 수를 그대로 유지한다. 통과. |
| 30 | evidence가 플랫폼 실패나 미리보기 동기화 제한을 숨기는가 | README는 SwiftShader AVD watchdog 원인과 host GPU 재실행, simulator/pre-compositor 한계를 공개한다. Tailnet pass50은 7,033개 선택 route의 preflight/copy/local hash와 backup 459/459, 원격 HTTP 200·hash 7,033/7,033을 기록하고 삭제는 하지 않았다. Agent Browser에서 roadmap의 `263/263`, `219/219`, `77/213`, STATUS의 unchecked C11.4, 계획·실행 근거와 Android/iOS 이미지 natural size를 확인한 뒤 해당 session을 닫았다. 통과. |

이 검토의 구현 근거는 [`README.md`](./README.md)와 [최종 full matrix evidence](../c11-4-pr515-prereq-regression-2026-10-22/README.md)다. PR screenshots는 commit-pinned `raw.githubusercontent.com` 경로로 연결하고, Android AVD host GPU와 iOS pre-compositor Metal의 증거 경계를 유지한다.
