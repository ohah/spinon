# C12.4 반복 session lifecycle 구현 실패 관점 검토 20개

검토 대상은 Android debug 전용 반복 lifecycle probe, ADB runner, 로그 판정기와 2026-10-19 API 37 AVD 실행이다.

1. **물리 기기 오선택** — runner 기본 target을 `android-avd`로 두고 `emulator-5554` 실행에서 `qemu=1`, API 37을 재확인한다. PASS.
2. **연결 대상 범위** — 전체 `adb devices -l`을 열거하지 않고 명시 serial의 `get-state`만 조회한다. PASS.
3. **후속 명령 누수** — 설치, getprop, shell, logcat, force-stop 호출은 공통 `adb` helper에서 같은 serial로 고정된다. PASS.
4. **일반 앱 진입 변경** — 반복 probe는 전용 intent extra를 받은 때만 진입하며 기본 Activity route는 그대로 둔다. PASS.
5. **잘못된 build 구성** — DEBUG, C12.4 진단 feature, run ID 형식을 모두 확인한 뒤 probe를 시작한다. PASS.
6. **UI thread 차단** — lifecycle 반복은 이름 붙은 별도 Java thread에서 실행한다. PASS.
7. **잘못된 cycle 수** — 완료 조건은 정확히 16 cycle이며 각 cycle에서 session 8개다. PASS.
8. **부분 생성 처리** — 생성 실패가 나면 이후 sample을 건너뛰고 생성된 handle만 역순으로 정리한다. PASS.
9. **worker sample 오류** — sample decode/runtime 오류는 실패 marker를 남기고 `finally` 해제로 들어간다. PASS.
10. **free 오류 은폐** — 하나라도 free가 실패하면 cycle과 최종 run 모두 실패한다. PASS.
11. **조기 성공 marker** — ready·freed·cycle-complete marker에 run ID, cycle, created/released 수를 포함한다. PASS.
12. **worker 누락** — validator는 각 cycle의 owner/CSS worker TID 집합과 ready/free 순서를 검사한다. PASS.
13. **해제 확인 시점** — runner가 앱 UID `run-as`로 `/proc/<pid>/task`를 읽어 해제 TID가 사라졌는지 확인한다. PASS.
14. **ID 재사용 처리** — cycle 사이 TID 전역 고유성을 요구하지 않고 cycle별 존재/부재만 비교한다. PASS.
15. **다음 cycle 경합** — cycle 사이 1.5초 대기 후 다음 cycle을 시작한다. PASS.
16. **timeout 및 crash** — marker timeout 중 앱 process가 사라지면 실패하며 마지막 logcat을 보존한다. PASS.
17. **메모리 과장** — PSS/RSS는 baseline·선택 peak·final 관측값으로만 기록하고 leak-free 기준으로 쓰지 않는다. PASS.
18. **빌드/기기/화면 출처** — run summary에 APK SHA-256, screenshot SHA-256, AVD serial/model/API, run ID를 기록하고 PASS·16/16 화면을 보존한다. PASS.
19. **재현 로그** — V8 pin mismatch와 오래된 JNI symbol build 실패를 구분해 남기고 성공 run을 따로 보존했다. PASS.
20. **완료 범위** — 결과는 단일 API 37 AVD의 session thread 생성·해제만 입증하며 C12.4 parent, renderer/GPU, WPT 및 전체 CSS 지원을 완료로 표시하지 않는다. PASS.

## 결과

20개 관점에서 차단 이슈 없음. AVD lifecycle run 16/16, 총 128 session; owner와 CSS worker 각각 16 cycle 검사 통과. 관련 순수 검증 테스트는 7/7 통과했다.
