# C11.4 direct PNG `minmax(auto, max-content)` mixed-track precomparison

이 자료는 Stage B의 좁은 후속 image slice를 위한 사전 oracle다. 기록된 Chrome 관측만 완료했고, Spinon layout/runtime 결과는 아직 실행하지 않았다. 구현 후보 출력은 규범 기대값의 근거로 사용하지 않았다.

## 고정 입력

- 저장소 HTML 입력: [`grid-image-auto-max-mixed-track-c114-precomparison.html`](../../../../tests/fixtures/css/c11/grid-image-auto-max-mixed-track-c114-precomparison.html)
- 입력 SHA-256: `e3b6edfe8452d3674a9e905de10ddd96774b192ac73c4b3c46a5a42606c92c2b`
- PNG: [`c14-1-png-basic.png`](../../../../tests/fixtures/resources/c14-1-png-basic.png), 97 bytes, `24×16` natural pixels, SHA-256 `5d8c9509a0fec2288f02611a8ccc22a23a90033d1a96b894b439525398bce4a2`
- 문서 root와 item ID는 각각 `c114-image-auto-max-mixed-grid`, `c114-image-auto-max-mixed-item`이다. Root는 `100×40px` border-box, `minmax(auto, max-content) 17px` 두 column, column gap `5px`, row `auto`를 사용한다. Image에는 authored `overflow` 선언이 없다. Image는 첫 칸에 자동 배치되고 width/height가 `auto`다. Padding, border, margin은 0이다. 재캡처 기준 커밋은 `a33b245983fae050b66efb9ff259b3fe1132a051`이다.
- Chrome은 `320×80` CSS px viewport, DPR 1에서 Agent Browser로 열었다. local `file://` fixture라 HTTP preview server를 시작하지 않았다.

## 독립 규범 기대값

CSS Grid 2025 CRD §7.2.1에서 `minmax(auto, max-content)`의 최소 함수는 grid item의 automatic/content-based minimum을 사용하고 최대 함수는 item의 max-content contribution을 사용한다. §6.6 조건은 이 span-1 image에 해당한다. PNG는 자연 너비·높이와 비율을 가진 raster replaced object이며, 이 입력은 별도 size, min/max, padding, border, margin이 없다. 따라서 column 방향 minimum, min-content, max-content 기여는 각각 `24px`다. CSS Grid §11.4의 초기 intrinsic base/growth limit은 `0px`/`+∞`다. Container inline size가 definite이므로 §11.5 auto-minimum track은 minimum contribution을 사용하며, auto width image의 §6.6 automatic minimum이 `24px`다. 같은 item의 max-content contribution은 `24px`여서 intrinsic growth limit도 `24px`가 된다. 두 번째 fixed track의 base/growth limit은 모두 `17px`이다.

Definite `100px` content width에서 최종 non-gutter columns는 `[24, 17]px`, 사이 gutter는 `5px`다. 전체 track과 gutter가 차지하는 폭은 `46px`이라 `54px`가 남으며 `justify-content:start`가 이를 trailing side에 둔다. Auto row와 natural image height는 `16px`; `align-content:start`는 남은 block space를 row에 넣지 않는다. 따라서 예상 frame은 root `(0,0,100,40)`과 image `(0,0,24,16)` CSS px다.

CSS Grid §11.4에서 초기 unbounded growth limit은 `+∞`다. Taffy trace는 모든 non-finite 값(`+∞`, `−∞`, `NaN`)을 `None`/JSON `null`로 매핑하므로 `Initialize`의 expected `null`은 그 projection만 확인하며 내부 값이 `+∞`였음을 증명하지 않는다. CSS Grid §11.5에서 span-1 max-content maximum은 item의 max-content contribution으로 resolve되고, Taffy `IntrinsicItemBatch` trace는 그 batch update 뒤에 기록된다. 따라서 이 fixture의 해당 stage에서는 column `Some(24px)`, row `Some(16px)`를 기대한다. 이 mapping과 trace tolerance `0.0001px`는 [`spec-derived-expectations.json`](./spec-derived-expectations.json)의 `/independent_derivation/expected_trace`에 고정했다.

Chrome CSSOM의 `<img>` `clip/clip`은 non-scrollable overflow이므로 CSS Grid §6.6 condition과 양립한다. 이 observation은 Chrome UA CSSOM 값으로만 보존하며 Spinon overflow extraction/projection, clipping, scrolling 또는 paint 동작은 이 sizing slice에서 검증하지 않는다.

