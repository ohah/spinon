# C11.4 direct-leaf text multi-span PR diff 적대 검토 30개

검토 범위는 runtime scene router/validator, FFI fixture source, Rust oracle test, Android/iOS app assertions, capture runner, evidence/plan/STATUS 및 PR #408 본문이다. Runtime 구현의 별도 20 failure-path 검토는 [implementation-review-20.md](./implementation-review-20.md)에 있다. PR head `507a566d7a0ab61a693214e77ec80becd79185a1`을 기준으로 검토했다.

1. **다른 C11.4 scene이 text route에 잘못 들어오는가?** — runtime과 FFI router는 네 exact scene ID만 선택하며 unknown ID를 새 text source로 받지 않는다. 통과.
2. **fixture ID prefix만으로 일반 document가 바뀌는가?** — 알려진 ID 뒤에는 `div` root, direct child 개수, exact item ID/order, topology, text 및 placement를 검증한다. 허용 밖 shape는 명시적 layout failure다. 통과.
3. **잘못된 root tag가 fixture로 받아들여지는가?** — known scene의 root tag가 `div`가 아니면 `unsupported_c114_text_multispan_root`를 반환한다. 통과.
4. **추가되거나 빠진 root child가 숨겨지는가?** — 직접 child 개수를 scene item 수와 비교하고 각 위치의 ID를 확인한다. 통과.
5. **DOM order와 style source order가 뒤섞이는가?** — scene별 기대 item slice를 snapshot child 순서와 zip하고 Rust/runtime trace의 source order를 별도로 비교한다. 통과.
6. **nested 또는 여러 text node가 하나로 합쳐지는가?** — item direct child를 하나로 제한하고 그 노드가 `Text`인지, parent 링크가 해당 item인지 검사한다. 통과.
7. **장면 ID만 맞고 text 내용이 다른가?** — 각 item의 expected ASCII text와 실제 내용을 정확히 비교한다. 통과.
8. **span 시작 line/길이가 변형되는가?** — computed placement의 column start와 span end를 scene별 line/length와 exact 비교한다. 통과.
9. **row placement의 end auto 동작이 빠지는가?** — row start뿐 아니라 `row_end == Auto`를 검사한다. 통과.
10. **grid style 누락 또는 display 변경이 통과하는가?** — computed grid 존재와 display, box sizing, item/content alignment를 검증한다. 통과.
11. **root 크기 제약이 contribution 사례를 바꾸는가?** — 고정 220×60 크기, min/max, flex basis 상태를 허용 목록과 비교한다. 통과.
12. **margin/padding/border/gap가 기대 track/frame을 바꾸는가?** — 양쪽 spacing, border와 row/column gap이 모두 0인지 확인한다. 통과.
13. **explicit track 수나 크기가 달라지는가?** — columns 4개 `min-content`, rows 3개 `20px`를 타입화된 computed track 목록에서 검증한다. 통과.
14. **auto repeat/template area가 explicit matrix를 덮는가?** — auto repeat과 template area가 없어야 한다고 검사한다. 통과.
15. **implicit placement가 아이템 위치를 바꾸는가?** — row auto-flow와 auto rows/columns를 exact 기본값으로 제한한다. 통과.
16. **line name이 line lookup에 영향을 주는가?** — columns/rows line-name 배열 길이와 각 배열의 empty 상태를 확인한다. 통과.
17. **font fallback이나 typography 차이가 intrinsic text input을 바꾸는가?** — Ahem 10px와 weight/style/line-height, wrapping, direction, break, spacing 관련 computed property를 확인한다. 통과.
18. **item box 제약이 frame을 바꾸는가?** — width/height와 min/max, margin/padding/border가 기대한 auto/none/zero 상태인지 검사한다. 통과.
19. **환경 시스템 font가 oracle에 섞이는가?** — `TextLayoutEngine::new_without_system_fonts`에 Ahem fixture font만 등록한다. 통과.
20. **기본 제품 artifact가 test fixture를 끌어오는가?** — 새 FFI module과 route는 기존 `c04-runtime-gpu-fixture-test` cfg 안에 있고 default/positive-control artifact 검사 기록이 있다. 통과.
21. **runtime source가 사용자 scene 문자열을 JS에 삽입하는가?** — source generator는 fixed scene/item table에서만 생성하며 unknown ID는 `None`; 문자열은 Rust Debug quoting을 거친다. 길이도 4 KiB 미만을 검사한다. 통과.
22. **source text node 수와 node ID 목록이 다르게 만들어지는가?** — generator test와 FFI route test가 element, text node, `nodeIds` 개수를 각각 예상 item 수와 비교한다. 통과.
23. **intrinsic oracle이 DOM order 또는 text 측정 누락을 감추는가?** — 네 Rust cases가 item ID/source order와 min/max content 폭·높이를 각 입력마다 확인한다. 통과.
24. **span stage가 DOM 순서를 따라 잘못 실행되는가?** — span-2-only scene은 `[2]`, mixed scene은 `[2, 3]` batch를 Rust와 mobile runner에서 검사한다. 통과.
25. **Minimum을 MinContent/MaxContent로 잘못 대체하거나 질의를 생략하는가?** — item별 Minimum=0, MinContent 값, repeated MinContent trace 순서와 개수를 모두 비교한다. MaxContent는 원본 측정 입력으로만 보존되는 범위를 계획에 설명한다. 통과.
26. **used track 합은 맞지만 item frame이 다르게 배치되는가?** — 네 테스트 장면의 4개 track과 모든 node의 x/y/width/height를 1/64 CSS px 이내로 검사한다. 통과.
27. **Android route의 box/frame 검사가 새 장면을 건너뛰는가?** — exact scene allowlist, root 220×60, expected box count, text input, node ID/order 및 finite frame 비교를 추가했다. 통과.
28. **iOS JSON trace parsing이 chunk 경계나 frame 순서를 놓치는가?** — balanced JSON object를 추출해 JSONSerialization으로 읽고 intrinsic input, exact node/order와 각 frame 값을 검사한다. capture runner도 trace chunk 총수와 JSON parse를 확인한다. 통과.
29. **capture runner가 emulator를 실기기로 오인하거나 다른 APK를 검사하는가?** — serial을 명시하고 `qemu=0`, SM-S731N, API≥36, arm64를 확인하며 설치 전후 APK hash를 일치시킨 뒤 serial-scoped logcat에서 run ID를 확인한다. 최신 실기기 run은 4/4 통과했다. 통과.
30. **화면/roadmap이 근거보다 넓은 완료를 주장하는가?** — PR screenshot은 최신 실기기 box surface이고 glyph count 0, iOS 실기기/WPT/compositor 이후 검증 미포함을 적었다. STATUS checkbox와 공식 완료 수는 유지한다. 통과.

최종 diff에서 blocker를 찾지 못했다. Android 실기기에서 APK 설치본 hash, 네 장면의 intrinsic trace·track·node frame·draw/present까지 재실행했다. iOS 실기기, glyph paint, WPT와 OS compositor 이후 픽셀은 검증하지 않았다.
