# C11.1 iOS Simulator V8→WGPU 검증

## 10-scene geometry matrix · 2026-10-12

- 실행: [비교 요약](./c11-grid-ios-matrix-20261011T200317362Z-9c5ad710/README.md) · [상세 JSON](./c11-grid-ios-matrix-20261011T200317362Z-9c5ad710/comparison.json)
- 대상: iPhone 17 Pro · iOS 26.2 Simulator · UDID `ACA7BF91-E2D5-4CF7-909A-08D1AD95FF3D`
- 고정 Chrome 154 reference의 10 scene·40 data node를 각각 새 앱 프로세스로 실행했다. 각 장면에서 adapter wrapper frame도 별도 검사했다.
- 모든 fixture node의 x/y/width/height가 기준 대비 ±0.5 CSS px 이내였다. 10/10 scene·40/40 data node가 일치했다.
- V8 평가, Grid layout, WGPU Metal renderer 및 scene별 draw 제출을 확인했다. PNG는 보조 시각 자료이며 실제 Metal texture 직접 pixel readback은 하지 않았다.
- iOS 로그 수집기의 실제 `SPINON_C0410_DRAW generation=… status=0` marker 누락을 고쳐 재실행했다. frame/draw parser 회귀 테스트를 추가했다.
- 이 결과는 iOS 실기기, 10-scene pixel equality, C11 WPT 적합성 또는 전체 CSS 지원을 뜻하지 않는다.

## 단일 장면 사전 실행 · 2026-10-11

- [원본 통합 로그](./c11-grid-ios-20261011T181215Z/log.txt) · [화면 캡처](./c11-grid-ios-20261011T181215Z/ios-screen.png)
- V8 HostDocument의 `fixed-tracks-gap` scene 5 node와 Metal presentation을 확인했다. 당시에는 단일 장면 smoke였으며 아래 10-scene matrix로 대체되지 않는다.
