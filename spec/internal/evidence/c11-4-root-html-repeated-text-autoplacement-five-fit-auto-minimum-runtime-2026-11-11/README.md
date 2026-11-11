# C11.4 다섯 fit auto-placement RuntimeSession·시뮬레이터 근거

## 범위와 기준

이 기록은 C11.4 전체가 아니라 grid-auto-rows가 정확히 다섯 fit-content()와 한 min-content·max-content·auto tail로 이루어진, 사전 비교에 고정한 Root HTML text-paint profile만 다룬다. Chrome 157의 [사전 비교 17,280 scene](../c11-4-root-html-repeated-text-autoplacement-five-fit-auto-min-precomparison-2026-11-11/README.md)을 독립 oracle로 사용한다. 여기서 확인한 결과는 전체 CSS Grid 지원, WPT 적합성, GPU pixel parity 또는 실기기 검증을 뜻하지 않는다.

## 구현·회귀 실행

| 실행 | 결과 |
| --- | --- |
| cargo test --locked -p spinon-runtime parsed_root_html_five_fit_auto_placement_matches_all_17280_chrome_scenes -- --nocapture | **1/1 통과**, 내부에서 17,280 scene 실행, 856.71초. 원본 stdout은 [five-fit-matrix-final.log](five-fit-matrix-final.log)에 보존 |
| Layout profile unit tests | 2/2 통과: 모든 fit 위치·tail 허용, 잘못된 개수·길이·cap 거부 |
| Style projection unit tests | 2/2 통과: 모든 fit 위치·tail 허용, 잘못된 개수·길이·cap 거부 |
| Runtime positive/control tests | 3/3 통과: candidate admission, absolute calc() px 정규화, 한 RuntimeSession의 light/dark cascade |
| Runtime rejection tests | 2/2 통과: out-of-profile mutation 거부, 원자적 실패와 같은 세션 복구 |
| 이전 fit-count·row-span 회귀 | 증거 폴더의 regression-*.log; 1/2/3/4-fit, 6-row, row-span 및 trace 회귀 로그를 보존 |
| Workspace Clippy | cargo clippy --locked --workspace --all-targets -- -D warnings 통과; [clippy-workspace.log](clippy-workspace.log) |
| Formatter·whitespace | cargo fmt --all -- --check, git diff --check 통과 |

Matrix는 8,640 baseline과 8,640 automatic-minimum candidate를 같은 8,640 key로 짝지었다. 2,160 authored track list, 6개 fit 위치 조합, 3개 tail 종류와 위치, 120개 cap 순열, start|stretch, DOM forward|reverse, min-height:0|auto를 교차한다. 57,600 candidate fit-row occurrence에서 content-floor 위반은 0, tail payload-floor 위반은 0, candidate item frame의 Grid 끝 초과는 0이었다. Baseline에서는 21,600 fit-row occurrence가 payload floor보다 작았다. auto/0px baseline payload의 Grid 끝 초과 600건과 auto tail의 align-content:stretch 차이는 candidate 결과와 분리해 assertion한다.

[focused-tests-final.log](focused-tests-final.log)는 변경 crate unit tests와 runtime positive/rejection tests의 개별 원본 출력을 포함한다. 초기 rejection 실행에서 테스트가 cascade snapshot 부재를 잘못 기대하고, 기존 4-fit 지원과 겹치는 tail 조합을 사용한 점을 발견해 assertion과 negative case를 고쳤다. 수정 후 focused tests와 전체 matrix를 다시 통과했다.

## iOS·Android visual smoke

두 플랫폼 모두 앱의 기본 Root HTML route에서 SPINON_ROOT_HTML_DEFAULT_APP_BOOT status=pass, layout=ready, SPINON_C0410_DRAW status=0 presented를 light/dark 각각 확인했다. 화면은 직접 열어 내용과 색 scheme 전환을 확인한 visual smoke다. frame을 Metal/PixelCopy로 읽어 oracle과 픽셀 단위 비교하지 않았다.

