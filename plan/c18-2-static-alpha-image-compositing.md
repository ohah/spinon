# C18.2 · 정적 PNG alpha source-over 합성

## 목표와 경계

C18.1의 opaque-only RGBA8 natural-size PNG paint에서 alpha pixel 거부를 제거하고, 불투명 backdrop 위에서 정적 PNG texel을 기본 source-over로 합성한다. 실제 Android·iOS 앱의 live V8 → HostDocument → Stylo → Taffy → WGPU 경로에서 같은 fixture를 표시하고 Chrome과 exact RGBA pixel로 비교한다.

이 slice는 caller-owned 고정 PNG 한 장의 기본 source-over만 검증한다. 이미지의 sRGB sample, PNG alpha 표현, source-over 외 compositing mode, `opacity`, group isolation, CSS blend mode, alpha-bearing root surface, scaled sampling, `object-fit`·`object-position`, image URL/network loader, async decode/replacement, 다른 포맷, 실기기 및 hardware GPU는 미완료로 둔다. C18.1 opaque/overflow 표본은 회귀 control로 다시 확인하되 C18·C14·U05 또는 공개 image/CSS 지원을 완료 처리하지 않는다.

## 구현 전 비교 모델

- 규범 참고: [CSS Compositing and Blending Level 1 §5.1](https://www.w3.org/TR/compositing-1/#simplealphacompositing)의 simple alpha compositing 및 source-over 식. 출력 alpha는 `αo = αs + αb × (1 − αs)`이며 이 fixture는 `αb=1`이다.
- 독립 oracle: Agent Browser 0.31.1을 통해 실행한 Google Chrome `155.0.8059.85`, macOS `26.5.1`, DPR 1. Chrome은 실제 HTML `<img>`를 페이지에 표시한다. screenshot 픽셀을 PNG decoder로 읽어 기준으로 고정했다. 결과는 [사전 비교 evidence](../spec/internal/evidence/c18-2-alpha-precomparison-2026-10-16/README.md)와 [원본 관찰 JSON](../spec/internal/evidence/c18-2-alpha-precomparison-2026-10-16/chrome155-observation.json)에 둔다.
- 재현 입력: `tests/fixtures/css/c18/image-alpha-source-over-c18-2.html`; 생성된 `tests/fixtures/resources/c18-2-alpha-source-over.png`; fixture PNG는 24×16 RGBA8, 3개 수직 띠이고 SHA-256은 `89852885917d6f22c6be237035a10447bfb85caa8c58d4ec29205ee6f652624e`이다. `python3 tools/fixtures/generate-c18-2-alpha-png.py`로 재생성하고 `node tools/css-reference/capture-c18-2-alpha-precomparison.mjs`로 Chrome 기준을 캡처한다. 캡처기는 전용 Agent Browser session을 닫는다.
- V8 입력은 고정 `img[src]` 하나와 80×40 CSS px root를 만들고 PNG bytes를 fixture adapter가 공급한다. Chrome HTML은 동일한 배경 `#112233`, 이미지 원점 `(8,12)`, natural frame `24×16`을 사용한다.
- geometry oracle: 이미지 frame `(8,12) 24×16`; 배경 `rgb(17,34,51)`; PNG source bands는 alpha 0 `(250,20,210,0)`, alpha 128 `(220,40,80,128)`, alpha 255 `(32,200,64,255)`이다. sample CSS coordinates와 expected RGBA는 사전 비교 표의 10개 점으로 고정한다.
- 판정: Android API 37 ARM64 AVD와 iPhone 17 Pro / iOS 26.2 Simulator에서 각 10개 drawable sample 및 대응하는 전체 화면 screenshot sample이 Chrome과 exact 일치해야 한다. viewport/surface scale, view origin/size, source/resource/document/style/render/surface/frame identity를 보존하고 fixture 밖의 제목·부제·상태·배경 anchor 네 점으로 전체 화면 crop과 정렬을 확인한다. Metal drawable readback과 Simulator 전체 화면 screenshot을 별도로 기록한다. C18.1 opaque/overflow fixture는 9/9 및 anchor 4/4를 유지해야 한다.
- 실패 기준: alpha 0 pixel에서 source RGB가 비치거나, alpha 128 결과가 Chrome 값과 다르거나, alpha 255/띠 경계/background anchor가 변하거나, 어떤 simulator surface format도 exact 결과를 내지 못하면 구현 slice는 통과하지 않는다. 색 형식 대응 변경이 필요하면 C18.1 opaque fixture 및 기본 color path를 함께 비교한다. 허용 오차로 결과를 감추지 않는다.

## 구현과 격리

1. `spinon-render-wgpu` 이미지 pipeline에 source-over blend를 적용하고 straight-alpha RGBA8 texel을 유지한다. 색 채널은 `src=SrcAlpha`, `dst=OneMinusSrcAlpha`, `operation=Add`; alpha 채널은 `src=One`, `dst=OneMinusSrcAlpha`, `operation=Add`의 고정 blend factor를 사용한다. alpha 0/128/255를 모두 허용하되 dimensions·byte length·revision·clip·resource 검증은 C18.1 경계를 유지한다. 첫 Android pass는 Chrome보다 반투명 R/B가 각 1 높았고, 첫 iOS `Bgra8UnormSrgb` pass는 source-over가 선형 surface에서 이루어져 반투명 sample이 달랐다. 수정 후 renderer는 가능한 경우 sRGB color space의 UNorm surface를 우선 선택하고 UNorm 출력용 sRGB re-encode 및 premultiplied 8-bit source-contribution quantization을 적용했다. 현재 표면 format fallback 전체의 exact 범위는 보증하지 않는다. C18.1 opaque/overflow regression을 두 플랫폼에서 함께 비교한다.
2. C18.2는 별도 `c18-2-alpha-image-compositing-fixture-test` feature, FFI symbol, Android/iOS build flag와 전용 selector/adapter를 둔다. C18.2 feature는 공용 surface readback/runtime 구현을 위해 opt-in C18.1 feature에 의존하지만 C18.1/C18.2 fixture module은 각각 별도 cfg를 유지한다. 기본 route와 공개 image API는 바꾸지 않는다. 고정 PNG payload, C18.2 label, JS route는 fixture opt-in에서만 포함한다.
3. Pixel comparator는 정확히 10개 oracle sample, fixture/image/screenshot hash, source/scene identity, CSS-to-surface mapping, fixture 밖 4색 anchor를 검증한다. 표본 누락·중복·좌표 변조·frame/hash/revision 불일치·배경 control 오염, alpha 0 source 누출, alpha 128을 0/255로 바꾼 잘못된 결과는 negative control로 실패해야 한다.
4. 두 simulator에서 app 내부 실제 이미지 draw와 후합성 screenshot을 수집한다. Android는 명시적으로 `emulator-5554` AVD만 쓰고 연결 실기기에는 명령을 보내지 않는다. iOS는 iPhone 17 Pro UDID만 사용한다. 다른 부팅 simulator와 공유 프로세스는 종료하지 않는다.
5. 기본/opt-in Cargo staticlib, Android APK native library 및 iOS app executable을 비교한다. 기본 artifact에서 전용 ABI·JS·PNG payload가 없어야 하며 opt-in positive control에 있어야 한다. `cargo fmt`, 관련 Rust/Node test, Clippy, fixture-boundary, Android/iOS 기본·opt-in build를 실행한다. 각 simulator run은 최신 source/build hash로 연결한다.
6. 구현 뒤 계획에서 재사용하지 않는 독립 20개 code/runtime 실패 관점 검토, 수정 및 영향 관점 재검증을 거친다. PR diff는 별도로 30개 관점으로 검토한다. merge는 rebase 방식이며, Tailnet 문서 preview는 main 반영 뒤 별도 검증하고 GitHub Pages는 배포하지 않는다.

## 완료 판정

- Agent Browser Chrome oracle의 고정 10개 RGBA와 fixture/image/screenshot hash가 저장소에 남는다.
- Android AVD PixelCopy와 앱 전체 화면 screenshot, iOS Metal readback과 iOS Simulator 전체 화면 screenshot이 각각 10/10 exact sample과 anchor 4/4를 통과한다. Android/iOS scene은 같은 80×40 CSS input과 fixed resource를 사용한다.
- C18.1 opaque/overflow 회귀는 기존 9/9 및 4/4 결과를 유지한다.
- 기본 artifact는 fixture code/ABI/payload가 없고 opt-in positive control은 포함한다.
- versioned internal interface, example, ownership/error/supported-range, status entry와 실제 실행 근거가 연결된다. C18 parent, C14, U05와 공개 CSS image 지원은 unchecked 상태를 유지한다.
