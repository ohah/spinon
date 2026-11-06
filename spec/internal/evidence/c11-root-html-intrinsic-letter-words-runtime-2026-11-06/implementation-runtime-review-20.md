# C11.4 Root HTML ASCII letter-word 구현·runtime 실패 관점 검토 20개

**대상:** `[A-Za-z]+` text gate, intrinsic sizing 및 RuntimeSession 경로, Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator 실행.

**범위:** 제한된 C11.4 후보의 변경 코드와 실행 동작. PR diff 검토와 C11.4 전체 완료 판정은 별도다.

| # | 서로 다른 실패 경로 | 실제 확인 및 판정 기준 | 결과 |
|---:|---|---|---|
| 1 | ASCII 판별 대신 Unicode alphabetic 문자를 허용한다 | `supports_text_source`는 UTF-8 token byte 각각에 `is_ascii_alphabetic`을 적용하고 전각 `Ａ`를 거부한다 | 통과 |
| 2 | 숫자가 글자 token에 붙어도 허용된다 | `A1`은 Style-to-layout, RuntimeSession 및 순수 grammar negative case에서 거부한다 | 통과 |
| 3 | 내부 punctuation이 단어를 나눈다고 오인된다 | `A-B`, `A!`, `A_`를 거부하고 기존 acceptance를 유지한다 | 통과 |
| 4 | 빈 문자열이나 whitespace-only 문자열이 `all()` 때문에 허용된다 | token 존재 여부를 별도 추적하고 empty·SPACE·TAB/LF/CR-only source를 거부한다 | 통과 |
| 5 | 연속·앞뒤 CSS collapsible whitespace 처리가 달라진다 | SPACE/TAB/LF/CR의 반복·혼합과 양끝 whitespace grammar case를 확인한다 | 통과 |
| 6 | 허용하지 않은 제어·Unicode 공백이 token 경계로 처리된다 | FF·VT·NBSP·NEL·em-space는 음성 case이며, 허용 separator를 정확히 네 ASCII 문자로 제한한다 | 통과 |
| 7 | 여러 text item 중 뒤쪽 잘못된 source가 먼저 계산된 item 일부를 게시한다 | invalid second-item RuntimeSession/Style-to-layout 회귀가 `B1`을 사용해 layout publication의 원자적 거부를 확인한다 | 통과 |
| 8 | 기존 `A` 전용 rejection 테스트가 더 넓은 범위 때문에 무의미해진다 | 오래된 `AA`/`A B` negative input을 새 계약의 숫자·구두점 거부로 교체하고 별도 positive word matrix를 추가했다 | 통과 |
| 9 | 실제 glyph advance 대신 token 길이로 min-content 폭을 계산한다 | 10개 고정 장면의 독립 monospace advance 기대값과 Grid track을 ±0.5 CSS px로 대조한다 | 통과 |
| 10 | `min-content`와 definite `auto` column을 같은 식으로 계산한다 | 각 열 mode를 별도 scene으로 실행하고 Chrome 관측에서 분리한 독립 columns 기대값과 비교한다 | 통과 |
| 11 | 자동 최소 크기보다 작은 container에서 unbreakable word가 잘못 clamp된다 | `AAAAAAAA`의 48.171875px track과 40px Grid 밖 item frame을 검사한다 | 통과 |
| 12 | 여러 단어의 normal wrapping이 줄 수나 implicit row contribution을 바꾼다 | `AA A A A`, `AB BBB A`, `aa bb ccc`의 각 독립 줄 수·row trace와 long/short item frame을 대조한다 | 통과 |
| 13 | DOM item 순서를 바꾸면 row contribution이 item identity와 분리된다 | min-content와 auto의 reverse case에서 long/short item y와 source order를 함께 검사한다 | 통과 |
| 14 | fixture 수정이 실제 10 scene에 적용되지 않거나 장면 하나가 빠진다 | RuntimeSession과 Style-to-layout 테스트가 같은 expected JSON을 읽고 각각 10개 case를 실행한다 | 통과 |
| 15 | text source를 trim·공백 정규화하거나 source node를 owner node로 덮어쓴다 | fixture-test trace에서 원문 source, 별도 Text node identity 및 item owner 관계를 검사한다 | 통과 |
| 16 | paint snapshot과 text layout이 서로 다른 document/style revision을 사용한다 | 각 text paint source generation, document revision, render-tree revision, style revision을 완료된 layout key와 비교한다 | 통과 |
| 17 | row geometry는 맞아도 text paint line/baseline 또는 paint owner가 유실된다 | line baseline 개수, origin, 두 text paint, 색으로 식별한 owner box를 10 scene에서 확인한다 | 통과 |
| 18 | Android 표본이 surface identity를 확인하지 않고 stale·다른 surface를 읽는다 | API 37 ARM64 AVD 실행 로그에서 viewport/density/surface mapping, current generation, presented draw 및 PixelCopy 결과를 대조한다 | 통과 |
| 19 | Android 표본이 Grid edge 안쪽이거나 text item 밖이라 overflow를 입증하지 못한다 | CSS `(42,19)`는 40px Grid 밖이며 Android PixelCopy `(111,51)`에서 item 배경 `[91,168,245,255]` 1/1 exact다 | 통과 |
| 20 | iOS 표본이 다른 좌표·색을 읽거나 기본 실행 파일에 검증 adapter가 포함된다 | iPhone 17 Pro Simulator Metal drawable `(127,58)`에서 같은 RGBA 1/1 exact이며 feature-on positive control과 default executable marker 부재를 함께 확인한다 | 통과 |

## 검토 결과

위 20개 구현/runtime 관점에서 수정할 결함을 찾지 못했다. Android AVD는 ANGLE/Vulkan/SwiftShader software 경로이고 iOS 표본은 compositor 제출 전 drawable이다. 지정 지점의 paint와 고정 10-scene Rust geometry matrix는 각각의 근거이며, 이 결과는 Android 실기기·hardware GPU·compositor 이후 parity·WPT·일반 CSS 지원 또는 C11.4 전체 완료를 뜻하지 않는다. 상세 입력·hash·화면·artifact boundary는 [실행 근거](./README.md)에 기록했다.
