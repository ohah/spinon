# C11.1 Root HTML intrinsic Grid tracks 구현 PR diff 적대 검토 20개

검토 대상은 `feat/c11-root-html-intrinsic-runtime-20261105`의 `0710e67b2` 기준 전체 변경이다. 계획 검토 20개와 구현/runtime 검토 20개를 재사용하지 않고, 병합 전 PR diff에 대한 별도 실패 경계 20개를 확인했다. 검토 중 발견한 runtime font advance 차이는 Chrome oracle와 app runtime 경로를 분리해 기록했고, text measurement와 독립된 layout trace 검사를 추가했다.

1. **intrinsic cascade flag의 profile 누출** — `allow_root_html_intrinsic_tracks`가 기본 `RuntimeRootHtmlTextPaintV1` 외 profile에 전달되는지 추적했다. profile 조건과 Root HTML profile projection 22/22 검사가 다른 경로에 typed intrinsic 값을 노출하지 않는 것을 확인했다.
2. **computed breadth의 문자열 재해석·방향 손실** — `auto`, `min-content`, `max-content`, percentage와 `fr`가 typed `GridTrackSize`/`MinMax`의 올바른 min·max 위치에 남는지 대조했다. 양 축 projection과 5×6 CSS-valid minmax matrix가 이를 확인한다.
3. **문법상 유효한 breadth 조합의 누락** — `minmax()`의 최소 5종과 최대 6종 조합을 모두 입력했다. 30개 조합이 typed projection을 통과하고 값·축·순서를 보존한다.
4. **무효 값이 CSS fallback 없이 지원 track으로 승격되는 경로** — `<flex>` minimum, negative/non-finite length·percentage/fraction, calc 및 unsupported breadth mutation을 확인했다. Projection이 해당 `grid-template-*`와 node를 진단하고 이 값들을 Taffy에 전달하지 않는다.
5. **repeat 확장 전 resource 제한 및 뒤쪽 overflow 우회** — integer `repeat()`에 intrinsic breadth를 넣고 track-count/line metadata 경계를 대조했다. 별도 aggregate overflow 변이는 첫 intrinsic track이 뒤의 큰 percentage/fixed 값을 숨기지 않고 partial layout을 내지 않는다.
6. **percentage basis의 축·크기 혼동** — positive definite width의 50% minimum/maximum과 percentage-minimum-plus-`1fr`를 Chrome observation 및 expected JSON과 비교했다. 양의 축 runtime matrix의 첫 item frame과 percentage floor 결과가 일치한다.
7. **flex maximum의 free-space/floor 오계산** — font-independent layout trace에서 `minmax(min-content, 1fr)` 160px 결과와 50% minimum + `1fr` + 100px 조합의 `[80, 100]` tracks를 0.02 CSS px 안에서 확인했다.
8. **`auto` stretch와 content alignment 합침** — `justify-content:start` intrinsic scene과 `normal` auto-stretch control을 나눠 비교했다. 기본 `normal`은 남는 definite space를 auto track에 반영하며 명시 `start`는 track 크기 계산과 구분된다.
9. **columns 전용 구현으로 rows가 빠지는 회귀** — direct `min-content-rows`/`max-content-rows` runtime scene, 양 축 style projection, 고정 row-height expected를 비교했다. rows 경로도 같은 typed intrinsic mode와 frame 검사에 도달한다.
10. **호스트 글꼴 차이가 잘못된 Grid 계산을 가리는 허용치** — 현재 runtime backend의 min-content item은 30.004883px, Chrome 156은 30.1094px였다. 통합 text frame에는 0.5px tolerance를 두고, font-independent track trace는 0.02px로 분리해 계산 오류를 가릴 수 있는 비교를 줄였다.
11. **multi-track span의 contribution 누락** — `grid-column: 1 / span 2` item을 Chrome 독립 expected 및 runtime frame과 대조했다. 두-track span scene이 root runtime matrix에 포함되고 다른 placement는 그 경계 검사에서 별도로 다룬다.
12. **hidden subtree의 sizing·paint 참여** — 긴 hidden text가 있는 두 번째 item에서 frame과 render box가 생기는지 확인했다. RuntimeSession assertion은 hidden node 양쪽을 모두 부재로 요구한다.
13. **max-content overflow의 root/container clamp** — 20px container의 `max-content` item을 확인했다. expected 60.2188px frame이 유지되고 container 너비로 줄어들지 않는다.
14. **auto와 one-axis placed item 혼합의 비결정 순서** — 고정 input layout test에서 auto item과 one-axis item을 섞어 실행했다. `InvalidGridPlacement`로 fail-closed하며, 허용한 span-only scene은 별도 positive matrix에 남는다.
15. **zero·auto·percentage container axis의 intrinsic sizing 진입** — width/height auto, percentage 및 zero mutations를 넣었다. `width`/`height` 진단, cascade ready, completed layout 부재를 확인했고 zero-basis 두 장면은 Chrome oracle only로 표시했다.
16. **gap/box edge가 intrinsic contribution에 조용히 섞이는 경우** — non-zero gap, margin, padding, border 변이를 검사했다. 각 장면은 속성 진단으로 거부되고 partial frame/render scene을 publish하지 않는다.
17. **범위 밖 template와 flow route 선택** — template areas, named lines, auto-repeat, intrinsic implicit tracks, dense/column flow가 intrinsic mode로 들어갈 수 있는지 확인했다. classifier/adapter 경계가 거부하며 implicit-row runtime mutation은 `grid-auto-rows` property를 보존한다.
18. **실패 layout의 부분 publish 또는 stale revision 혼합** — implicit/축/edge/aggregate negative mutations 11개 모두 completed layout이 없다. Positive 17-scene document는 completed layout key와 cascade key가 같고 54 boxes를 한 snapshot으로 publish한다.
19. **Android/iOS evidence의 입력·지원 범위 혼동** — AVD와 iPhone 17 Pro Simulator 앱 번들의 runtime fixture hash를 같은 1cb65aa1…7018과 대조했다. Android는 ANGLE/SwiftShader이고 두 OS screenshot은 visual smoke이며 PixelCopy/Metal drawable readback 또는 compositor pixel parity라고 주장하지 않는다.
20. **fixture 전용 기능의 제품 유출 및 완료 과장** — 제품 `c04-runtime-gpu` check와 fixture boundary 33/33을 확인했다. 전체 style/layout suites의 기록된 실패(394/404, 443/444)를 green으로 세지 않고, `spec/STATUS.md` 72/213 및 C11.1/C11 미완료를 유지한다.

## 판정

위 20개 관점에서 추가 수정이 필요한 구현 결함은 발견하지 못했다. PR 병합 전 최종 CI/merge 상태는 PR을 만든 뒤 별도로 확인하며, 빈 check rollup은 CI 성공으로 기록하지 않는다.
