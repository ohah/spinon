# C11.4 multi-span ordering PR diff 적대 검토 30개

검토 대상은 이 PR의 Rust fixture/test, Android assertion, Chrome fixture와 oracle, 계획·상태 문서 및 기기 실행 증거다. 구현 검토는 별도 [20개 기록](implementation-review-20.md)에 남겼고, 아래에서는 서로 다른 diff 실패 경로를 다시 확인했다.

1. **기대값 출처** — track 수치는 CSS Grid §11.5.1에서 먼저 도출했고 Chrome 155 사전 관찰로 대조했다. 구현 trace에서 oracle을 역산하지 않았다. 통과.
2. **Chrome 관찰 환경** — Chrome 버전, macOS, ARM64, DPR 1이 기록되어 있고 Chrome 화면을 Spinon 실행 결과로 오인하지 않도록 구분했다. 통과.
3. **frame 좌표 기준** — Chrome grid가 container 안쪽 x=98에 놓인 사실과 item의 root-relative 좌표를 분리했다. Android root-relative frame과 비교할 때 parent offset이 섞이지 않는다. 통과.
4. **경쟁 span 입력** — 80px와 120px의 겹치는 span-2 item이 같은 두 열을 경쟁하며 예상 열 크기 `40,60,60`을 만든다. 통과.
5. **동일 span source order** — forward와 reversed case의 DOM 순서와 결과 item 순서가 각각 유지되면서 열 크기는 같음을 검사한다. 통과.
6. **짧은 span이 긴 span보다 DOM에서 뒤에 있는 경우** — reversed mixed case에서 span-3 item이 먼저여도 sizing batch는 길이 기준 `[2,3]`이다. 통과.
7. **실제 단계 관찰 여부** — 최종 track만 우연히 일치해도 통과하지 않도록 `IntrinsicItemBatch` span 순서도 검사한다. 통과.
8. **span-2 다음 span-3 누적** — 200px span-3 item은 앞선 160px 합계에 남은 40px을 요구하고 세 track으로 분배된다. Chrome used value와 수치 비교했다. 통과.
9. **span 밖 track 오염** — 4번째 열이 최종 used track 결과에서 계속 0인지 Rust와 Android trace에서 확인했다. 통과.
10. **track 개수에 gutter 포함** — Rust assertion은 gutter를 제외한 네 used track을 선택하고 기대 길이와 비교한다. 통과.
11. **기본 정렬로 인한 잔여 공간 분배** — `justify-content:start`와 non-flex max-content track을 사용해 root의 남는 20px이 각 track을 늘리지 않는 조건을 고정했다. 통과.
12. **implicit row 배치 간섭** — item마다 명시적 row를 분리하고 x와 y를 각각 검사해 세로 겹침이 column 결과를 숨기지 않는다. 통과.
13. **gap 처리 간섭** — 양쪽 fixture의 gap은 0이며 root와 item의 border/padding/margin도 0이다. spacing contribution이 oracle에 섞이지 않는다. 통과.
14. **overflow 픽셀 근거 경계** — 최종 동일 APK의 PixelCopy 표본은 root 안·밖, 바깥 item, 배경 4/4 exact다. 문서는 compositor 이후 화면 또는 border stroke의 증거로 확장하지 않는다. 통과.
15. **trace owner 선택** — 순수 Rust 검사는 root node의 Grid trace와 Columns pass를 고른다. child 또는 Rows trace를 잘못 읽는 경로가 없다. 통과.
16. **contribution 단계 completeness** — item별 `Minimum`, `MinContent`, `MaxContent`의 순서·span·handle·값을 모두 검증한다. 통과.
17. **frame와 used track 연결** — item 시작 x는 기대 used track 누적합이며 width·height·row y까지 비교한다. 통과.
18. **입력 수명과 handle collision** — 각 test case는 별도 `HostDocument`와 owner를 만들고 item handle을 document 생성 순서에 결합한다. 통과.
19. **fixture scene routing** — 네 고정 scene만 source를 생성하며 unknown ID는 `None`이다. 등록된 다른 C11.4 route보다 앞에서 exact ID를 찾는다. 통과.
20. **JS source 삽입 경계** — ID는 상수 입력이고 JS 문자열은 Rust debug quoting을 거친다. request text를 코드로 붙이지 않는다. 통과.
21. **layout profile 일치** — test와 runtime 모두 220×40 root, 네 `minmax(0,max-content)` 열, 고정 item 폭을 사용한다. 두 방식에서 flex/auto-repeat profile을 암묵적으로 추가하지 않았다. 통과.
22. **Android scene registry** — 기대 scene 목록에 네 ID가 있고 root frame, box count도 case별 3/4개로 등록했다. 통과.
23. **기존 expected-frame 경로 충돌** — 새 scene은 단일 item-frame 및 기존 고정 node-frame 문자열에서 제외하고 multi-span parser 하나로 전체 preorder node frame을 검사한다. 통과.
24. **Android frame 구조 오류** — node 수, preorder index/node identity, 각 field prefix를 검증한다. 수정 전 Chrome 문자열과 runtime f32 문자열의 strict equality가 두 case를 잘못 거부했으며, 보존된 prerun 로그와 최종 수치 검증으로 수정 결과를 확인했다.
25. **비정상 수치 입력** — `Double.parseDouble("NaN")`은 비교문을 통과할 수 있음을 찾아 `NaN`과 Infinity를 명시 거부하도록 고쳤다. 최종 Java compile과 실기기 case를 다시 실행했다.
26. **허용 오차 경계** — Chrome 1/64 CSS px used-value와 runtime f32 차이를 1/64 CSS px 이내로 제한한다. 실제 최대 track `0.010415`, frame `0.005207 CSS px`다. 통과.
27. **fixture 실행 기능 의존** — Android 빌드 플래그와 Cargo `c04-runtime-gpu-fixture-test`가 함께 켜진 설치 빌드에서 실행했다. simulator 또는 debug source만으로 실기기라고 표기하지 않았다. 통과.
28. **제품 아티팩트 음성 대조** — `c04-runtime-gpu` 정적 라이브러리에는 multi-span scene string이 없었다. `c04-runtime-gpu-fixture-test` 정적 라이브러리에서는 positive control이 확인됐다. 통과.
29. **실기기 동일성** — device model/API/GPU와 local·설치 APK SHA-256을 대조했고 네 case 및 overflow run ID를 실행 로그와 observation JSON에 연결했다. 통과.
30. **범위·완료 상태** — 계획 표에 네 scene ID를 등록하고 mixed track 조합, nested/hidden case, WPT와 전체 Grid가 남았음을 명시한다. STATUS에는 실행 근거와 두 review 기록을 연결했으며 C11.4 checkbox와 완료 수는 올리지 않았다. 통과.

최종 코드·증거 재검사에서도 blocker는 발견되지 않았다. Android evidence는 SurfaceView PixelCopy까지이며 OS compositor 이후 화면은 별도 검증으로 남는다.
