# C11.4 overflow automatic minimum PR 변경분 검토 30개

PR diff를 feature boundary, cascade, Grid projection, runtime fixture, FFI trace, Android/iOS harness, capture script, evidence와 상태 문서 경계로 나눠 서로 다른 회귀·오해 가능성을 확인했다.

1. `spinon-style` fixture feature의 기본값은 꺼져 있고 새 공개 re-export도 cfg에 묶여 있다.
2. `spinon-style-to-layout`은 style/layout fixture features를 함께 전파한다.
3. runtime feature는 기존 `c11-4-grid-track-sizing-fixture-test`에 연결되고 제품 feature 조합을 바꾸지 않는다.
4. `RuntimeBlockFlexGridTrackSizingV1` profile 이름/version은 그대로 유지된다.
5. 기존 public track-sizing cascade는 동일 author property allowlist를 쓴다.
6. overflow x/y 허용 override는 전용 fixture cascade 경로에서만 전달된다.
7. computed overflow data는 기존 cascade style snapshot에 담기며 새 일반 CSS 기능을 선언하지 않는다.
8. fixture root 탐지는 HTML div와 작업 전용 ID prefix를 제한 조건으로 쓴다.
9. prefix를 공유하는 미등록 고정 scene은 `prepare`에서 거부된다.
10. tagged root만 incremental style reuse를 생략해 일반 root 캐시 경로는 유지한다.
11. runtime layout dispatch도 profile과 root marker가 둘 다 맞아야 선택된다.
12. 새 route 선택 시 다른 C11.4 fixture adapter들이 덮어쓰기/중복 준비되지 않도록 조건을 묶었다.
13. 새 Ahem paragraph/font input은 정확히 고정 item 한 개에만 붙는다.
14. projection option의 기본값은 기존 nested-grid와 일반 caller에서 모두 false다.
15. `allow_c11_4_overflow_auto_minimum_fixture`는 전용 public fixture wrapper에서만 true다.
16. `project_styles`의 overflow projection gate가 기존 overflow clip profile과 새 opt-in fixture flag로 제한된다.
17. 새 FFI JavaScript scene 생성은 다섯 고정 ID 외에는 `None`을 반환한다.
18. generated source의 ID·overflow 값은 compile-time 고정 문자열에서만 보간된다.
19. trace JSON에는 expected 값과 computed-style에서 읽은 actual overflow 값을 구분해 싣는다.
20. trace JSON의 revision과 frame/track data는 기존 C11.4 trace revision 검증을 공유한다.
21. Android scene allowlist에 다섯 ID가 각각 한 번 들어가며 전체 count 증가는 517→522다.
22. iOS scene allowlist와 count 증가도 같은 다섯 ID를 반영한다.
23. 양쪽 harness는 root `80×20`, item `100×20` 또는 `80×20`, root/item node 수 2를 확인한다.
24. Columns/Rows track 목록과 두 axis trace를 각각 검증한다.
25. visible/clip 및 scrollable automatic minimum source/value를 분리해 검사한다.
26. MaxContent contribution 100px를 Minimum 검증과 별도로 유지한다.
27. Android PixelCopy 표본의 CSS 좌표와 target ID를 검사하며 로그에서 exact RGBA도 확인된다.
28. iOS Metal 표본은 drawable 시점으로 한정해 compositor 이후의 OS 화면 근거처럼 쓰지 않는다.
29. capture script는 dedicated run kind와 evidence directory를 만들고 다섯 scene만 선택했는지 확인한다.
30. README와 STATUS는 이 runtime slice만 연결하고 C11.4 checkbox 및 `72/213`을 올리지 않는다.

## 검토 중 수정한 점

- runtime frame map이 중복 frame을 덮어쓰는 경계를 찾아 전체 frame count와 고유 node 수를 정확히 둘로 제한했다.
- project projection이 line-name metadata에 빈 그룹을 보존할 수 있어, metadata 자체가 비었는지 대신 실제 name이 있는지 확인한다.
- Clippy의 too-many-arguments와 clone-on-copy를 구조체 옵션 및 불필요한 clone 제거로 고쳤다.

각 검토 후 최종 Android/iOS fixture app을 다시 빌드하고 각 5 scene을 재실행했다.
