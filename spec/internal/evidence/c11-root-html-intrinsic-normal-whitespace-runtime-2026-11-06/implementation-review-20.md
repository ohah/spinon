# C11.4 normal whitespace implementation review — 20 failure perspectives

대상은 `origin/main` 위의 제한 Root HTML intrinsic-row 변경과 `pass05` Android/iOS Simulator 실행이다. 각 관점은 계획 검토와 겹치지 않게 입력 grammar, source identity, track sizing, publish 경계 또는 native 실행 실패 경로를 확인한다.

| # | 실패 관점 | 확인 결과 | 판정 |
| --- | --- | --- | --- |
| 1 | 허용 집합 밖의 공백 문자를 실수로 separator로 취급 | SPACE/TAB/LF/CR만 tokenizer에 전달하며 NBSP·NEL·EM SPACE 등은 거부한다. | PASS |
| 2 | 여러 separator를 빈 token 오류로 거부 | 빈 token을 건너뛰고 유효한 `A` token은 그대로 검사한다. 반복·혼합 공백 24-scene matrix가 통과했다. | PASS |
| 3 | leading/trailing whitespace 때문에 유효 source를 거부 | 앞뒤의 허용 공백을 받아들이며 원문을 trim하지 않는다. | PASS |
| 4 | empty source를 유효 text item으로 승격 | `has_token`이 false여서 빈 source는 거부된다. | PASS |
| 5 | whitespace-only source가 0폭 contribution으로 새 admission | `has_token`이 false여서 whitespace-only source는 거부되고 partial layout은 publish되지 않는다. | PASS |
| 6 | `AA`, `B`, full-width `Ａ`처럼 token이 아닌 문자를 허용 | token은 정확히 ASCII `A`와 일치해야 하며 unit/RuntimeSession negative controls가 이를 확인한다. | PASS |
| 7 | UTF-8 split 경계가 비ASCII token을 잘못 자름 | Rust `str::split`은 지정한 ASCII `char` 경계에서만 나누며 비ASCII token 전체가 불일치로 남는다. | PASS |
| 8 | selector가 CSS whitespace collapse를 미리 수행해 원문을 손실 | admission은 source를 바꾸지 않는다. feature-enabled diagnostics에서 raw parsed source가 보존된다. | PASS |
| 9 | CR·LF가 한 줄바꿈으로 오인되어 text source가 달라짐 | Rust source trace와 앱 DOM assertion을 분리해 확인했다. CR character reference와 다음 LF는 앱의 `textContent`에서 CRLF로 유지된다. | PASS |
| 10 | 여러 text item이 하나의 anonymous source로 합쳐짐 | 24개 scene마다 각 direct item의 text source와 text owner NodeId가 별도로 보존된다. | PASS |
| 11 | text item이 source order와 무관한 row contribution을 사용 | forward/reverse 24-scene 조합에서 item ID, order, row height, contribution을 각각 확인한다. | PASS |
| 12 | `min-content` column에서 각 token이 한 줄로 wrap되지 않음 | 6.03125px Chrome reference, six-line long text, `[10,60,10,34]` row model을 확인한다. | PASS |
| 13 | `auto` column에서 long text의 two-line wrap가 달라짐 | 40px used column, two-line long text, `[10,20,10,34]` row model을 확인한다. | PASS |
| 14 | 두 columns의 supported scope 밖 조합이 암묵적으로 admission | 24-scene matrix는 `min-content`/`auto`만 사용하고 기존 profile guard는 그대로 유지된다. | PASS |
| 15 | text contribution과 grid track trace가 서로 다른 결과를 표시 | feature-enabled test에서 각 item의 min-content contribution, used rows, used column, pass axis를 대조한다. | PASS |
| 16 | text source NodeId가 painted item NodeId와 동일하다고 잘못 가정 | tests는 source text NodeId와 owner NodeId가 다름을 확인하고 owner identity를 유지한다. | PASS |
| 17 | document/style/layout revision이 stale snapshot과 섞임 | text paint source revision과 completion key, diagnostics revision을 24개 모든 scene에서 비교한다. | PASS |
| 18 | text paint가 배경 또는 다음 Grid item 뒤로 재정렬 | runtime snapshot의 box/text command order를 item source order와 비교한다. | PASS |
| 19 | Grid 경계가 마지막 fixed item의 overflow를 잘라냄 | last item의 y/h 및 bottom > 40px invariant를 Rust tests와 두 Simulator 화면에서 확인한다. 화면은 visual smoke로만 취급한다. | PASS |
| 20 | 잘못된 second text source 뒤에 앞 item의 frame/paint가 남음 | 8개 invalid runtime 입력은 contextual `grid-auto-rows` error를 내고 completed layout을 publish하지 않는다. Android/iOS 성공 장면은 기본 route와 fixture flags off로 실행했다. | PASS |

검증: style-to-layout focused tests 9/9, style Clippy, RuntimeSession focused tests 13/13, fixture-diagnostics tests 2/2, runtime Clippy, rustfmt, Android API 37 ARM64 AVD, iPhone 17 Pro iOS 26.2 Simulator. Feature-enabled runtime Clippy는 변경 없는 기준선의 `anonymous_text_hyphens.rs:117` 및 `anonymous_text_text_transform.rs:507` 두 `too_many_arguments` 오류로 실패하므로 성공 검사로 세지 않는다. Simulator screenshots는 PixelCopy/Metal readback이 아니다.
