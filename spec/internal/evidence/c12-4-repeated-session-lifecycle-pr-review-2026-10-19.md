# C12.4 반복 session lifecycle PR 변경 적대 검토 30개

검토 대상은 Android debug-only 반복 probe, 선택 기기 ADB runner, 계획/공식 상태/로드맵 문서, API 37 AVD 실행 증거와 PASS 화면이다.

1. **기준 revision** — PR #328의 Android JNI gate 수정이 포함된 최신 main 위에서 rebase했다. PASS.
2. **변경 격리** — C12.4 기존 작업을 보존한 채 별도 worktree에서 계속했다. PASS.
3. **ADB 기본 대상** — serial 인자만 주면 `android-avd`를 기본 target으로 선택한다. PASS.
4. **기기 열거** — runner에서 전체 `adb devices -l` 호출을 제거했다. PASS.
5. **serial 범위** — `get-state`, getprop, install, logcat, run-as, screencap, pull, cleanup, force-stop이 선택 serial에 고정된다. PASS.
6. **AVD 식별** — Android API 37과 qemu=1을 확인하고 실기기 분류를 거부한다. PASS.
7. **물리 기기 검증** — 이번 실행은 emulator-5554만 사용했고 실기기를 조회하거나 명령하지 않았다. PASS.
8. **fixture 노출** — probe intent는 DEBUG와 C12.4 진단 feature가 없으면 진행하지 않는다. PASS.
9. **기본 화면 회귀** — 전용 intent 분기 안에서만 상태 TextView를 설정하고 일반 앱 경로는 변경하지 않는다. PASS.
10. **입력 검증** — run ID 정규식 검사를 통과해야 probe를 시작한다. PASS.
11. **cycle 수량** — 16 cycle × 8 session = 128 생성으로 성공 marker와 최종 parser가 일치한다. PASS.
12. **부분 생성 실패** — 실패 시 이미 생성된 handle만 역순 해제한다. PASS.
13. **sample 예외** — worker sample 예외는 성공 marker를 막고 `finally` cleanup으로 들어간다. PASS.
14. **free 예외** — handle 해제 예외를 기록하고 run을 실패 처리한다. PASS.
15. **ready 판정** — cycle별 owner/CSS worker TID가 ready 시 `/proc/<pid>/task`에 있어야 한다. PASS.
16. **free 판정** — 매 cycle 해제 이후 해당 TID가 사라져야 다음 cycle 성공을 인정한다. PASS.
17. **ID 재사용** — Android TID 재사용 때문에 cycle 간 전역 고유성을 가정하지 않는다. PASS.
18. **프로세스 상태** — marker timeout과 앱 process 종료는 마지막 logcat과 함께 실패한다. PASS.
19. **메모리 표현** — process PSS/RSS는 관측값이며 leak-free 주장에 쓰지 않는다. PASS.
20. **렌더링 범위** — renderer, GPU surface, layout, CSS 결과, WPT를 검증했다고 주장하지 않는다. PASS.
21. **실패 진단** — 저장된 두 과거 실패는 ADB preflight 이후의 V8 pin mismatch와 오래된 JNI C symbol build failure로 구분했다. PASS.
22. **V8 경로** — 재현은 고정 V8 source directory를 지정했고 그 사용자 경로는 PR에 넣지 않았다. PASS.
23. **최종 기기 정보** — run 기록에 emulator-5554, sdk_gphone16k_arm64, API 37, qemu=1이 있다. PASS.
24. **APK 동일성** — APK SHA-256을 device와 summary 기록에 보존했다. PASS.
25. **화면 결과** — 최종 debug screen은 `PASS · 16/16 cycles`를 표시하고 summary가 PNG SHA-256을 보유한다. PASS.
26. **화면 대비** — 검토된 캡처에서 흰색 PASS 글자가 어두운 화면에 읽힌다. PASS.
27. **focused test** — ADB target 분류와 run log validator 테스트가 7/7 통과했다. PASS.
28. **plan 범위** — 새 AVD run을 과거 실기기 증거와 분리하고 미검증 한계를 명시했다. PASS.
29. **공식 상태** — C12.4 parent checkbox는 미완료로 유지되고 완료 수치를 바꾸지 않았다. PASS.
30. **roadmap 문구** — AVD session-thread 결과만 추가하며 C12.4/전체 CSS 지원 미완료 경계를 반복한다. PASS.

## 판정

30개 관점에서 차단 이슈 없음. 이 diff는 제한된 session lifecycle 진단 도구와 API 37 AVD 재검증을 기록한다.

## PR #330 본문 화면 확인

- [PR #330](https://github.com/ohah/spinon/pull/330) 본문에 성공 AVD 화면을 직접 삽입했다. 원본 URL은 HTTP 200 `image/png`로 응답했다.
- Agent Browser의 PR 렌더링에서 해당 이미지가 complete 상태이며 자연 크기 1080×2400으로 표시되는 것을 확인했다. 확인 후 세션을 닫았고 활성 세션과 브라우저 프로세스가 남지 않았다.

## 문서 빌드 확인

- 저장소 고정 Rspress 2.0.22로 전체 문서 빌드를 실행했다. spec 전반의 571개 dead-link 보고 때문에 전체 빌드는 실패했으며, 실패 로그에서 이번 C12.4 계획·근거·리뷰 파일 경로는 보고되지 않았다. 전체 문서 빌드를 통과했다고 주장하지 않는다.
