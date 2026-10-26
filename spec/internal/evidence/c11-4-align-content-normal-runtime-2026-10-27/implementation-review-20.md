# C11.4 균등 auto 행 구현·런타임 20관점 검토

2026-10-27 기준의 구현·고정 입력·Android/iOS 결과를 계획 검토와 분리해 확인했다. 20개 항목은 각각 다른 입력, identity, artifact 또는 제품 경계를 본다. 정적 검토에서 발견한 renderer stage 수·Chrome 색/viewport·runner 대상/installed hash·오래된 checksum 문제를 수정하고 해당 항목을 최종 상태에서 재확인했다.

| # | 실패 관점 | 최종 판정과 근거 |
|---:|---|---|
| 1 | Scene ID suffix나 미등록 값이 admission되는가 | **통과** — Rust dispatch는 정확한 ID만 반환하고 suffix/unknown을 거부한다. 새 fixture 단위 test가 고정 문자열·unknown ID를 확인한다. |
| 2 | Scene 트리의 root, child 수·순서·높이가 달라지는가 | **통과** — 정적 source에는 root 하나와 높이 `10/20/5px` child 세 개가 있고 단위 test가 count/source를 확인한다. |
| 3 | fixture source·payload·FFI가 기본 제품 artifact에 섞이는가 | **통과** — default/fixture FFI scan은 각각 negative/positive control이다. Android/iOS host metadata ID는 native FFI 경계와 나눠 기록했다. |
| 4 | `justify-content`가 Chrome·CSS 입력·app source 사이에 달라지는가 | **통과** — 런타임이 허용하지 않은 `flex-start` 실패를 보존하고 세 입력을 `start`로 맞췄다. |
| 5 | 기대 tracks가 구현 결과에서 역산되는가 | **통과** — 독립 산술 `100 - 10 - 20 - 5 - 10 - 10 = 45`, 두 auto 행에 `22.5px`씩 배분해 `[32.5,42.5,5]`를 고정했다. |
| 6 | gutter를 row track으로 세거나 크기를 놓치는가 | **통과** — runner와 host matcher가 7개 alternating track/gutter entry, 양쪽 collapsed edge, 내부 gutter `10px` 둘을 확인한다. |
| 7 | child 순서나 frame이 최종 rows와 불일치하는가 | **통과** — 세 경로가 root와 node ID별 frame을 `1e-4 CSS px` tolerance로 검증한다. |
| 8 | 다른 Grid/root의 trace를 가져오는가 | **통과** — trace는 한 개, root node `1`, Columns 후 Rows, Rows pass 하나를 요구한다. |
| 9 | 일부 stage 누락·중복·순서 변경을 허용하는가 | **통과** — 세 matcher가 실제 여섯 stage를 정순·중복 없이 확인한다. 이전 4-stage matcher 실패는 보존했고 최종 시뮬레이터 run에서 통과했다. |
| 10 | 내부 track index·gutter·collapsed 식별이 엇갈리는가 | **통과** — index `0…6`, gutter parity, 양 끝 collapsed 상태와 각 base를 모두 비교한다. |
| 11 | stretch 전 base 또는 fixed 행 유지가 잘못되는가 | **통과** — Initialize `[0,0,5]`, 이후 stretch 직전 `[10,20,5]`, 마지막 `[32.5,42.5,5]`를 검사한다. |
| 12 | Android trace와 보고 layout revision이 서로 다른가 | **통과** — style/environment revision 및 HostDocument generation/document/render-tree revision을 대조한다. |
| 13 | iOS trace와 보고 layout revision이 서로 다른가 | **통과** — Android와 동일한 report↔diagnostics 및 HostDocument revision 연결을 검사한다. |
| 14 | Android environment와 draw/present가 다른 generation인가 | **통과** — viewport/root/box와 revision identity를 묶고 presentation generation·sequence와 nonempty surface를 대조한다. |
| 15 | iOS environment와 draw가 다른 generation인가 | **통과** — 동일 identity와 surface generation·presentation sequence의 일치를 요구한다. |
| 16 | Android PixelCopy가 잘못된 수·대상·색을 통과시키는가 | **통과** — 정확히 다섯 고정 sample ID/node/좌표/RGBA를 비교하고 최종 AVD log는 5/5 matched다. |
| 17 | iOS Metal readback 표본이 범위·색을 놓치는가 | **통과** — 고정된 다섯 ID/node/좌표/RGBA를 비교하고 최종 Simulator draw log는 다섯 색 모두 일치한다. |
| 18 | gutter나 root 밖 픽셀을 실제로 표본화하지 않는가 | **통과** — gutter는 `y=37`로 rows 사이에 있고 root 밖은 `x=160`으로 오른쪽 경계 바깥이다. 두 좌표 모두 viewport 안이다. |
| 19 | runner가 복수 scene·다른 iPhone·다른 installed app을 허용하는가 | **통과** — 대상 ID를 포함한 복수 scene과 단독 run-kind 선택을 simulator command 전 거부한다. 최종 run은 정확히 한 ID, iPhone `iPhone18,1`을 사용했고 Android APK와 iOS app executable의 설치본 SHA-256이 build 산출물과 일치한다. |
| 20 | Chrome oracle 색/viewport 또는 증거 checksum이 stale한가 | **통과** — Chrome root/body background와 viewport를 app fixture에 맞췄으며 CSSOM·5 pixel target/color가 기대와 일치한다. precomparison `SHA256SUMS`의 6개 항목 모두 재생성 후 검증됐다. |

**결과:** 최종 20/20 통과. 이 검토와 별도의 PR diff 30관점 검토는 PR 리뷰 단계에서 수행한다. 실기기, 전체 화면 pixel equality, 일반 CSS admission과 전체 C11.4 적합성은 이 검토 범위가 아니다.
