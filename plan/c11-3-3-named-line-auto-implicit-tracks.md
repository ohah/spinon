# C11.3.3 trailing named-line implicit `auto` tracks

**상태:** 구현 전 계획. 공식 `spec/STATUS.md`의 C11.3.3/C11 체크와 `72/213`은 이 제한 작업에서 변경하지 않는다.

## 결손과 목표

C11.3.3의 제한 template placement는 template area가 만든 positive implicit track에서 `grid-auto-rows:auto` / `grid-auto-columns:auto`를 허용한다. Template area가 없는 positive named-line placement는 같은 auto sizing 입력을 현재 validator에서 거부한다. 이 작업은 이미 지원되는 definite, positive trailing named-line placement에서만 auto implicit track sizing을 연결한다.

## 비교 기준과 입력

- 규범 기준: [CSS Grid Layout Module Level 1, 2025 Candidate Recommendation Draft §7.5](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#implicit-grids), [§7.6](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#auto-tracks), [§8.3](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#line-placement). Explicit grid 밖에는 implicit track이 생성되고 `grid-auto-*`가 크기를 정한다. 정수 occurrence가 생략된 named line은 첫 번째를 가리키며, 이름이 부족하면 implicit line을 해당 이름이 있는 것으로 보고 찾는다.
- 고정 입력·Chrome 기준·독립 기대값: [사전 비교](../spec/internal/evidence/c11-3-3-named-line-auto-implicit-tracks-precomparison-2026-10-31/README.md). Chrome 156 Agent Browser는 auto columns `40px 40px 0px 30px`, auto rows `20px 0px 25px`, fixed-px 대조군 `40px 40px 30px 30px`를 기록했다.
- columns 장면 root는 80×20px이고, 30×10px item은 root 기준 x=80px에서 시작한다. rows 장면 root는 40×20px이고, 20×25px item은 y=20px에서 시작해 5px overflow를 드러낸다. fixed-px 대조군의 item은 root 기준 x=110px다.

## Implementation boundary

1. 현재 template area가 없는 `auto` columns/rows 입력이 fail-closed로 거부되는지 재현하고 fixed-px track-list 동작을 보존한다.
2. 기존 `TemplateGridImplicitTracks` positive trailing mode에만 `auto`를 허용한다. `TemplateGridSignedImplicitTracks`의 거부 동작은 유지한다. projection이 범위 밖 숫자 배치나 자동 배치를 새 허용 mode에 넣지 않는지 확인한다.
3. 기존 Taffy track sizing 경로를 재사용한다. 일반 CSS parser 추가, 공개 CSS admission 변경, named-line resolution 변경, area-backed sizing 의미 확장은 하지 않는다.
4. columns, rows, fixed-px 대조군, negative/signed 거부와 기존 C11.3.3 회귀를 위한 제한 Rust/typed-projection 및 fixture-runtime 검증을 추가한다. frame, computed track, root overflow를 각각 확인하고 CSS px 단위 geometry를 Chrome과 대조한다.
5. 기존 C11.3.3 fixture adapter를 사용해 Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 고정 runtime 장면을 실행한다. PixelCopy와 compositor 제출 전 Metal drawable 결과를 geometry 및 Chrome oracle과 분리해 기록한다.
6. 새 fixture ABI/payload가 기본 artifact에는 없고 opt-in fixture-test positive control에는 있는지 확인한다.
7. 구현/runtime을 서로 다른 실패 관점 20개로, 실제 PR diff를 별도의 30개 관점으로 검토한 뒤 병합한다. 구현 PR은 Rebase and merge하고 최종 병합 상태를 확인한다.

## Acceptance and preserved limits

- Chrome의 고정 세 장면과 기대 root-relative geometry가 Spinon typed output 및 대상 앱 runtime과 0.5 CSS px 이내에서 일치하고, track sequence는 CSS px 기준 exact하게 일치한다.
- rows overflow는 20px root보다 5px 더 보이고 columns overflow는 80px root 밖으로 이어진다. 정상 root frame 검사에서 item을 clip하거나 clamp하지 않는다.
- fixed-px와 기존 area-backed 사례는 바뀌지 않는다. `auto`를 쓰는 signed/leading implicit placement, numeric implicit placement, auto-placement, 임의 non-px track list, 일반 author CSS, Spinon WPT 적합성, hardware GPU/실기기 결과와 iOS compositor 이후 pixel은 범위 밖이다.
- C11.3.3, C11.4, C11을 완료로 체크하지 않는다. 공식 집계 `72/213`을 유지한다.
