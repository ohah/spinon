# C11.4 auto-fit + area 확장 계획 · 실패 관점 검토 20개

계획 [`c11-4-auto-fit-area-expansion.md`](./c11-4-auto-fit-area-expansion.md)을 Chrome 관측, CSS Grid 규범, cascade/projection/layout 경계와 기존 41-scene 수를 대조해 구현 전에 별도로 검토했다. 각 항목은 서로 다른 실패 경로를 대상으로 한다.

| # | 실패 관점 | 검토 및 처리 |
| --- | --- | --- |
| 1 | auto-fit repeat count를 area가 확장한 3개 track을 기준으로 계산한다. | §7.2.3.2의 auto-repeat count는 track list와 definite 52px content size, 20px track 및 4px gap으로 먼저 계산한다. 세 번째 area track은 이후 explicit grid를 늘리는 경우다. 계획은 2개 반복으로 고정했다. |
| 2 | 반복 수 계산에서 gap을 빠뜨려 세 개를 허용한다. | 3개는 `3×20 + 2×4 = 68px`라 52px에 맞지 않는다. fixture가 52px fixed content box와 4px gap을 사용하고 observation은 2개 repeat를 확인한다. |
| 3 | 모든 cell이 `.`인 template areas는 explicit dimension을 만들지 않는다고 가정한다. | template-areas 행·열 문자열 자체가 grid dimension을 정한다. `fit-fit-empty`는 named cell을 두지 않아도 3×3 area dimension이 반복 track보다 큰 상태를 시험한다. |
| 4 | area metadata 선언만으로 auto-fit track을 occupied 처리한다. | 규범상 empty는 그 안에 배치되거나 가로지르는 in-flow item이 없는 track이다. `fit-fit-empty`에서 반복 둘 다 0으로 접히는 기대를 분리했다. |
| 5 | area가 추가한 third track도 auto-fit 대상에 포함되어 collapse된다. | Auto-fit은 반복 component가 만든 track만 접는다. `grid-auto-*` 크기를 받은 area-added explicit track `[16]`/`[12]`은 `fit-fit-empty`에서도 유지되며 Chrome 관측이 일치한다. |
| 6 | area item이 세 번째 cell에 있어도 line index가 1-based/0-based 변환에서 한 칸 어긋난다. | `fit-fit-item`이 row 3·column 3의 hero에 배치되고 Chrome에서 접힌 repeat 뒤 상대 origin `[0,0]`에 `[16,12]`로 나온다. Rust scene은 동일 named-area placement를 유지한다. |
| 7 | 두 반복 사이 gutter만 접고 repeat 경계의 마지막 gutter는 남는다. | Empty auto-fit track 양옆 gutter collapse를 별도 확인해야 한다. `fit-fit-item`과 track arrays는 added track 시작을 origin으로 옮기므로 잔여 4px gutter가 frame을 밀면 실패한다. |
| 8 | 반복 track을 item이 가로지르는 경우에도 collapse한다. | `fit-fit-span`의 3×3 hero item은 두 repeated rows/columns를 모두 span한다. 네 반복 track은 각각 occupied여야 하며 Chrome used tracks `[20,20,16]`/`[20,20,12]`가 이를 확인한다. |
| 9 | auto-fill과 auto-fit을 한쪽씩 섞었는데 두 축 모두 같은 collapse 동작을 적용한다. | `fit-fill-item`과 `fill-fit-item`은 각각 한 축만 auto-fit으로 두고 상대 축을 auto-fill로 둔다. 축별 array와 scroll extent를 별도 assertion한다. |
| 10 | area-added track이 implicit sizing cycle 중 잘못된 `grid-auto-*` offset을 받는다. | 신규 track은 repeat 단위와 다른 고정 16px/12px을 쓰므로 repeat 값 복제 오류가 드러난다. 계획은 신규 grammar를 열지 않고 고정 auto track 및 expected list를 그대로 유지한다. |
| 11 | 한 축에만 area expansion이 있을 때 반대 축에서 남은 반복도 전부 접는다. | `fit-fit-column`에서는 item이 첫 row를 점유하므로 row tracks `[20,0]`이고, `fit-fit-row`에서는 column tracks `[20,0]`이다. 두 controls가 축 독립성을 고정한다. |
| 12 | fixed container 크기를 border box로 해석해 repeat count 또는 frame이 달라진다. | Fixture는 border 0, padding 0, `box-sizing:border-box`라 content box도 52px다. runtime root도 같은 zero-edge 조건을 요구한다. |
| 13 | scroll extent를 track content와 item overflow 중 하나만으로 판정한다. | span scene은 `[64,60]`; mixed controls는 `[52,60]`과 `[64,52]`다. 계획은 root size와 scroll extent를 따로 기록하고 frame과 사용 track을 함께 본다. |
| 14 | 정수 px의 expected frame에 허용오차를 둬 한 pixel 경계 오류를 숨긴다. | repeat, gap, auto-track과 root 모두 정수 CSS px라 exact 비교한다. subpixel tolerance가 필요한 intrinsic sizing은 제외했다. |
| 15 | Chrome observation에서 기대값을 역산해 잘못된 browser behavior를 규범으로 채택한다. | 규범 계산·Chrome CSSOM 관측을 문서와 JSON에서 분리한다. expected list는 먼저 CSS Grid §7.1/7.2.3.2/7.5에서 계산한다. |
| 16 | area metadata의 dimensions 또는 generated coordinates가 기존 상한을 넘는다. | 신규 fixture는 3×3이고 기존 `GRID_MAX_TRACKS`/metadata byte 제한 안이다. 계획은 limit 확대를 명시적으로 제외하고 경계 guard 회귀를 유지한다. |
| 17 | zero tracks만 있는 area, malformed area 또는 invalid metadata가 partial layout을 만든다. | 사전 장면은 유효한 CSS만 사용한다. 기존 malformed/metadata error controls를 별도로 유지하고 새 product code가 생길 때 partial frame/publication 부재를 검사한다. |
| 18 | non-target profile도 area+auto-repeat을 받아 지원 범위를 넓힌다. | cascade `allow_auto_repeat`과 projection profile gate를 유지한다. 오직 `RuntimeBlockFlexGridTrackSizingV1` 대상의 positive control을 더한다. |
| 19 | 새 scene이 default/product artifact에 들어가 fixture surface가 우연히 공개된다. | 기존 dedicated fixture-test feature의 default negative와 opt-in positive static archive 검사에 새 sentinel/entry symbol을 추가한다. 이 검사로 APK/IPA exclusion을 주장하지 않는다. |
| 20 | 신규 7 scene만 통과하고 직전 no-area/single-area/dual-area 조합이 깨진다. | exact matrix를 기존 12+13+9에 신규 7을 더한 41 scene으로 고정하고 Android AVD와 iOS Simulator에서 각각 같은 ID set을 요구한다. |

**판정:** 구현 전 계획 관문을 통과했다. 관측과 고정 입력은 [사전 비교](../spec/internal/evidence/c11-4-auto-fit-area-expansion-precomparison-2026-10-23/README.md)에 있다. 구현 완료나 플랫폼 통과를 의미하지 않는다.
