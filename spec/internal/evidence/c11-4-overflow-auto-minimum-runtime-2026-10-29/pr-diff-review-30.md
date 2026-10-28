# C11.4 overflow automatic minimum PR diff 실패 관점 30개

검토 대상은 이번 14-scene runtime 후보의 코드·테스트·문서·시뮬레이터 evidence diff다. main checkout 한 곳에서 순서대로 검토했다. 같은 항목의 표현만 바꿔 반복하지 않고 cascade, host 입력, typed projection, Taffy trace, app adapter, 화면 evidence와 공식 상태 경계를 따로 확인했다.

| # | 실패 관점 | 확인과 판정 |
|---:|---|---|
| 1 | scene catalog 밖 입력이 fixture route에 진입할 수 있음 | Runtime case와 FFI case가 정확히 14개 ID를 고정하며 미등록 root/scene ID는 거부한다. PASS |
| 2 | 잘못된 문서 root가 같은 ID로 route를 탈 수 있음 | root는 HTML div, 문서 최상위 자식, 지정 Grid profile이어야 하고 root ID도 scene과 일치해야 한다. PASS |
| 3 | Grid item의 owner나 tree shape가 바뀌어도 trace를 통과할 수 있음 | root의 direct child 하나, parent identity, item ID/tag, text leaf 또는 image leaf를 검사한다. PASS |
| 4 | Columns intrinsic input이 시스템 폰트나 다른 문자열에 의존할 수 있음 | 시스템 폰트 없는 text engine, 포함된 Ahem, 고정 10px 크기, 정확한 열 글자와 horizontal writing mode를 사용한다. PASS |
| 5 | Rows replaced input이 예상 PNG와 다를 수 있음 | 고정 URL을 document request와 연결하고 포함된 PNG를 decode해 10×10 natural dimensions를 확인한다. PASS |
| 6 | root style geometry가 fixture별 expected values와 달라질 수 있음 | 축별 root width/height, 1fr track, cross-axis fixed track, zero edges/gaps, align-items:stretch를 computed style에서 검증한다. PASS |
| 7 | item sizing이나 placement 변화가 minimum 결과에 섞일 수 있음 | direct grid item의 line 1–2 placement, width/height, min/max dimensions, edges와 computed overflow를 확인한다. PASS |
| 8 | authored overflow 값을 computed overflow 값으로 오인할 수 있음 | 두 축의 authored strings와 두 축의 computed enum을 별도로 저장하고 app report에서 실제 style 값도 비교한다. PASS |
| 9 | overflow 축의 computed normalization이 반대축에 잘못 적용될 수 있음 | hidden/auto/scroll target의 반대축 visible→auto와 clip target의 반대축 visible 유지가 case oracle 및 runtime report와 맞는다. PASS |
| 10 | clip을 scrollable overflow로 셀 수 있음 | clip scene은 content-based minimum source와 authored/computed clip을 요구하고 scrollbar나 clipping 동작은 주장하지 않는다. PASS |
| 11 | visible/clip content minimum이 zero로 처리될 수 있음 | 두 축의 visible/clip target trace는 각각 Ahem 100px과 PNG 10px Minimum source/value를 요구한다. PASS |
| 12 | hidden/auto/scroll automatic minimum이 content minimum으로 남을 수 있음 | 두 축의 scrollable target은 OverflowAutomaticMinimum 0px과 60px/5px track을 요구한다. PASS |
| 13 | specified item minimum control이 automatic minimum 성공으로 잘못 셀 수 있음 | min-width/height:0 control은 별도 SpecifiedMinimum source와 0px contribution을 요구한다. PASS |
| 14 | zero track minimum control이 item minimum 계산을 우회할 수 있음 | minmax(0,1fr) control은 60px/5px track과 target pass의 item Minimum query 부재를 요구한다. PASS |
| 15 | align-items:stretch 허용이 기본 profile에 새어 나갈 수 있음 | projection option 기본값은 꺼져 있고 stretch 허용은 track-sizing profile과 C11.4 overflow fixture flag가 함께 있을 때만 열린다. 기본 profile rejection 회귀 검사가 있다. PASS |
| 16 | overflow longhand 허용이 공개 author CSS admission으로 넓어질 수 있음 | overflow-x/y는 전용 fixture cascade override에서만 허용되고 기존 base-profile rejection 검사로 경계를 확인한다. PASS |
| 17 | visibility property allowlist가 미지원 collapse 값까지 통과시킬 수 있음 | inline 및 stylesheet declaration을 value까지 검사하고 visibility:collapse author stylesheet 거부 회귀 검사를 추가했다. PASS |
| 18 | cascade style과 typed GridContainerInput이 서로 다를 수 있음 | computed root track과 projected columns/rows, axis, placement mode, auto-flow, repeat, names, areas를 독립 검증한다. PASS |
| 19 | image natural size revision이 계산 결과에서 누락될 수 있음 | image decode가 만든 intrinsic revision을 CurrentLayoutInputs에 반영하며 latest AVD/iOS matrix에서 동일 revision identity를 통과했다. PASS |
| 20 | trace 누락 또는 잘못된 axis sequence가 정상 결과로 보일 수 있음 | 단일 Grid trace, 두 축 존재, Columns 시작과 연속 sequence를 검사하고 반복 same-axis subpass도 각 pass contribution을 검증한다. PASS |
| 21 | extra subpass 검증이 정상 Taffy trace를 거부할 수 있음 | exact-two-pass 조건을 제거했다. column-hidden-zero-minimum의 정상 추가 subpass를 허용하면서 축과 contribution 불변식은 유지한다. 수정 후 양 플랫폼 전체 matrix를 재실행했다. PASS |
| 22 | Minimum contribution이 다른 node나 span에서 와도 통과할 수 있음 | 대상 item, span 1, Minimum kind, flexible-track crossing 조건을 target subpass에서 고정한다. PASS |
| 23 | flexible maximum이 요청하지 않아야 할 intrinsic contribution을 조용히 받아들일 수 있음 | item의 MaxContent/MinContent 및 예기치 않은 contribution은 target pass validator에서 실패한다. PASS |
| 24 | zero-track control에 Minimum query가 추가되어도 화면만 보고 놓칠 수 있음 | track-minimum case는 target pass contribution 목록에 item Minimum이 없어야 한다고 검사한다. PASS |
| 25 | output frame이 예상한 root/item 외 node를 포함하거나 빠뜨릴 수 있음 | 고정 two-node frame set, node identity, root geometry와 item width/height를 각각 확인한다. PASS |
| 26 | FFI trace report가 다른 revision의 style/frame을 합칠 수 있음 | report는 observation의 root/item identity와 style revision, document/render-tree revision, 현재 frame 및 selected-axis stage에서 값을 읽는다. PASS |
| 27 | fixture scene이나 PNG marker가 일반 product archive에 들어갈 수 있음 | 일반 c04-runtime-gpu archive에서 route marker가 없고 fixture-test archive positive control에 14 scene ID와 PNG marker가 있음을 static archive scan으로 확인했다. APK/IPA 전체 배제라고 확대하지 않는다. PASS |
| 28 | Android 결과가 이전 surface 또는 다른 run의 draw를 증명할 수 있음 | Android presentation identity는 scene/request identity, surface generation, sequence를 비교한다. API 37 ARM64 AVD 14/14와 PixelCopy 표본 evidence를 연결했다. PASS |
| 29 | iOS 결과가 compositor 이후 화면이나 실기기 parity로 과장될 수 있음 | iPhone 17 Pro/iOS 26.2 Simulator 14/14를 확인했고 Metal 표본은 제출 전 drawable로만 기록한다. OS compositor·실기기 결과를 주장하지 않는다. PASS |
| 30 | visible overflow 표본의 경계 색이나 PR screenshot이 잘못된 장면을 가리킬 수 있음 | 양 플랫폼 row-visible screenshot과 원본 runtime logs를 대조했다. root 바깥 red PNG sample과 item 아래 black sample은 exact다. STATUS, conformance, parent plan과 내부 계약은 후보 scope를 기록하고 C11.4 미완료·72/213을 유지한다. 이는 border stroke, clipping 또는 scrolling 검증이 아니다. PASS |

## 검토 결과

이번 diff 검토에서 새 결함은 찾지 못했다. 구현/runtime 20관점 검토에서 발견한 image intrinsic revision 누락, row image draw attachment 누락, trace pass 수 오탐과 visibility value allowlist 문제는 수정한 뒤 해당 회귀 검사 및 양 플랫폼 14-scene matrix에서 다시 확인했다. 공식 C11.4 항목은 미완료이고 완료 수는 72/213으로 유지한다.
