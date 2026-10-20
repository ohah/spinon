# C11.4 실기기 text matrix PR 변경 검토 30개

이번 문서·실행 근거 변경을 병합 전 30개 서로 다른 실패 관점으로 확인했다. 이 PR은 이미 구현된 내부 fixture의 Android 실기기 검증 결과를 추가하며 앱/Rust 동작 코드나 versioned interface를 바꾸지 않는다.

1. **작업 범위가 PR 제목과 일치하는가?** — 기존 text matrix의 physical Android 재검증만 추가했다. 통과.
2. **공식 구현 ID가 바뀌었는가?** — C11.4는 계속 미완료로 기록했다. 통과.
3. **완료 수를 임의로 증가했는가?** — `STATUS.md` 완료 checkbox와 집계는 수정하지 않았다. 통과.
4. **C11.4 checkbox가 우발적으로 완료됐는가?** — 상위 checkbox는 `[ ]`로 유지된다. 통과.
5. **새 evidence 경로가 상태 대장에 연결됐는가?** — Android 실기기 실행 README 링크가 있다. 통과.
6. **계획 기록이 이전 AVD 결과를 실기기 결과로 바꾸는가?** — 2026-10-21 follow-up으로 별도 기록했다. 통과.
7. **기존 2026-10-19 simulator 기록이 덮였는가?** — 원본 계획 문단과 evidence directory는 유지했다. 통과.
8. **Chrome oracle이 구현 결과에서 역산됐는가?** — 기존 fixed Chrome 155 matrix의 사전 값이 기준이다. 통과.
9. **10 scene 이름이 잘리거나 다른 allowlist로 바뀌었는가?** — README table과 result JSON에 같은 10 IDs가 있다. 통과.
10. **root/item 예상 크기가 scene 순서와 어긋나는가?** — table, JSON, runtime frame logs를 대조했다. 통과.
11. **node 좌표의 허용 오차가 임의로 넓어졌는가?** — 1/64 CSS px를 명시했고 observed maximum은 0이다. 통과.
12. **text intrinsic source가 없는 장면을 text 성공으로 세었는가?** — 각 result에 source kind와 font registry revision이 기록됐다. 통과.
13. **revision 숫자가 다른 run에서 가져왔는가?** — 10개 run log마다 `fontRegistryRevision=1`을 확인했다. 통과.
14. **Columns 또는 Rows만 trace되었는가?** — 두 축 모두 10/10 result JSON에 기록됐다. 통과.
15. **불완전한 trace chunk를 완전한 JSON으로 잘못 보고했는가?** — 각 실행 chunk 연속성·개수와 JSON parse가 collector에서 assertion됐다. 통과.
16. **V8 evaluation 실패를 layout pass로 오인했는가?** — 각 run의 C11.4 eval `status=0`을 분리 확인했다. 통과.
17. **Surface 제출을 증명하지 못하는 log를 present로 불렀는가?** — `SPINON_C0410_DRAW status=0 presented`를 확인했다. 통과.
18. **설치한 APK와 빌드 APK가 다를 수 있는가?** — device `base.apk`와 local APK SHA-256을 대조했다. 통과.
19. **APK가 어느 source인지 빠졌는가?** — source commit과 APK SHA-256이 연결돼 있다. 통과.
20. **에뮬레이터를 실기기로 오기록했는가?** — `ro.kernel.qemu=0`, serial, model, OS/API를 기록했다. 통과.
21. **렌더러 출처를 앱 GPU adapter로 과장했는가?** — README는 SurfaceFlinger renderer가 보고한 Xclipse/ANGLE/Vulkan 문자열로 한정한다. 통과.
22. **화면 크기나 density가 빠져 환경 재현이 어려운가?** — 1080×2340, density 450을 기록했다. 통과.
23. **screenshot이 다른 scene에서 찍혔는가?** — 두 파일명과 run ID가 normal/break-all scene 결과에 연결됐다. 통과.
24. **screenshot을 PixelCopy exact로 과장했는가?** — screen capture는 시각 확인으로만 기술했다. 통과.
25. **glyph paint가 통과했다고 오해하게 되는가?** — fixture에 glyph가 없다는 한계를 README와 STATUS에 썼다. 통과.
26. **compositor 이후 numeric pixel 정확도를 주장하는가?** — 해당 비교를 하지 않았다고 명시했다. 통과.
27. **WPT나 전체 CSS Grid 지원을 암시하는가?** — 두 문서와 public roadmap이 명시적으로 미완료를 유지한다. 통과.
28. **재현 명령이 실제 package/extra 이름과 일치하는가?** — Android manifest handoff와 성공한 launch log를 대조했다. 통과.
29. **Android build와 repository-wide docs build 상태를 혼동하는가?** — Android APK build는 성공했다. 전체 RSPress build는 기존 dead image·Shiki 오류로 실패했고 `docs-build.log` 및 README에 남겼으며 성공으로 쓰지 않았다. 통과.
30. **versioned CSS contract나 공개 API가 부당하게 바뀌었는가?** — 추가된 건 내부 execution evidence 및 roadmap note뿐이다. 통과.

세부 실행 검토는 [실기기 근거 검토 20개](./physical-evidence-review-20.md)에 별도로 기록했다.
