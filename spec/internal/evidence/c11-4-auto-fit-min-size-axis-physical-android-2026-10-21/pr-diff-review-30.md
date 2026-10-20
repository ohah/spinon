# 최종 PR diff 실패 관점 검토 30개

대상 diff: Android 실기기 capture harness, 보충 계획·근거, C11.4 상태와 공개 roadmap 갱신
실행 근거: [README](./README.md) · [실기기 result](./android-device-c114-auto-fit-min-size-axis-android-device-20261020T203145959Z-6d7d1d53/result.json) · [AVD 비교](./android-device-c114-auto-fit-min-size-axis-android-device-20261020T203145959Z-6d7d1d53/physical-vs-avd-comparison.json)

1. **부분 실기기 결과를 C11.4 완료로 오인** — STATUS와 공개 roadmap은 C11.4 전체를 미완료로 유지한다.
2. **공식 완료 집계가 바뀜** — diff에서 체크 상태나 완료 수 변경은 없다.
3. **새 공개 API가 생긴 것으로 기록** — 변경은 fixture capture harness와 evidence뿐이며 계약/API 버전 변경이 없다.
4. **계획이 부모 계획과 단절** — 후속 계획은 축별 auto-fit 계획, CSS Grid §7.2.3.2와 연결한다.
5. **실행 전에 기준값을 바꿈** — 입력 53 ID는 저장된 AVD 결과에서 가져오고 normative expectation은 수정하지 않았다.
6. **Chrome 관찰값을 정답으로 삼음** — 수치 acceptance는 spec과 고정 fixture만 사용한다.
7. **계획 검토를 구현 검토로 재사용** — 계획 20개, 구현/runtime 20개, PR diff 30개를 별도 파일로 둔다.
8. **에뮬레이터를 물리 기기 결과로 표시** — qemu property `0` 및 `device.json`을 증거로 남겼다.
9. **다른 모델·기기의 로그를 섞음** — physical mode는 SM-S731N을 고정 검사하고 serial을 명시한다.
10. **기기 Android 버전 누락** — Android 16/API 36, ABI, display, renderer와 fingerprint를 보존한다.
11. **AVD 검증이 우발적으로 느슨해짐** — AVD는 API 37, qemu 1, ARM64 gate를 유지한다.
12. **GLES query가 AVD 호환성을 깨뜨림** — renderer query는 physical mode로 한정했다.
13. **이전 APK를 current source 결과로 주장** — 실행 전 opt-in build를 새로 완료했다.
14. **fixture ABI가 제품 artifact라고 해석** — build command는 `c04-runtime-gpu-fixture-test`를 명시하고 README에 scope를 한정한다.
15. **기기 설치본이 다른 바이너리임** — local APK와 remote base.apk SHA-256 일치를 기록한다.
16. **capture 대상 scene 수가 줄어듦** — 53 ID 수와 uniqueness, 결과의 unique ID set을 검증한다.
17. **scene definition이 바뀌어 의미가 달라짐** — capture harness는 선택 ID가 현재 definition과 모두 일치하지 않으면 중단한다.
18. **node frame은 기대값만 기록** — 물리 기기의 관측 frame을 result에 저장하고 AVD raw log와 비교한다.
19. **track count만 비교하고 collapse를 놓침** — track 크기, 축별 목록, collapsed flag와 gutter를 scene별 비교한다.
20. **fractional 오차 기준을 완화** — 1/64 CSS px 허용치를 유지하며 이번 delta는 0이다.
21. **trace 일부를 전체로 셈** — 각 장면에서 chunk completeness/sequence 복원 assertion을 통과했다.
22. **layout ready를 paint 완료로 표현** — 53/53 box count가 맞는 draw/present marker가 별도 확인됐다.
23. **표면 PixelCopy를 compositor 이후 정확도로 확장** — README와 roadmap에서 앱 SurfaceView 한계와 4/4 sample만 말한다.
24. **PR 화면이 다른 장면의 이미지** — README/PR image path는 실제 SM-S731N에서 캡처한 second-repeat 및 overflow scene이다.
25. **이미지 URL이 branch 파일과 어긋남** — PR #405의 raw GitHub image URL 두 개를 Agent Browser로 열어 1080×2340 natural dimensions를 확인했다.
26. **PR 첨부 때문에 이미지가 변형됨** — PNG 원본을 그대로 커밋하고 각 파일 SHA-256을 manifest에 기록한다.
27. **빌드 warning을 숨김** — compile SDK 37.2와 AGP 8.13.2 검증 범위 경고를 README에 남겼다.
28. **증거가 재현 불가능** — 고정 AVD 결과, serial, run ID, build command, runner, comparator, raw logs, screenshots와 hash가 연결된다.
29. **무관한 미추적 evidence가 PR에 들어감** — stage는 새 후속 evidence 디렉터리와 정확히 열거한 계획/status/roadmap/harness만 포함한다.
30. **CI 또는 PR gate를 사전 통과로 주장** — PR #405는 mergeStateStatus=CLEAN, mergeable=MERGEABLE이며 `gh pr checks --required`는 checks 없음으로 응답했다. CI 통과라고 보고하지 않는다.

이 검토는 최종 diff와 수집된 Android 실기기 자료를 기준으로 했다. PR 본문의 두 image URL은 게시 후 Agent Browser로 실제 로드를 확인하고 해당 browser process를 닫는다.
