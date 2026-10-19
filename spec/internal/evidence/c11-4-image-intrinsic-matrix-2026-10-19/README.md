# C11.4 direct-leaf image intrinsic matrix

2026-10-19에 C11.4 opt-in runtime fixture에 고정 HTML `img` direct-leaf 장면 9개를 연결했다. 기준은 저장소의 PNG fixture를 decode한 결과와 고정 Chrome 155 observation이며, 앱은 Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 실행했다. 이 변경은 제품 이미지 paint나 전체 C11.4를 구현하지 않는다.

## 판정 범위

- PNG 입력: `tests/fixtures/resources/c14-1-png-basic.png`, natural size 24×16 CSS px. fixture 경로만 고정 URL을 허용하고 원격 네트워크 요청을 하지 않는다.
- Chrome oracle: [기존 고정 Chrome 155 관찰](../c11-4-grid-track-sizing-precomparison-2026-10-19/chrome/chrome155-observation.json), DPR 1. 각 장면의 computed track list, root/item frame, natural image size, overflow를 사용한다. 좌표·크기 허용오차는 0.05 CSS px다.
- Android: Android API 37 ARM64 AVD `emulator-5554`, SwiftShader. iOS: iPhone 17 Pro / iOS 26.2 Simulator, Metal drawable readback은 compositor 제출 전이다. 실기기나 hardware GPU 결과가 아니다.
- 각 장면에서 root/item frame, 24×16 replaced intrinsic input, `intrinsicSize` revision 1, Columns/Rows stage trace, min-content query와 최종 track/gutter를 검증했다. Android node frame도 원본 로그에서 따로 대조한다.
- [Chrome·Android·iOS 비교 JSON](./cross-platform-comparison.json)은 9개 scene 모두의 geometry와 normalized trace가 일치함을 기록한다. 고정 12px track 장면의 PixelCopy 및 Metal 표본은 각각 3/3 exact다.
- Screenshot은 fixture item의 **배경 상자와 overflow 경계**를 보여준다. PNG texel paint는 C11.4 fixture에서 연결하지 않았으므로 이미지 픽셀 렌더링 증거가 아니다.

## 장면

| Scene | Chrome columns / rows | Root | Item | 확인 지점 |
| --- | --- | --- | --- | --- |
| `auto-image-definite` | 24 / 16 | 100×40 | 24×16 | `auto`의 natural contribution |
| `minmax-auto-image` | 24 / 16 | 100×40 | 24×16 | `minmax(0, auto)` |
| `minmax-min-max-image` | 24 / 16 | 100×40 | 24×16 | `minmax(min-content, max-content)` |
| `fit-content-below-minimum` | 24 / 16 | 100×40 | 24×16 | 12px limit보다 큰 item minimum |
| `fit-content-above-intrinsic` | 24 / 16 | 100×40 | 24×16 | 40px limit과 intrinsic track |
| `fit-content-percent` | 24 / 16 | 100×40 | 24×16 | definite 50% fit-content limit |
| `fixed-small-track-overflow` | 12 / 16 | 12×40 | 24×16 | 고정 track의 item overflow |
| `spanning-min-content-items` | 10 4 10 / 16 | 100×30 | 24×16 | 두 min-content track, 4px gutter, span 2 |
| `min-content-row-image` | 100 / 16 | 100×80 | 24×16 | row min-content contribution |

Chrome의 spanning 장면에서 각 min-content track은 10px이고 가운데 gutter는 4px다. Android/iOS trace는 동일한 최종 track state와 24px/span-2 contribution을 보존했다.

## 재현 및 파일

- [Simulator 전체 matrix 실행기](./run-simulator-matrix.sh)
- [로그 비교 및 cross-platform JSON 생성기](./compare-platform-logs.mjs)
- [Android 빌드·실행·frame 로그와 화면](./android-avd/)
- [iOS 빌드·실행·Metal 표본 로그와 화면](./ios-simulator/)
- [기본/fixture staticlib 분리 증거](./artifact-boundary/README.md)
- [구현 실패 관점 20개](./implementation-review-20.md)
- [PR diff 실패 관점 30개](./pr-diff-review-30.md)

Rust 확인은 [workspace test](./workspace-test.log)에서 1,113 passed·4 ignored·0 failed였고, [기본 workspace Clippy](./workspace-clippy.log)와 [runtime](./runtime-clippy.log)/[FFI fixture-feature Clippy](./ffi-clippy.log)가 모두 `-D warnings`로 통과했다. Runtime C11.4 focused tests 5개(HTML namespace negative case 포함)와 [FFI C11.4 focused tests 5개](./ffi-test.log)가 통과했다. [runtime namespace-negative](./namespace-negative-test.log)·[scene table unit](./image-fixture-unit-test.log) 결과도 보존한다.

최신 소스에서 Android/iOS 앱을 다시 빌드하고 새 `20261019-r4` run ID로 전체 matrix를 실행했다. Android PixelCopy cache의 중복 파일명 충돌을 피하기 위해 재실행마다 새 ID를 쓴다. 실행기는 해당 AVD와 Simulator의 Spinon 앱만 종료한다.

## 남은 범위

여기서는 direct-leaf HTML image의 제한된 intrinsic sizing만 확인했다. anonymous/nested Grid item, 여러 replaced item, image/text 동시 contribution, aspect-ratio 전달, percentage/calc box edges, dynamic image replacement/cache invalidation, decoded texel paint, compositor 이후 표면, Android hardware GPU, 실기기, WPT, 표준 전체 stage matrix는 미검증이다. C11.4와 C11은 계속 미완료다.
