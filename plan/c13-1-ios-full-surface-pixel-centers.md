# C13.1 iOS 전체 CSS pixel-center 표면 대조 계획 · 2026-10-15

## 목표

[PR #162](https://github.com/ohah/spinon/pull/162)의 Android API 37 ARM64 AVD 전체 표면 대조와 같은 Chrome 155 `300×100 CSS px` fixture를 iPhone 17 Pro / iOS 26.2 Simulator의 WGPU Metal surface에서도 비교한다. 30,000개 CSS pixel center의 RGBA를 exact 비교한다. 각 fixture 요소의 border/frame 바깥이지만 viewport 안에 있는 좌표도 전체 grid에 포함한다.

현재 iOS 근거는 41/41 frame, 52/52 hit query와 Simulator 화면이다. 화면 캡처는 Metal drawable pixel readback이 아니므로 이 계획의 시작 기준은 “iOS 시각 픽셀 parity 미검증”이다. 이 단계가 통과해도 iOS compositor scanout, physical display, hardware GPU, 물리 drawable의 모든 byte, 제품 CSS 지원이나 C13.1 완료를 주장하지 않는다.

## 독립 oracle과 고정 입력

- Oracle은 [C13.1 compact surface matrix의 Chrome 155 PNG와 observation](../spec/internal/evidence/c13-1-compact-surface-matrix-2026-10-14/README.md)이다. Chrome `155.0.8059.40`, revision, fixture HTML/JS source hash, viewport `300×100`, DPR 1, scroll `(0,0)`를 validator가 매 capture 다시 확인한다.
- 원본 300×100 PNG와 observation을 재사용한다. 앱 결과를 expected 값으로 복사하지 않는다. comparator mutation test는 border 밖 좌표의 단일 RGBA byte 차이도 실패시켜야 한다.
- iOS 대상은 iPhone 17 Pro / iOS 26.2 Simulator의 기존 C13.1 compact fixture 및 30,000점 hit-identity route와 같은 runtime scene이다. 기기 모델, iOS build, Simulator UDID, app executable SHA, Cargo feature/compile flag, fixture source revision, viewport, WGPU texture format·크기를 각 fresh process에서 고정한다.
- 목표 결과는 Android와 같은 범위의 `30,000/30,000` CSS pixel center exact RGBA다. 허용 오차, 평균 오차, 선택 좌표 대체는 두지 않는다.

## Surface sample 수집 설계

- UIKit screenshot이나 Android PixelCopy, renderer의 계산상 paint color를 iOS Metal pixel 결과로 대신하지 않는다. C13.1 fixture-test host가 WGPU surface의 실제 Metal drawable texture에서 직접 sample을 요청하고 같은 frame의 draw report에 반환한 RGBA를 읽는다. 현재 Metal readback 경로의 64점 상한을 유지해 메모리와 한 frame의 copy 수를 제한한다. 30,000점은 정확히 469개 이하의 순서 있는 batch로 나눈다.
- 전용 iOS 실행 인자 `--spinon-c131-compact-metal-pixel-centers`는 `--spinon-c131-compact-hit-identity-grid`와 함께 올 때만 허용한다. 두 route가 기존 다른 C13.1/CSS fixture와 충돌하면 host 생성 전에 거부한다. feature-off 실행은 성공 report를 만들지 않는다.
- CSS 좌표는 row-major로 `y=0..99`, `x=0..299` 순서다. 매 capture에서 report가 제공하는 drawable texture 너비·높이로 `scaleX=drawableWidth/300`, `scaleY=drawableHeight/100`을 각각 계산하고, 표본 위치를 `floor((x+0.5)×scaleX)`, `floor((y+0.5)×scaleY)`로 만든다. 화면 density를 대신 쓰거나 x/y 비율이 같다고 가정하지 않는다. scale이 1보다 작거나 매핑이 중복되거나 drawable 밖이면 실패한다.
- 매 batch는 run ID, surface generation, batch index, count, 고정 좌표 목록과 결과 sample 좌표를 검증한다. run 동안 viewport, generation, texture size/format이 달라지면 일부 표본을 섞지 않고 전체 run을 실패시킨다. 중복, 누락, 재정렬, 이전 generation 결과는 모두 거부한다.
- batch readback은 UIKit main thread를 막지 않는다. 기존 `renderDrawLane`에서 한 batch를 draw·map한 뒤 성공적으로 저장했을 때만 다음 batch의 draw를 요청한다. 매 batch는 queue에 한 번만 걸고, 전체 run의 300초 deadline과 batch progress deadline을 둔다. 시간 초과, map 오류, route 종료 시 incomplete run을 성공 처리하지 않는다.
- 앱은 batch마다 검증한 sample만 임시 `.jsonl.part`에 append한다. 30,000개가 모두 확인된 뒤 파일을 flush/close하고 manifest를 atomic rename한다. Manifest는 run ID, source/revision, simulator, viewport, drawable size·format, x/y scale, batch/sample 수, Chrome oracle SHA, artifact SHA-256 및 `complete=true`를 기록한다. 앱 container에서 artifact를 꺼내 evidence directory에 복사한 뒤 hash를 재검증한다. Unified log는 요약과 실패 위치만 기록하고 sample 전체를 하나의 길이 제한 log line에 싣지 않는다.

## 구현 순서와 게이트

1. 이 계획을 구현 전에 서로 다른 실패 관점 20개로 검토하고, 수정 사항을 반영한 뒤 기록한다.
2. 기존 iOS C13.1 route와 WGPU readback owner를 다시 확인한다. 현재 `surface_pixel_readback_test`/`request_surface_pixel_readback_for_test` feature gate가 C13.1 opt-in host에서만 켜지도록 확장한다. C11/C12 fixture feature를 C13 앱에 억지로 추가하지 않는다.
3. C13 전용 batch FFI를 추가한다. count `1..=64`, pointer/null, multiplication overflow, surface bounds, host lifetime, output capacity를 검증한다. 기본 artifact에는 새 fixture symbol·payload가 없어야 하고 C13.1 feature positive control에는 포함되어야 한다.
4. iOS app route가 최대 64 samples를 한 batch로 요청·수집하며, 동일 run의 30,000개 row-major result를 최대 469 batch와 300초 안에 app-container artifact로 완결한다. 매 batch 전후 draw generation/size를 비교하고 route 충돌·초기화 오류·재시작·중간 실패의 완료 표식을 확인한다.
5. `tools/css-reference`에 JSONL/manifest validator와 comparator를 추가한다. 기존 Chrome observation validator를 재사용하되 CSS 좌표 누락·중복·재정렬, physical 좌표 계산 오류, 잘못된 길이/hash/format/run/generation, 잘못된 RGBA, 바깥-border pixel mutation을 각각 거부한다.
6. Android API 37 ARM64 AVD에서 기존 30,000점 PixelCopy comparator를 다시 실행해 공통 Rust/C feature 변화 후에도 30,000/30,000 exact를 확인한다. Android 앱의 앱 화면·PixelCopy를 새 evidence와 연결한다.
7. iPhone 17 Pro / iOS 26.2 Simulator에서 서로 다른 fresh process 3회 실행한다. 매 회 41/41 frame, 30,000/30,000 hit identity, 30,000/30,000 Metal surface pixel을 같은 C13.1 fixture와 run revision으로 연결한다. iOS 화면, direct drawable artifact와 비교 JSON을 보존한다.
8. `cargo fmt`, 영향 crate focused tests, workspace tests, Clippy, Node reference tests, Android/iOS Simulator build·실행과 `git diff --check`를 수행한다. AVD/Simulator 이외의 하드웨어·physical display·WPT를 통과로 보고하지 않는다.
9. 코드와 runtime을 대상으로 계획 검토와 겹치지 않는 신규 구현 실패 관점 20개를 수행한다. 수정 후 영향 관점을 다시 실행한다. 이후 PR 변경을 별도 30개 관점으로 검토하고 필요한 checks·mergeability를 재조회해 Rebase and merge 한다.

## 비교 도구 판정

- 입력 artifact는 30,000개 JSONL sample이며 각 entry는 `index`, `cssX`, `cssY`, `drawableX`, `drawableY`, `rgba`, `runId`, `generation`을 가진다. 모든 숫자는 유한한 범위 내 정수다.
- validator는 index `0..29999`와 CSS row-major 좌표가 정확히 한 번씩 나타나는지 확인한다. 각 physical 좌표는 manifest의 texture dimensions로 계산한 pixel-center mapping과 정확히 같아야 하고 unique해야 한다.
- expected RGBA는 고정 Chrome PNG의 CSS 좌표를 직접 읽는다. observed RGBA는 Metal drawable sample에서 RGBA로 정규화해 저장한 값이다. 채널 120,000개 전부 exact equality여야 한다.
- 비교 JSON은 기준·실제 해시, simulator/run/revision/generation, viewport/texture/scale, sample/channel 수, mismatch 수와 모든 mismatch 좌표 및 양쪽 RGBA를 기록한다. 표준 출력은 요약만 출력한다.
- Negative controls는 border 바깥이지만 viewport 안인 Chrome 픽셀의 R/G/B/A 각 채널 1-byte 변경, 첫/마지막 sample 누락, duplicate/reorder, wrong run/generation, wrong drawable size·scale, PNG/JSONL hash mismatch를 각각 실패시킨다. 잘못된 입력을 수정해 comparator가 다시 통과하는 positive control도 유지한다.

## 범위 밖과 완료 금지선

- 30,000 CSS pixel-center exact는 iOS Simulator에서 고정 viewport·fixture의 표본 중심만 증명한다. 전체 physical drawable byte equality, 다른 density/orientation/viewport, 다른 simulator/OS, compositor/scanout, 실기기·hardware GPU는 별도 근거가 없다.
- DOM event/CSSOM scroll, scroll range, unsupported `border-radius`/transform, float geometry parity, WPT와 전체 C13.1 지원은 이 계획에 포함하지 않는다. paint sample success를 hit-test rejection이나 event dispatch success로 바꾸어 보고하지 않는다.
- C13.1/상위 C13과 제품 CSS 지원 상태는 미완료로 유지한다.

## 계획 검토와 PR

- 계획 자체 실패 관점 20개는 [별도 검토 기록](../spec/internal/evidence/c13-1-ios-full-surface-pixel-centers-plan-review-2026-10-15.md)에 둔다.
- 계획 PR도 저장소 필수 규칙에 따라 별도로 30개 변경 관점 검토를 거친다. 구현 PR에는 새 20개 구현 관점과 별도 30개 PR 변경 관점을 보존한다.
- 계획/구현 PR은 Rebase and merge를 사용한다. Pages workflow는 실행하지 않는다.
