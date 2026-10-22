# C11.4 양축 cyclic `minmax(calc())` minimum 구현·runtime 실패 관점 20개

계획 검토와 분리해 실제 변경 코드, 고정 scene 등록, Android/iOS 실행 및 표본 경계를 검토했다. 아래 20개는 서로 다른 실패면을 대상으로 한다.

1. **양축 percentage가 같은 intrinsic basis를 참조하는가?** Columns 50px와 Rows 10px 결과가 `[104]/[22]`로 분리되고 양 플랫폼 trace에서 일치했다.
2. **row cyclic 값이 wrapped text의 사전 높이를 건너뛰는가?** definite 10px column 대조는 20px intrinsic height와 42px final row를 보였다.
3. **percentage basis에 border 두께가 섞이는가?** border-box root는 54×14px이나 content bases 50×10px을 써 최솟값 104/22px을 얻는다.
4. **intrinsic phase에서 cyclic minimum을 먼저 resolve하는가?** root intrinsic size와 final track trace를 별도 비교해 50×10px 산정 뒤 resolve를 확인했다.
5. **`max-content`가 더 작을 때 minimum flooring이 빠지는가?** definite control은 24/22px minimum track을 반환했다.
6. **column-only control이 row cyclic 경로도 켜는가?** definite 10px row track과 `[104]/[10]` 결과를 두 플랫폼에서 확인했다.
7. **row-only control이 column 폭을 임의로 늘리는가?** column 10px과 row 42px 결과 및 item overflow가 일치했다.
8. **scene ID가 일부만 허용되어 임의 CSS 입력이 통과하는가?** 신규 네 ID만 등록하고 미등록 prefix 거부 경계를 유지했다.
9. **FFI HTML generator 등록만 하고 runtime text-scene registry를 빠뜨리는가?** 서로 다른 두 registry에 네 ID를 모두 추가하고 각각의 집합 테스트를 통과시켰다.
10. **fixture HTML에 text 또는 node ordering이 달라지는가?** root→item 두 node, 고정 ID·`aa aa` source를 FFI 테스트와 runtime intrinsic input으로 확인했다.
11. **시뮬레이터 argument allowlist만 추가하고 Android pixel-scene allowlist를 누락하는가?** 첫 opt-in APK 실행에서 거부를 재현하고 별도 pixel-matrix allowlist를 추가했다.
12. **pixel 표본이 존재하지 않는 node를 가리키는가?** Android sample ID마다 grid/item target, CSS 좌표, 중복을 검사하는 gate가 네 scene에서 통과했다.
13. **앱 RGB 기대값 테이블에 새 표본이 빠지는가?** red item, gold left border와 black outside-item 색 분류를 신규 suffix 전부에 등록했다.
14. **definite both-axis root가 fallback geometry를 쓰는가?** Android helper가 `100×40px`를 기대했던 결함을 `14×14px`로 고치고 전체 재실행했다.
15. **item 바깥의 right-edge sample이 실제 item 안을 가리키는가?** `x=25`가 red item 내부임을 표면으로 확인하고 다음 outside pixel `x=27`로 옮겨 전체 matrix를 재실행했다.
16. **Swift scene ID, root/item frame, node frame 또는 box count 중 한 gate가 빠지는가?** iOS dispatcher의 네 목록과 frame 기대값을 일치시키고 네 장면을 통과했다.
17. **iOS pixel sample이 실제 renderer 제출 전과 후를 혼동하는가?** Metal drawable readback 28/28 exact만 주장하고 compositor 이후 pixel 증거라고 표현하지 않는다.
18. **Android screenshot/log가 이전 binary 결과를 최신 소스로 오인하는가?** stale APK SHA를 기록했고 명시적 fixture build 및 설치 SHA 일치 후 결과만 성공 근거로 사용했다.
19. **opt-in scene이 기본 release static archive에 들어가는가?** 기본 archive에서 새 scene payload 및 C11.4 fixture ABI 이름이 없고 positive control에 포함됨을 확인했다.
20. **검증 결과가 일반 CSS 또는 C11.4 완료로 과장되는가?** 고정 fixture, AVD/Simulator, WPT·실기기·hardware/compositor 한계를 README와 status에 분리한다.

**판정:** 발견한 Android pixel-scene allowlist, definite-root 기대값, item 경계 표본과 오래된 APK 실행 경로를 수정 또는 명시한 뒤 최종 Android 4/4 및 iOS 4/4 capture를 다시 통과시켰다. glyph paint, WPT, Android 실기기, hardware GPU와 iOS compositor-after pixels는 미검증이다.
