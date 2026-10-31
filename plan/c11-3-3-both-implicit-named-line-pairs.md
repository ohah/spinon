# C11.3.3 양 끝 positive implicit named-line placement

**상태:** 계획 PR 전 검토 완료 · 구현 전 · 공식 C11/C11.3 체크와 `72/213`은 유지한다.

**상위:** [C11 CSS Grid](./c11-css-grid.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md)

**규범:** [CSS Grid Layout Module Level 1, 2025-03-26 CRD §§7.5–7.6, 8.3–8.3.1](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/)

**Chrome 사전 비교:** [고정 9-scene 관찰·화면·hash](../spec/internal/evidence/c11-3-3-both-implicit-named-lines-precomparison-2026-11-01/README.md)

**검토:** [계획 실패 관점 20개](../spec/internal/evidence/c11-3-3-both-implicit-named-lines-precomparison-2026-11-01/plan-review-20.md)

## 결손과 목표

현재 제한 profile은 고정 크기 item의 positive trailing named start와 `auto` end에 대해 no-area implicit `auto` track sizing을 허용한다. Projection은 두 named endpoint를 numeric line으로 풀 수 있지만, `allows_positive_auto_track_sizing` admission은 raw placement가 `NamedLine / Auto`인 경우만 인정한다. 따라서 양 끝이 모두 implicit named line으로 해석되는 Chrome 유효 장면은 generic implicit mode로 떨어져 `grid-auto-* : auto` sizing 경계에서 거부된다.

이번 slice는 한 축의 양 끝이 모두 positive trailing implicit named line인 단일 고정 크기 item에 대해 기존 Taffy auto-track sizing을 연결한다. 규범상 같은 line의 end 제거, 역순 line swap, 여러 implicit track span의 분배도 독립 장면으로 고정한다.

## 고정 범위

- `RuntimeBlockFlexGridTemplateV1` opt-in projection에서만 적용하고, `grid-template-areas`가 없는 grid로 한정한다.
- 한 grid container에 layout-participating direct grid item 하나만 허용한다. 해당 item은 width와 height가 모두 fixed여야 한다.
- Rows 또는 Columns 중 정확히 한 축에서 start/end가 모두 custom-ident named line이고, resolver가 두 endpoint 모두 explicit grid의 positive trailing implicit line으로 해석해야 한다. Bare custom-ident는 기본 occurrence 1로 해석한다.
- 양수 occurrence는 같은 값, 인접 값, 다중 track span, 역순을 포함한다. 같은 line이면 CSS Grid §8.3.1에 따라 end를 제거하고, 역순이면 두 선을 swap한다. 최종 track 수는 기존 10,000 track 제한 이내여야 한다.
- 반대 축은 기존 profile이 허용하는 definite placement를 사용한다. implicit track size는 현재 승인된 `auto` 또는 fixed-px 입력 범위만 쓴다.
- 후보를 구분할 별도 `GridPlacementMode`를 사용해 기존 `TemplateGridPositiveNamedLineAutoImplicitTracks` admission과 동작을 바꾸지 않는다. 이름·area resolution과 계산 track 길이는 기존 Taffy 경로에 맡긴다.

## 구현 순서와 판정

1. 먼저 현재 코드의 rejection을 재현하고, 기존 positive named-start + auto-end 및 fixed-px control 결과가 유지되는 baseline을 기록한다.
2. projection metadata가 두 named endpoint 각각의 implicit 여부와 resolved line을 보존하도록 확장한다. 두 endpoint 모두 explicit-grid 마지막 line보다 큰 양수일 때만 pair mode를 고른다. 명시 named line과 implicit named line이 섞인 placement, endpoint 한쪽만 implicit인 placement, signed/leading line, span, 숫자 line은 기존 fail-closed 경로에 남긴다.
3. 별도 placement mode는 layout-participating direct item이 하나인지, 해당 item의 양 끝이 허용한 pair인지, 두 축 중 implicit 축 하나만 존재하는지와 fixed dimensions를 layout 경계에서 다시 확인한다. `display:none` subtree, 추가 layout-participating child, omitted/unknown node, unsupported `grid-auto-*` grammar, track limit 초과를 성공으로 처리하지 않는다. `visibility:hidden`은 CSS Grid layout participation을 유지하므로 단순히 보이지 않는다는 이유로 제외하지 않는다.
4. Chrome reference 9개를 고정 runtime scenes로 연결한다. 성공 입력에서는 explicit/implicit track 순서·used track size·item/root frame을 검사한다. equal/reversed/multi-track cases를 분리 assertion한다. 한쪽 endpoint만 implicit, negative occurrence, named span, mixed axis, auto-sized item, 다중 child는 기대 거부 matrix로 둔다.
5. CSS Typed projection과 layout-core focused tests 뒤 Android API 37 ARM64 AVD 및 iPhone 17 Pro/iOS 26.2 Simulator에서 같은 고정 scenes를 실행한다. DPR/sample origin을 기록하고 root 밖 overflow item의 frame을 포함해 검사한다. 표면 표본을 수집하는 경우 geometry 결과와 PixelCopy 또는 compositor 제출 전 Metal drawable을 별도 항목으로 보고한다.
6. 기본 product artifact와 opt-in C11.3.3 fixture-test positive control을 각각 검사해 새 scene marker/payload와 fixture ABI가 feature-off artifact에 없고 opt-in artifact에는 있는지 확인한다. Static archive 검사 범위를 APK/IPA 전체로 확대하지 않는다.
7. 구현 및 runtime 경계는 계획 검토와 다른 20개 실패 관점으로 새로 검토한다. 구현 PR 실제 diff는 다시 독립된 20개 관점으로 적대 검토한다. 계획 PR도 별도 20개 diff 검토 후 Rebase and merge하며, GitHub check 결과와 merge state를 확인한다.
8. 병합 뒤 내부 계약 0059와 `spec/STATUS.md`의 C11.3.3 candidate history를 갱신하고, 공개 roadmap은 제한 slice·미완료 범위를 정확히 반영한다. 공식 C11.3/C11 check, 전체 C11 완료 수는 올리지 않는다. 사용자가 승인한 Tailnet preview를 동기화하고 Agent Browser로 제공 파일·모바일 화면을 확인한 뒤 browser 프로세스를 닫는다.

## 완료 판정과 남는 한계

- Chrome 156의 9-scene CSSOM track sequence와 root-relative frame을 고정 입력에서 재현한다. Frames 허용치는 축별 `0.5 CSS px`, track sizes는 축별 `0.01 CSS px`로 판정한다.
- 잘못된 endpoint provenance, implicit 축 두 개, 대상 외 추가 layout-participating child, auto-sized item, unsupported track sizing grammar, 10,000 초과 track 요청은 성공 scene·부분 layout·draw output을 만들지 않고 지정 error로 거부한다.
- Android AVD와 iOS Simulator 검증은 지정 환경의 앱 실행 근거로만 기록한다. 실기기 GPU, iOS compositor 이후 surface, WPT/전체 CSS Grid 적합성은 검증하지 않는다.
- 이 제한 runtime slice는 일반 CSS author admission, C11.3.3/ C11.4/C11 완료를 의미하지 않는다. 공식 완료 집계 `72/213`은 유지한다.
