# C11.4 anonymous Grid border overflow 구현 실패 관점 검토 20개

검토 범위는 새 Rust fixture·layout·render 경로와 두 native adapter의 실행 계약이다. Chrome 규범 계산과 계획의 위험 검토는 [계획 리뷰](../spec/internal/evidence/c11-4-anonymous-grid-border-overflow-precomparison-2026-10-21/plan-review-20.md)에 별도로 있다. PR diff 관점 검토는 [별도 기록](./c11-4-anonymous-grid-border-overflow-pr-diff-review-20.md)한다.

1. **기본 제품 경로에 fixture 코드가 섞일 수 있음** — FFI 호출·image attach는 `c04-runtime-gpu-fixture-test`, layout 준비·border adapter는 `c11-4-grid-track-sizing-fixture-test` 아래에 있다. [artifact byte scan](../spec/internal/evidence/c11-4-anonymous-grid-border-overflow-2026-10-21/feature-boundary/byte-scan.json)에서 기본 archive의 ABI, marker와 PNG payload가 모두 빠져 있다.
2. **새 fixture가 기존 모든 scene에 적용될 수 있음** — `runtime_source`는 두 ID만 반환하고 layout 준비는 두 scene ID만 허용한다. 유사 prefix지만 모르는 scene은 unsupported 오류로 닫힌다.
3. **HTML 문서 안에서 root를 찾지 못할 수 있음** — FFI attach는 document root부터 재귀 탐색하고 고정된 두 scene ID 중 하나만 받아들인다. 실제 Android/iOS 실행에서 두 scene 모두 report, draw까지 완료했다.
4. **중복 root에서 임의 scene을 고를 수 있음** — 선택 root가 정확히 하나인지 확인하고 중복이면 실패한다.
5. **child 순서나 종류가 달라도 통과할 수 있음** — layout fixture가 각 scene의 네 direct child 종류와 순서를 확인한다. text 두 개와 image 두 개 외의 topology는 거부한다.
6. **두 direct text run이 하나의 anchor로 합쳐질 수 있음** — 개별 `XX` text node 두 개를 추적하며 진단 입력의 anchor, 원문, source order를 검사한다. 두 순서 모두 simulator trace에 반영됐다.
7. **시스템 font 대체가 intrinsic 크기를 바꿀 수 있음** — fixture는 system font 없는 `TextLayoutEngine`에 고정 Ahem을 등록하고 min/max-content 크기 `20px`을 검사한다.
8. **PNG URL과 실제 asset이 달라질 수 있음** — image request URL을 고정하고 bundled PNG를 decode한다. natural dimensions가 `24×16`인지 확인한다.
9. **image 크기가 다른 revision의 결과일 수 있음** — 두 image completion은 단일 `ImageResourceSnapshot`에 모으며 layout 진단은 동일한 intrinsic-size revision임을 검사한다. render object는 document, render-tree, intrinsic-size key와 scene style revision을 보유한다.
10. **border 선언이 있어도 layout box가 border-box를 지키지 못할 수 있음** — style validation과 runtime test가 root `40×184`, 네 변 `2px`, content width `36px`을 따로 확인한다.
11. **border paint가 geometry와 따로 누락될 수 있음** — fixture profile render adapter가 네 side의 width와 opaque sRGB paint를 `RuntimeBorder`에 복사한다. style-to-render test가 width와 top paint를 직접 확인한다.
12. **border 지원이 다른 profile이나 제품 표면으로 번질 수 있음** — 변환은 C11.4 fixture-test feature 및 해당 runtime profile에만 적용된다. 다른 profile은 기존 mapping을 사용한다.
13. **grid template 변경이 숨겨질 수 있음** — validator가 4개 min-content column, 3개 60px row, gap 0, row auto-flow, 빈 line-name/areas 및 auto-repeat 부재를 검사한다.
14. **image의 row/column placement가 바뀌어도 허용될 수 있음** — 각 image는 고정 `grid-column`, `grid-row`, auto width·height, box edges와 aspect-ratio 입력을 검사한다.
15. **큰 image intrinsic contribution이 이전 text-only 로직을 탈 수 있음** — diagnostics는 source-kind, source order, span stages와 최종 tracks를 점검한다. 두 순서 모두 columns `[12,28,20,20]`, rows `[60,60,60]`이다.
16. **frame만 맞고 overflow가 잘릴 수 있음** — layout test는 큰 image 끝과 root border-box 끝의 차가 `22px`인지 확인하고 Android PixelCopy/iOS Metal은 border 밖 PNG sample을 읽는다.
17. **렌더 scene에 두 image가 붙지 않을 수 있음** — FFI attach는 정확한 두 image node를 찾아 현재 scene box에서 frame을 읽고 decoded RGBA를 `RuntimeRenderImage`에 연결한다. 실행 결과는 `runtime_image_count=2 image_draw_count=2`를 기록했다.
18. **CSS 기본 object-fit이 fixture에서 다를 수 있음** — 생략된 CSS 속성의 초기값 `fill`과 중앙 위치를 render object에 설정한다. natural `24×16`과 stretched `48×32` frame의 색 sample이 예상과 일치한다.
19. **Android/iOS가 scene을 다르게 인식할 수 있음** — 두 adapter 모두 scene allowlist, root/image frames, track trace와 sample 위치를 고정한다. 새 scene 2개와 기존 anonymous text + image scene 4개가 각 simulator에서 통과했다.
20. **단일 sample과 simulator 결과를 과장할 수 있음** — Android API 37 ARM64 AVD PixelCopy와 iOS 26.2 Simulator drawable에서 두 고정 RGBA 지점만 검증했다. README는 iOS compositor 이후, 전체 PNG texel, 물리 기기, WPT와 전체 C11.4를 명시적으로 제외한다.

이 구현 관점 검토에서 남은 수정 blocker는 발견하지 못했다. 실행 수치는 [시뮬레이터 근거](../spec/internal/evidence/c11-4-anonymous-grid-border-overflow-2026-10-21/README.md), 변경 파일과 제출 범위는 PR diff 리뷰에서 다시 확인한다.
