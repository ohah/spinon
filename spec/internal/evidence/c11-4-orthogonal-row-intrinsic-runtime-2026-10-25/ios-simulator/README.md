# C11.4 직교 text/image row intrinsic — iOS Simulator

이 문서는 최종 lint 수정 전 첫 orthogonal 실행을 보존한다. 기존 8-scene 회귀와 최종 Clippy source의 4-scene 재실행, source manifest 및 앱 해시는 [source comparison evidence](../regression-previous-row-scenes/README.md)를 기준으로 한다.

2026-10-25, iPhone 17 Pro / iOS 26.2 Simulator에서 C0410 runtime route로 네 scene을 각각 실행했습니다. 각 실행은 native matcher, 3개 layout frame, intrinsic trace, WGPU draw 보고를 통과했습니다. 실행 원본은 scene별 unified log와 화면 PNG에 보존했습니다.

## 빌드와 환경

- 빌드: `SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST=1 mise exec -- bun run build:ios-sim` — `** BUILD SUCCEEDED **`
- Xcode: 26.2 (17C52)
- Simulator UDID: `00D4D861-56AA-4552-9181-58D88770EDDA`
- Device type: `com.apple.CoreSimulator.SimDeviceType.iPhone-17-Pro`
- Runtime: `com.apple.CoreSimulator.SimRuntime.iOS-26-2`
- Bundle: `dev.spinon.bootstrap`
- 각 scene 실행 인자: `--spinon-c114-grid-track-sizing --spinon-c114-grid-scene=<scene-id> --spinon-c114-grid-run-id=<run-id>`
- 빌드 로그: [build.log](./build.log)
- device inventory: [실행 전](./simulator-before.json), [실행 중](./simulator-at-run.json), [종료 후](./simulator-after.json)
- 해시와 실행별 상세 메타데이터: [simulator-metadata.json](./simulator-metadata.json), [SHA256SUMS.txt](./SHA256SUMS.txt)

## 결과

| Scene | Native matcher / draw | Root · text · image frame (CSS px) | Rows 기여 기록 |
| --- | --- | --- | --- |
| `direct-orthogonal-row-min-content-forward` | pass · 3 boxes | root `74×50`; text `50×50` at `(0,0)`; image `24×16` at `(50,0)` | text MinContent `50` ×2; image MinContent `16` ×2 |
| `direct-orthogonal-row-min-content-reversed` | pass · 3 boxes | root `74×50`; text `50×50` at `(0,0)`; image `24×16` at `(50,0)` | text MinContent `50` ×2; image MinContent `16` ×2 |
| `direct-orthogonal-row-max-content-forward` | pass · 3 boxes | root `74×80`; text `50×80` at `(0,0)`; image `24×16` at `(50,0)` | text MaxContent `80`, MinContent `50`, MaxContent `80`; image MaxContent `16`, MinContent `16`, MaxContent `16` |
| `direct-orthogonal-row-max-content-reversed` | pass · 3 boxes | root `74×80`; text `50×80` at `(0,0)`; image `24×16` at `(50,0)` | text MaxContent `80`, MinContent `50`, MaxContent `80`; image MaxContent `16`, MinContent `16`, MaxContent `16` |

각 scene은 `layout=ready boxes=3`, frame summary `count=3 marker=present`, `SPINON_C0410_DRAW status=0 presented boxes=3`을 기록했습니다. 네 trace 모두 Grid 하나에서 Columns sequence 0 `[50,24]` 다음 Rows sequence 1 `[50]` 또는 `[80]` 순서였습니다. logical text intrinsic은 inline min/max `50/80`, block min/max `20/10`; writing mode은 `VerticalRl` → `HorizontalTb`, orientation은 `Mixed`였습니다. 실행별 원본 trace chunk는 unified log에 있으며 메타데이터 JSON에도 frame·contribution 요약을 넣었습니다.

| Scene | 화면 캡처 | Unified log |
| --- | --- | --- |
| min forward | [PNG](./direct-orthogonal-row-min-content-forward.png) | [log](./direct-orthogonal-row-min-content-forward-unified.log) |
| min reversed | [PNG](./direct-orthogonal-row-min-content-reversed.png) | [log](./direct-orthogonal-row-min-content-reversed-unified.log) |
| max forward | [PNG](./direct-orthogonal-row-max-content-forward.png) | [log](./direct-orthogonal-row-max-content-forward-unified.log) |
| max reversed | [PNG](./direct-orthogonal-row-max-content-reversed.png) | [log](./direct-orthogonal-row-max-content-reversed-unified.log) |

## draw 범위와 정리

draw 보고는 네 scene 모두 `status=0`, `presented boxes=3`, `surface_texture=960x720`이었습니다. 같은 보고의 `runtime_image_count`, `image_draw_count`, `runtime_text_count`, `glyph_batch_draw_count`는 모두 `0`이므로 이 캡처는 layout geometry와 surface 제출 증거이며 이미지 픽셀이나 글리프 페인팅 증거는 아닙니다.

지정한 전용 Simulator만 종료했습니다. 실행 전 이미 Booted였던 iPhone 17 Pro 두 대는 실행 후에도 Booted 상태로 남았습니다. Agent Browser는 사용하지 않았고 종료 뒤 관련 프로세스도 발견되지 않았습니다.
