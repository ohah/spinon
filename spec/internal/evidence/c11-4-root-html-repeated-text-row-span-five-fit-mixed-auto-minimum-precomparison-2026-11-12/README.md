# C11.4 five-fit repeated-text row-span mixed `min-height` 사전 비교

## 기준과 범위

- 기준 구현은 Agent Browser 0.31.1의 HeadlessChrome `157.0.0.0`이며, macOS arm64, DPR 1, viewport `1280×900 CSS px`에서 관측했다. Terminal Browser는 사용하지 않았다.
- [HTML fixture](../../../../tests/fixtures/css/c11/root-html-grid-repeated-text-row-span-five-fit-mixed-auto-minimum-precomparison.html)는 #1360 five-fit row-span의 typed cap list, definite `40×220px` Grid, direct-text payload와 fixed placements를 유지한다. span-2 topology A–E는 32개 mask, span-2/span-3 topology A–D는 16개 mask를 만들고, forward/reverse DOM order와 `align-content:start|stretch`를 교차한다.
- 2,160개 track list × (32×2 + 16×2) masks/order × 2 alignments로 414,720개 scene을 캡처했다.
- 이 결과는 Chrome의 CSS 동작이다. Spinon RuntimeSession, 모바일 앱, surface pixels 또는 C11.4 구현 결과가 아니다.

## 독립 검증 결과

검증기는 scene 연속 번호, mask bit order, authored cap position/type, singleton tail, computed row serialization, definite Grid bounds, item/payload/line geometry, 실제 spanned-track eligibility, source-order pair, alignment pair, endpoint 회귀와 파일 hash를 확인했다. 기하 오차 허용치는 `0.01 CSS px`다.

| 측정 | 결과 |
| --- | ---: |
| Chrome scene / 고유 track list | 414,720 / 2,160 |
| fit 위치 조합 / typed cap 순서 / tail | 6 / 120 / 3 |
| topology별 mask | span2Six 32 · span2Span3Six 16 |
| 기존 uniform five-fit endpoint 재비교 | 34,560 scenes · 최대 차이 0 CSS px |
| eligible automatic-minimum item 관측 | 1,935,360 |
| `min-height:auto` content-floor 위반 | 0 |
| `min-height:0` payload-floor 미달 | 132,480 |
| Grid block-end 초과 | auto 0 · zero 1,440 |
| source-order geometry 차이 / non-auto stretch 변화 | 0 / 0 CSS px |
| item frame 대 covered-row 합 최대 오차 | 0.00045 CSS px |

`min-height:0` floor 미달과 Grid block-end 초과는 기준선의 실제 관측으로 보존한다. 이를 혼합 candidate의 acceptance로 간주하지 않으며, RuntimeSession 구현 결과는 별도 비교가 필요하다. `min-height:auto` 관측은 이 fixture에서 eligible item의 content floor를 유지했고 Grid 끝을 넘지 않았다.

## 재현 산출물

- [Agent Browser collector](./capture-chrome.mjs) · [capture manifest](./capture-manifest.json) · [independent verifier](./verify-chrome-matrix.py) · [검증 요약](./verification.json)
- [압축 전체 Chrome observation](./chrome157-five-fit-mixed-observation.ndjson.gz) · [대표 장면](./chrome157-five-fit-mixed-sample.png) · [Agent Browser lifecycle](./agent-browser-lifecycle.json) · [SHA-256](./SHA256SUMS)
- verifier 실행: `python3 spec/internal/evidence/c11-4-root-html-repeated-text-row-span-five-fit-mixed-auto-minimum-precomparison-2026-11-12/verify-chrome-matrix.py`

Capture session `spinon5mix12/5mixpre`는 `opened=true`, `closed=true`로 기록됐으며 동일 session 식별자를 가진 process도 확인되지 않았다.

## RuntimeSession용 균형 sample

전체 414,720-scene Chrome oracle은 그대로 보존하고, RuntimeSession 반복 비교에는 독립 verifier가 원본 scene key와 hash를 확인한 3,984-scene subset을 제공한다. 120개 typed-cap 순서는 모두 들어 있다. 그중 18개 track list는 여섯 fit-position 조합과 세 tail 값을 모두 대표하고 각 문맥에서 span2Six 30개와 span2Span3Six 14개의 중간 mask를 전부 포함한다. 나머지 102개 list는 각 topology의 회전 중간 mask를 양쪽 source order와 alignment 값에 적용한다. 기존 five-fit uniform RuntimeSession 회귀는 2,160개 list 전부에서 mask 0/all-auto endpoint를 별도로 확인한다.

