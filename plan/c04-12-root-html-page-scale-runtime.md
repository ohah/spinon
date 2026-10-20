# C04.12 후속 · 기본 root HTML runtime의 초기 page scale

- **문서 유형:** 구현 계획 · 공식 구현 상태는 [`spec/STATUS.md`](../spec/STATUS.md)에서 관리
- **상위:** [앱 루트 HTML과 문서 환경 metadata](c04-root-html-document-environment.md), [기본 root HTML runtime](c04-12-root-html-default-rendering.md), [초기 page scale WGPU 변환](c04-12-page-scale-rendering.md)
- **상태:** 구현과 Android/iOS 시뮬레이터 확인 완료, PR 적대적 diff 검토와 병합 대기. C04.12와 공개 지원 상태는 미완료다.
- **계획 실패 검토:** [독립 관점 20개](../spec/internal/evidence/c04-12-root-html-page-scale-runtime-plan-review-20-2026-10-20.md)
- **구현 실패 검토:** [새로운 독립 관점 20개](../spec/internal/evidence/c04-12-root-html-page-scale-runtime-2026-10-20/implementation-review-20.md)

## 목표와 경계

현재 `RuntimeSession::set_root_html_environment()`는 HTML viewport metadata에서 계산한 `device_scale_factor`와 `page_scale`을 반환하지만, 기본 root HTML scene은 그 값을 `RuntimeRenderSnapshot`의 WGPU mapping에 연결하지 않는다. 이 작업은 동일한 HTML5 parsed document에서 계산한 초기 `page_scale`을 일반 root HTML GPU host가 surface에 그릴 때 적용한다. 기존 `RuntimePageScaleTransform`과 WGPU geometry 경로를 재사용한다.

layout과 media query는 CSS layout viewport 좌표를 계속 사용한다. `RuntimeRenderSnapshot`의 CSS hit-test 좌표, root Canvas clear, native surface 크기와 device scale은 page scale과 구분한다. 선택된 HTML scene의 box, border, image, glyph paint는 `device_scale_factor × page_scale` 배율을 한 번 적용한다. 물리 surface 경계가 확대된 장면을 자른다. 기본 `initial-scale=1` 화면은 이전과 같은 위치와 크기로 남아야 한다.

이 단계는 초기 시각 변환이다. pinch/pan, scroll 뒤 visual viewport 이동, native pointer 좌표 변환, 글꼴 재래스터화, safe-area, arbitrary HTML/CSS 또는 C04.12 전체 완료를 포함하지 않는다. 입력 처리는 별도 미지원 상태로 유지하고, hit-test API가 물리 좌표를 받는 것처럼 설명하지 않는다.

## 고정 비교 기준

- 기준 브라우저는 Agent Browser CLI `0.31.1`, Chrome `155.0.8059.85`다. 이전 iPhone 12 emulation 캡처는 브라우저 동작과 DPR×page-scale의 선형 mapping을 고정하는 oracle로 재사용한다. 실제 C0410 앱 검증 host는 양 플랫폼에서 `320×240` CSS px surface를 사용하므로 iPhone 12의 `390×844` viewport 값을 앱 환경 값으로 주장하지 않는다.
- 앱 검증 환경의 CSS viewport는 `320×240`이고, 앱이 전달하는 DPR은 각 시뮬레이터에서 관측한다. Android API 37 AVD는 물리 `1080×2400`, density `420`(DPR `2.625`)이다. 활성 iPhone 17 Pro Simulator에는 기존 Spinon 검증 프로세스가 있어 종료하지 않는다. 같은 iOS 26.2 runtime의 별도 iPhone 17 Pro Max Simulator를 사용하고 실행 log의 display scale을 기록한다. 기본 CSS probe는 `(100,50)`의 `20×10` box다. 보조 `surface-edge-probe`는 앞 probe 아래 `(200,60)`의 `20×10` box다. surface origin은 `(0,0)`이며 safe-area inset은 적용하지 않는다.

