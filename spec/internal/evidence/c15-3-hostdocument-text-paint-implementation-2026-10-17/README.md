# C15.3 HostDocument text paint 구현 근거

**날짜:** 2026-10-17

**브랜치:** `feat/c15-3-hostdocument-text-paint-20261017`

**범위:** fixture-owned `HostDocumentSnapshot` → Taffy text layout snapshot → glyph raster → runtime paint → Android/iOS simulator surface

## 판정 기준

고정 Ahem `XX` fixture에서 paragraph frame은 `(0,0) 15×10 CSS px`, 한 줄 text advance는 `20 CSS px`, 오른쪽 overflow는 `5 CSS px`다. 같은 layout snapshot과 font registry에서 만든 glyph raster를 paint하고, 첫 glyph 안쪽 표본 2개와 owner 오른쪽 바깥의 두 번째 glyph 표본 4개가 화면 surface에서 불투명 잉크 `[9,33,112,255]`로 읽혀야 한다. 전체 사전 비교와 fixture 한계는 [고정 oracle](../c15-3-hostdocument-text-paint-precomparison-2026-10-17/README.md)에 기록했다.

## 실행 결과

| 관문 | 결과 |
| --- | --- |
| opt-in FFI 전체 테스트 | 49 passed, 0 failed, 2 ignored. 두 ignored는 기존 Android/iOS 앱 실행용 V8 runtime tests다. [명령](full-ffi-test-command.txt) · [최종 전체 로그](full-ffi-test-final.log) |
| Rust 포맷·lint | `cargo fmt --all -- --check`, feature-enabled `spinon-ffi` Clippy, `--workspace --all-targets` Clippy가 warning 없이 통과했다. [최종 명령과 로그](rust-lint-final.md) |
| fixture module/route 경계 검사 | Node 6 passed, 0 failed. 회귀 검사는 C15 전용 route allowlist를 포함한다. [최종 로그](fixture-module-boundary-final.log) |
| iOS route 수정 | 첫 iOS 실행에서 `--spinon-c153-hostdocument-text-paint`가 C13 route-conflict 검사에서 거부되는 결함을 발견했다. Swift allowlist와 회귀 assertion을 수정한 뒤 iPhone 17 Pro Simulator에서 재빌드·재실행했다. |
| Android 17/API 37 ARM64 AVD | 최종 분할 소스 Gradle build, install, launch 성공. PixelCopy summary `status=matched`; 첫 glyph 2/2, owner 경계 밖 glyph 4/4, 합계 6/6. 표본 모두 `[9,33,112,255]`. 이 AVD의 software-rendered surface 근거이며 실기기/hardware GPU 결과가 아니다. [최종 화면](android-api37-avd-final2-20261017T085535Z/android-screen.png) · [PixelCopy](android-api37-avd-final2-20261017T085535Z/pixelcopy.png) · [logcat](android-api37-avd-final2-20261017T085535Z/android-logcat.txt) · [APK 빌드 hash](android-api37-avd-final2-20261017T085535Z/sha256.txt) |
| iPhone 17 Pro/iOS 26.2 Simulator | 최종 분할 소스 Xcode build, install, launch 성공. Metal drawable readback `status=matched`; 첫 glyph 2/2, overflow glyph 4/4. 표본 모두 `[9,33,112,255]`, `renderer=metal-drawable-before-compositor`. [최종 화면](ios-simulator-iphone17pro-final2-20261017T085535Z/ios-screen.png) · [unified log](ios-simulator-iphone17pro-final2-20261017T085535Z/ios-unified.log) · [앱 바이너리 hash](ios-simulator-iphone17pro-final2-20261017T085535Z/sha256.txt) |
| default/opt-in Rust artifact 경계 | 최종 분할 소스로 다시 빌드한 뒤 default `libspinon_ffi`에는 C15.3 HostDocument/glyph FFI symbol과 Ahem/Noto Sans KR bytes가 없었다. opt-in archive에는 두 symbol과 두 font payload가 있었다. Noto Sans Arabic/Emoji payload는 두 artifact 모두에서 발견되지 않았다. [default build](artifact-boundary-default-final.log) · [opt-in build](artifact-boundary-optin-final.log) · [byte/symbol scan](artifact-boundary-scan-final.json) |

fixture 소스는 HostDocument/layout snapshot, raster/sample, runtime bridge의 세 책임으로 나눠 각 Rust 파일을 500줄 이내로 유지했다.

## 재현 명령과 산출물

테스트와 빌드는 공간이 비어 있는 전용 `/Volumes/ZTSBuild/spinon-c15-3-20261017/target`을 사용했다. 저장소의 기존 target과 다른 worktree는 보존했다.

- [opt-in FFI test 명령](full-ffi-test-command.txt)
- [Rust formatter·Clippy 최종 명령과 로그](rust-lint-final.md)
- [default artifact build 명령](artifact-boundary-default-command.txt) · [최종 log](artifact-boundary-default-final.log)
- [opt-in artifact build 명령](artifact-boundary-optin-command.txt) · [최종 log](artifact-boundary-optin-final.log)
- [artifact scan 방법](artifact-boundary-scan-command.txt) · [최종 scan](artifact-boundary-scan-final.json)
- [최종 iOS Xcode build log](ios-simulator-iphone17pro-final2-20261017T085535Z/build.log) · [Metal readback log](ios-simulator-iphone17pro-final2-20261017T085535Z/ios-unified.log)
- [최종 Android Gradle log](android-api37-avd-final2-20261017T085535Z/build.log) · [PixelCopy log](android-api37-avd-final2-20261017T085535Z/android-logcat.txt)
- [독립 구현 실패 관점 20개](implementation-review-20.md)
- [독립 PR 변경 적대 검토 30개](pr-delta-review-30.md)

## 남은 범위

이 slice는 합성 fixture-owned HostDocument 하나와 직접 text child `XX`, 고정 Ahem font에 한정한다. 일반 live V8 DOM text admission, computed CSS font/style projection, fallback/font loading, 일반 wrapping/inline formatting, 제품 text hit target·DOM event, 실기기, Android hardware GPU, iOS compositor 이후 surface 및 전체 CSS Text 적합성을 입증하지 않는다. C15.3과 상위 C15는 계속 미완료이며 공식 완료 수는 `61/197`이다.
