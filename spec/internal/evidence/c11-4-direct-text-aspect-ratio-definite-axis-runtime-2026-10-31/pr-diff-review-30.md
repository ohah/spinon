# 구현 PR diff 실패 관점 검토 30개

**대상:** `feat/c11-4-direct-text-definite-axis`의 staged source, platform adapter, capture harness, plan/status와 최종 evidence. 구현/runtime 20관점 검토와 별도로 수행했다.

| # | PR diff에서 확인한 별도 실패 경로 | 결과 |
| ---: | --- | --- |
| 1 | 새 collector가 기존 `generic_direct_text` 파일에 중복 구현되거나 fixture 경계를 벗어나는가? | 별도 `aspect_ratio_definite_axis.rs` 하위 모듈로 분리되어 기존 dispatcher가 route한다. |
| 2 | 구현이 새 public CSS/API/FFI symbol을 우연히 노출하는가? | 전용 public API/ABI는 없다. 기존 opt-in runtime source route만 재사용한다. |
| 3 | `C11_4TextInput` 새 field 추가로 기존 초기화 경로가 무작위로 owner를 갖는가? | 기존 profile의 모든 constructor가 `None`을 명시하고, 새 profile만 ratio owner를 지정한다. |
| 4 | 새 runtime branch가 minmax 또는 content-box ratio 특례보다 먼저 실행되는가? | 기존 `aspect_ratio_minmax_node` 우선순위를 보존하고, 새 definite-axis branch를 그 뒤 별도 owner로 분기했다. |
| 5 | owner NodeId는 기록했지만 paragraph style map 누락 시 layout을 계속하는가? | fixture dispatch와 runtime layout 양쪽에서 owner membership을 확인하고 오류를 반환한다. |
| 6 | no-ratio controls가 새 custom transfer branch로 잘못 라우팅되는가? | Control의 owner는 `None`이고 기존 일반 text-paragraph layout 경로를 유지한다. |
| 7 | all-auto numeric ratio가 새 definite-axis collector에 가로채이는가? | axis dimension이 없으면 기존 automatic-ratio collector에 fall through하며 회귀 테스트가 이를 확인한다. |
| 8 | runtime fixture-test feature 없이 route가 빌드되는가? | 기본/product archive에서 신규 scene ID·module marker 0건, opt-in archive positive control에서만 발견된다. |
| 9 | grid child traversal에서 anonymous text와 element 후보를 섞어 수를 세는가? | direct child 종류별로 분리하며 meaningful anonymous text와 extra item은 거부한다. |
| 10 | text 값과 item 역할을 DOM source order와 다르게 해석하는가? | `A`/`AAAA`를 직접 읽어 short/long role을 지정하고 frame·source order를 모두 검사한다. |
| 11 | display:none child가 in-flow count에 포함되는가? | computed display가 none이면 후보 집합에서 제외한다. |
| 12 | replaced image가 generic direct text source처럼 취급되는가? | candidate validation 뒤 HTML image 존재를 찾아 unsupported source로 거부한다. |
| 13 | ratio를 가진 item이 long text이면서 width/height style owner인지 구분하지 못하는가? | `AAAA` item에만 typed ratio와 serialization을 일치시켜 owner를 고정한다. |
| 14 | duplicated long/short text item이 한 owner table에 덮어써지는가? | 중복 insert와 item 수/paragraph map size를 거부한다. |
| 15 | computed typed dimension과 CSS property serialization이 diverge하는가? | 둘을 각각 검사해 허용한 px 또는 auto 문자열만 통과시킨다. |
| 16 | fixture profile의 paragraph constraints가 ratio min/max 범위로 확장되는가? | definite-axis paragraph profile은 min/max를 허용하지 않고 minmax profile은 별도로 유지한다. |
| 17 | root validation이 columns만 보고 row/gap/alignment/direction을 누락하는가? | 고정 root shape를 기존 `validate_grid_root`로 확인하고 row auto 조건도 별도로 검사한다. |
| 18 | Ahem font 등록 실패를 기본 font으로 조용히 대체하는가? | system font 없이 engine을 만들고 고정 Ahem 등록 오류는 layout failure로 반환한다. |
| 19 | item paragraph style이 wrong revision 또는 baseline/text mode를 상속하는가? | snapshot revision과 제한 paragraph profile을 검사하며 nowrap 및 고정 font/line-height 조건을 요구한다. |
| 20 | `C11_4TextInput` 분기에 서술용 node ID만 있고 Taffy가 computed style ratio를 받지 못하는가? | owner 있을 때 일반 typed style-to-layout text path를 실행해 computed ratio를 Taffy Grid item에 전달한다. 두 방향의 최종 track/frame이 Chrome oracle와 맞는다. |
| 21 | Android adapter가 비율 결과와 control을 같은 expected value로 검증하는가? | width와 height ratio/control 네 case의 root, item, text measurement, trace 기대값을 각각 분리한다. |
| 22 | iOS adapter route가 Java와 다른 ratio scene set 또는 root identifier를 사용하는가? | scene ID, root/item, ratio 방향, control expected values를 네 scene마다 분리하고 iOS 55/55 matrix로 확인했다. |
| 23 | Pixel sample이 item 경계 바깥인데도 같은 색을 읽는가? | 첫 item, long item, unused root anchor 세 좌표를 각 4 scene에서 검사하며 양 플랫폼 12/12 exact다. |
| 24 | app이 다른 APK/IPA를 실행했는데 현재 build 소스로 귀속하는가? | Android local/installed APK SHA-256과 iOS local/installed executable SHA-256이 각 result metadata에 기록됐다. |
| 25 | C114 Swift scene selector가 prefix를 누락해 scene evaluation을 건너뛰는가? | definite-axis prefix 및 네 exact ID를 dispatcher/validator에 연결하고 모든 신규 장면에서 eval summary를 받았다. |
| 26 | 신규 Swift validator가 기존 one-item multispan에서 out-of-range index를 읽는가? | 첫 실행에서 실제 `Index out of range`를 찾아 prefix와 item-count guard가 먼저 평가되도록 수정했다. 최종 iOS 55/55가 수정 후 회귀 검증이다. |
| 27 | FFI test fixture가 네 source 외 임의 route를 생성하는가? | 고정 네 ID table과 unknown-ID negative control을 포함하며 unit test가 통과했다. |
| 28 | capture filter가 빈 matrix, unknown ID 또는 중복 ID로 성공처럼 보고하는가? | 선택 ID는 known/non-empty/unique인지 검사하고 `sceneCount`는 실제 선택 수를 기록한다. 기본 실행은 55 scene이다. |
| 29 | runtime observation과 oracle asset 사이 hash 연결이 stale plan/source를 가리키는가? | plan 수정 후 두 플랫폼 matrix를 다시 실행했고 result JSON의 모든 `sourceHashes`가 현재 파일과 일치한다. |
| 30 | 후보 slice가 공식 C11.4/C11 완료로 집계되거나 Simulator 근거를 physical/compositor 증거로 부르는가? | `STATUS.md`는 C11/C11.4 unchecked와 `72/213`을 유지하고, AVD software renderer 및 iOS pre-compositor 한계를 명시한다. |

**검토 결론:** 이 staged PR diff 범위에서 30개의 독립 실패 경로를 확인했다. 공식 C11/C11.4 완료, 일반 CSS admission, physical-device/hardware GPU, Spinon WPT 적합성 또는 iOS compositor 이후 픽셀을 승인하지 않는다.
