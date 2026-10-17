# C13.1 전체 물리 픽셀 parity 실행 근거 · 2026-10-18

## 판정 범위

고정 Chrome 155 표면과 API 37 ARM64 Android AVD PixelCopy, iOS 26.2 iPhone 17 Pro Simulator Metal drawable을 같은 `300×100` CSS viewport·DPR 3·`900×300` 물리 surface에서 비교했다. 두 앱 경로는 각각 서로 다른 앱 process 3개에서 `270,000` texel·`1,080,000` RGBA channel을 전부 byte exact로 비교했다. 모든 최종 실행은 mismatch 0, 최대 channel 오차 0이다.

이 증거는 13-case C13.1 fixture에 한정된다. C13.1/C13 제품 지원, 일반 HTML/CSS 조합, WPT, DOM event dispatch, 실기기·hardware GPU, Android 실제 패널 표시 또는 iOS compositor 이후 픽셀을 뜻하지 않는다.

## 고정 Chrome oracle

- 브라우저: Google Chrome `155.0.8059.85`, revision `0e66b94549a8bd85f03fbdf189b752d6e7b63c32`.
- Agent Browser `0.31.1`의 live Chrome tab에서 DPR 3 CDP capture를 사용했다. CSS viewport는 `300×100`, PNG는 `900×300`이다.
- Chrome PNG SHA-256: `bc66cd2bf37a19eb1b98c92867186213ffdb25f0660ac2f4e1e7b10799f84425`.
- Chrome observation SHA-256: `64319a76c86672a0d8f4e9a8918f610c5d96f2e3d0a9e555a5ab8048765d1d0b`.
- 기존 geometry observation SHA-256: `515fc95f811399fda9e336a13e7583f2d22555fce4727f1fe7f734981abd0299`.
- [DPR 3 screenshot](./chrome155-dpr-3.png) · [DPR 3 observation](./chrome155-dpr-3-observation.json) · [fixture alignment 계획 검토 20개](../c13-1-full-physical-pixel-parity-dpr-alignment-plan-review-20-2026-10-18.md).

## Android API 37 AVD

`emulator-5554` 한 대만 지정했다. API 37 ARM64 QEMU AVD는 ANGLE/Vulkan/SwiftShader software renderer를 사용했다. 기본 물리 density `420`에서 실행 중 override `480 dpi`를 설정해 양 축 scale 3을 만들었다. display size는 실행 전·중·후 `1080×2400`으로 같았고, 각 run 뒤 density를 `420`으로 되돌리고 fixture app process 종료를 확인했다. 각 앱 PID와 SurfaceView generation이 PixelCopy 로그에 연결됐다.

| Run ID | 앱 PID | Surface generation | Full surface | 결과 |
|---|---:|---:|---:|---|
| `1b4c38dc969b` | 9243 | 2 | 270,000 texel / 1,080,000 channel | exact, mismatch 0 |
| `1b4f3bb2e5a3` | 9447 | 2 | 270,000 texel / 1,080,000 channel | exact, mismatch 0 |
| `1b511357b3fa` | 9646 | 2 | 270,000 texel / 1,080,000 channel | exact, mismatch 0 |

각 run에서 기존 30,000 CSS-center PixelCopy와 120,000 RGBA channel도 독립 comparator로 exact였고, 52/52 fixed sample, 41/41 node frame 및 30,000/30,000 hit identity가 함께 확인됐다. Full-surface 비교는 앱 screenshot이 아니라 `900×300` SurfaceView PixelCopy 파일을 사용했다. fixture 요소의 오른쪽 border 밖인 `(270,72)` / 물리 pixel center `(811,217)`도 포함된다.

- [Android run 1](../c13-1-android-avd-full-surface-2026-10-18/c13-1-android-avd-20261017T164549179Z-1aa00573/README.md)
- [Android run 2](../c13-1-android-avd-full-surface-2026-10-18/c13-1-android-avd-20261017T164604252Z-fb38334a/README.md)
- [Android run 3](../c13-1-android-avd-full-surface-2026-10-18/c13-1-android-avd-20261017T164612205Z-c495c87a/README.md)

