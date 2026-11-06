# C11.4 normal whitespace implementation PR diff review — 30 failure perspectives

검토 대상은 `origin/main`에서 현재 구현 후보까지의 실제 변경이다. 범위에는 selector, style-to-layout/runtime 회귀, static Root HTML fixture, app DOM assertion, 시뮬레이터 runner와 `pass01`~`pass05` 원본 evidence가 포함된다. 이전 plan PR review를 재사용하지 않았다.

| # | PR diff 실패 관점 | 확인 근거 | 판정 |
| --- | --- | --- | --- |
| 1 | ASCII SPACE 외 문자를 모두 허용 separator로 확장 | selector의 split 집합이 SPACE/TAB/LF/CR 네 문자로 제한된다. | PASS |
| 2 | Rust `split`이 비ASCII grapheme을 ASCII token으로 바꿈 | `A` 외 token은 전체 token equality에서 거부되며 NBSP/NEL/EM SPACE controls가 있다. | PASS |
| 3 | 여러 공백이 빈 token을 만들어 admission을 막음 | 빈 token은 건너뛰고 최소 한 `A` token을 요구한다. | PASS |
| 4 | empty string 또는 whitespace-only source를 새로 허용 | `has_token` 조건과 negative tests가 둘 다 거부한다. | PASS |
| 5 | `AA`가 두 개 `A` token으로 잘못 분해 | 허용 separator로 나뉘지 않아 단일 불일치 token으로 reject된다. | PASS |
| 6 | `B` 또는 Unicode token이 기존 source처럼 허용 | style unit 및 RuntimeSession invalid controls가 전체 계산을 거부한다. | PASS |
| 7 | form feed/vertical tab 같은 non-collapsible controls가 허용 | U+000C/U+000B, NEL, NBSP와 U+2003 negative matrix를 확인했다. | PASS |
| 8 | selector 내부에서 trim/replace를 해 raw source를 잃음 | production 변경은 admission predicate만 바꾸며 source text를 수정하지 않는다. | PASS |
| 9 | CR character reference와 다음 LF가 한 code point로 접힘 | Android/iOS app check와 feature diagnostics는 raw CRLF와 parsed source를 따로 보존한다. | PASS |
| 10 | 24 scene count와 source variants가 기대 matrix와 어긋남 | 고정 expected JSON의 6 variants × 2 columns × 2 orders를 읽고 개수 24를 확인한다. | PASS |
| 11 | forward/reverse setup이 실제 DOM item 순서를 바꾸지 않음 | Rust fixture builder가 두 item HTML을 바꾸고 test는 painted NodeId 순서를 비교한다. | PASS |
| 12 | Grid columns 변경이 unintended style replacement를 건드림 | `grid-template-columns: min-content` 한 선언만 교체하며 expected values는 Chrome 156 precomparison에서 왔다. | PASS |
| 13 | min-content row에서 wrapping과 row height가 불일치 | six line / 60px long row와 source-order-dependent row list를 대조한다. | PASS |
| 14 | auto column에서 40px wrapping result가 달라짐 | two line / 20px long row, 10px short row, 40px used column을 확인한다. | PASS |
| 15 | invalid second item이 valid first item의 partial output을 남김 | runtime negative tests는 `completed.is_none()`과 contextual `grid-auto-rows` error를 요구한다. | PASS |
| 16 | source text NodeId와 item owner NodeId를 혼동 | feature diagnostics는 source NodeId가 owner와 다르고 text-layout snapshot의 revision이 맞는지 확인한다. | PASS |
| 17 | document/style/render revision이 scene 간 섞임 | runtime test는 각 `RuntimeTextSnapshot` source revision을 같은 completion key와 비교한다. | PASS |
| 18 | text draw command가 background/next item보다 앞으로 이동 | per-item box command < text command < 다음 item box command invariant를 검사한다. | PASS |
| 19 | 마지막 fixed control이 Grid 40px boundary에서 잘림 | Rust frame assertion은 bottom > 40px이며 pass05 screenshot은 overflow를 보여준다. Pixel readback은 주장하지 않는다. | PASS |
| 20 | 새 runtime fixture가 opt-in FFI/test profile을 타고 default route를 검증했다고 오인 | Simulator runner는 `c11` fixture flags off를 기록하고 default `index.html` Root HTML route를 사용한다. | PASS |
| 21 | Android APK가 다른 HTML/JS asset을 실행 | APK asset hash가 fixture/source hash와 일치하며 pass05 log는 layout/draw/app eval을 기록한다. | PASS |
| 22 | iOS app bundle에 fixture copy가 빠지거나 오래된 asset 실행 | iOS app bundle hash가 fixture와 같고 app log에서 layout/draw/eval을 확인했다. | PASS |
| 23 | Android 실기기에 명령이 잘못 전달 | 모든 adb 명령은 `-s emulator-5554`를 고정하며 Samsung serial은 evidence에 사용하지 않았다고 기록한다. | PASS |
| 24 | iOS 다른 booted simulator에 설치 | 모든 `simctl` 명령은 iPhone 17 Pro UDID `9DCCE836-7659-4914-B6C1-13F54BFFA9DB`로 고정한다. | PASS |
| 25 | runner가 기존 앱 자산을 덮고 복구하지 않음 | pass01~pass05 각각 bootstrap 및 iOS bundle `cmp` restoration이 PASS다. | PASS |
| 26 | failed harness attempt를 삭제하거나 성공으로 표시 | pass01~pass04의 원본 logs/screenshots/environment 파일을 보존하고 원인과 성공 범위를 README에 분리했다. | PASS |
| 27 | simulator screenshot을 pixel parity로 해석 | README는 visual smoke라고 명시하며 PixelCopy/Metal readback이나 compositor comparison을 주장하지 않는다. | PASS |
| 28 | fixture/runner가 일반 CSS, public API 또는 WPT support를 뜻함 | 계획과 내부 계약은 허용 grammar 및 reject 경계를 고정하고, README는 WPT·public CSS·arbitrary text 지원을 명시적으로 제외한다. | PASS |
| 29 | baseline Clippy 오류를 숨기거나 전체 green으로 서술 | feature-enabled runtime Clippy의 두 unchanged baseline 오류를 기록하고 command를 passed라 하지 않는다. Non-feature runtime Clippy와 scoped style Clippy는 실제 통과다. | PASS |
| 30 | 이 제한 slice를 C11/C11.4 공식 완료로 승격 | PR 변경은 `spec/STATUS.md` 체크나 완료 수를 수정하지 않는다. 공식 집계 `78/214`는 그대로다. | PASS |

PR check rollup은 생성 후 GitHub에서 다시 확인한다. Local Rust/Simulator evidence는 CI 상태를 대신하지 않는다.