| HTML viewport metadata | 앱의 CSS layout viewport | visual viewport | Android DPR `2.625`: scale · probe | iOS DPR `3`: scale · probe |
| --- | ---: | ---: | ---: | ---: |
| `initial-scale=0.5` | `640×480` | `640×480` | `1.3125` · `[131.25,157.5) × [65.625,78.75)` | `1.5` · `[150,180) × [75,90)` |
| `initial-scale=2` | `320×240` | `160×120` | `5.25` · `[525,630) × [262.5,315)` | `6` · `[600,720) × [300,360)` |
| `width=500, initial-scale=1` | `500×375` | `320×240` | `2.625` · `[262.5,315) × [131.25,157.5)` | `3` · `[300,360) × [150,180)` |

CSS viewport 값은 root HTML environment report와 대조한다. iPhone의 정수 물리 경계는 정확 비교하고, Android AVD처럼 `DPR × page_scale`이 분수인 표면은 pixel-center coverage를 확인하며 경계 오차를 최대 1 physical pixel로 제한한다. 두 번째 `surface-edge-probe`는 각 배율에서 보이는지 surface 밖에서 잘리는지 확인한다. `(10,10)`은 Canvas clear가 전체 surface에서 유지되는지 확인한다. 이는 그 외 scene 색상이나 전체 페이지의 Chrome pixel parity를 주장하지 않는다. 현재 simulator 캡처의 색상 mask 측정은 [검증 기록](../spec/internal/evidence/c04-12-root-html-page-scale-runtime-2026-10-20/simulator-pixel-analysis.md)에 둔다.

## 구현 요구사항

1. root HTML GPU host가 `set_root_html_environment()`에서 얻은 환경 snapshot의 `environment_revision`, DPR, 초기 page scale을 보관한다. 새 surface/DPR/OS scheme 입력으로 환경을 바꾸면 이전 mapping도 무효화한다.
2. layout 결과를 받은 뒤 scene의 `environment_revision`과 보관한 환경 snapshot이 같을 때만 `with_page_scale_transform(dpr, page_scale)`을 적용한다. 값이 없거나 revision이 다르면 root HTML scene을 게시하지 않고 구체적인 stale/environment 오류를 반환한다.
3. 변환은 root HTML default-render host에만 적용한다. 일반 JavaScript GPU host, 기존 scene fixture 및 mapping 없는 renderer 호출은 현재 surface-fit 동작을 보존한다.
4. 변환은 완성된 scene의 images, glyphs, paint commands를 포함해 presentation publication 전에 한 번 적용한다. scene 재발행·image attachment 순서 때문에 배율이 사라지거나 두 번 곱해지지 않아야 한다.
5. native Canvas clear와 app system UI는 surface 전체의 기존 물리 경계를 유지한다. layout, `RuntimeLayoutFrame`, CSS hit-test frame, media query와 renderer viewport 값은 CSS px로 유지한다.
6. metadata에서 나온 배율이 유한한 양수가 아니거나 DPR과의 곱셈이 overflow하면 scene mapping 생성이 실패한다. 실패한 환경의 부분 장면이나 이전 scale 장면을 새 revision 성공으로 게시하지 않는다.
7. `initial-scale=1`의 기존 Android/iOS default app screen을 regression control로 유지한다. 테스트 HTML은 `tests/fixtures/css/c04/` 및 두 플랫폼 bundle에 같은 bytes로 둔다. C0410 opt-in route는 일반 `spinon_runtime_gpu_host_new_with_root_html()`와 `set_environment()`를 호출하며 검증 viewport는 `320×240` CSS px surface다. scale `0.5`, `2`, fixed width `500`을 Android API 37 ARM64 AVD와 iPhone 17 Pro Max/iOS 26.2 Simulator의 앱 GPU surface에서 확인한다. 실기기는 사용하지 않는다.
8. 각 case에서 최종 앱 입력 HTML의 SHA-256, metadata, environment revision, scene key, CSS probe frame과 mapping 배율을 보존한다. 화면 캡처에서 물리 probe bounds, `surface-edge-probe`의 clip, Canvas clear를 대조한다.
9. 내부 계약 0077/0081과 `spec/STATUS.md`에 transform 적용 범위와 input mapping 미지원을 적는다. C04.12 상위 checkbox, 공개 API 완료 수와 내부 숫자 버전은 바꾸지 않는다.

