# C11.4 직교 row 및 #611/#620 iOS 회귀

정확한 staged source snapshot으로 opt-in 앱을 빌드하고, 전용 iPhone 17 Pro / iOS 26.2 Simulator에서 12개 scene을 실행했다. 기존 direct #611/#620 회귀 8개와 C11.4 직교 text/image row scene 4개가 모두 native matcher를 통과했다.

## 빌드와 source snapshot

- 빌드: `SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST=1 mise exec -- bun run build:ios-sim` — `BUILD SUCCEEDED`
- Xcode: Xcode 26.2 (Build version 17C52)
- HEAD commit: `d37ee6ea8c985ab27eb32770491c8998dcbf756c`
- 빌드에 사용한 staged index tree: `45bb3313f5c126fd724eb54e877f86746107a715`
- staged diff SHA-256: `e5c870f07de8345841aca43fee29bba53b50b81c4f4bff27bfe863ff0739224e`
- source manifest: [source-manifest.json](./source-manifest.json), 807개 source/config 파일, manifest SHA-256 `954ef11bb9478de7391a0cc06ec63ccd6455ff43f4792627e5a680c7da94435f`
- 앱 executable SHA-256: `ceadb3552b5bb50fcc9a75e813e58c9927f988ed4a10a20315d3c284f3563b93`
- 실행 후 source 확인: [source-manifest-after-run.json](./source-manifest-after-run.json)은 12회 실행 동안 HEAD, index tree, staged diff와 source 파일이 바뀌지 않았음을 확인한다.

staged tree에는 `crates/spinon-style-to-layout/src/projection.rs`의 fail-closed style-only guard가 포함되어 있다. HEAD 단독에는 staged 변경이 없으므로 index tree와 source manifest가 실제 앱 소스를 특정한다. 빌드 디렉터리에는 unstaged source 변경이 없었다.

## Scene 결과

모든 scene에서 `status=0 layout=ready boxes=3`, 정확히 세 개의 node frame, 완전한 trace, `SPINON_C0410_DRAW status=0 presented boxes=3`을 기록했다. source order를 뒤집은 scene도 예상 node ID와 item frame을 유지했다.

