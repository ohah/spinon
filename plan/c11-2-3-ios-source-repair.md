# C11.2.3 iOS 실행 경로 복구 계획

## 상태

iOS 수정 후보 검증 완료 · PR 경로 준비됨 · `origin/main` 재현 대기. 2026-10-13 기준 수정 후보는 iOS 25 scene matrix와 99개 Metal drawable 표본에서 통과했다. 병합 후보에는 C11.2.3 iOS 검증 경로 복구만 담고, 병합 뒤 `origin/main`에서 동일 행렬을 다시 실행한다.

## 재현된 문제

`origin/main`을 깨끗하게 분리해 iOS Simulator 빌드를 시도했을 때 Swift 코드가 호출하는 C11.2.3 Objective-C 메서드 선언 두 개가 `SpinonRunner.h`에 없고, 긴 중첩 조건식이 Xcode type-check 제한에 걸렸다. 선언과 조건식을 임시 수정해 다시 실행했을 때 앱 프로세스에는 C11.2.3 장면 인수가 전달됐지만 `AppDelegate`의 GPU 화면 라우팅 조건에 C11.2.3 인수가 빠져 흰 기본 화면이 열렸다. 그 결과 첫 장면에서 C11.2.3 runtime 표식을 받지 못하고 timeout됐다.

`AppDelegate`가 C11.2.3 인수를 GPU 데모 화면으로 라우팅하도록 수정한 뒤 같은 후보에서 다음을 확인했다.

- 25 scene: 성공 19/19, 기대 거부 6/6
- 107 node, geometry 최대 오차 0 CSS px (허용 0.5 CSS px)
- iOS Metal drawable: 19 scene·99/99 RGBA exact, 최대 채널 오차 0

상세 원본: [수정 후보 실행 기록](../spec/internal/evidence/c11-2-3-dense-column-placement-ios-simulator-runtime-matrix-2026-10-13/c1123-i-mv6mhrrq-785dbe7a/README.md). 이 결과는 기존 `origin/main`에서 재현된 후보 근거이며, 병합 뒤 저장소 기준의 재현은 별도 완료 조건이다.

## 범위

1. `AppDelegate`가 `--spinon-c1123-grid-auto-placement`와 `--spinon-c1123-grid-scene=`를 기존 C0410 GPU runtime demo 경로로 전달한다.
2. `SpinonRunner.h`에 이미 구현된 C11.2.3 Objective-C 메서드의 정확한 선언을 추가한다.
3. Swift에서 중첩 조건식을 같은 우선순위로 순차 분기해 Xcode가 type-check 할 수 있게 한다.
4. 계획·상태 대장·증거를 이 수정의 실제 범위와 제한에 맞춘다.

## 범위 밖

- C11.2.3 grid 배치 알고리즘, Android 경로, FFI 함수 구현 변경
- 출시 계약 버전 증가, 공개 CSS 지원 확대, WPT 적합성 주장
- iOS 실기기, OS compositor 이후 화면, 성능 검증
- C11.3.1 구현과 같은 PR에 묶기

## 완료 조건

- iOS Simulator 앱 빌드 성공
- scene ID 단독 인수로 정상 GPU route 진입 및 boolean flag 단독 인수의 명시적 거부 확인
- 고정 Chromium 154 기반 25 scene의 성공/거부 결과와 geometry가 기준에 일치
- 성공 19 scene의 99 Metal drawable 표본이 exact이며 변조 좌표가 readback 전에 거부
- 새 프로세스 격리 실행 및 대표 화면·원시 로그·비교 JSON을 저장
- C11.2.2 iOS 행렬이 계속 동작하는 회귀 확인
- [구현 검토](../spec/internal/evidence/c11-2-3-ios-source-repair-implementation-review-2026-10-13.md)를 변경 후보에 대해 마친다
- 정확한 변경으로부터 구현 적대 검토를 마치고, C11.2.3 전용 PR 하나에서만 리뷰·병합
