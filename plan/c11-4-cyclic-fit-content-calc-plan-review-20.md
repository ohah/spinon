# `fit-content(calc())` 계획 실패 관점 검토 20개

2026-10-22에 계획, 고정 입력, 독립 계산과 Chrome observation을 검토했다. 이 검토는 구현이나 runtime 결과를 검토한 것이 아니다.

| # | 실패 관점 | 확인 결과 |
| ---: | --- | --- |
| 1 | 계획이 `fit-content(calc())`와 직전 일반 `calc()` slice를 같은 문법 경계로 취급 | 이전 계획은 direct `calc()` track을, 이번 계획은 `fit-content()` argument 안의 calc를 구분한다. |
| 2 | percentage가 없는 definite case만 cyclic 테스트로 오해 | cyclic width `max-content`와 height `max-content` scene을 분리했다. |
| 3 | percentage basis가 intrinsic sizing 도중 사용돼 cyclic성을 놓침 | column root는 max-content width, row root는 max-content height로 고정했다. |
| 4 | 규범 기대값을 Chrome 결과에서 역산 | Ahem contributions와 CRD sizing 순서에서 기대값을 독립 계산하고 JSON에 따로 저장했다. |
| 5 | fit-content clamp 수식의 항 순서를 반대로 계산 | CRD §7.2.1의 `max(auto minimum, min(limit, max-content))`를 사용한다. |
| 6 | cyclic percentage 내부의 absolute length가 사라짐 | calc limit은 최종 basis에서 percentage와 px offset을 함께 계산한다: 29, 22, 24, 9px. |
| 7 | column의 intrinsic max-content와 min-content 경계가 구별되지 않음 | `aa aa`는 Ahem 10px 기준 min-content 20px, max-content 50px이고 cyclic limit은 29px이다. |
| 8 | row intrinsic basis가 text wrapping과 무관 | 20px column에서 `a a a a` 네 line, 40px intrinsic root height로 고정한다. |
| 9 | definite control이 cyclic case와 같은 basis를 사용 | definite control 두 개는 width 40px과 10px으로 limit 24px과 9px을 분리한다. |
| 10 | small-limit track이 item automatic minimum으로 9px 아래로 막힘 | 해당 scene은 `min-width:0`을 명시하고 Chrome에서 root/item/track 10/9/9px을 관찰했다. |
| 11 | root 밖 red sample이 root 내부의 item 배경으로 오인 | `(15,5)`는 10px root와 9px item 둘 다 밖이고 screenshot의 실제 RGB를 기록한다. |
| 12 | 샘플 좌표가 글리프나 root 위치와 함께 임의 이동 | 상대 기준 `(5,5)`, `(15,5)`, `(25,5)` control을 저장하고 root의 absolute y는 acceptance에서 제외한다. |
| 13 | 9px item의 overflow를 CSS border stroke 근거로 확대 | 계획은 text/item overflow 색 표본만 검사하고 border stroke를 명시적으로 제외한다. |
| 14 | HTML fixture에 다른 direct child나 inherited default가 sizing을 바꿈 | root, direct text item, Ahem font와 zero edge/gap만 허용한다. |
| 15 | font가 늦게 로드되어 예상 contribution이 달라짐 | Chrome observation에서 `document.fonts.status=loaded` 및 Ahem check true를 확인했다. |
| 16 | 브라우저 버전 또는 viewport가 결과 재현에 빠짐 | User-Agent, Chrome 156 build, viewport `1280×633`, DPR 1을 기록했다. |
| 17 | computed style만 통과하고 실제 root/item box는 다름 | 계획 acceptance는 used track list와 root/item frame을 함께 검사한다. |
| 18 | fixture scene을 임의 CSS 지원으로 읽음 | 네 scene ID만 허용하는 opt-in fixture-test profile이며 generic admission은 변경하지 않는다. |
| 19 | simulator surface 경계를 검증하면서 실기기·compositor까지 검증했다고 과장 | Android AVD, iOS Simulator, PixelCopy, compositor 전 Metal 경계를 명시하고 물리 device와 compositor 이후는 제외한다. |
| 20 | 계획 또는 후속 구현 통과로 C11.4 전체를 완료 처리 | 상위 checkbox와 `72/213`을 유지하며 이 계획은 구현 결과가 아니다. |

## 판정

고정 scene 네 개와 세 단계 기대값 경계가 분리되어 계획 작업을 시작할 수 있다. 구현 후에는 이 표와 다른 관점으로 별도의 20개 implementation/runtime 검토와 PR diff 30개 검토가 필요하다.
