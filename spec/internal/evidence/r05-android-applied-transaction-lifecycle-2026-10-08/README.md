# R05.3 Android 적용 transaction surface lifecycle 실행

**실행일:** 2026-10-08 · **결과:** API 37.2 ARM64 emulator debug fixture 통과 · **R05.3/R05:** 미완료

## 검증 범위

기존 R08 `GLSurfaceView`가 `SurfaceControl.Transaction`을 다음 frame에 적용한 뒤 전달되는 Android transaction-completed callback을 확인했다. callback executor를 막아 실제 `TransactionStats` callback을 대기열에 둔 상태에서 같은 surface view를 떼고 다시 붙였다. 이전 세대 요청의 취소·늦은 callback 분류와 새 surface generation의 정상 callback을 검사했다.

이 결과는 debug fixture의 Android API 37.2 실행이다. `MotionEvent`를 직접 dispatch한 `synthetic_fixture` 입력이므로 물리 입력이나 입력 지연 측정으로 계산하지 않는다. `TransactionStats` 존재와 request 수명주기를 검증했으며 optical scanout이나 제품 성능을 입증하지 않는다.

## 실행 근거

| 항목 | 결과 |
|---|---|
| Android 대상 | `emulator-5562`, `sdk_gphone16k_arm64`, API 37.2, ARM64, 16,384-byte page |
| AVD 상태 | `spinon_api37_2.ini`는 남아 있었지만 `spinon_api37_2.avd` 경로는 없었다. 이미 실행 중인 QEMU PID 10605를 사용했고 해당 프로세스가 삭제된 AVD 파일을 열어 둔 것을 `lsof`로 기록했다. cold boot를 새로 하지 않았다. |
| V8 소스 | 고정 revision `7b50b62cb18f28617959e8452e2cd18195b38bcf`, clean checkout. 새 worktree의 기본 V8 경로는 비어 있어 기존 pinned checkout을 `SPINON_V8_DIR`로 지정했다. 다른 worktree의 파일은 변경하지 않았다. |
| APK | Debug `assembleDebug`·설치·실행 성공. APK digest와 빌드 로그는 run 폴더에 보존했다. AGP 8.13.2의 compile SDK 37.2 검증 범위 경고는 남았다. |
| 전체 fixture | 12 checks, 0 failures, timeout/callback 경합 100회, idle 관찰 60초, 최종 pending/queue/active 모두 0 |
| R08 frame 확인 | revision 1·generation 1, revision 2·generation 1, revision 3·generation 2에 대한 `SPINON_R05_DRAW` marker를 runner가 각각 요구한다. |

세 실제 callback 경로:

1. 현재 generation의 정상 대조 callback: `TransactionStats` 존재, 현재 surface 일치, callback 1회, timeout/map 정리.
2. 실제 callback task가 executor 대기열에 들어간 뒤 surface destroy/recreate: production `surfaceDestroyed` 경로가 이전 요청을 timeout보다 먼저 취소했다. callback 해제 후 실제 stats callback은 `late_after_cancel`, 이전 generation, `current_surface=false`, `fence_signal_usable=false`로 처리됐다.
3. 새 generation 복구 대조 callback: 실제 stats callback이 한 번 처리되고 현재 surface와 generation이 일치했다.

이 emulator 실행에서는 양성 대조 fence가 `signaled`로 관찰됐고 stale callback의 fence 상태도 `signaled`였지만 unusable로 분류됐다. fence signal 여부는 fixture의 필수 합격 기준이 아니며, 이 한 환경의 관찰을 기기 지원이나 event-to-present 시간으로 일반화하지 않는다.

## iOS 별도 회귀 확인

Android의 `SurfaceControl.TransactionStats`와 같은 iOS API는 없으므로 동일 lifecycle 계약이라고 합치지 않는다. 대신 현재 R05 iOS 경계에 맞춰 [callback ledger 자체 시험을 20회 재실행](ios-callback-ledger-20-rerun.log)했고 20/20 실행에서 각 6개 그룹이 통과했다. iPhone 17 Pro / iOS 26.2 Simulator의 UI smoke도 1/1 통과해 WGPU 도형 탭과 drawable-acquire 귀속을 확인했다. 첫 UI 실행은 격리 worktree에 iOS WGPU 정적 라이브러리가 없어 link 단계에서 실패했다. 고정된 cargo lockfile로 simulator target 라이브러리를 만든 뒤 다시 실행해 통과했다. [XCTest 로그](ios-simulator-ui-rerun.log) · [WGPU backend 빌드 로그](ios-wgpu-backend-build-rerun.log) · [탭 뒤 화면 캡처](ios-simulator-ui-rerun-attachments/4E89032B-9EC1-417D-BEE0-9AFDFEEF4648.png).

UI smoke는 격리 harness에서 V8 앱 래퍼를 제외하고 저장소의 R08 Swift layer와 callback ledger 및 실제 WGPU 정적 라이브러리를 연결한 Simulator 검증이다. iOS 전체 앱 bundle·실기기·광학 표시 시각을 검증한 결과는 아니다.

## 원본 자료

- [적대적 재실행 환경·실행 기록](api37_2_adversarial-rerun/environment.txt)
- [적대적 재실행 전체 Logcat](api37_2_adversarial-rerun/logcat.txt)
- [적대적 재실행 결과 화면](api37_2_adversarial-rerun/result.png)
- [최종 API 37.2 실행 환경·V8·AVD 상태](api37_2_verified-final/environment.txt)
- [QEMU PID와 삭제된 AVD 파일 핸들](api37_2_verified-final/avd-process-state.txt)
- [전체 Logcat](api37_2_verified-final/logcat.txt)
- [Android debug 빌드 로그](api37_2_verified-final/android-build.log)
- [APK digest](api37_2_verified-final/apk.sha256)
- [소스 파일 digest](api37_2_verified-final/source-files.sha256)
- [기기 결과 화면](api37_2_verified-final/result.png)
- [최종 runner](../../../../tools/verify-r05-android-callback-faults.sh)
- [구현 실패 관점 검토](implementation-review.md)
- [구현 전 별도 계획과 계획 실패 관점](../../../../plan/r05-android-applied-transaction-lifecycle.md)

처음 새 worktree에서 실행한 빌드 preflight는 로컬 pinned V8 checkout이 없어 중단됐다. pinned·clean V8 checkout을 명시해 다시 빌드했다. 검증 중 AVD console의 `OK` 응답을 이름으로 읽던 runner 결함과 draw-marker 검사 옵션 오류를 발견해 수정한 뒤 최종 runner를 재실행했다. 중간 실패 실행 원본도 같은 폴더에 보존했으며 위 표의 최종 결과는 `api37_2_verified-final/`이다.

## 남은 범위

- cold boot AVD, API 35·37.0·37.1의 실제 적용 transaction lifecycle, API 29–34 fallback은 이번 실행에 포함되지 않았다.
- 실기기·물리 터치·iOS callback runtime·release 앱·하드웨어 GPU·광학 표시·제품 입력 latency는 검증하지 않았다.
- R05.3의 전체 입력→GPU 표시 신호 상관, iOS device callback·clock residual 및 기기 검증은 남아 있다. R05.3과 R05는 미완료다.