근거 문서: [CSS Grid Layout Module Level 1, 2025 CRD](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/) §6.6, §7.2.1, §10.5, §11.3–§11.6; [CSS Box Sizing Module Level 3, 2026-09-04 WD](https://www.w3.org/TR/2026/WD-css-sizing-3-20260904/) §2.2, §5.1–§5.2; [CSS Images Module Level 3, 18 December 2023 CRD](https://www.w3.org/TR/2023/CRD-css-images-3-20231218/) §4.1, §4.3.1. CSS Images CRD의 2026-10-26 원문 응답은 353703 bytes, SHA-256 `74eb4b475aa0845a50ebedb4bbb3ba7bf3966f975817d1c5cf8e3d512f53447e`다. 산술과 acceptance 값은 [`spec-derived-expectations.json`](./spec-derived-expectations.json)에 고정했다.

## Agent Browser 관측

Chrome `156.0.8078.48`, macOS `26.5.1` ARM64, DPR 1에서 이미지가 decode된 뒤 측정했다. HTML/CSS에는 authored item overflow 선언이 없다. Chrome computed item overflow x/y는 UA computed value `clip/clip`이다.

| 관측 | Chrome 값 | 독립 기대값 |
| --- | ---: | ---: |
| PNG natural size | `24×16` | `24×16` |
| item computed overflow x/y | `clip / clip` | overflow 동작은 범위 밖 |
| item computed width/height | `24px / 16px` | natural size `24px / 16px` |
| item min/max width | `auto / none` | `auto / none` |
| item display/position | `block / static` | `block / static` |
| item grid placement | column/row start/end 모두 `auto` | 자동 배치, span 1 |
| computed columns | `24px 17px` | `24px 17px` |
| computed rows | `16px` | `16px` |
| Grid frame | `(0,0,100,40)` | `(0,0,100,40)` |
| Image frame | `(0,0,24,16)` | `(0,0,24,16)` |

원 관측: [`chrome-agent-browser-observation.json`](./chrome-agent-browser-observation.json), 브라우저 메타데이터: [`chrome-agent-browser-metadata.json`](./chrome-agent-browser-metadata.json), 화면: [`chrome-agent-browser.png`](./chrome-agent-browser.png). 전용 Agent Browser session `spinon-c114-img-stageb-review04-20261026`을 닫은 뒤 session info는 `active:false,pid:null`, active session list는 비어 있음을 확인했다. 전용 Chrome profile `agent-browser-chrome-c35a8bf8-e838-4f74-84a3-915b2fb96f8a`의 Chrome/crashpad process match는 0개다. 남은 user Chrome Crashpad와 별도 HWPJS Agent Browser/Chrome session은 보존했다. Server는 실행하지 않았다.

캡처 연결 SHA-256: 입력 HTML `e3b6edfe8452d3674a9e905de10ddd96774b192ac73c4b3c46a5a42606c92c2b`; 관측 JSON `82c020de9cb5768d4ca15670b0a79d29fd746899e4a71b67eb56d78ebce635f4`; 브라우저 metadata JSON `5043ffde790fbbc68669394761c1ed6a08574e794b905fc62ca4e81448f1bb90`; PNG screenshot `4182f752a1f9f20e8530c9e166683eed6c916eaacecb2bcff1faf7d3c36625eb`. Observation와 metadata 양쪽은 같은 HTML fixture hash와 screenshot hash를 기록한다. Metadata는 `a33b245983fae050b66efb9ff259b3fe1132a051`의 capture source와 session cleanup 결과를 기록한다.

## Spinon runtime evidence

구현 candidate는 같은 document/source revision에 묶인 `ImageResourceSnapshot`을 기존 layout intrinsic snapshot으로 전달해야 한다. Columns/Rows의 `Minimum` contribution은 `AutomaticMinimum` source의 전체 flags와 세 `None` fixed-limit 값을 JSON과 대조하고 min/min-content/max-content 입력도 비교한다. CSS track 수와 raw trace slot 수는 다르다. 2개 CSS column은 collapsed boundary sentinel을 포함해 5개 internal entries, 1개 CSS row는 3개 entries를 가진다. Column의 실제 inter-track gutter는 idx2에서 `Initialize`부터 `5/5`이고, boundary sentinel은 idx0/4에서 `0/0`이다. `Initialize`의 intrinsic track growth projection만 `null`이고, post-update `IntrinsicItemBatch`부터 기대 growth values는 finite `24/16px`다. 자세한 slot mapping은 [child plan](../../../../plan/c11-4-image-auto-max-mixed-track.md)과 JSON `/independent_derivation/expected_trace`에 고정했다.

첫 Android API 37 ARM64 AVD 및 iOS 26.2 Simulator candidate run에서는 runtime `status=0`, intrinsic size, source provenance 및 frame은 관측했으나 기존 native matcher가 구형 trace slot/stage 기대를 사용해 양쪽 검증이 실패했다. 해당 raw logs는 [runtime evidence directory](../c11-4-image-auto-max-mixed-track-runtime-2026-10-26/) 아래 run ID `c114-image-auto-max-20261026T000220Z-040a5360`에 보존했으며 passing evidence가 아니다. 새 stage contract를 반영한 app matcher의 후속 run은 별도 run ID로 기록한다. CSSOM은 Chrome 비교 oracle이고, C11.4 stage trace 판정의 대체물이 아니다. Native screenshots는 representative simulator UI 증거이며 PixelCopy/Metal readback sample을 대신하지 않는다.

Overflow 동작은 이 sizing slice의 runtime 계약 밖이다. Positive fixture에는 overflow 속성을 author하지 않는다. Chrome의 `clip/clip`은 browser CSSOM 관측으로만 기록하며 app computed value, projection, clipping, scrolling 또는 overflow paint behavior를 주장하지 않는다. Runtime acceptance는 image natural-size contribution, final tracks, frame, revision provenance로 한정한다.

## 자료 한계

이 precomparison은 Chrome 값과 규범 계산을 고정한 자료다. Spinon Taffy 출력, native 앱 frame, stage trace, WGPU surface, PixelCopy, Metal drawable, artifact feature boundary 또는 WPT를 검증하지 않았다. 이 단일 direct image case는 일반 CSS admission, 다른 image source/format, text 혼합, span, nested/anonymous items, row-axis intrinsic sizing, percentage/calc, auto-repeat, overflow, 전체 C11.4 지원을 뜻하지 않는다.
