# C11.4 · depth-three nested Grid anonymous text + image 계획

**상위:** [C11.4 Grid track sizing](./c11-4-grid-track-sizing.md) · [depth-three 양 축 계획](./c11-4-nested-grid-depth-three.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md) · [공식 상태](../spec/STATUS.md#c11-css-grid)

**상태:** 계획 및 Chrome 156 사전 비교 완료. 구현은 시작하지 않았다. C11.4 checkbox와 공식 완료 수 `72/213`은 변경하지 않는다.

## 목적과 경계

현재 고정 fixture는 depth-three nested Grid의 양 축 `min-content`/`max-content` 전달을 direct Ahem text leaf로 확인한다. 별도 fixture는 한 Grid 안에서 anonymous text와 natural HTML image를 결합한다. 아직 이 두 입력원이 depth-three nested chain의 가장 안쪽 Grid에서 함께 기여하고, 그 크기가 세 부모 Grid 경계를 통과하는 고정 실행이 없다.

이번 slice는 `root → middle → branch → inner` chain 네 Grid에서 inner의 직접 anonymous text와 직접 natural PNG image가 `min-content` 또는 `max-content`의 두 축 sizing으로 같은 부모 contribution에 합쳐지는지 확인한다. 각 intrinsic mode에서 source order 두 가지를 실행해 총 네 새 scene을 만든다. 신규 scene은 C11.4 opt-in fixture-test 경로에서만 허용한다. 일반 cascade admission, 제품 API, 전체 CSS 지원 선언, 내부 계약 숫자 버전에는 변화가 없다. 기존 depth-one·two·three text 장면 열 개는 회귀 control로 같이 실행한다.

## 독립 기준과 입력

고정 HTML은 [precomparison fixture](../tests/fixtures/css/c11/grid-track-sizing-c114-nested-depth-three-anonymous-image-precomparison.html)다. CSS Grid 2025 CRD의 [§6 Grid Items](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#grid-items), [§6.1 Grid Item Display](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#grid-item-display), [§8.5 auto-placement](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#auto-placement-algo), [§11.5 intrinsic track sizing](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-content)을 Ahem metrics와 PNG natural size에 적용한다. 별도 Chrome 156 Agent Browser 관측은 규범 기대값을 계산하는 근거로 사용하지 않는다.

Ahem `XX XXXXXXX`의 max-content 폭은 `100px`, min-content 폭은 가장 긴 공백 없는 `XXXXXXX` 조각인 `70px`이다. 70px에서는 text가 `20px` 높이 두 줄, 100px에서는 `10px` 높이 한 줄이다. PNG natural size는 `24×16px`이다. 모든 box edge/gap은 0px, inner Grid의 두 column과 한 row는 현재 intrinsic mode와 동일한 sizing function을 쓴다. 세 바깥 Grid는 각 한 child와 단일 column/row track을 갖는다.

| 장면 | 모든 Grid frame | outer Columns / Rows | inner Columns / Rows | inner item frame |
| --- | --- | --- | --- | --- |
| min-content text → image | `94×20px` | `[94] / [20]` | `[70,24] / [20]` | text `(0,0) 70×20`; image `(70,0) 24×16` |
| min-content image → text | `94×20px` | `[94] / [20]` | `[24,70] / [20]` | image `(0,0) 24×16`; text `(24,0) 70×20` |
| max-content text → image | `124×16px` | `[124] / [16]` | `[100,24] / [16]` | text `(0,0) 100×10`; image `(100,0) 24×16` |
| max-content image → text | `124×16px` | `[124] / [16]` | `[24,100] / [16]` | image `(0,0) 24×16`; text `(24,0) 100×10` |

Chrome Headless `156.0.8078.48`은 네 scene 모두 이 독립 기대값과 일치했다. viewport는 `1280×633`, DPR 1이며 Ahem load와 PNG decode/natural size가 준비된 상태를 기록했다. [사전 비교 README, JSON, 화면과 hash](../spec/internal/evidence/c11-4-nested-depth-three-anonymous-image-precomparison-2026-10-22/README.md).

## 구현 및 판정 절차

1. 이 계획과 네 scene의 expected JSON을 고정한 후, 별도 계획 실패 관점 20개 및 계획 PR diff 검토를 마치고 계획 PR을 Rebase and merge한다. 기존 main checkout을 쓰며 새 worktree는 만들지 않는다.
2. 기존 nested depth scene adapter와 anonymous text/image fixture adapter 경계를 각각 추적한다. 신규 네 root ID만 명시 허용하고 정확한 `root → middle → branch → inner` ownership, inner의 direct child 두 개, text/image source order와 고정 style을 검증한다.
3. Ahem paragraph와 image intrinsic snapshot이 같은 document/style/layout revision으로 합쳐지는지 확인한다. 두 축 nested intrinsic projection과 anonymous/replaced 입력이 별도 opt-in이 필요하면 이를 fixture 전용 typed path에만 좁게 결합한다. 알 수 없는 shape나 중복·누락 resource는 부분 `LayoutOutput` 없이 실패해야 한다.
4. 네 신규 scene에서 각 Grid node의 parent/child order, Columns·Rows used track, width·height, root-relative frame 및 track contribution trace를 검사한다. Inner text item은 anonymous source/provenance와 Ahem line fragment, image item은 element identity·decoded natural size·intrinsic contribution source/revision을 확인한다. source-order 반전은 inner columns/item x 위치만 바꾸고 상위 크기는 유지해야 한다.
5. 기존 depth-one·two·three ten controls와 신규 네 scene 총 14개를 Android API 37 ARM64 AVD 및 iPhone 17 Pro/iOS 26.2 Simulator fixture app에서 실행한다. 모두 frame·두 축 track·Grid trace·draw/present와 installed/local artifact hash를 검사한다. Android 실기기는 연결하거나 사용하지 않는다.
6. `cargo fmt --all -- --check`, focused style-to-layout/runtime/FFI tests, workspace all-target Clippy `-D warnings`, Android/iOS fixture builds를 실행한다. 기본 product archive에서는 신규 scene id와 고정 fixture code/data가 없어야 하고 fixture-test positive control에는 포함돼야 한다. 실행하지 않은 검사는 통과로 보고하지 않는다.
7. 코드 변경 뒤 계획 검토와 겹치지 않는 구현/runtime 실패 관점 20개를 새로 검토하고, PR diff는 별도 30개 실패 관점으로 검토한다. Simulator 동작 화면을 PR 본문에 연결하고 Agent Browser에서 이미지 표시를 확인한다. 만든 Agent Browser 세션을 닫고 session 목록과 helper/Chrome 프로세스를 확인한다.
8. 현재 required checks와 mergeability를 다시 확인한 뒤 Rebase and merge한다. 병합 결과를 상태 대장·상위 계획에 연결하고, Tailnet 작업 미리보기 갱신 시 선택 빌드·백업·hash·Agent Browser 화면까지 확인한 후 브라우저 프로세스를 정리한다.

## 거부 조건과 한계

- scene id 외의 depth, direct item 수, node tag/order, track 수, sizing keyword, text, image path/natural size 변경은 지원하지 않고 fail-closed한다.
- font/image load 실패, revision mismatch, malformed topology, unsupported intrinsic query, trace overflow 및 Spinon-owned allocation failure는 partial output을 발행하지 않는다.
- four fixed scene은 arbitrary nested Grid, subgrid, span/multiple rows/columns, inline descendants, whitespace/comment/display-none edge matrix, authored image dimensions/ratio transfer, image pixel paint, border/overflow surface, WPT 또는 전체 CSS Grid 지원을 검증하지 않는다.
- Chrome screenshot은 DOM layout 관측이다. Simulator screenshot은 화면·프레임 보조 증거이며 glyph pixel equality나 iOS compositor 이후 표면을 주장하지 않는다.
- C11.4 status와 `72/213` 공식 집계는 계속 미완료로 유지한다.

## 리뷰 산출물

- [계획 실패 관점 20개](./c11-4-nested-depth-three-anonymous-image-plan-review-20.md)
- 계획 PR diff 실패 관점 30개는 precomparison evidence directory에 기록한다.
- 구현 후 독립 20-perspective review와 별도 30-perspective PR diff review를 만든다.