| Scene | Run ID | Root | Text frame | Image frame | Columns / rows | Trace chunks | 원본 근거 |
| --- | --- | --- | --- | --- | --- | ---: | --- |
| `direct-min-content-forward` | `c114sty01` | `120×48` | `2: (0,0) 50×20` | `4: (0,24) 24×16` | `[50]` / `[24,24]` | 8 | [PNG](./direct-min-content-forward.png) · [로그](./direct-min-content-forward-unified.log) · [trace](./direct-min-content-forward-trace.json) |
| `direct-min-content-reversed` | `c114sty02` | `120×48` | `3: (0,0) 50×20` | `2: (0,24) 24×16` | `[50]` / `[24,24]` | 8 | [PNG](./direct-min-content-reversed.png) · [로그](./direct-min-content-reversed-unified.log) · [trace](./direct-min-content-reversed-trace.json) |
| `direct-max-content-forward` | `c114sty03` | `120×48` | `2: (0,0) 80×10` | `4: (0,24) 24×16` | `[80]` / `[24,24]` | 8 | [PNG](./direct-max-content-forward.png) · [로그](./direct-max-content-forward-unified.log) · [trace](./direct-max-content-forward-trace.json) |
| `direct-max-content-reversed` | `c114sty04` | `120×48` | `3: (0,0) 80×10` | `2: (0,24) 24×16` | `[80]` / `[24,24]` | 8 | [PNG](./direct-max-content-reversed.png) · [로그](./direct-max-content-reversed-unified.log) · [trace](./direct-max-content-reversed-trace.json) |
| `direct-row-min-content-forward` | `c114sty05` | `74×20` | `2: (0,0) 50×20` | `4: (50,0) 24×16` | `[50,24]` / `[20]` | 8 | [PNG](./direct-row-min-content-forward.png) · [로그](./direct-row-min-content-forward-unified.log) · [trace](./direct-row-min-content-forward-trace.json) |
| `direct-row-min-content-reversed` | `c114sty06` | `74×20` | `3: (0,0) 50×20` | `2: (50,0) 24×16` | `[50,24]` / `[20]` | 8 | [PNG](./direct-row-min-content-reversed.png) · [로그](./direct-row-min-content-reversed-unified.log) · [trace](./direct-row-min-content-reversed-trace.json) |
| `direct-row-max-content-forward` | `c114sty07` | `74×20` | `2: (0,0) 50×20` | `4: (50,0) 24×16` | `[50,24]` / `[20]` | 8 | [PNG](./direct-row-max-content-forward.png) · [로그](./direct-row-max-content-forward-unified.log) · [trace](./direct-row-max-content-forward-trace.json) |
| `direct-row-max-content-reversed` | `c114sty08` | `74×20` | `3: (0,0) 50×20` | `2: (50,0) 24×16` | `[50,24]` / `[20]` | 8 | [PNG](./direct-row-max-content-reversed.png) · [로그](./direct-row-max-content-reversed-unified.log) · [trace](./direct-row-max-content-reversed-trace.json) |
| `direct-orthogonal-row-min-content-forward` | `c114sty09` | `74×50` | `2: (0,0) 50×50` | `4: (50,0) 24×16` | `[50,24]` / `[50]` | 8 | [PNG](./direct-orthogonal-row-min-content-forward.png) · [로그](./direct-orthogonal-row-min-content-forward-unified.log) · [trace](./direct-orthogonal-row-min-content-forward-trace.json) |
| `direct-orthogonal-row-min-content-reversed` | `c114sty10` | `74×50` | `3: (0,0) 50×50` | `2: (50,0) 24×16` | `[50,24]` / `[50]` | 8 | [PNG](./direct-orthogonal-row-min-content-reversed.png) · [로그](./direct-orthogonal-row-min-content-reversed-unified.log) · [trace](./direct-orthogonal-row-min-content-reversed-trace.json) |
| `direct-orthogonal-row-max-content-forward` | `c114sty11` | `74×80` | `2: (0,0) 50×80` | `4: (50,0) 24×16` | `[50,24]` / `[80]` | 9 | [PNG](./direct-orthogonal-row-max-content-forward.png) · [로그](./direct-orthogonal-row-max-content-forward-unified.log) · [trace](./direct-orthogonal-row-max-content-forward-trace.json) |
| `direct-orthogonal-row-max-content-reversed` | `c114sty12` | `74×80` | `3: (0,0) 50×80` | `2: (50,0) 24×16` | `[50,24]` / `[80]` | 9 | [PNG](./direct-orthogonal-row-max-content-reversed.png) · [로그](./direct-orthogonal-row-max-content-reversed-unified.log) · [trace](./direct-orthogonal-row-max-content-reversed-trace.json) |

마지막 직교 scene 네 개의 text source는 child writing mode `VerticalRl`, parent writing mode `HorizontalTb`, orientation `Mixed`, logical min/max inline size `50/80`, logical min/max block size `20/10`, font registry revision `1`을 기록한다. min-content row trace의 text 값은 `50`, image 값은 `16`이다. max-content trace는 text `80/50/80`, image `16/16/16`을 포함한다. 모든 contribution의 span은 `1`, `crossesFlexibleTrack=false`다.

앞의 8개 scene은 direct min/max content와 direct row min/max content를 source order 정방향·역방향으로 다룬다. 파싱한 전체 값과 status 검사는 [validation.json](./validation.json)에 있다. 각 `*-unified.log`는 원본 trace 조각을 보존하고, `*-trace.json`은 이를 재조립한 JSON이다.

## Simulator 종료 및 증거 범위

전용 simulator만 이 실행에서 시작·종료했다. 증거 수집 뒤 해당 simulator를 종료했고, 다른 iPhone 17 Pro UDID `9DCCE836-7659-4914-B6C1-13F54BFFA9DB`, `ACA7BF91-E2D5-4CF7-909A-08D1AD95FF3D` 두 대는 Booted 상태로 유지했다. [simulator-before.json](./simulator-before.json), [simulator-at-run.json](./simulator-at-run.json), [simulator-before-shutdown.json](./simulator-before-shutdown.json), [simulator-after-shutdown.json](./simulator-after-shutdown.json), [simulator-cleanup.json](./simulator-cleanup.json)에 근거가 있다. 브라우저는 사용하지 않았다.

Draw 결과는 surface 제출과 세 개 box를 보여주지만 image와 glyph paint counter는 0이다. 캡처는 layout geometry, trace, 제출만 입증하며 실제 pixel painting은 입증하지 않는다. [SHA256SUMS.txt](./SHA256SUMS.txt)가 이 디렉터리의 증거 파일 체크섬을 담는다.
