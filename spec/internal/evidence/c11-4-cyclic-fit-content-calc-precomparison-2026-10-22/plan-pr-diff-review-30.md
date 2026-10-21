# `fit-content(calc())` 계획 PR 적대적 diff 검토 30개

검토 대상은 새 집중 계획, 상위 C11.4 계획의 후속 링크, Chrome 156 precomparison fixture·관측·기대값과 20개 계획 검토다. 구현 코드나 simulator 실행 결과를 포함하지 않는다는 경계를 확인했다.

| # | 실패 관점 | 확인 결과 |
| ---: | --- | --- |
| 1 | 계획 PR이 미구현 runtime을 완성으로 기록 | 계획 상태와 README가 runtime을 아직 실행하지 않았다고 밝힌다. |
| 2 | C11.4 상위 checkbox 또는 완료 수를 올림 | 문서에서 C11.4 미완료와 `72/213`을 유지한다. |
| 3 | 이미 merged #480과 범위가 중복됨 | #480은 direct `calc()` track이며 이 계획은 `fit-content()` argument 안 calc를 분리한다. |
| 4 | CRD draft를 규범 고정본으로 착각 | 2025-03-26 CRD URL을 계획과 evidence에 고정한다. |
| 5 | `<track-size>`가 calc로 된 fit-content 값을 허용하지 않는다고 오해 | §7.2.1 grammar의 `<length-percentage>` argument와 실제 Chrome parsing을 근거로 둔다. |
| 6 | cyclic percentage가 intrinsic root size에 재귀 적용 | cyclic column·row root를 각각 max-content로 고정하고 CRD의 두 단계 resolve를 분리한다. |
| 7 | fit-content clamp formula가 규범과 역순 | `max(auto minimum, min(limit, max-content))`을 사용한다. |
| 8 | calc의 absolute px offset이 누락 | 각 scene의 최종 limit 29·22·24·9px을 식과 함께 적었다. |
| 9 | Ahem column contribution이 기대값과 맞지 않음 | Ahem 10px `aa aa`의 min/max contribution 20/50px을 별도 설명한다. |
| 10 | row basis가 줄바꿈에 의존하지 않음 | 20px width의 네 10px line으로 intrinsic height 40px을 고정한다. |
| 11 | cyclic column 결과가 max-content intrinsic width를 모호하게 둠 | root 50px, final limit/track 29px, item 29px을 독립 표에 기록한다. |
| 12 | cyclic row 결과가 root size와 최종 track을 혼동 | root height 40px과 final row/item height 22px을 별도 기록한다. |
| 13 | definite 40px control이 cyclic basis를 재사용 | 40px basis와 24px limit control이 있다. |
| 14 | lower definite limit이 track floor를 검증하지 않음 | width 10px, `min-width:0`, limit/track 9px control이 포함된다. |
| 15 | 9px item frame으로 text clipping을 확인하지 않음 | Ahem 20px word의 10px root 바깥 표본을 후속 PixelCopy/Metal 관문에 포함한다. |
| 16 | overflow sample을 item 내부에서 취함 | root 기준 `(15,5)`는 10px root와 9px item 바깥이며 Chrome screenshot pixel이 `#e5484d`다. |
| 17 | 성공색만으로 실제 밖 영역을 과장 | inside `(5,5)`와 outside-glyph `(25,5)` controls도 함께 기록한다. |
| 18 | text overflow를 CSS border stroke 근거로 확대 | 계획과 evidence가 border stroke를 명시적으로 제외한다. |
| 19 | fixture font asset 누락 또는 늦은 load | Chrome observation은 Ahem loaded/check true를 기록하고 manifest는 사용한 Ahem font asset의 SHA-256을 보존한다. |
| 20 | browser version, viewport, DPR를 재현하지 못함 | HeadlessChrome 156, 1280×633 viewport와 DPR 1을 JSON에 기록한다. |
| 21 | scene ID와 CSS rule이 observation 사이에 다름 | 네 HTML IDs, CSS expressions, Chrome observation과 expectation JSON이 같은 이름을 쓴다. |
| 22 | absolute document y 위치가 acceptance를 불안정하게 함 | root-relative 좌표와 frame을 쓰고 flow-dependent y를 제외한다. |
| 23 | 여러 item·nested grid가 결과를 섞음 | root당 direct text item 하나와 단일 explicit track으로 제한한다. |
| 24 | border/gap/padding이 percentage basis를 바꿈 | fixture style은 margin·padding·border·gap을 0으로 고정한다. |
| 25 | precomparison 결과를 Spinon runtime 결과로 읽음 | README가 runtime·app·surface 비교를 이 단계에서 수행하지 않았다고 밝힌다. |
| 26 | simulator 범위가 물리 device로 잘못 표기 | Android AVD/iOS Simulator만 계획하고 실기기를 사용하지 않는다고 명시한다. |
| 27 | iOS Metal sample이 compositor 이후임을 암시 | 계획에서 iOS sample은 compositor 제출 전이라고 명시한다. |
| 28 | default product artifact에 fixture code가 들어간다고 약속 없이 변경 | 기본/opt-in archive 경계 검사는 후속 구현 acceptance로 분리했다. |
| 29 | plan review 20과 PR diff review 30을 합쳐 셈 | 계획은 별도 20개, 현재 계획 PR은 별도 30개 review 파일을 가진다. |
| 30 | 증거의 화면·JSON·fixture hash 또는 Agent Browser 종료 누락 | SHA manifest 검증, 화면·JSON, browser 종료 기록을 추가하고 전용 세션 종료 및 empty process scan을 남겼다. |

## 판정

계획 PR 범위는 고정 입력·독립 기대값·oracle evidence에 한정된다. 이 검토는 implementation/runtime 20개 실패 관점이나 후속 구현 PR의 별도 30개 diff 검토를 대신하지 않는다.
