# C11.4 anonymous text run과 replaced image의 bordered Grid overflow 계획

**상위:** [C11.4 track sizing](./c11-4-grid-track-sizing.md) · [anonymous Grid text](./c11-4-anonymous-grid-text-items.md) · [anonymous text와 복수 image](./c11-4-anonymous-text-multispan-images.md) · [공식 상태](../spec/STATUS.md#c11-css-grid)

**상태:** Chrome 155 Agent Browser 사전 비교와 독립 규범 계산, 계획 실패 관점 20개 검토를 마쳤다. 구현을 opt-in fixture로 추가하고 Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 forward/reversed 두 장면을 각각 확인했다. 두 장면 모두 columns `[12, 28, 20, 20]`, 48×32 image의 22 CSS px overflow, border와 overflow image의 고정 pixel sample이 통과했다. 기존 anonymous text + image 4개 장면도 양쪽 simulator에서 회귀 확인했고 기본 archive 경계 검사와 Clippy가 통과했다. [구현 실패 관점 20개](./c11-4-anonymous-grid-border-overflow-implementation-review-20.md)와 [PR diff 실패 관점 20개](./c11-4-anonymous-grid-border-overflow-pr-diff-review-20.md) 검토를 마쳤다. 실행 근거는 [README](../spec/internal/evidence/c11-4-anonymous-grid-border-overflow-2026-10-21/README.md)에 둔다. PR [#431](https://github.com/ohah/spinon/pull/431)은 2026-10-21 07:22:55 UTC에 Rebase and merge되어 merge SHA `420be2c40cb0a9f68023121f66ff122158494ffa`를 만들었다. PR 본문 screenshot 링크 두 개는 Agent Browser에서 모두 정상 표시됐다. GitHub는 branch checks를 보고하지 않았고 main required status checks도 설정되어 있지 않았다. Tailnet preview pass01은 원본 2,992개 파일의 HTTP 200과 SHA-256 일치를 확인했고 Agent Browser에서 roadmap·상태·계획·실행 근거 페이지와 Android/iOS 화면을 열었다. 상세 결과는 [Tailnet pass01 검증 기록](../spec/internal/evidence/c11-4-anonymous-grid-border-overflow-tailnet-sync-2026-10-21/README.md)에 있다. 이 문서 변경이 main에 병합되면 미리보기를 다시 동기화한다. 이 후보는 C11.4 체크와 공식 완료 수를 올리지 않는다.

## 목적

병합된 C11.4 fixture는 anonymous text 한 run과 복수 image를 함께 계산하지만 CSS border가 없고 image frame이 root 안에 있다. 별도의 auto-repeat 장면은 root 바깥 item overflow를 확인했지만 anonymous text와 image가 함께 있는 bordered Grid는 검사하지 않았다. 이 계획은 두 anonymous text run이 direct image에서 분리되는 동작, image multi-span contribution, solid border layout/paint와 `overflow:visible`인 image가 border box 밖으로 나오는 동작을 같은 제한 fixture에서 검증한다.

## 고정 입력

입력은 [Chrome fixture](../tests/fixtures/css/c11/grid-track-sizing-c114-anonymous-text-multi-image-border-overflow.html)에 고정한다.

- Grid root: `box-sizing:border-box`, `40×184px`, 네 변 `2px solid #f0c674`, 배경 `#101820`, `overflow:visible`, 네 `min-content` columns, 세 `60px` rows, gap 0.
- 직접 text: Ahem `10px/10px`의 `XX` 두 run. 각각 사이의 direct HTML `img`가 anonymous text item 경계가 된다.
- 이미지: 동일한 고정 `24×16` PNG, natural ratio `3:2`, 첫 image는 `height:16px`·column `1 / span 2`·row 1, 두 번째는 `height:32px`·column `2 / span 2`·row 2. 두 image item 배경은 `#e5484d`다.
- Scene: `anonymous-runs-two-images-forward`와 `two-images-anonymous-runs-reversed`. child order만 뒤집고 size, placement, asset, border와 다른 style은 고정한다.

## 독립 규범 기대값

기준은 [CSS Grid Layout Module Level 1, 26 March 2025 CRD §6, §8.5, §11.4, §11.5.1](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/)이다.

1. §6의 anonymous block-container item 규칙에 따라 서로 다른 direct text sequence는 별도 item이며, 중간 image는 별도 blockified grid item이다.
2. §8.5의 definite-position 처리 후 두 anonymous item은 row-sparse auto-placement로 첫 빈 cell인 row 1, columns 3과 4에 놓인다. 각 `XX`의 Ahem min-content는 20px이다.
3. 첫 image의 transferred inline contribution 24px은 columns 1–2에 12px씩 반영된다. anonymous text의 single-span 20px 기여로 column 3과 4가 각각 20px이 된다.
4. 두 번째 image의 48px contribution은 columns 2–3의 현재 합 32px보다 16px 크다. §11.5.1의 planned-increase/up-to-limit 분배에서 column 3은 20px growth limit에 머물고 나머지 16px은 column 2에 반영한다. 따라서 두 source order의 최종 columns는 `[12, 28, 20, 20]`이고 합은 80px이다.
5. root border-box 폭은 40px, content-box 폭은 36px이다. 두 번째 image의 Grid-local frame은 `(14, 62)`에서 `48×32px`이며 오른쪽 끝은 local x=62다. 따라서 image box는 root border-box 오른쪽 끝 x=40을 22px 넘는다. reverse-order scene에서도 같은 geometry여야 한다.

Chrome 155 관측은 규범 기대와 별도로 기록한다. 고정 실행에서 두 장면 모두 used columns `12px 28px 20px 20px`, root border `40×184px`, content width `36px`, second image overflow `22px`가 나왔다. Text Range rect는 glyph fragment 관찰이므로 anonymous item box frame의 대체값으로 사용하지 않는다.

## 구현 및 검증 관문

1. 기존 opt-in `RuntimeBlockFlexGridTrackSizingV1` fixture-test 경로에서 두 scene만 allowlist한다. 알 수 없는 topology/style/source를 부분 input으로 처리하지 않고 기존 fail-closed 오류를 유지한다.
2. 같은 document/style/layout revision에서 두 text run anchor/source order, 두 image node/natural size/revision, Columns/Rows stage trace, root/image/anonymous-item frame을 검사한다. CSS used columns는 두 순서에서 `[12, 28, 20, 20]`이어야 한다.
3. 각 scene에서 root border-box와 content-box 치수, 2px solid border geometry/color를 검사한다. `overflow:visible`인 두 번째 image frame이 Grid-local 오른쪽 border edge를 22 CSS px 넘는지 별도 비교한다.
4. 사용자가 선택한 기기 범위에 따라 Android API 37 ARM64 AVD 하나와 iPhone 17 Pro/iOS 26.2 Simulator 하나만 실행한다. Android는 `emulator-*` serial을 명시하고 physical ADB target을 사용하지 않는다. 두 플랫폼 모두 각 장면의 `drawPresented`, frame, trace와 화면 캡처를 보존한다.
5. Chrome screenshot에서 확인한 top border `#f0c674` sample과 border 바깥 `22px` 위치 안쪽의 PNG 단일 `#00ff40` sample을 Android PixelCopy와 iOS Metal drawable에서 고정 CSS 좌표로 비교한다. 이 한 점은 overflow item의 가시성을 확인하며 PNG 전체 texel equality를 뜻하지 않는다. DPR·Surface/viewport scale 변환을 기록하고 선택한 surface 범위만 주장한다. iOS readback은 compositor 제출 전이며 scanout 검증이 아니다.
6. Rust focused runtime/FFI tests, 기본·opt-in fixture artifact 대조, `cargo fmt --all -- --check`, workspace Clippy, Android/iOS fixture 앱 build를 수행한다. fixture ABI/scene/asset이 기본 제품 archive에 없어야 한다.
7. 구현 코드와 runtime 결과는 계획 검토와 중복되지 않는 새 실패 관점 20개로, 구현 PR diff도 사용자가 지정한 20개 관점으로 별도 검토한다. required checks가 실제 설정된 경우 모두 확인한 뒤 Rebase and merge한다.

## 범위 밖

이 slice는 고정 Ahem text와 PNG, 두 text run·두 direct image, 네 개 `min-content` track, opaque 단색 solid border, 두 source order와 가시 overflow 한 장면에 한정한다. text glyph raster, PNG 전체 texel equality, dashed/rounded/alpha border, clipping/scroll, 다른 positioning/stacking, arbitrary HTML/CSS, WPT, physical device, iOS compositor 이후 표면 및 전체 C11.4/CSS Grid 지원은 검증하지 않는다. 공식 C11.4 checkbox와 `72/213` 완료 집계는 유지한다.

## 근거

- [Agent Browser Chrome 155 관찰·화면·hash](../spec/internal/evidence/c11-4-anonymous-grid-border-overflow-precomparison-2026-10-21/README.md)
- [관찰 JSON](../spec/internal/evidence/c11-4-anonymous-grid-border-overflow-precomparison-2026-10-21/chrome155-observation.json)
- [Agent Browser 화면](../spec/internal/evidence/c11-4-anonymous-grid-border-overflow-precomparison-2026-10-21/chrome155-agent-browser.png)
- [계획 실패 관점 20개](../spec/internal/evidence/c11-4-anonymous-grid-border-overflow-precomparison-2026-10-21/plan-review-20.md)
- [구현 실패 관점 20개](./c11-4-anonymous-grid-border-overflow-implementation-review-20.md)
- [PR diff 실패 관점 20개](./c11-4-anonymous-grid-border-overflow-pr-diff-review-20.md)
