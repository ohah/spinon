# C18.2 iPhone 17 Pro Simulator — 2026-10-16

## 실행 환경

- 대상: iPhone 17 Pro / iOS 26.2 Simulator, UDID `ACA7BF91-E2D5-4CF7-909A-08D1AD95FF3D`만 사용했다. 이미 부팅된 iPad Pro Simulator는 건드리지 않았다.
- backend는 Metal / Apple iOS Simulator GPU, drawable format `Bgra8Unorm`이다. drawable은 `240×120 px`, viewport `80×40 CSS px`, scale `3×3`이다.
- C18.2 route process ID `13092`, surface generation `1`, document revision `11`, render-tree revision `2`다. canvas 원점은 `(161,417)` UI points, 전체 화면 좌표 `(483,1251)` px다.
- opt-in app executable SHA-256: `f117783216247c8d73d8906ade6e35279fa859e1952c9bd012432f3405c1d5e4`.

## C18.2 결과

Metal drawable readback의 고정 표본 10개가 Chrome 155 RGBA oracle과 모두 exact했고, iOS Simulator 전체 화면 캡처의 대응 pixel 10개와 fixture 밖 제목·부제·상태·배경 anchor 4점도 exact였다. alpha 0은 backdrop을 유지하고 alpha 128은 `(118,37,65,255)`, alpha 255는 `(32,200,64,255)`로 나왔다. Comparator는 drawable `10/10`, 전체 화면 `10/10`, anchors `4/4`를 반환했다.

| 근거 | 파일 | SHA-256 |
| --- | --- | --- |
| iPhone 전체 화면 | [full-screen.png](./full-screen.png) | `4c2f9219e40567b52154a8193dc164e9175ade165a198ff4ae81ee14b7117ad0` |
| PID 13092 원본 simulator log | [simulator-log.txt](./simulator-log.txt) | `fe4a6b564824f2a43767b76f67950c048f91d49a1471f5b9539147ed6a714f6e` |
| 전체 화면·drawable 좌표 manifest | [post-compositor-observation.json](./post-compositor-observation.json) | — |

iOS sample readback은 Metal drawable에서 OS compositor 제출 전에 수행했다. Full-screen screenshot은 `simctl io screenshot`으로 별도 캡처해 후합성 화면에서 대응 pixel을 확인했다. 재현 비교:

```sh
node tools/css-reference/compare-c182-alpha-compositor-pixels.mjs \
  spec/internal/evidence/c18-2-alpha-image-ios-simulator-2026-10-16/post-compositor-observation.json
```

## C18.1 회귀 결과

현재 renderer source의 C18.1 opaque image/overflow/hidden clip route도 같은 Simulator에서 실행했다. drawable 표본 `9/9`, 전체 화면 대응 표본 `9/9`, fixture 밖 anchor `4/4`가 exact였다. 이는 sRGB surface blending 실패를 UNorm으로 바꾼 뒤 opaque C18.1 결과가 유지됨을 확인한다.

- process ID `23535`, drawable generation `1`, document revision `27`, render-tree revision `5`.
- [전체 화면](./c181-regression/full-screen.png) · [원본 simulator log](./c181-regression/simulator-log.txt) · [9-point manifest](./c181-regression/post-compositor-observation.json).
- Comparator: `node tools/css-reference/compare-c181-compositor-pixels.mjs spec/internal/evidence/c18-2-alpha-image-ios-simulator-2026-10-16/c181-regression/post-compositor-observation.json` → `matched`, drawable `9/9`, screenshot `9/9`, anchors `4/4`.

## 첫 실행과 범위

첫 iOS pass는 `Bgra8UnormSrgb` surface를 선택해 alpha-128 R sample이 162로 관측됐고 Chrome의 118과 달랐다. Android 첫 pass는 `Rgba8Unorm`이었으나 반투명 R/B가 각각 1 높았다. 실패 screenshot과 원본 로그를 `first-pass-*`로 보존했다. surface format 선택과 8-bit source-contribution 양자화 적용 후 UNorm drawable에서 10/10 exact로 바뀌었다.

이 결과는 지정 iOS Simulator GPU에 한정한다. iPhone 실기기, 별도 hardware GPU, OS compositor 이후 물리 display pixel, UNorm이 없는 surface 구성, 제품 URL loader와 C14/C18/U05 전체는 검증하지 않았다.
