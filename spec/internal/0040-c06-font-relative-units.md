# 0040 · C06.4 글꼴 상대 길이 단위

- **상태:** 현재 작업 브랜치 구현·검증 완료 · 미병합 · 공개 API 아님
- **내부 계약 숫자 버전:** 출시 전 0.1.0 고정
- **문서 식별자:** 0040은 내부 문서 번호이며 제품·계약 숫자 버전이 아니다.
- **구현 계획:** C06.4 글꼴 상대 길이 단위
- **상위 계약:** C06 값·단위 변환

## 계약 목표

현재 runtime layout profile에서 CSS em·rem 길이 및 font-size cascade 결과를 Stylo computed typed value로 계산하고, supported layout properties에 CSS px로 전달한다. font-size는 텍스트 렌더링을 제공한다는 선언이 아니며 길이 단위 해석에 필요한 cascade 값이다.

## 계약 경계

- Stylo cascade는 가상 Document 아래 합성 HTML `<html>` 문서 루트를 둔다. 기존 HostRoot 직속 요소는 앱 mount subtree다. 합성 루트는 `html`·`:root` selector와 `rem` 계산에는 참여하지만 HostDocument NodeId, Taffy node, layout frame 또는 GPU paint node로 출력하지 않는다.
- 이 합성 루트는 현재 내부 CSS cascade adapter의 모델이다. 공개 JS `document.documentElement`가 이를 관찰할 수 있다는 뜻은 아니며, 공개 DOM facade가 구현될 때 HostDocument 단일 소유 모델과 이행 계약을 정해야 한다.
- 합성 루트 cascade 후 그 computed font-size를 Stylo Device root font-size에 반영하고 mount subtree를 계산한다. 문서 루트 자신의 `rem`은 initial font-size 기준으로 계산한다.
- runtime UA baseline은 합성 `<body>`를 `display:block; margin:0`으로 둔다. html/body computed `width`·`height`·`flex-basis`, nonzero/auto margin, nonzero padding, `display:block` 이외의 display와 투명 이외의 `background-color`는 출력 box가 없으므로 `UnsupportedSyntheticDocumentStyle`로 실패한다. `font-size`, `direction`, custom property inheritance는 mount에 전달한다.

- em·rem 계산은 Stylo가 소유하며 native layout adapter가 CSS unit formula를 중복 구현하지 않는다.
- em on font-size는 부모 computed font size에, 다른 길이 property의 em은 해당 요소 computed font size에 대응한다.
- rem은 합성 문서 루트 computed font size에 대응한다. runtime mount root는 합성 문서 루트의 자손이다.
- ex, rex, ch, rch, cap, rcap, ic, ric, lh, rlh는 actual font/line-height metric provider가 정해지기 전까지 지원하지 않으며 기본값으로 대체하지 않는다.
- 지원 범위 밖의 metric unit, CSS math, invalid typed value는 오류로 실패한다.
- 텍스트 shaping·line layout·font loading/fallback은 범위 밖이다.
- flex item의 Chromium `getComputedStyle(width/height)`는 flex layout 뒤 resolved/used value가 될 수 있다. Stylo computed typed value와 이를 사용한 Taffy final frame을 각각 검증하며, 사전 cascade 값과 사후 used value를 혼동하지 않는다.

## 상태

구현·고정 Chromium 비교·Rust workspace/Clippy 검사·Android API 37 emulator와 iPhone 17 Pro / iOS 26.2 Simulator runtime 화면 검증을 현재 작업 브랜치에서 마쳤다. 병합 전이며, 이 문서는 공개 CSS 지원 또는 출시 호환성을 선언하지 않는다. 별도 구현 실패 경로와 캡처·로그는 [구현 검토 기록](evidence/c06-font-relative-units-implementation-review-2026-10-10.md)에 둔다. 숫자 계약 버전 `0.1.0`은 유지한다.
