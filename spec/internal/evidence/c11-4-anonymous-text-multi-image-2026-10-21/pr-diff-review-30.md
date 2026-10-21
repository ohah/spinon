# C11.4 anonymous text + multiple spanning image PR 변경분 검토 30개

계획 검토 20개와 구현 실패 관점 검토 20개를 재사용하지 않고, PR의 교차 파일 계약·근거 연결·변경 경계에서 서로 다른 실패 가능성을 확인했다.

1. **상태 대장 수치가 구현 범위에 맞게 갱신됐는가?** 기존 `72/213` 집계는 유지하고 새 제한 fixture를 완료 개수에 더하지 않았다. 통과.
2. **C11.4 전체 완료로 오해할 수 있는 체크가 생겼는가?** 상위 checkbox는 미완료로 유지하고 본문도 제한 slice라고 명시한다. 통과.
3. **계획과 상태 대장의 세-image 기대값이 서로 다른가?** 둘 다 독립 규범 계산 `[18,34,20,0]`으로 맞췄다. 통과.
4. **Chrome 155 관측을 규범 기대값으로 둔갑시켰는가?** Chrome `[16,32,24,0]`과 차이를 나란히 기록하고 원인을 확정하지 않는다. 통과.
5. **2-image·3-image 장면 중 한쪽만 시뮬레이터에 연결됐는가?** 네 scene을 Android와 iOS 각각 4/4 실행했다. 통과.
6. **source order 역전 장면의 문서 예시가 배열과 어긋나는가?** 계획의 forward/reversed child order가 FFI와 simulator 결과 JSON과 일치한다. 통과.
7. **규범 계산에서 span별 track 증가량이 빠졌는가?** 계획은 text growth limit, span-2 48px 분배, span-3 12px 추가 단계를 적는다. 통과.
8. **시뮬레이터 결과를 Chrome parity로 과장했는가?** Android/iOS 실행은 Spinon 내부 계산 일치로 기록하고 Chrome 차이는 별도 한계로 유지한다. 통과.
9. **지원 범위가 일반 HTML/CSS로 확대 서술됐는가?** fixed Ahem `XX`, 2~3 direct leaf PNG image, 네 source-order fixture로 제한한다. 통과.
10. **공개 API·제품 기능 완료로 잘못 분류했는가?** API 명세나 지원 체크를 추가하지 않고 내부 fixture 검증으로 유지한다. 통과.
11. **내부 계약 버전이 범위 추가만으로 올라갔는가?** 숫자 버전 및 계약 파일을 변경하지 않았다. 통과.
12. **새 Rust module이 기존 fixture route와 충돌하는가?** 제한 scene allowlist에 먼저 연결하고 다른 C11.4 route는 기존 흐름을 유지한다. 통과.
13. **FFI test scene count와 capture scene count가 다른가?** FFI 고정 matrix는 4개, 전체 capture 목록은 165개에서 169개로 늘어 중복 없는 4개가 추가됐다. 통과.
14. **Android·iOS·FFI·capture tool 중 scene ID 표기가 빠졌는가?** 네 경로에 같은 4개 ID가 모두 등록되어 있다. 통과.
15. **anonymous text가 renderable node 수와 섞여 box count가 틀리는가?** frame list는 root+image만 세고, anonymous text frame은 별도 grid item diagnostics에서 검사한다. 통과.
16. **직접 text source 순서와 native node-frame 순서를 혼동하는가?** sourceOrder와 data-node frame list를 각각 검사한다. 통과.
17. **Android와 iOS build flag가 다른 feature를 켜는가?** 두 플랫폼 모두 C04 runtime GPU와 opt-in fixture-test를 함께 켰다. 통과.
18. **Cargo feature와 native compile define 사이가 끊겼는가?** Android preparation log 및 iOS build log에 `c04-runtime-gpu-fixture-test`가 포함되어 실제 앱 빌드가 성공했다. 통과.
19. **기본 앱 빌드가 개발 fixture를 강제로 포함하는가?** 기본 feature archive에서 scene ID가 빠지는 것을 따로 확인했고 opt-in positive control은 포함한다. 이 검사는 최종 release bundle 검증으로 부풀리지 않았다. 통과.
20. **고정 source URL이 네트워크 fetch를 의미한다고 읽히는가?** decoder는 고정 PNG bytes를 사용하며 source URL은 fixture identity 확인에 쓴다. README는 이를 자원 loader 지원으로 주장하지 않는다. 통과.
21. **intrinsic data hash와 screenshot metadata가 다른 APK에서 왔는가?** 최종 Android 설치 APK와 local APK SHA-256이 같고 simulator metadata에 남아 있다. 통과.
22. **Android trace JSON parse 오류가 성공처럼 삼켜지는가?** `JSONException`, `ClassCastException`, span number parse 오류는 false 결과가 되며 app assertion이 실패한다. 통과.
23. **iOS trace chunk 분할이 전체 JSON을 잘라먹는가?** iOS capture는 chunked diagnostics 전체를 조합해 네 scene의 revision·tracks·contributions를 검사했다. 통과.
24. **scene 선택 인자가 일부 입력을 조용히 버리는가?** capture tool은 요청 ID 중복과 누락을 검사하고 네 ID 모두를 결과에 기록한다. 통과.
25. **긴 run ID가 Android intent나 iOS argument에서 잘리는가?** capture tool은 실제 run prefix 기준 최대 길이를 사전에 제한한다. 통과.
26. **Android 결과가 실기기 실행으로 잘못 표시되는가?** `emulator-5558`, API 37, ARM64 AVD metadata를 저장했고 실기기 검증으로 표기하지 않는다. 통과.
27. **iOS 결과가 다른 booted simulator에서 왔는가?** 지정 UDID, iPhone 17 Pro 이름, iOS 26.2 runtime을 JSON에 저장했다. 통과.
28. **첫 실패한 capture 시도가 숨겨졌거나 성공으로 집계됐는가?** verifier 기대 순서를 고친 과정과 실패 로그 경로를 evidence README에 기록하고 최종 run만 4/4로 집계한다. 통과.
29. **screenshot을 texel, border, overflow 또는 compositor 검사라고 주장하는가?** README와 plan 모두 해당 항목이 이번 범위 밖임을 명시한다. 장면 자체에 CSS border도 없다. 통과.
30. **PR 검증 전에 병합 상태나 CI를 앞서 단정하는가?** evidence는 local test/build/Simulator만 보고하며 merge와 live required-check 상태는 PR 생성 후 따로 확인한다. 통과.

## 결과

이 30개 교차 파일 검토에서 추가 코드 수정이 필요한 결함은 발견하지 않았다. 수정 이력에 있는 trace 기대 순서와 Android numeric fail-closed 보강은 최종 변경에 반영되어 있고, 최종 simulator 결과는 Android 4/4 및 iOS 4/4다. 미해결 항목은 Chrome 155 3-image 차이, PNG pixel readback, CSS border/overflow paint, compositor 이후 pixels, WPT 및 C11.4 전체 지원이다.
