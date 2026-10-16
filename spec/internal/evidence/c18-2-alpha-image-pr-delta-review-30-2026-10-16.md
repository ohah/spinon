# C18.2 PR 변경 적대 검토 — 2026-10-16

검토 대상은 C18.2 구현, 내부 계약 0065, 상태·conformance 변경, Android/iOS fixture 진입점, artifact 경계 및 실행 근거다. 기존 계획 검토 20개와 구현 검토 20개에서 확인한 항목을 복사하지 않고, PR 통합 경계의 별도 실패 경로 30개를 현재 diff와 저장된 실행 산출물에 대조했다.

| # | 실패 관점 | 확인 결과 |
| ---: | --- | --- |
| 1 | UNorm surface에 sRGB PNG 값을 선형 값처럼 직접 출력해 Chrome과 색이 어긋남 | PNG texture는 `Rgba8UnormSrgb`로 샘플링하고 UNorm 출력 경로에서만 sRGB encode를 수행한다. Chrome oracle 10개 표본과 양 플랫폼에서 exact다. |
| 2 | sRGB surface 경로에 UNorm용 재인코딩을 중복 적용함 | sRGB 형식은 `encode_srgb=0` 경로를 사용한다. 현재 완료 범위는 실측한 UNorm 형식이며 sRGB-only fallback의 exact 결과는 주장하지 않는다. |
| 3 | WGPU source-over의 RGB와 alpha factor를 혼동함 | image pipeline은 straight source RGBA와 `ALPHA_BLENDING`을 함께 사용한다. 불투명 backdrop 출력 alpha는 표본 전체에서 255다. |
| 4 | alpha 0 texel의 숨은 magenta RGB가 배경에 번짐 | transparent interior와 경계 sample은 Android PixelCopy/full-screen 및 iOS Metal/full-screen에서 backdrop `(17,34,51,255)`와 exact다. |
| 5 | alpha 128을 opaque 처리하거나 허용 오차로 가림 | 최초 차이 `(119,37,66)`을 보존했고 현재 expected `(118,37,65)`와 zero-tolerance 비교한다. alpha를 0/255로 바꾸는 negative control은 거부된다. |
| 6 | blend 수정이 C18.1 opaque image/overflow를 회귀시킴 | 현재 renderer source의 C18.1 route를 각 플랫폼에서 별도 재실행했다. 표면/drawable 9/9, full-screen 9/9, 외부 anchor 4/4가 exact다. |
| 7 | PNG texel을 linear texture로 읽어 입력 색이 달라짐 | texture format은 `Rgba8UnormSrgb`이며 encoded PNG 및 decoded RGBA SHA-256을 두 manifest에서 고정했다. |
| 8 | straight-alpha PNG를 premultiplied 입력으로 다시 곱함 | caller의 RGBA bytes를 그대로 업로드하고 fragment에서 색상 변환만 한다. 부분 alpha와 투명 source 표본이 oracle과 exact다. |
| 9 | alpha 0에서 premultiplied 보정의 0 나눗셈이 발생함 | 8-bit 보정은 `0 < alpha < 1`일 때만 계산한다. alpha 0 표본이 배경을 유지한다. |
| 10 | sRGB piecewise 함수가 경계 또는 음수 입력에서 부정확함 | shader가 `0.0031308` 기준과 `max(channel, 0)`을 사용한다. 주어진 RGBA8 입력 표본에서 Android/iOS 모두 strict exact다. |
| 11 | surface format 선택이 color-space capability와 무관함 | 후보는 `format_capabilities`에 있고 sRGB color space를 지원할 때만 선택한다. UNorm을 먼저 고른 뒤 sRGB texture fallback을 둔다. |
| 12 | 지원 surface format이 없는데 다른 형식으로 조용히 진행함 | 지원 후보가 없으면 명시적 initialization 오류를 반환한다. 미측정 fallback을 pass로 확장하지 않는다. |
| 13 | renderer clear/color encoding과 image shader 인코딩 설정이 불일치함 | `color_encoding(format)`은 sRGB 형식에서 Linear, UNorm에서 Srgb를 선택하고 image output flag는 같은 format의 `is_srgb()`에서 파생한다. |
| 14 | full-screen screenshot 표본이 잘못된 view origin/scale로 우연히 통과함 | manifests에 surface origin·크기·viewport scale을 보존하고 comparator가 CSS, drawable, screen pixel 좌표를 모두 검증한다. |
| 15 | iOS Metal readback을 OS compositor 뒤 결과로 잘못 표시함 | contract와 evidence는 drawable readback을 compositor 전으로 구분하고 Simulator full-screen screenshot을 별도 후합성 표본으로 기록한다. 물리 scanout은 제외했다. |
| 16 | intrinsic PNG 크기를 DPR 또는 화면 크기로 확대해 우연히 표본을 맞춤 | input은 24×16이고 CSS frame은 24×16 natural size다. viewport는 80×40이며 physical scale은 surface 변환에만 반영한다. |
| 17 | band 경계에서 선형 보간으로 색이 섞임 | sampler는 nearest/clamp이고 표본은 alpha band 좌우 경계 및 내부를 포함한다. comparator는 10개 exact 점을 모두 검사한다. |
| 18 | 이전 document/resource revision의 이미지를 최신 render tree에 붙임 | image resource key가 generation·document·render-tree·intrinsic revision을 포함하고 fixture report의 revision과 실제 draw log를 대조한다. |
| 19 | 같은 node/dimensions에서 PNG bytes만 변해도 cache hit로 stale texture를 사용함 | `ImageCacheKey`에 RGBA SHA-256을 포함한다. 동일한 기하·node의 변경 pixel bytes는 다른 cache key를 만든다. |
| 20 | dimension 곱셈 overflow 또는 잘못된 byte length가 GPU upload 전에 걸러지지 않음 | width×height×4를 checked arithmetic으로 계산하고 변환 가능한 정확한 RGBA length만 허용한다. |
| 21 | 대형 texture가 기기 limit을 넘겨 검증 오류 또는 불필요한 할당을 유발함 | device의 `max_texture_dimension_2d`보다 큰 image를 `create_image` 전에 거부한다. |
| 22 | image 목록·vertex buffer 확장 할당 실패가 panic으로 이어짐 | image key와 geometry vertex reserve에 fallible reserve를 사용하고 실패를 오류 문자열로 반환한다. |
| 23 | texture creation/upload의 validation 또는 OOM 오류가 성공 report로 숨겨짐 | WGPU texture와 pipeline 생성에 validation/OOM error scope를 두고 오류를 표면화한다. simulator log의 draw report는 success만 포함한다. |
| 24 | HTML이 아닌 namespace의 `img` 또는 다른 tag를 fixture image로 오인함 | traversal은 HTML namespace와 local name `img`를 함께 확인하고 고정 id/source 이외의 입력은 실패한다. |
| 25 | img 누락·중복 또는 예상 밖 요소가 임의로 선택됨 | fixture attachment는 HTML img 정확히 하나, 고유 고정 id, 고정 source를 요구한다. missing/unexpected/duplicate/changed source negative unit case가 있다. |
| 26 | image가 layout snapshot에 없거나 clip/frame이 달라도 고정 report를 만듦 | 동일 document snapshot에서 request/decode/resource snapshot을 만들고 그 node의 render box/frame/clip을 찾아 image를 scene에 결합한다. |
| 27 | 기본 runtime build에 C18.2 fixture 심볼 또는 payload가 포함됨 | Cargo·Android APK native library·iOS app executable의 default/opt-in scan에서 전용 ABI, JS route, PNG bytes가 default에는 없고 opt-in에만 있음을 기록했다. |
| 28 | C18.2 build flag가 C04 runtime GPU 없이 켜지거나 잘못된 값으로 통과함 | Android Gradle 및 iOS shell build는 flag를 0/1로 제한하고 C04 runtime GPU가 켜진 경우만 허용한다. |
| 29 | Android 기본 Activity 실행이 fixture를 자동으로 열거나 잘못된 host를 사용함 | 별도 C18.2 opt-in flag가 route 선택과 adapter source를 제어한다. default app artifact에는 route가 없고 검증 대상은 API 37 ARM64 AVD뿐이다. |
| 30 | iOS selector·Objective-C++ ABI·Swift build setting이 서로 다르거나 기본 실행에 노출됨 | C header, Objective-C++ wrapper, Swift selector, Xcode build setting과 native feature flag 이름을 대조했다. 기본값은 off이며 fixture run은 지정 iPhone 17 Pro Simulator에 한정한다. |

## 결과와 남은 경계

- Android API 37 ARM64 AVD: PixelCopy 10/10, full-screen 10/10, fixture 밖 anchor 4/4.
- iPhone 17 Pro iOS 26.2 Simulator: Metal drawable 10/10, full-screen 10/10, fixture 밖 anchor 4/4.
- C18.1 회귀: Android/iOS 각 surface/drawable 9/9, screenshot 9/9, anchor 4/4.
- 렌더러·FFI·Node 검사, targeted Clippy, rustfmt, 기본/opt-in artifact 경계 결과는 [Android 근거](./c18-2-alpha-image-android-api37-2026-10-16/README.md), [iOS 근거](./c18-2-alpha-image-ios-simulator-2026-10-16/README.md), [artifact 경계](./c18-2-alpha-image-android-api37-2026-10-16/artifact-boundary/README.md)에 연결했다.
- sRGB-only surface exactness, alpha-bearing root surface, URL/network loader, async replacement, image scaling, 실제 iPhone/Android 기기 및 hardware GPU는 검증하거나 완료 처리하지 않았다.
