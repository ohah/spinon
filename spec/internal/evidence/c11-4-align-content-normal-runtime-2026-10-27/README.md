# C11.4 · `align-content: normal` 3행 auto-track stretch 런타임 증거

이 디렉터리는 기존 C11.4 `align-content: normal` Rust oracle과 CSS 사전 비교에서 고정한 입력 한 개를 opt-in Android/iOS 앱 fixture에서 실행한 결과다. 구현 전 비교 입력과 판정 기준은 [사전 비교 README](../c11-4-align-content-normal-runtime-precomparison-2026-10-27/README.md)에 있다. 이 fixture는 일반 CSS 지원이나 C11.4 전체 완료를 주장하지 않는다.

## 기대 geometry

Root는 definite `160×100 CSS px`, explicit rows는 `auto auto 5px`, `row-gap`은 `10px`, `align-content:normal`이다. 직접 자식 높이는 `10`, `20`, `5px`다. gutter 합 `20px`을 제외한 두 auto row의 초기 base `10+20px` 이후 positive free space는 `45px`이므로 각 auto row가 `22.5px`씩 늘어난다. 기대 track size는 `[32.5, 42.5, 5]px`, item y 좌표는 `[0, 42.5, 95]px`이고 fixed 마지막 row는 `5px`로 유지된다.

세 host의 root 크기와 row-axis 입력은 같지만 앱 containing block 및 Chrome viewport는 `320×240px`, pure Rust oracle viewport는 `160×100px`다. 이 결과는 root·track·child geometry만 비교하며 viewport/root sizing 정책의 parity를 주장하지 않는다.

## Android API 37 AVD

- 기기: `sdk_gphone16k_arm64`, API 37, `arm64-v8a`; serial `emulator-5554`.
- 최종 runner·계획 문서 snapshot 실행: [result.json](./android-avd-c114-align-content-normal-equal-auto-rows-android-avd-20261026T222150868Z-94e8b191/result.json), [simulator.json](./android-avd-c114-align-content-normal-equal-auto-rows-android-avd-20261026T222150868Z-94e8b191/simulator.json), [runtime log](./android-avd-c114-align-content-normal-equal-auto-rows-android-avd-20261026T222150868Z-94e8b191/runtime-align-content-normal-equal-auto-rows.log), [화면](./android-avd-c114-align-content-normal-equal-auto-rows-android-avd-20261026T222150868Z-94e8b191/align-content-normal-equal-auto-rows.png).
- 정확히 한 scene이 실행됐고 trace는 두 축을 포함한 Rows pass 하나, 단일 정순 stage 목록, 7개의 track/gutter entry, 모든 단계의 row base, 최종 frame과 revision identity를 확인한다. 최종 rows와 item frames는 위 기대값과 일치하고 `drawPresented=true`다.
- PixelCopy는 root 안의 첫 row `(80,5)`, gutter `(80,37)`, 둘째 row `(80,52)`, fixed row `(80,97)`와 root 바깥 `(160,5)`를 모두 정확히 읽었다. 5/5 표본이 기대 RGBA와 일치했다: `[229,72,77,255]`, `[16,24,32,255]`, `[58,134,255,255]`, `[6,214,160,255]`, `[0,0,0,255]`.
- 빌드 APK와 설치 APK SHA-256은 모두 `6f0963c546c89e5854b9453c8daecef809412d81c75d346255f74a2c5456817b`다. artifact scan은 [경계 기록](./artifact-boundary/android-fixture-apk-scan.json)에 있다.

## iOS Simulator

- 기기: iPhone 17 Pro (`iPhone18,1`), iOS 26.2 Simulator; 실행용 clone UUID `ACA7BF91-E2D5-4CF7-909A-08D1AD95FF3D`.
- 최종 runner·계획 문서 snapshot 실행: [result.json](./ios-simulator-c114-align-content-normal-equal-auto-rows-ios-simulator-20261026T222207272Z-dce6182a/result.json), [simulator.json](./ios-simulator-c114-align-content-normal-equal-auto-rows-ios-simulator-20261026T222207272Z-dce6182a/simulator.json), [runtime log](./ios-simulator-c114-align-content-normal-equal-auto-rows-ios-simulator-20261026T222207272Z-dce6182a/runtime-align-content-normal-equal-auto-rows.log), [화면](./ios-simulator-c114-align-content-normal-equal-auto-rows-ios-simulator-20261026T222207272Z-dce6182a/align-content-normal-equal-auto-rows.png).
- 정확히 한 scene이 실행됐고 동일한 rows·item frames, 단일 Rows pass trace, stage/track/gutter 및 revision identity assertions를 통과했다. `drawPresented=true`이며 Metal surface 표본 5개가 위와 같은 RGBA와 일치했다.
- 빌드 앱 실행 파일과 설치 앱 실행 파일 SHA-256은 모두 `39f8266be63eb5a7db6347c7f4e8d99d0d7b8307dd2b18b533630b449f35c422`다.

