# C11.4 anonymous Grid text `white-space:pre-line` PR diff 적대적 검토 30개

대상은 구현 PR [#867](https://github.com/ohah/spinon/pull/867)의 변경이다. 각 관점은 별도 입력·불변 조건·경계의 실패 가능성을 확인했다. 첫 Android trace 재검증에서 경계 gutter entry를 활성 track으로 오인하는 검증기 결함을 발견해 고친 뒤 Android fixture 앱을 재빌드하고 두 장면을 다시 실행했다. 현재 남은 범위와 시각 근거의 한계는 [실행 근거](./README.md)에 기록했다.

1. **일반 HostDocument 경로 우회:** `text_paragraphs::collect`는 `allow_c11_4_anonymous_grid_text_items`가 false인 product 호출에서 익명 Grid text 수집을 켜지 않는다. 확인: `host_document.rs`, `host_document/anonymous_grid_text.rs`.
2. **Block paragraph 범위 확장:** `PreLine` 추가는 익명 Grid 분기 안에만 있어 일반 `display:block` paragraph 규칙과 collector 경로는 바뀌지 않았다. 확인: `host_document/text_paragraphs.rs`.
3. **scene ID spoofing/unknown:** runtime fixture는 고정 scene 목록에 없는 ID를 거부하고 FFI source builder도 unknown ID에서 `None`을 반환한다. 확인: runtime fixture와 FFI fixture.
4. **중복 scene 등록:** runtime과 FFI 9개 matrix의 ID가 유일하며 기존 7개를 보존하고 두 newline scene만 추가했다. 확인: 두 Rust matrix tests.
5. **텍스트 source 변조:** 두 신규 scene은 direct text node 하나와 정확한 `XX\nXX`만 허용한다. 노드 수·종류·문자열이 다르면 runtime validator가 거부한다.
6. **Normal의 newline 모드:** 대조 scene은 `white-space-collapse:collapse`와 `text-wrap-mode:wrap`을 모두 검증한다. LF를 줄바꿈으로 보존하는 잘못된 매핑은 통과하지 않는다.
7. **PreLine의 newline 모드:** PreLine scene은 `preserve-breaks/wrap`을 함께 검증하고 `WhiteSpaceMode::PreLine`을 명시 전달한다. 단어 wrap을 끄거나 공백을 보존하는 다른 profile은 허용하지 않는다.
8. **style profile 오염:** 고정 root style의 font, line-height, 방향, writing-mode, order 및 edge 조건을 scene validator가 확인한다. 값 하나가 달라지면 고정 fixture가 중단된다.
9. **computed style와 paragraph profile 불일치:** runtime fixture는 computed `white-space-collapse`와 `ParagraphStyle.white_space`를 scene별 expected mode와 각각 비교한다.
10. **HostDocument revision/source 유실:** Android/iOS diagnostics 검사는 HostDocument source revision, style/environment/intrinsic revision, node와 source order를 확인한다.
11. **Normal min-content 회귀:** 기존 Normal `XX XX` min-content control과 white-space 테스트를 유지하고, newline 전용 테스트는 LF가 collapsed space가 되는 max-content 결과를 확인한다.
12. **NoWrap 회귀:** 기존 NoWrap min-content 테스트는 단일 긴 줄과 50×10 item overflow를 계속 확인한다.
13. **PreLine intrinsic 크기:** 새 layout 테스트는 min/max-content width와 height가 각각 20px이고, 최종 item과 root가 20×20px인지 확인한다.
14. **Normal newline intrinsic 크기:** 대조 테스트는 max-content width 50px/height 10px와 root 너비 20px에서 30px 넘치는 item을 별도 검증한다.
15. **Columns track 기여:** 두 장면 모두 단일 MaxContent column의 final base size를 각각 50px와 20px로 확인한다.
16. **Rows track 기여:** auto row는 Normal 10px, PreLine 20px이며 final row contribution과 track base size가 같은지 확인한다.
17. **anonymous item identity:** layout 결과에서 text item node와 source node가 일치하고 source order 0이며 source list가 하나인지 확인한다.
18. **item frame 누락:** simulator validator는 root node frame뿐 아니라 diagnostics의 Grid item frame과 intrinsic text layout 크기도 검사한다.
19. **축 순서/trace 누락:** Android와 iOS validator는 Columns 다음 Rows의 두 pass, 각 final stage와 trace 배열 존재를 요구한다.
20. **Android gutter 오판:** 첫 강화 시도는 경계 gutter entry를 포함한 전체 trace track 수가 하나라고 가정해 정상 scene을 거부했다. 이를 수정해 gutter/collapsed 필드가 실제 Boolean인지 확인하고 경계 entry를 제외한 활성 track 한 개의 크기를 검사했다. Android 재빌드와 두 scene 실행이 통과했다.
21. **Android 기여 span:** Android newline trace는 item 2, `span:1`, expected contribution value, Columns `MaxContent` kind를 확인한다. Grid root가 두 개의 implicit active track을 만드는 경우에도 track count 검사가 거부한다.
22. **Android malformed diagnostics:** diagnostics marker 누락, null pass/stage/track/contribution, JSON 파싱 오류와 잘못된 object type은 모두 false로 닫힌다.
23. **Android 수치 경계:** NaN/infinite 또는 기대값에서 1/64px 넘게 벗어난 수치는 숫자 검사에서 거부된다.
24. **Android route assertion:** 두 scene은 layout-ready, expected root frame/node frame, trace validator와 기존 draw/presentation/retry 검사를 모두 통과해야 한다.
25. **iOS scene catalog 누락:** Swift scene ID set, expected frame/box-count table에 두 신규 scene을 추가했고 626-scene capture boundary 검사도 ID를 확인한다.
26. **iOS source/revision 변조:** diagnostics에서 HostDocument source, non-negative revisions, node 2/order 0, text `XX\nXX`, Ahem registry revision 1을 확인한다.
27. **iOS paragraph/item profile:** iOS validator는 PreLine/Normal paragraph style, intrinsic dimensions, item frame과 node identity를 검사한다.
28. **iOS trace parity:** iOS는 두 축의 final active track, contribution item/span/value와 Columns MaxContent kind를 확인한다. Android와 결과 계약이 일치한다.
29. **fixture binary 경계:** 기본 product static archive에는 두 scene marker와 관련 host export가 없고 fixture-test positive control에는 각각 존재한다. 증거는 archive 범위에 한정하며 APK/IPA 전체 제외를 주장하지 않는다.
30. **상태·공개 범위·이미지 과장:** `STATUS.md`에서 C11.4/C11 미완료와 `72/213`을 유지한다. 내부 계약은 `0.1.0`이며 일반 author CSS/WPT/glyph pixels/실기기를 지원한다고 쓰지 않는다. PR raw 이미지 두 개는 Agent Browser에서 로드와 크기를 확인했고 black viewport 한계를 캡션에 밝혔다.

## 재검증 결과

- 구현/runtime 별도 검토 20개는 [구현·runtime 검토](./implementation-runtime-review-20.md)에 남아 있다.
- Android validator 수정 후 Android fixture app build와 Android API 37 AVD의 두 scene이 성공했다. 정확한 결과·실패 수정 과정은 [실행 근거](./README.md)의 Android·iOS 시뮬레이터 절을 참조한다.
- `git diff --check`, format, focused tests와 Clippy 결과는 같은 README의 검증 표 및 원본 로그에 연결한다.
- PR required status checks와 최종 병합 상태는 검토 문서 생성 이후 별도로 확인한다. 이 문서는 CI 또는 merge 완료를 뜻하지 않는다.
