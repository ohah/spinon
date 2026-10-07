# S04 비대칭 y 좌표 사전 비교 기준

**상태:** 구현 전 기준 고정 · 실행 결과는 [S04.8 실행 근거](s04-asymmetric-y-platforms-2026-10-07.md)에 기록

## 목적과 제외 범위

기존 S04 픽스처의 모든 box가 `y=0`이어서 GPU 좌표 변환에서 y축 반전이나 서로 다른 세로 위치·높이 오류를 판별하지 못한다. 새 CSS fixture를 Chromium oracle과 비교하고, 같은 snapshot을 Android·iOS GPU 표면과 정확 RGBA readback에 연결한다. 기존 `S04-flex-paint-v1`과 그 Chromium 자료는 변경하지 않는다.

이 작업은 비대칭 fixture의 계산·좌표 변환만 검증한다. 동적 앱 runtime, 입력 hit-test, 제품 frame queue, 실제 표시 시각, CSS 일반 지원, 하드웨어 GPU와 실기기 검증을 뜻하지 않는다.

## 고정 입력과 기준 oracle

- Fixture: `S04-asymmetric-y-v1`, CSS viewport `301 × 65`, device scale factor `1`.
- 부모: `display:flex`, `flex-direction:column`, `row-gap:3px`, 크기 `301 × 65`.
- 자식: 폭은 기본 stretch, 높이는 각각 `12`, `18`, `24` CSS px.
- 자식 시작 y: `0`, `15`, `36` CSS px. 자식 사이 간격은 각각 `3` CSS px이고 자식 높이는 서로 다르다.
- Chromium: `154.0.8037.98`, headless, offline, `en-US`, UTC, light scheme.
- 고정 reference: [JSON](../../../../tests/fixtures/css/references/s04-asymmetric-y-v1-chromium-154.0.8037.98-f7ffacb8763c-06ff4aab2ac9-ccffd5c5fe77.json).
- Fixture SHA-256: `f7ffacb8763c1d86367c86749659b45bc34960615f742842c06abad6b841dfad`.
- CSS SHA-256: `06ff4aab2ac9c112d1b2dc1c81a71df183b4cdc2ead8e909cfeb020548cc3fc5`.

Chromium geometry 기대값은 부모 `(0,0,301,65)`, 자식 A `(0,0,301,12)`, B `(0,15,301,18)`, C `(0,36,301,24)`이다. computed property 문자열과 불투명 배경색은 fixture의 선택 속성마다 정확히 같아야 한다. x/y/width/height는 노드별·축별 최대 절대 오차 `0.5 CSS px` 이하로 판정하고 평균으로 실패를 상쇄하지 않는다.

GPU readback은 CSS pixel center 기준 x `[0,150,300]`과 y `[0,11,12,14,15,32,33,35,36,59,60,64]`를 검사한다. 자식 내부, 3px 간격 양쪽, 부모의 아래 여백을 포함하며 각 채널은 author `#RRGGBB`의 RGBA8 값과 정확히 같아야 한다. 한 픽셀 색상 오류도 실패다. sRGB sample 검사는 y 좌표의 전체 기하 정확도를 대신하지 않으므로 Chromium·snapshot 좌표 대조를 따로 한다.

GPU vertex의 CSS-to-surface 기준은 다음 식으로 독립 계산한다. `scale = min(density, surfaceWidth / viewportWidth, surfaceHeight / viewportHeight)`, `offsetX/Y = (surfaceExtent - viewportExtent * scale)/2`; NDC는 x에 `2*x/surfaceWidth-1`, y 위쪽에 `1-2*y/surfaceHeight`를 쓴다. 별도 단위 fixture의 viewport `100×80`, surface `240×180`, density `1.5`에서는 scale `1.5`, offset `(45,30)`이다. y `[7,25,56]`과 높이 `[11,23,14]`인 box의 NDC top/bottom은 각각 `(0.55, 0.3666667)`, `(0.25, -0.1333333)`, `(-0.2666667, -0.5)`이며 허용 오차는 `1e-5`다. 이는 상하 반전·letterbox·높이 반영을 판별하는 CPU 기준이며 실제 플랫폼 출력을 증명하지 않는다.

## 재현 명령

```sh
mise exec -- bun run css:reference:s04-y
```

새 reference 생성 시 기존 파일을 덮어쓰지 않는다. 구현 검증은 별도 Rust fixture·snapshot 좌표 assertion과 Android·iOS 시뮬레이터 실행 근거에서 기록한다.