## 브라우저 사전 비교와 source identity

- Agent Browser `0.31.1`, Chrome `156.0.8078.48`, viewport `320×240`, DPR 1에서 CSSOM rows `32.5px 42.5px 5px`, root/child geometry, gutter 및 root 밖 표본을 관찰했다. [Chrome screenshot](../c11-4-align-content-normal-runtime-precomparison-2026-10-27/chrome156-normal-auto-track-rows.png), [관찰 JSON](../c11-4-align-content-normal-runtime-precomparison-2026-10-27/chrome156-observation.json). 브라우저 session을 닫고 활성 session 0개를 확인했다.
- 두 최종 simulator 실행은 계획 PR 병합 commit `77531e7fee663a260dcbbfb68660516eab7d803b`에 구현 working-tree diff를 적용한 동일 source snapshot `d106d71d91b35a87489789bdbc0683d4897a04f659fc5afe5777e337689bb49f`에서 생성됐다. 각 run의 base commit, tracked diff hash와 신규 fixture hash는 개별 `simulator.json`에 보존했다.
- Rust 기본/fixture artifact 경계의 negative/positive control은 [artifact-boundary](./artifact-boundary/)의 JSON과 build log에 있다. Android APK의 Java host DEX와 iOS host executable에는 scene ID가 host metadata로 남을 수 있다. 정적 archive 검사는 최종 APK/앱 패키지 전체에서 fixture가 빠짐을 증명하지 않는다.
- Xcode build log 네 개는 `.log.gz`로 보관한다. 압축 해제 SHA-256을 원본 log와 대조해 bytes가 보존됐음을 확인했다. 이 출력은 Xcode가 생성한 trailing-space 빌드 설정 표기를 포함한다.
- 구현·runtime 별도 20관점 검토는 [검토표](./implementation-review-20.md), 독립된 구현 PR diff 30관점 검토는 [검토표](./pr-diff-review-30.md)에 있다. 각각 최종 상태 20/20, 30/30 통과했고 발견 사항 수정·재확인을 기록했다.
- Runner에서 대상 ID와 다른 scene을 함께 선택하거나 단독 run-kind만 지정하면 simulator command 전에 거부한다. 두 음성 입력은 [복수 scene](./runner-multi-scene-selection-rejected.log)과 [scene 없는 run-kind](./runner-run-kind-without-scene-rejected.log) 기록으로 보존했다.

## 보존된 실패와 한계

- 첫 Android 입력의 `justify-content:flex-start`는 현재 runtime에서 거부돼 fixture를 `start`로 맞췄다. 실패 로그 `android-first-attempt-unsupported-justify-content.log`는 원인 기록으로 보존한다.
- 중간 Android pass는 matcher가 6개 rows stage 중 4개만 예상해 실패했다. 최종 matcher는 실제 `Initialize`, `IntrinsicItemBatch`, `IntrinsicSizingComplete`, `MaximizeTracks`, `ExpandFlexibleTracks`, `StretchAutoTracks`를 정확한 순서로 확인하며 최종 run에서 통과했다. 이전 실패 run은 삭제하지 않았다.
- 검증 범위는 이 단일 row-axis scene과 Android AVD/iOS Simulator다. 실기기, 하드웨어 GPU 전반, compositor 이후 픽셀 equality, 다른 alignment/track sizing 조합, 일반 CSS admission 및 전체 C11.4 적합성은 검증하지 않았다. 공식 [상태 대장](../../../STATUS.md)의 C11.4 상태와 완료 집계 `72/213`은 변경하지 않았다.
