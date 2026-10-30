# C11.4 anonymous Grid text pre-line 구현/runtime 실패 관점 검토 20개

각 관점은 Normal positive control과 PreLine scene을 함께 확인했다. 검토 대상은 구현/runtime 변경이며 이 목록을 PR 변경분 검토로 재사용하지 않는다.

1. **LF 의미가 style mode와 일치하는가?** Normal은 collapsed space, PreLine은 forced line break로 계산됐다. Layout tests에서 각각 50px/한 줄과 20px/두 줄을 확인했다.
2. **Computed longhand와 paragraph mode가 어긋나는가?** Runtime validator는 Normal의 `collapse/wrap`과 PreLine의 `preserve-breaks/wrap`을 scene ID별로 정확히 요구한다. Mismatch는 거부한다.
3. **PreLine이 whitespace를 모두 보존하는 모드로 잘못 취급되는가?** 공백 collapse와 segment-break preservation을 별도로 표현한다. `pre`, `pre-wrap`, `break-spaces`는 추가하지 않았다.
4. **Max-content width가 줄바꿈 뒤 잘못 합산되는가?** Normal은 `20 + 10 + 20 = 50px`, PreLine은 각 line의 최대 폭 20px로 평가했다. Chrome 156 oracle과 일치한다.
5. **Min-content width 기여가 별도로 틀리는가?** 두 줄 각각 20px이고 각 줄 안에 break opportunity가 없으므로 두 scene의 min-content width는 20px다. Intrinsic input assertions가 이를 고정한다.
6. **Auto row가 두 번째 line을 놓치는가?** Normal row는 10px, PreLine row는 20px로 산출됐다. Rows final stage와 text item height를 모두 검사한다.
7. **Columns 값이 Rows 값과 섞이는가?** Trace는 `Columns` 뒤 `Rows` 두 pass만 요구하며 각각 `[50]/[10]`, `[20]/[20]`을 확인한다.
8. **Contribution이 다른 item이나 span에서 오는가?** 두 simulator validator는 NodeId 2, span 1과 max-content contribution 값 및 Grid owner 1을 확인한다.
9. **Text node의 source가 exact fixture에서 벗어나는가?** Runtime은 단일 direct child의 정확한 `XX\nXX`를 요구한다. 앞뒤 공백, 다른 문자열, 다른 노드 종류는 이 scene으로 수락하지 않는다.
10. **Whitespace-only sibling 때문에 anonymous item 수가 달라지는가?** 두 장면은 단일 meaningful text node만 허용하며 FFI source와 runtime child validator가 같은 tree를 고정한다.
11. **Nonzero `order`가 새로 허용되는가?** 기존 negative test는 계속 `UnsupportedTextParagraph`를 요구하고 최종 layout test set에 포함된다.
12. **Unsupported paragraph properties가 PreLine 추가로 느슨해지는가?** paragraph allowlist 변경은 white-space enum 한 값에 한정되며 다른 text profile 속성은 기존 exact validation을 유지한다.
13. **두 validation layer 사이에 누락된 gate가 있는가?** 첫 focused run에서 `compute.rs`가 Normal/NoWrap만 허용해 거부하는 불일치가 발견됐다. `compute.rs`와 `text_paragraphs.rs`를 함께 보완했고 12 layout tests가 통과했다.
14. **잘못된 root/style profile이 anonymous route를 우회하는가?** Runtime은 root ID scene table, `RuntimeBlockFlexGridTrackSizingV1`, `div`, exact Grid style과 revision-bound computed properties를 검사한다.
15. **TextLayoutEngine 결과가 다른 source revision에서 재사용되는가?** Android/iOS reports는 HostDocument source와 layout input revision을 확인한다. 각 결과의 document revision은 7이다.
16. **20px root보다 큰 normal track이 clamp되어 oracle을 잃는가?** Normal track과 item은 50px를 유지하고 root는 20px다. 예상 30px overflow를 root-size assertion과 trace가 보존한다.
17. **Track trace가 일부 chunk만 수집된 상태에서 통과하는가?** Capture tool의 기존 complete-chunk validation을 유지했고 새 scene은 Android 7개·iOS 13개 chunk를 수집했다.
18. **실패 scene이 이전 화면을 새 성공처럼 보고하거나 malformed trace가 validator를 crash시키는가?** Scene별 status, generation, document/environment revision, draw/present 및 superseded retry를 검사한다. Android의 null pass/stage guard를 보완해 malformed JSON은 실패로 닫는다. 두 플랫폼 모두 status 0, draw/present true, retry 0이다.
19. **Scene catalog가 중복되거나 캡처에서 누락되는가?** FFI source test는 9개 고정 고유 ID를 확인하고, 새 JS boundary test는 두 ID의 capture/FFI/runtime/Android/iOS route와 총 626개 catalog를 검증한다.
20. **일반 빌드가 고정 fixture payload를 끌어오는가?** Product archive에서 두 신규 ID와 C11.4 host export가 각각 0개, fixture positive control에서는 각각 2개였다. 정적 archive 범위만 주장한다.

처음 두 경로 검사에서 발견한 allowlist 불일치를 수정했고, scene count를 626으로 갱신한 뒤 구조 검사는 24/24 통과했다. 이 검토는 glyph raster/pixel, 실기기, Android hardware GPU, iOS compositor 이후 화면, 일반 CSS admission, WPT 적합성 또는 전체 C11.4를 검증하지 않는다.