- [sample 생성기](./build-runtime-sample.py) · [sample 독립 verifier](./verify-runtime-sample.py) · [생성 manifest](./runtime-sample-manifest.json) · [RuntimeSession 입력](./runtime-matrix-3984.ndjson.gz)
- verifier 실행: `python3 spec/internal/evidence/c11-4-root-html-repeated-text-row-span-five-fit-mixed-auto-minimum-precomparison-2026-11-12/verify-runtime-sample.py`

이 subset은 전체 Chrome matrix를 RuntimeSession에서 전부 실행했다는 뜻이 아니다. 전체 oracle 414,720 scenes, 혼합 RuntimeSession sample 3,984 scenes, 기존 uniform endpoint 회귀 34,560 scenes는 서로 다른 검증 범위로 기록한다.

### RuntimeSession sample 실행 결과

RuntimeSession mixed-mask sample 3,984 scenes는 120개 track-list batch를 세 청크로 나누어 전부 실행했다. 각 scene의 used row, item/payload frame, eligible automatic minimum, Grid-end overflow, source-order pair와 `align-content` pair를 Chrome 관측과 비교했다.

| Batch index | Track-list 구간 | 결과 | 예상/실제 floor shortfall (zero/auto) | 예상/실제 Grid-end overflow (zero/auto) |
| --- | ---: | --- | --- | --- |
| 1 | 0–40 | PASS · 272.14s | `[1180, 0]` / `[1180, 0]` | `[2, 0]` / `[2, 0]` |
| 2 | 40–80 | PASS · 33.93s | `[120, 0]` / `[120, 0]` | `[2, 0]` / `[2, 0]` |
| 3 | 80–120 | PASS · 69.32s | `[88, 0]` / `[88, 0]` | `[0, 0]` / `[0, 0]` |
| **전체** | **0–120** | **PASS · 375.39s** | **`[1388, 0]` / `[1388, 0]`** | **`[4, 0]` / `[4, 0]`** |

각 청크는 별도의 Cargo test 실행으로 통과했다. 집계 배열은 `[min-height:0, min-height:auto]` 순서다. 따라서 `auto`의 floor shortfall과 Grid-end overflow는 모두 0이고, `zero`의 비영 값은 Chrome baseline 관측과 같다. 이 증거는 sample 3,984 scenes에만 해당하며 전체 Chrome oracle의 RuntimeSession 실행을 뜻하지 않는다. sample 독립 검증기, focused crate/runtime tests, implementation/runtime review, 30개 관점 PR diff review 및 기본 Root HTML platform smoke도 통과했다. 실제 원격 PR diff·CI·merge gate는 별도로 남아 있다. [실제 diff review](./pr-diff-review-30.md).

chunk 2를 수정된 집계 라벨로 한 번 더 실행해 `zero/auto=[120, 0]`과 `[2, 0]`, 33.93s를 확인했다. [재실행 로그](./mixed-sample-label-order-regression.log).

### 기존 five-fit uniform endpoint 회귀

공유 track-rounding 변경 뒤 기존 RuntimeSession endpoint matrix도 네 청크 모두 통과했다. 2,160개 track list의 baseline/candidate와 source order·alignment 조합 총 34,560 scene을 재실행했다.

| 구간 | 결과 | candidate payload-floor 위반 | baseline payload-floor 위반 |
| --- | --- | ---: | ---: |
| 0–540 | PASS | 0 | 3,152 |
| 540–1,080 | PASS | 0 | 4,512 |
| 1,080–1,620 | PASS | 0 | 2,784 |
| 1,620–2,160 | PASS | 0 | 2,800 |
| **전체** | **4/4 PASS · 560.19s** | **0** | **13,248** |

재현 명령은 `cargo test -p spinon-runtime --features c11-4-grid-track-sizing-fixture-test parsed_root_html_five_fit_row_span_matrix_chunk -- --nocapture`다. [원본 Cargo 실행 로그](./uniform-endpoint-runtime-regression.log)를 보존한다. 이 matrix는 five-fit uniform endpoint 회귀이며 혼합 sample 3,984개나 전체 Chrome oracle 414,720개와 별도 범위다.

### 기존 six-fit row-span 회귀

같은 sizing simulator가 처리하는 기존 six-fit uniform row-span matrix도 네 청크 모두 통과했다. 11,520 Chrome RuntimeSession scenes를 실행했다.

| 구간 | 결과 | candidate payload-floor 위반 | baseline payload-floor 위반 |
| --- | --- | ---: | ---: |
| 0–180 | PASS | 0 | 1,568 |
| 180–360 | PASS | 0 | 1,168 |
| 360–540 | PASS | 0 | 1,216 |
| 540–720 | PASS | 0 | 848 |
| **전체** | **4/4 PASS · 186.93s** | **0** | **4,800** |