## 검증 계획

- Rust tests: mapping 부착 host/profile 경계, environment revision 불일치, 새 environment 뒤 이전 transform 무효화, `initial-scale=1` identity, scene publication 실패 원자성, CSS frame/hit-test 불변성을 확인한다.
- Renderer/WGPU tests: 세 고정 scale case의 boxes와 borders, image, glyph, clip, opacity group 및 transform 없는 legacy scene을 확인한다. 기준 geometry는 위 Chrome 표와 기존 page-scale WGPU readback fixture를 사용한다.
- Android: API 37 ARM64 AVD에서 실제 root HTML GPU app route의 scale `0.5`, `2`, fixed width `500`, 그리고 기본 앱 `initial-scale=1`을 실행한다. Surface capture의 pixel-center, clip edge와 clear background를 저장한다.
- iOS: iPhone 17 Pro Max/iOS 26.2 Simulator에서 같은 네 case를 실제 Metal drawable에 그리고 표본과 revision 로그를 저장한다.
- product feature build, fixture-positive control, Android/iOS build, `cargo fmt`, 관련 crate tests/Clippy를 기록한다. 미실행 항목은 통과로 보고하지 않는다.
- 구현 완료 뒤 별도 구현 실패 관점 검토 20개와 전체 PR diff의 30개 review를 남긴다. Agent Browser 프로세스는 이번 작업에서 시작한 것이 있을 경우 확인 후 종료한다. 기존 tmux·Tailnet 서버와 보존 중인 캡처는 종료·삭제하지 않는다.

## 구현 결과

- root HTML default GPU host가 environment snapshot의 revision·DPR·`page_scale`을 같은 revision의 render scene에 publication 전에 적용한다. CSS layout, media query, hit-test frame, Canvas clear, surface 크기는 CSS/physical 기준을 각각 유지한다.
- Android API 37 ARM64 AVD와 iPhone 17 Pro Max/iOS 26.2 Simulator에서 default, scale 0.5, scale 2, fixed width 500 사례를 확인했다. 두 플랫폼의 화면 pixel bounds, surface clear 표본, scale 2의 surface-edge clipping과 실제 HTML source hash는 [분석 기록](../spec/internal/evidence/c04-12-root-html-page-scale-runtime-2026-10-20/simulator-pixel-analysis.md)에 있다.
- focused Rust tests 5개, 기존 RuntimeRender page-scale test 1개, WGPU geometry/readback/glyph/image page-scale tests 6개, 제품 feature build, fixture feature Clippy, format, Android/iOS Simulator app builds가 통과했다. 구현 실패 관점 20개 검토도 끝났다. 전체 PR diff의 별도 30개 검토, PR checks, rebase merge는 남아 있다.

## 완료 판정과 한계

세 viewport case에서 metadata, CSS layout viewport와 실제 app surface의 scene 변환이 같은 revision을 사용하고 Chrome 기준 geometry와 정해진 허용치 안에서 일치해야 한다. 기본 scale 1과 변환 없는 GPU host는 회귀가 없어야 한다. CSS frame과 hit-test 좌표는 변하지 않아야 한다.

이 결과는 simulator와 WGPU surface 범위다. 실기기, Android hardware GPU, iOS compositor 제출 이후 픽셀, pinch/pan/scroll input, text rerasterization, 일반 HTML/CSS 호환성, WPT 또는 C04.12 전체 완료의 증거로 확대하지 않는다.
