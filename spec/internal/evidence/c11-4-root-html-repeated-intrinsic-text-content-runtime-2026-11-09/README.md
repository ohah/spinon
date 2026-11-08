# C11.4 repeated intrinsic text content · simulator/runtime smoke

**판정:** Android와 iOS 기본 Root HTML 앱 경로에서 세 corpus가 모두 부팅되어 `layout=ready`를 반환했다. 이는 화면 초기화와 제한된 fixture의 앱 통합 smoke다. Chrome과 비교한 810-scene 기하 판정은 별도 RuntimeSession 테스트에서 했으며, 시뮬레이터에서 810개 장면을 실행하지 않았다.

## 기기와 빌드

- Android Emulator `spinon_c114_android37_runtime_20261108`, `emulator-5554`, API 37, Android 17, `arm64-v8a`, `ranchu`. Android 앱은 `mise exec -- bun run build:android`로 빌드했다. Emulator AVD는 소프트웨어 그래픽을 사용하므로 실기기/hardware GPU 근거가 아니다.
- iPhone 17 Pro Simulator `ACA7BF91-E2D5-4CF7-909A-08D1AD95FF3D`, iOS 26.2, Xcode 26.2 (`17C52`). 고정 V8 revision `7b50b62cb18f28617959e8452e2cd18195b38bcf`의 iOS Simulator static library가 없어 `bash tools/v8/build-ios-iphonesimulator.sh`로 빌드한 뒤 `mise exec -- bun run build:ios-sim`을 실행했다.
- 세 원본 fixture의 SHA-256은 Android와 iOS run별 `*-source.sha256`에 기록했다. 앱 빌드용 wrapper는 첫 `<style>`을 Root HTML color-scheme smoke가 요구하는 고정 내용으로 두고, fixture CSS/Grid와 `main#app` marker를 넣었다. wrapper hash는 `*-wrapped-app-asset.sha256`에, 빌드 입력 hash는 Android에서 `*-built-app-asset.sha256`에 기록했다. 각 명령 종료 시 `examples/bootstrap/index.html`은 원래 SHA-256 `e7ecee32a5c5f645cc90ea7bd0c524caaf9da93fd4c780ff9dbb29d0894dd3b9`로 복원됐다. `root-app.js`는 수정하지 않았다.

## 실행 결과

| Corpus | Android AVD | iPhone 17 Pro | 화면 |
|---|---|---|---|
| 기존 #1290 `legacy-1290` | `layout=ready`, `boxes=21` | `layout=ready`, `boxes=21` | [Android](android/legacy-1290.png), [iOS](ios/legacy-1290.png) |
| `varied-ascii` | `layout=ready`, `boxes=21` | `layout=ready`, `boxes=21` | [Android](android/varied-ascii.png), [iOS](ios/varied-ascii.png) |
| `length-boundary-32` | `layout=ready`, `boxes=21` | `layout=ready`, `boxes=18` | [Android](android/boundary-32.png), [iOS](ios/boundary-32.png) |

각 플랫폼의 per-corpus build/run log와 app asset hash는 `android/` 및 `ios/` 아래에 있다. 검토용 Android run log와 iOS build log에서는 줄 끝 공백·마지막 빈 줄만 정리했고, 직전 원본은 같은 파일명의 `.raw.gz`로 보존했다. Android varied ASCII 첫 설치 시 build script 성공 뒤 APK 경로를 잘못 지정한 설치 시도가 한 번 실패했다. 올바른 `platforms/android/app/build/outputs/apk/debug/app-debug.apk` 경로로 재시도해 설치와 앱 실행은 통과했다. 이 혼선은 빌드 실패가 아니며 데이터 초기화 명령은 실행하지 않았다.

## 앱 데이터 연속성

iOS 앱을 다시 설치한 뒤 app data container UUID가 `805C6572-E396-401D-80FF-B6091C3BBE03`에서 `00EBD278-C602-45C9-8841-FE2EFF0323B3`로 바뀌었다. 이전·이후 `Documents`의 18개 파일을 상대 경로와 SHA-256으로 정규화해 비교했고 모두 byte-identical이었다. 비교 자료는 `app-data-before.sha256`, `app-data-after.sha256`, `documents-before-normalized.sha256`, `documents-after-normalized.sha256`이다. 이전 container의 `Library/Saved Application State` 파일은 새 container에 없었으며, 앱 문서 파일과 구분되는 OS 복원 상태다. 시뮬레이터 erase나 앱 데이터 수동 삭제는 하지 않았다.

## 테스트 자원 정리

세 corpus의 실행과 캡처를 마친 뒤, C11.4 전용 Android AVD `spinon_c114_android37_runtime_20261108`(약 2.1 GB)을 삭제했다. 저장소에는 재현에 필요한 실행 로그·기기 메타데이터·화면을 보존했고, 공유 Android API 37 system image는 남겼다. iPhone 17 Pro Simulator는 종료했지만 표준 Xcode 기기와 Documents 데이터는 보존했다. `S03 Pilot`이라는 iOS Simulator 기기는 이 검사 때 등록되어 있지 않았다.

## 해석 범위

화면은 OS screenshot visual smoke이며 PixelCopy, Metal surface readback 또는 픽셀 동등성 측정이 아니다. Android/iOS 실기기, hardware GPU, 810-scene simulator matrix, 일반 HTML/CSS 호환 범위는 검증하지 않았다. 전체 `spinon-style-to-layout --lib` 테스트는 기존과 원인 귀속이 미확정인 C07.3 aspect-ratio computed-value 비교에서 실패했고 focused rerun에서도 재현됐다. C11.4/C11 완료 체크와 공식 `78/214`는 유지한다.