재현 명령은 `cargo test --locked -p spinon-runtime --features c11-4-grid-track-sizing-fixture-test parsed_root_html_six_fit_row_span_matrix_chunk -- --nocapture`다. [원본 실행 로그](./six-fit-runtime-regression.log)를 보존한다. 이는 six-fit uniform row-span 11,520 scenes이며 #1369 six-fit mixed-mask 138,240-scene matrix의 재실행을 뜻하지 않는다.

### #1369 six-fit mixed-mask 대표 회귀

공유 simulator의 mixed-mask 경로도 chunk 1/12를 재실행했다. 60개 cap-order에 포함된 11,520 RuntimeSession scenes가 Chrome 157의 rows, item/payload frames, forward/reverse DOM 및 start/stretch pair와 일치했다.

| 재실행 범위 | 결과 | 실행 시간 |
| --- | --- | ---: |
| chunk 1/12 · 60/720 cap-order · 11,520 scenes | PASS · 1/1 | 1,053.28s |

재현 명령은 `cargo test --locked -p spinon-runtime --features c11-4-grid-track-sizing-fixture-test parsed_root_html_six_fit_mixed_minimum_matrix_chunk_01_matches_chrome -- --nocapture`다. [원본 실행 로그](./six-fit-mixed-regression-chunk01.log)를 보존한다. 전체 six-fit mixed matrix의 138,240 scenes 중 나머지 126,720 scenes는 이번 branch에서 재실행하지 않았다.

## Android/iOS 기본 Root HTML 앱 smoke

두 플랫폼에서 변경 branch를 빌드한 뒤 기본 앱의 Root HTML route를 light/dark로 열었다. 로그에서 environment `layout=ready`와 draw `status=0 presented`를 확인하고 OS screenshot을 남겼다. 이는 기본 앱 진입/그리기 smoke이며 five-fit mixed row-span feature matrix나 surface/compositor pixel parity가 아니다.

| 플랫폼 | 환경 | light/dark 결과 | 화면 |
| --- | --- | --- | --- |
| iOS | iPhone 17 Pro · iOS 26.2 · 402×778 CSS px · scale 3 | 각 appearance에서 `SPINON_ROOT_HTML_DEFAULT_APP_BOOT status=pass`, draw `status=0 presented` | [light](./ios-default-light.png) · [dark](./ios-default-dark.png) |
| Android | `spinon_api37_2_presenttime_probe_20261018` · API 37.2 · arm64-v8a · 320×240 CSS px · scale 2.625 · Vulkan/SwiftShader | 각 appearance에서 `SPINON_C0410_ENVIRONMENT ... layout=ready`, draw `status=0 presented` | [light](./android-default-light.png) · [dark](./android-default-dark.png) |

- Build logs: [iOS](./ios-build.log) · [Android](./android-build.log). Android Gradle은 API 37.2와 AGP 8.13.2 호환 경고를 출력했지만 APK build는 성공했다.
- Launch/runtime logs: iOS [light](./ios-light.log) · [dark](./ios-dark.log), Android [light](./android-light.log) · [dark](./android-dark.log); AVD 정보 [환경](./android-smoke-environment.txt).
- 일부 원본 command capture는 출력의 후행 공백까지 보존한 `.raw.gz`로 함께 둔다: [iOS build](./ios-build.log.raw.gz) · [Android emulator](./android-emulator.log.raw.gz) · [mixed sample rerun](./mixed-sample-label-order-regression.log.raw.gz) · [five-fit uniform](./uniform-endpoint-runtime-regression.log.raw.gz) · [six-fit uniform](./six-fit-runtime-regression.log.raw.gz) · [six-fit mixed chunk 1](./six-fit-mixed-regression-chunk01.log.raw.gz). 연결된 `.log`는 읽기 편하게 trailing whitespace만 정규화했다.
- 재현 script: iOS [capture/restore/shutdown](./run-ios-default-root-html-smoke.sh) · Android [read-only AVD capture/shutdown](./run-android-default-root-html-smoke.sh). iOS는 기존 light appearance를 복원하고 shutdown했다. Android는 `-read-only -no-snapshot-load -no-snapshot-save`로 실행해 AVD와 ADB daemon을 종료했다. Gradle daemon도 stop했다.
- S03 이름의 simulator는 device inventory에 없었고, smoke 후 iOS booted devices는 0개다. 이 smoke는 S03 기기 데이터를 삭제하거나 공유 simulator 데이터를 초기화하지 않았다.

## 한계

두 fixed row-span topology, 다섯 typed fit caps, 한 definite Grid와 고정 text/font/placement만 포함한다. 일반 CSS admission, 다른 fit 개수, auto-placement, WPT 전체, Android/iOS runtime geometry, device pixels, compositor parity, 실기기·hardware GPU 및 C11.4/C11 완료는 검증하지 않는다.
