# C15.3 non-zero scroll text transform 실행 근거

이 제한 slice는 `RuntimeRenderText`를 base origin과 현재 ancestor scroll offset 합으로 다시 계산한다. horizontal `3→6→0`, vertical `18→7→0`, nested `4+5→2+3→0+0` sequence에서 glyph origin, effective owner clip, box-owner hit-test를 확인한다. base geometry 재사용으로 offset 변경을 반복해도 이동량이 누적되지 않는다.

고정 CSS fixture는 scrollport 밖 visible ink, hidden clip 안팎, 수평·수직·nested scroll glyph를 포함한다. Android API 37 AVD PixelCopy와 iOS 26.2 Simulator Metal drawable에서 6개 표본을 모두 비교했다. 각 플랫폼 결과는 `status=matched`, `6/6`이며 표본 좌표는 [Android 로그](https://github.com/ohah/spinon/blob/main/spec/internal/evidence/c15-3-text-scroll-transform-2026-10-17/android-api37-avd.log)와 [iOS 로그](https://github.com/ohah/spinon/blob/main/spec/internal/evidence/c15-3-text-scroll-transform-2026-10-17/ios-26-2-simulator.log)에 남겼다. iOS 표면은 compositor 제출 전 Metal drawable이다.

## 화면

아래 화면은 stage 경계 밖의 overflow ink와 세 scroll 사례를 보여준다. 표면 색상 판정은 로그의 실제 PixelCopy/Metal drawable 표본으로 한다.

![Android API 37 ARM64 AVD 화면](https://raw.githubusercontent.com/ohah/spinon/main/spec/internal/evidence/c15-3-text-scroll-transform-2026-10-17/android-api37-avd.png)

![iPhone 17 Pro iOS 26.2 Simulator 화면](https://raw.githubusercontent.com/ohah/spinon/main/spec/internal/evidence/c15-3-text-scroll-transform-2026-10-17/ios-26-2-simulator.png)

## 검증

- Rust 단위·fixture·Clippy·rustfmt [로그](https://github.com/ohah/spinon/blob/main/spec/internal/evidence/c15-3-text-scroll-transform-2026-10-17/rust-validation.log): renderer 95, WGPU 46, style-to-render 71 passed/1 ignored, opt-in FFI 47 passed/2 ignored. Clippy 경고는 없다.
- Android API 37 AVD [빌드 로그](https://github.com/ohah/spinon/blob/main/spec/internal/evidence/c15-3-text-scroll-transform-2026-10-17/android-build.log)와 [PixelCopy 실행 로그](https://github.com/ohah/spinon/blob/main/spec/internal/evidence/c15-3-text-scroll-transform-2026-10-17/android-api37-avd.log): `visible_outside_ink`, `hidden_inside_ink`, `hidden_outside_background`, horizontal, vertical, nested scroll 표본 각각 기대 RGBA와 일치했다.
- iOS Simulator [빌드 로그](https://github.com/ohah/spinon/blob/main/spec/internal/evidence/c15-3-text-scroll-transform-2026-10-17/ios-simulator-build.log)와 [Metal drawable 실행 로그](https://github.com/ohah/spinon/blob/main/spec/internal/evidence/c15-3-text-scroll-transform-2026-10-17/ios-26-2-simulator.log): 같은 여섯 표본이 `Bgra8Unorm` drawable에서 기대 RGBA와 일치했다.
- 빌드 환경과 실행 조건은 [environment.txt](https://github.com/ohah/spinon/blob/main/spec/internal/evidence/c15-3-text-scroll-transform-2026-10-17/environment.txt)에 고정했다. 파일 SHA-256은 [sha256.txt](https://github.com/ohah/spinon/blob/main/spec/internal/evidence/c15-3-text-scroll-transform-2026-10-17/sha256.txt)에 기록한다.
- 새 계약·근거·계획·색인의 local link target은 [targeted-links.log](https://github.com/ohah/spinon/blob/main/spec/internal/evidence/c15-3-text-scroll-transform-2026-10-17/targeted-links.log)에서 확인했다.
- 저장소 전체 Rspress `docs:build`는 다른 문서의 dead link 진단 490건과 과거 C13.1/S03-R06 문서의 누락 이미지 참조 6건으로 실패했다. 이번 계약 0074와 이 근거 README는 오류 목록에 없다. 전체 결과는 [docs-build.log](https://github.com/ohah/spinon/blob/main/spec/internal/evidence/c15-3-text-scroll-transform-2026-10-17/docs-build.log)에 보존했다.
- [PR #261](https://github.com/ohah/spinon/pull/261) 본문은 Agent Browser에서 확인했다. Android와 iOS raw screenshot이 모두 로드되어 각각 1080×2400, 1206×2622로 표시됐다.
- 구현 실패 경로 20개와 변경 diff 실패 경로 20개는 [구현 검토](implementation-review-20.md), [PR diff 검토](pr-delta-review-20.md)에 있다.

텍스트 로그의 줄 끝 공백은 저장소 diff 검사를 위해 제거했다. 실행·빌드 결과 표식과 실질 출력은 유지한다. `sha256.txt`는 정규화된 최종 로그를 가리킨다.

## 경계

이는 고정 opt-in runtime fixture에서의 glyph paint, clip, box-owner hit-test 결과다. 일반 DOM text admission·glyph별 DOM hit target·text-own scroll container는 다루지 않는다. Android 결과는 API 37 ARM64 AVD의 software graphics path이며 실기기/GPU 증거가 아니다. iOS 결과는 Simulator의 drawable readback으로 compositor 이후 scanout을 뜻하지 않는다. C15.3과 C15 상위 항목은 미완료다.

내부 계약은 [0074](../../0074-c15-3-runtime-glyph-scroll-transform.md), 계획은 [C15 inline text layout](https://github.com/ohah/spinon/blob/main/plan/c15-inline-text-layout.md), 공식 상태는 [STATUS.md](../../../STATUS.md)다.
