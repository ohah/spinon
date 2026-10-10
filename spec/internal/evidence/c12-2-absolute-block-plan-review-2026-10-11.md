# C12.2 Block absolute 계획·비교 기준 실패 관점 검토

**검토일:** 2026-10-11 · **범위:** C12.2 계획, 내부 계약 초안, Chrome fixture/reference · **성격:** 구현 전 계획 검토. Rust layout이나 모바일 기능이 동작한다는 증거가 아니다.

계획·계약·fixture를 CSS Position 3, pinned Chrome 154, Taffy 0.14.0 및 현재 HostDocument/layout 경계와 대조했다. 서로 다른 실패 관점 20개를 확인했고, 실제로 찾은 누락은 비교 fixture와 계획을 수정해 다시 캡처했다.

| # | 실패 경로 질문 | 대조와 판정 | 반영한 조치 |
| ---: | --- | --- | --- |
| 1 | 구현 전 Chromium oracle 없이 Taffy의 `Absolute` 결과를 정답으로 삼는가 | fixture/reference를 작성하기 전에 layout 코드는 수정하지 않았다. Chrome 실행 파일·DevTools revision·binary SHA가 고정돼 있다. | C12.2 구현 전에 23 case·82 node의 Chrome reference를 생성하고 source hash를 검사하도록 계획·명령을 고정했다. |
| 2 | absolute 자식이 원래 DOM parent에서 containing block을 잘못 상속하는가 | CSS owner는 가장 가까운 non-static positioned ancestor이며 source parent와 다를 수 있다. | `skip-static-wrapper`와 `nested-inner` owner를 별도 case로 만들고 source parent/owner를 독립 비교한다. |
| 3 | absolute 부모가 자손의 positioned containing block이 되지 않는가 | Chrome에서 `abs-parent` 자체가 absolute이고 `abs-nested`의 owner다. | nested absolute case와 계약의 owner walk에 absolute box를 포함했다. |
| 4 | containing block의 border edge를 원점으로 사용하는가 | 4px border·10px padding owner에서 inset 5px/7px target은 `(9,81)`이다. | border thickness를 제외한 padding-edge origin을 계약과 exact reference에 기록했다. |
| 5 | left/right percentage를 content width나 border-box width로 잘못 계산하는가 | 200px border-box에서 border 4px·padding 10px은 padding-box width 192px이고 25%는 48px이다. | 좌우 basis와 target 좌표를 별도 asserted case로 만들었다. |
| 6 | top/bottom percentage가 width basis를 사용하거나 높이 basis를 잃는가 | padding-box height 112px에서 top 10% computed value는 11.1875px다. | 상·하·좌·우 모두 지정한 fixture와 vertical basis assertion을 추가했다. |
| 7 | root element의 viewport와 CSS initial containing block을 동일 parent로 취급하는가 | root는 static이어도 직접 absolute child의 owner는 `viewport`이며 root의 CSS box는 별도다. | viewport fallback case와 synthetic viewport 비노출 조건을 계약에 넣었다. |
| 8 | Android safe area·Surface pixel·dp/point가 containing block 크기로 섞이는가 | Chrome 기준은 CSS viewport 360×800이며 device scale은 raster 단위만 바꾼다. | CSS px/environment revision을 기준으로 고정하고 DPR 1/2의 geometry 불변을 검사한다. |
| 9 | HostDocument를 실제 containing block 아래로 재부모화해 source order·수명을 바꾸는가 | `static-diff-absolute`의 source parent는 `static-diff-wrapper`, owner는 `static-diff-owner`다. | 이 차이를 직접 표현하는 추가 case와 parent/owner 독립 단언을 넣었다. |
| 10 | absolute box가 auto-height 부모나 in-flow sibling을 늘어뜨리는가 | auto parent height는 앞·뒤 Block 12px 두 개만 포함해 24px이고 뒤 형제 y는 292px다. | out-of-flow parent size와 sibling 위치를 exact reference로 고정했다. |
| 11 | 모든 inset auto인 box의 hypothetical flow 위치를 containing-block 원점으로 대체하는가 | Chrome의 `static-absolute` 위치는 preceding flow box 아래에 놓이며 flow probe와 일치한다. | normal-flow 변환 probe와 별도 containing-block owner 좌표 case를 추가했다. |
| 12 | static position을 계산할 때 nonzero margin을 버리거나 두 owner 좌표계를 혼합하는가 | 다른-owner case의 flow box는 left margin 3px·top margin 2px를 포함해 `(11,1472)`다. | flow probe target에 signed-axis origin과 margin을 넣고 reference를 다시 캡처했다. |
| 13 | 한쪽 inset만 auto일 때 static position을 강제로 적용하는가 | `right:13px`만 definite인 case는 remaining left를 width·margin에 맞춰 계산한다. | 단일 auto side fixture와 computed/used 좌표 단언을 둔다. |
| 14 | 양쪽 inset과 auto width/height에서 stretch 크기를 min/max 전에 잘못 계산하는가 | definite insets의 auto size는 172×60이며 별도 min/max case는 width를 130px로 clamp한다. | stretch와 min/max를 독립 case로 두고 cross-axis 크기도 비교한다. |
| 15 | auto margin을 지원하지 않거나 음수 inset·margin을 0으로 바꾸는가 | 가운데 정렬 auto margin, negative inset과 signed margin은 서로 다른 기준값을 가진다. | auto margin case와 `edge-margin-absolute` 음수 좌표 regression을 추가했다. |
| 16 | left/right 모두 definite일 때 LTR over-constraint에서 잘못된 면을 버리는가 | `direction:ltr`에서 left 7px가 used되며 owner border·padding을 합한 x는 9px다. | LTR direction과 opposite inset을 포함한 exact case를 고정했다. |
| 17 | box-sizing, padding, border, aspect ratio 조합이 inset 계산 뒤 덮이는가 | content-box는 border/padding을 더한 outer size 80×50, border-box는 60×30이며 ratio auto-height는 20px다. | C07.1–C07.3와 접하는 별도 layout cases를 두었다. |
| 18 | absolute `display:inline`을 inline fragment로 만들거나 BFC margin을 흐름 밖으로 흘리는가 | Chrome computed `display`는 blockification된 `block`; absolute Block 안 자식의 10px margin은 22px height에 포함된다. | computed blockification assertion 및 independent formatting context case를 넣었다. |
| 19 | `display:none` 하위 absolute node를 viewport root로 승격하거나 owner로 남기는가 | hidden parent·child는 box/owner가 없고, sibling은 독립적으로 표시된다. | `NoBox` expected owner 및 box presence를 독립 판정한다. |
| 20 | fixture 일부 통과를 WPT·CSS·모바일 완료로 과장하거나 기준을 바꿔 비교하는가 | 기준은 23 case·82 node, DPR 1/2·0.5 CSS px; 5 WPT 경로는 pinned commit에서 존재 확인만 했고 suite는 미실행이다. | WPT 실행 상태를 `not-run`으로 고정하고 C12.2 성공·모바일 smoke·하드웨어 GPU 한계를 계약과 계획에 명시했다. |

## 검토 뒤 남은 구현 전제

- Chrome 사전 기준 unit test가 pass해야 구현을 시작한다. 기준을 다시 만들 때는 별도 reference ID와 검토를 남기며 기존 JSON을 조용히 덮지 않는다.
- Block absolute의 layout-parent graph와 HostDocument source tree는 별개로 유지한다. static-position owner가 다를 때 실제 구현이 coordinate transform을 표현하지 못하면 그 조합은 명시 오류이며, 임시 parent frame으로 성공시키지 않는다.
- C14/C15 intrinsic text/replaced sizing, C10.3.5 Flex static-position, C11/X09 Grid, C16 ancestor effect, C17 RTL, C22 paint는 이 단계의 완료 증거로 취급하지 않는다.
- WPT 경로의 파일 존재 확인과 WPT 실행은 구분한다. 이 문서는 feature code review, Android/iOS runtime evidence, 공개 지원 완료 판정을 대신하지 않는다.
