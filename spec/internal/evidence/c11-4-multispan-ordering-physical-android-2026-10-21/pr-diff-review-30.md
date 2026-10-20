# C11.4 multi-span ordering PR diff 적대 검토 30개

검토 범위는 Rust multi-span oracle test와 fixture adapter, Android/iOS fixture assertions, Chrome HTML oracle, 142-scene capture runner, 계획·STATUS·실행 evidence다. 구현별 검토는 [Rust/Android 구현 20개](implementation-review-20.md)와 [iOS fixture/Simulator 검토 20개](ios-simulator-review-20.md)에 분리했다.

1. **독립 oracle이 구현 trace를 복사했는가?** — track 수치는 §11.5.1 규칙에서 도출하고 Chrome 155 사전 관찰과 비교했다. 코드 결과에서 기대값을 만들지 않았다. 통과.
2. **Chrome 환경과 quantization이 생략됐는가?** — 버전, OS, architecture, DPR와 used-value 1/64 CSS px 직렬화를 README 및 observation JSON에 남겼다. 통과.
3. **네 scene ID가 플랫폼마다 일치하는가?** — Rust router, Android expected-frame parser, Swift allowlist, capture runner에 동일한 네 고정 ID가 있다. 통과.
4. **동일 span의 source order가 used track을 바꾸는가?** — 두 span-2 순서가 Chrome/Rust/Android/iOS에서 모두 `40,60,60,0`이다. 통과.
5. **DOM 순서를 CSS span 크기 우선 단계와 혼동하는가?** — mixed reversed case에서도 runtime trace stage는 `[2,3]`이며 순수 Rust 검사가 stage와 contribution 순서를 별도로 확인한다. 통과.
6. **planned increase가 item마다 즉시 누적되어 결과가 과성장하는가?** — 같은 span batch의 track별 planned increase 합계와 다음 span 단계의 누적 기반값을 비교했다. 통과.
7. **Minimum 기여가 MinContent/MaxContent로 대체되는가?** — 세 contribution kind 모두 item ID, span, 값, 순서를 검사했다. 통과.
8. **item/span 매핑이 바뀌었는데 track 합만 같은가?** — contribution 목록과 각 preorder item frame을 독립 assertion했다. 통과.
9. **span-3 증가가 밖의 네 번째 track에 전파되는가?** — mixed cases에서 fourth used track이 0으로 남는다. 통과.
10. **0-width gutter가 used track count에 포함되는가?** — oracle과 mobile capture validator는 gutter를 필터링하고 네 track을 비교한다. 통과.
11. **container 잔여 공간이 max-content track에 섞이는가?** — 220×40 root, `justify-content:start`, max-content columns 및 0 gap을 양쪽 fixture에 고정했다. 통과.
12. **Chrome parent offset이 앱 frame 비교에 포함되는가?** — browser item 좌표는 grid root 기준으로 환산하고 앱은 document root-relative frame을 사용한다. 통과.
13. **row 위치가 column ordering 실패를 감추는가?** — 네 scene은 item row를 분리하고 frame x/y와 폭/높이를 모두 확인한다. 통과.
14. **runtime f32 출력이 Chrome 문자열과 달라 정상 장면을 실패시키는가?** — Android capture는 numeric tolerance를 사용하고 iOS runner도 네 frame field마다 1/64 CSS px 범위를 적용한다. 통과.
15. **`NaN` 또는 Infinity가 작은 오차처럼 통과하는가?** — Android expected-frame parser의 finite-number rejection을 보존하고 fixture log 네 장면에서 비정상 값이 없다. 통과.
16. **iOS에서 새 scene에 잘못된 기본 root/item frame이 쓰이는가?** — Swift root, first-item, 전체 preorder node frame 및 box count 기대값을 네 scene별로 명시하고 Simulator assertion을 통과했다. 통과.
17. **Android와 iOS의 box tree 개수가 어긋나는가?** — span-2는 3 boxes, mixed span은 4 boxes를 각 native assertion 및 draw/present marker에서 확인했다. 통과.
18. **scene argument가 다른 C11 route와 함께 선택되는가?** — Swift route guard의 C11.2/C11.3 상호 배제와 exact C11.4 scene allowlist를 유지했다. 통과.
19. **여러 trace chunk 중 일부가 빠졌는데 result가 만들어지는가?** — Android current run과 iOS capture 모두 part count와 declared total을 비교하고 JSON parse를 끝냈다. 통과.
20. **Simulator test가 마지막 scene의 span 단계 순서를 검증하지 않는가?** — runner에 `[2]` 및 `[2,3]` sequence assertion을 추가했고 현재 iPhone 17 Pro 실행을 다시 통과시켰다. 통과.
21. **화면 캡처가 다른 앱이나 이전 scene에서 오는가?** — iOS runner는 지정 UDID와 PID/run ID의 summary, trace, expected draw/present가 모인 뒤 screenshot을 찍는다. 화면도 열어 fixture scene을 확인했다. 통과.
22. **사용자가 연결한 Android 실기기를 emulator로 잘못 분류하는가?** — serial, `ro.kernel.qemu=0`, SM-S731N, API 36과 ABI를 기록했다. APK local/install SHA가 일치한다. 통과.
23. **다른 Android 기기가 APK를 실행하는가?** — install, launch, PID-scoped logcat, package SHA 조회 모두 `R5KYB06SMPE` serial을 지정했다. 통과.
24. **Android draw/present를 OS compositor 확인으로 확대 해석하는가?** — 문서는 PixelCopy를 SurfaceView 관찰로 한정하고 OS compositor 이후 및 border stroke 검증이 아니라고 적었다. 통과.
25. **기본 제품 artifact에 테스트 fixture가 포함되는가?** — Rust staticlib positive/negative build boundary 기록이 남고 Android/iOS 앱은 fixture-test flag를 켠 opt-in 빌드다. 통과.
26. **iOS 앱의 빌드 flag와 runtime route가 분리되는가?** — Xcode build에서 runtime GPU 및 C04 fixture-test flag가 각각 enabled임을 기록하고 default 제품 설정은 변경하지 않았다. 통과.
27. **새 scenes를 runner 목록에 추가하면서 기존 ID가 사라지거나 중복되는가?** — 원래 138 scene의 기존 assertion 뒤에 4개를 추가하고 count 142 및 전체 uniqueness를 실행 시 검사한다. 통과.
28. **evidence 경로가 iOS 실행을 Android 실기기 결과로 섞는가?** — multi-span simulator run은 별도 `ios-simulator-*` directory로 기록하고 metadata에 simulator name/runtime/UDID와 app hash를 남긴다. 통과.
29. **로드맵이 근거 이상으로 구현 완료를 선언하는가?** — plan/STATUS는 제한 slice와 남은 조합을 명시하고 C11.4 checkbox 및 공식 완료 수는 유지한다. 통과.
30. **PR 내용에서 남은 범위 또는 화면 근거가 빠지는가?** — PR template 기준 API/인터페이스, 범위, Android/iOS 실행 근거와 화면 link를 제공하며 전체 CSS Grid·WPT 완료를 주장하지 않는다. 통과.

최종 diff 재검사에서 blocker를 찾지 못했다. Android 실기기 SurfaceView와 iOS Simulator의 draw/present까지 확인했으며 physical iOS와 OS compositor 이후 검증은 별도 범위다.