| 플랫폼 | 기기·환경 | 화면 |
| --- | --- | --- |
| iOS | iPhone 17 Pro, iOS 26.2 Simulator, 402×778 CSS px viewport | [light](ios-iphone17pro-ios26_2-light.png), [dark](ios-iphone17pro-ios26_2-dark.png) |
| Android | Pixel 6 AVD, Android 37.1, ARM64, Google APIs 16 KB page-size image, 411.43×841.52 CSS px viewport | [light](android-api37_1-light.png), [dark](android-api37_1-dark.png) |

Android 캡처는 spinon_api37_1_compat16k의 APFS copy-on-write task clone에서 실행했다. clone의 기존 앱만 제거한 후 candidate APK를 설치했다. 캡처 뒤 task AVD를 삭제했고 원본 AVD의 .ini와 config.ini SHA-256은 android-avd-clone.log에 기록된 값과 clone 전·후 모두 일치한다. 기존 공유 AVD의 APK signature 검증 실패는 [android-api37_1-readonly-probe.log](android-api37_1-readonly-probe.log)에 남겼다. 해당 AVD의 앱·데이터를 수정하지 않았다.

iOS 기본 앱 data hash는 실행 전 6267901e1ecb1d6221833ebcc09dfce27a0e5246763841020b7c2fb69e080e89, 실행 후 d2bbfd9119c4cdb2db334b9bbe064bddacbdbde5237d775a5bc2f54d171ffdb5였고, 백업 복원 뒤 다시 실행 전 hash와 일치했다. Android task clone의 이전 앱 제거·앱 캡처 후 제거 및 task AVD 삭제는 android-task-avd-preparation.log, android-app-restore.log, android-avd-delete.log와 cleanup-wrapper.log에 기록됐다.

### 입력·산출물 식별자

| 항목 | SHA-256 |
| --- | --- |
| Android candidate APK | 322e6676d37a485dec3c8ebf595e89a6aeaaa8e18cc7a84886f359b1dfc4e9be |
| iOS SpinonBootstrap.app file-tree aggregate | e49c8b0a2ab037ca76e747d04e19df9890f58127a115dafa517c1d4de9bf5c70 |
| Root HTML fixture | 2b7c1c1b0b229d84c067c10f89dc72851d87acd738bd651bd7d5d8247a12330e |
| task app JS | c5cbc04ef8a601a5291f04655768938db3f84c1156828599641ea130d05dcf28 |
| iOS light screenshot | d3e131f09851d2d9399d8d4fec50c5092b1b487598f7393ab0615558941c56f9 |
| iOS dark screenshot | e557b78ec23e3561a062d6e9a8eae2cce575cc414d58033c45e819626bad2d80 |
| Android light screenshot | 756891fdb9b01bd5ae1e183bdc3015d60e966ea596d7216f0f3c5e1b8163b709 |
| Android dark screenshot | a41273bab819b0e1c5633925acd9861973117744767944107a5f0ab665399e06 |

## 종료·미완료 범위

cleanup-wrapper.log는 iOS booted list와 ADB device list가 비어 있고 task AVD가 제거됐음을 기록한다. 현재 Android task clone, iOS Simulator, task Agent Browser가 실행 중이지 않다. 관련 없는 Agent Browser 세션은 이번 검증에서 열거나 종료하지 않았다. V8 source/build artifact는 이어지는 Spinon roadmap 빌드에서 재사용하므로 보존했고, 임시 depot_tools overlay는 제거했다.

이 candidate는 PR 단위 slice다. C11.4/C11 완료 checkbox, 공식 집계 78/214, 내부 numeric contract 0.1.0은 변경하지 않는다. 실기기·hardware GPU·PixelCopy/Metal drawable readback·일반 CSS Grid 적합성은 미검증이다.

실제 PR diff를 대상으로 한 별도 30개 실패 관점 검토는 [pr-diff-review-30.md](pr-diff-review-30.md)에 기록한다.

PR #1349를 최초 생성한 head `18f6269db9bd80f649d357fea35cbaec8390158a`에서 GitHub가 반환한 86-file diff는 local diff와 922,918 bytes 및 SHA-256 `5680c2e1e31f27feea1cc2a8e43c4d8cb9847a9ee24328bc5704f56ef071e3ed`가 일치했다. Agent Browser에서 PR 본문에 넣은 iOS/Android light/dark 이미지가 모두 로드됐고 원본 크기를 반환하는 것을 확인했다. PR check는 0건으로 CI 통과를 뜻하지 않는다.