다른 연결된 `localhost:5555` emulator는 건드리지 않았다. 이 AVD 실행 결과를 Android 실기기나 hardware renderer 결과로 일반화하지 않는다.

## iOS 26.2 Simulator

고정 iPhone 17 Pro Simulator UDID `ACA7BF91-E2D5-4CF7-909A-08D1AD95FF3D`에서 서로 다른 앱 PID 41611, 43627, 44818로 실행했다. 각 run은 Metal drawable `900×300`, `Bgra8Unorm`, generation 1에서 row-major 4,219 batches로 전체 270,000 texel을 읽었고, 1,080,000 channel이 Chrome과 exact였다. 각 run에는 41/41 frame, 52/52 fixed query, 30,000/30,000 hit identity도 연결됐다. 네 채널 각 1 byte mutation, 빠진/추가/재배열 texel, padding 혼입 및 잘못된 identity를 comparator 테스트에서 거부했다.

- [iOS 3-run 묶음과 화면·원본 artifact](../c13-1-ios-full-physical-pixels-2026-10-18/c13-1-ios-full-physical-20261017T164128008Z-b6ae6818/README.md)
- 잘못된 Metal capture route 단독 실행과 route 충돌은 두 경우 모두 host 생성 전에 거부됐고, runner 종료 뒤 앱 process도 종료했다.
- iOS 결과는 Simulator drawable을 compositor에 제출하기 전 읽은 값이다. iOS OS compositor나 표시된 화면의 최종 pixel 결과는 측정하지 않았다.

## 구현·artifact 회귀

- `node --test tools/css-reference/compare-c13-1-full-physical-pixels.test.mjs`: 7/7 통과. 전체 byte cardinality, border 밖 표본 mutation, 각 RGBA channel mutation, 누락/추가/중복/재정렬/padding, identity·generation·hash·dimensions·format 변조 및 scaling 거부를 포함한다.
- `cargo test --locked --workspace --quiet`: 1,047 passed, 4 ignored, 0 failed.
- `cargo clippy --locked --workspace --all-targets -- -D warnings`, `cargo fmt --all -- --check`, 변경 스크립트 `node --check`, `git diff --check`: 통과.
- [기본/opt-in static archive symbol·string 확인](./fixture-artifact-boundary.md).
- [구현 실패 관점 검토 20개](./implementation-review-20.md).
- [원본 로그 보존 manifest](./raw-logs.json): 공백 정리 전 원본은 gzip byte-preserving 보관본으로 남겼고, 사람이 읽는 `.log`는 줄 끝 공백만 제거한 사본이다. manifest에는 원본·압축본·정규화 사본의 SHA-256과 변환 설명이 있다.

## calibration 실패 기록

DPR을 맞추지 않은 2.625/2.626… 비교는 합격 자료에서 제외했다. 기존 alignment plan review는 첫 비교 11,720 mismatch와 Chrome을 DPR 2.625로 맞춘 뒤 재비교 11,281 mismatch를 기록한다. 확인 가능한 전체 comparison JSON이 있는 첫 시도는 [진단 실행](../c13-1-android-avd-full-surface-2026-10-18/c13-1-android-avd-20261017T162707943Z-e428a27d/full-physical-comparison.json)에서 mismatch 11,720이다. 두 번째 결과의 별도 raw JSON은 현재 evidence tree에서 찾지 못했으므로 이 값은 계획 검토 기록으로만 인용한다. 어떤 결과도 tolerance를 낮춰 통과 처리하지 않았고 두 시도 모두 최종 run 집계에 포함하지 않는다.

초기 실행기의 build-flag 검사와 Android run-ID 유효성 문제, iOS route wiring/negative-control 문제도 해당 capture bundle의 `failure.txt`·log에 보존했다. 이를 통과 실행으로 세지 않으며, 수정 이후의 source revision `85f289938b5445ff2187e8b93644d9fa9e3ab5af`에 연결된 위 여섯 최종 앱 실행만 이 결과의 pass 자료다.
