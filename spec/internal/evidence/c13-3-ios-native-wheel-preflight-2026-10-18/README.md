# C13.3 iPadOS native wheel 입력 사전 점검 · 2026-10-18

## 범위

iPadOS Simulator에서 `.discrete` mouse-wheel 및 `.continuous` trackpad recognizer가 native callback을 받는지 확인하기 위한 재실행이다. 이 실행에서는 native scroll 입력을 보내지 못해 callback, source 분류, translation 부호, queue와 draw 결과를 관찰하지 않았다. 따라서 iOS adapter 구현 gate는 계속 미통과다.

## 빌드와 실행

- 소스 기준: `12e5daa75f8ae15dfd1c720e08b1ca739dcbb637`.
- iPad Pro 13-inch (M5) / iOS 26.2 Simulator, UDID `9F29B0ED-C7E6-46AE-A933-14E1A87FC4DF`.
- 고정 V8 revision `7b50b62cb18f28617959e8452e2cd18195b38bcf`를 사용했다.
- `SPINON_ENABLE_C04_RUNTIME_GPU=1`, `SPINON_ENABLE_C13_3_GESTURE_FIXTURE_TEST=1`로 `mise exec -- bun run build:ios-sim`을 실행했고 `** BUILD SUCCEEDED **`를 확인했다. 원본 출력은 [build.log](build.log)에 있다.
- 설치된 앱을 `--spinon-c133-scroll-gesture --spinon-c133-native-wheel-preflight` 인자로 실행했다. 앱은 `SPINON_C133_READY`에서 viewport `400×240 CSS px`, canvas bounds `400×240 UI points`, screen scale `2.0`, scroll revision `0`, 초기 offsets `0,0`으로 준비됐다. 앱 로그는 [initial-launch.log](initial-launch.log), 시작 화면은 [initial-launch.png](initial-launch.png)다. 이 이미지는 ready 상태만 보여주며 wheel 동작 화면이 아니다.
- 앱 실행 파일 SHA-256은 `9b5f146e99d9d131bd0d848929f3b2cf507c271c4ae561055bde9530ab38100a`다. 상세 결합값은 [source-revision.json](source-revision.json)에 기록했다.

## 입력이 도달하지 않은 이유

Simulator 앱 프로세스는 살아 있고 세 개의 Simulator 창도 WindowServer 목록에 존재했다. 그러나 목록의 최상단에 `loginwindow`가 화면 전체 크기 `1920×1080`, layer `2004`로 있고 그 아래에 `30000×30000`, layer `2001` 창이 있었다. 전경 앱은 System Settings였다. Simulator 활성화와 창 올리기를 요청해도 전경 앱은 바뀌지 않았다. [host-window-state.txt](host-window-state.txt)와 [host-frontmost-app.txt](host-frontmost-app.txt)에 관찰값을 보존했다.

macOS 잠금 화면이 덮인 세션에 입력을 우회 주입하지 않았다. 따라서 이번 실행에서 event를 보내지 않았고, recognizer의 callback 부재를 native 입력 거부 결과로 해석하지 않는다. 실제 Simulator 입력을 전달할 수 있는 잠금 해제 GUI 세션에서 discrete와 continuous를 각각 관찰해야 한다.

## 판정

- iOS native callback/source/sign: 미관찰.
- queue admission, scroll offset, draw revision 및 pixel 결과: 실행하지 않음.
- iOS adapter와 C13.3 전체 상태: 미완료.
- Android API 29/37 AVD 결과와 별개이며, 물리 장치·외장 입력장치 검증을 주장하지 않는다.

## PR 변경 관점 검토

[서로 다른 실패 가능성 20개](pr-delta-review-20.md)를 이 증거·인덱스 변경에 대조했다.
