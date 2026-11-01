# C11 parsed root HTML Grid runtime 계획 실패 관점 20개

검토 대상: [root HTML Grid runtime 계획](./c11-root-html-grid-runtime.md), Chrome 156 사전 비교와 고정 HTML source.

이 계획 검토는 구현 전 목표·입력·계약 경계를 살폈다. 코드, test, build, Android/iOS runtime 실행은 하지 않았다.

| # | 별도 실패 관점 | 확인 및 판정 |
|---:|---|---|
| 1 | C11.1 integration을 C11.4 완료로 승격 | 계획은 C11.1 root HTML 연결이며 C11.4·C11·72/213은 미승격이라고 적었다. PASS |
| 2 | fixture constructor만 바꾸고 실제 default entry를 건드리지 않음 | `RuntimeRootHtmlDefaultRender` production entry를 명시했다. PASS |
| 3 | C11.4 `TrackSizing` profile을 C11.1 explicit tracks에 잘못 사용 | 기존 `RuntimeBlockFlexGridV1` path가 기준이고 C11.4 fixture는 배경으로만 썼다. PASS |
| 4 | 현재 root renderer가 `grid`를 거부한다는 사실을 누락 | `RuntimeRootHtmlTextPaintV1`과 block/inline-only check를 코드 위치와 함께 기록했다. PASS |
| 5 | Grid 계산이 root HTML cascade 밖에서 실행 | parsed source, Stylo, typed projection, Taffy와 paint revision을 하나의 root HTML 경로에서 검사한다. PASS |
| 6 | fixture root/item NodeId에 의존해 성공 | root/item identity와 무관한 새 HostNode IDs 및 control을 계획했다. PASS |
| 7 | class/id/DOM index가 adapter selector로 사용됨 | CSS selector는 정상 author cascade로만 쓰고 adapter 성공 선택자는 금지했다. PASS |
| 8 | source order가 placement oracle를 우연히 만족 | child 순서를 역전해도 explicit numeric placement frame이 같은 control을 요구한다. PASS |
| 9 | CSS source가 실문서가 아닌 합성 `ComputedStyleSnapshot` | root HTML parser의 viewport meta와 inline stylesheet를 입력 경계로 고정했다. PASS |
| 10 | UA/body 기본 여백이 Grid 위치를 바꿈 | fixture에서 html/body margin 0, item/root box edges 0을 고정했다. PASS |
| 11 | track 산술이 Chrome 결과에서 순환 도출 | 독립적으로 40+80=120, 20+50=70을 계산하고 Chrome은 교차 확인으로만 쓴다. PASS |
| 12 | used track과 border box 또는 item offset 혼합 | 모든 frame을 CSS px root-relative 좌표로 정의하고 border/padding을 0으로 고정했다. PASS |
| 13 | expected의 item 순서/semantic identity 혼선 | 두 semantic class의 numeric placement와 각 frame을 명시했다. PASS |
| 14 | device scale/viewport가 오라클과 섞임 | Chrome viewport·DPR를 고정하고 app source hash 및 platform viewport를 별도로 기록한다. PASS |
| 15 | media, viewport meta, color-scheme 경로 회귀 | 구현 전 같은 root HTML profile의 environment·media·scheme 경계를 소스와 기존 regression으로 확인한다. PASS |
| 16 | unsupported Grid 선언이 Block fallback으로 성공 | unsupported input은 정확한 오류로 끝내고 Block fallback을 금지한다. PASS |
| 17 | stale document/style revision을 한 layout에 혼합 | source, computed style, layout revision과 runtime output identity를 매트릭스 수락에 넣었다. PASS |
| 18 | simulator frame 결과가 GPU/pixel 적합성으로 확대 | AVD/iOS Simulator geometry와 별도 readback의 한계를 분리했다. PASS |
| 19 | default build에 test-only fixture ABI/payload 유입 | product/default artifact에서 test-only selector·marker·payload가 빠지는 negative control을 요구한다. PASS |
| 20 | 새 integration이 전체 WPT/general CSS support로 게시 | 공개 CSS admission 전반, WPT, C11.3/4, 전체 C11과 공식 집계는 명시적으로 미완료로 둔다. PASS |

## 판정

**PASS 20 / PARTIAL 0 / FAIL 0.** 계획은 parsed root HTML author-CSS Grid integration을 기존 C11.1 typed path에 연결하는 데 집중하고 기존 environment와 C11.4 fixture 경계를 확인한다. 이 검토는 계획 수준이며 구현 승인이나 runtime 통과 증거가 아니다.
