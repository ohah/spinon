# C11.4 overflow automatic minimum runtime 계획 · 실패 관점 20개 검토

검토 대상은 [axis-specific runtime 계획](./c11-4-overflow-auto-minimum-runtime.md), 고정 fixture와 [14-case Chrome 사전 비교](../spec/internal/evidence/c11-4-overflow-auto-minimum-runtime-precomparison-2026-10-28/README.md)다. 구현 코드에는 적용하지 않은 계획 검토다.

| # | 실패 관점 | 확인 결과 및 판정 |
|---:|---|---|
| 1 | Grid §6.6의 자동 최소 분기와 다른 규칙을 기대값으로 삼을 수 있음 | 기존 14-case 규범 계산을 재사용하고 대상 축의 computed overflow를 확인하도록 했다. PASS |
| 2 | `hidden`/`auto`/`scroll`을 non-scrollable로 분류할 수 있음 | CSS Overflow §3.1에서 scrollable 집합을 확인했다. PASS |
| 3 | cross-axis computed value를 authored value와 동일하다고 오인할 수 있음 | 대상 축만 값을 바꾸고 반대 축 `visible`을 `auto`로 정규화하는 기대를 독립 기록했다. PASS |
| 4 | `clip`을 `hidden`/`auto`와 같은 automatic-minimum 분기로 다룰 수 있음 | target-axis `clip`은 non-scrollable control로 남고 content-based minimum을 기대한다. PASS |
| 5 | Row scene에 남은 `img` UA 기본 `overflow:clip`이 visible input을 오염할 수 있음 | 사전 fixture의 두 overflow 축에 visible을 명시하고 Chrome computed values로 확인했다. 초기 관측 오류를 수정했다. PASS |
| 6 | clip scene의 비대상 축을 실수로 clip 처리할 수 있음 | Columns clip은 y visible, Rows clip은 x visible로 분리해 observed computed styles와 대조했다. 초기 fixture 오류를 수정했다. PASS |
| 7 | Ahem 미로딩으로 `XXXXXXXXXX` min-content가 달라질 수 있음 | 관측 전에 `document.fonts.ready`와 `document.fonts.check('10px Ahem')`를 요구한다. PASS |
| 8 | image decode 전에 natural size를 0으로 읽거나 data URL 오류를 놓칠 수 있음 | 고정 SHA의 로컬 10×10 RGBA PNG를 사용하고 모든 image `decode()` 완료와 natural size를 확인한다. PASS |
| 9 | Image aspect-ratio가 Columns 또는 Rows target size를 섞을 수 있음 | Columns는 text, Rows는 `10px` CSS width의 natural `10×10` image만 사용한다. PASS |
| 10 | Columns item의 wrapping mode가 달라져 min-content 100px이 변할 수 있음 | 고정 Ahem font, 10px line-height, 공백 없는 10자 word와 baseline hash를 고정했다. PASS |
| 11 | 지정 `min-width/height:0`을 automatic minimum 성공으로 잘못 셀 수 있음 | 두 axis별 지정 최소값 control은 `SpecifiedMinimum(0)` source/value를 따로 요구한다. PASS |
| 12 | `minmax(0,1fr)` control에서 item Minimum query가 호출될 수 있음 | zero-track-min은 exact query 부재를 검사하도록 했다. PASS |
| 13 | Columns 분기와 Rows 분기를 하나의 축으로 혼동할 수 있음 | 7개 scene씩 분리하고 axis, item owner, track 배열, contribution source를 matcher에 포함한다. PASS |
| 14 | 최종 frame만 맞고 contribution source가 틀린 경로를 놓칠 수 있음 | Minimum source/value와 모든 track stage를 별도 비교하고, `1fr` max에서 불필요한 item MaxContent trace가 생기지 않는지 확인하도록 했다. PASS |
| 15 | 오래된 document/style/environment revision의 trace를 최신 frame으로 오인할 수 있음 | report와 draw identity에서 같은 revision binding을 검사하도록 했다. PASS |
| 16 | 알 수 없는 root, 깊은 item, 다른 text/image로 fixed route에 진입할 수 있음 | exact root/item topology, node kind/content, image dimensions 및 scene allowlist를 fail-closed 조건으로 명시했다. PASS |
| 17 | malformed input이 이전 frame이나 일부 scene을 게시할 수 있음 | negative route는 layout/frame publication 이전 오류 원자성을 검사하도록 했다. PASS |
| 18 | `overflow:clip` 또는 `auto`의 실제 clipping/scrolling을 입증하지 않은 채 주장할 수 있음 | clipping, scroll offset, scrollbar, compositor-after pixels를 명시적으로 범위 밖으로 뒀다. PASS |
| 19 | `visible` scene의 box가 root 밖으로 나가도 simulator 검증이 root 내부만 확인할 수 있음 | 두 플랫폼의 visible scene에 root 밖 고정 PixelCopy/Metal sample을 포함하도록 했다. PASS |
| 20 | fixture FFI/payload가 제품 archive로 유출되거나 simulator/browser 화면을 서로 오인할 수 있음 | product/off와 opt-in positive archive 대조, 화면 출처·platform 한계 표기를 gate에 넣었다. PASS |

구현 중 추가된 입력 검토: Chrome 156에서 root alignment를 `normal`로 두면 overflow automatic minimum이 0인 5px row control의 replaced image item frame이 10px이 되고, `stretch`에서는 5px이 된다. 따라서 fixture와 사전 비교의 공통 root에 `align-items:stretch`를 명시하고, 해당 값 수용을 전용 C11.4 overflow projection option으로 제한했다. 나머지 profile은 계속 거부한다. 이 차이는 [alignment contrast 관측](../spec/internal/evidence/c11-4-overflow-auto-minimum-runtime-precomparison-2026-10-28/chrome156-align-items-contrast.json)으로 남겼다.

검토 중 사전 fixture의 UA `overflow-y` 누락, Rows clip의 반대축 선언, malformed SVG와 decode 순서 문제를 찾아 수정했다. 수정 후 Agent Browser HeadlessChrome 156에서 Ahem 및 image readiness를 확인하고 14/14 expected geometry·tracks·computed overflow 대조를 다시 통과했다. CSS Overflow 규칙도 W3C 문서 본문에서 재확인했다. 계획은 구현 전 기준 및 route isolation을 포함하므로 구현 검토 후 시작할 수 있다.
