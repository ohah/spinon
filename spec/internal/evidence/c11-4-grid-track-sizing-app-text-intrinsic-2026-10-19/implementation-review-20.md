# C11.4 Ahem text 앱 경로 구현 적대 검토 20개

**대상:** C11.4 opt-in direct-leaf text scene 추가, `c11_4_text::prepare`, text-enabled Grid projection, renderer text-node filtering, Android/iOS fixture assertion 및 iOS feature guard. 검토 기준은 해당 구현 diff와 저장된 Chrome 155·Android API 37 AVD·iOS 26.2 Simulator 실행 자료다.

| # | 실패 관점 | 확인 내용과 판정 |
|---:|---|---|
| 1 | fixture helper가 요구된 fixture module 밖에 있거나 default Cargo feature에 노출되는가 | 후속 검토에서 발견한 helper 위치를 `runtime_layout/fixtures/c11_4_text.rs`로 옮겼고 `fixtures` module과 helper 모두 opt-in feature 아래에 둔다. 기본 `spinon-ffi` artifact 검색에서 scene, item, text, Ahem marker가 없어 기본 빌드 오염은 관찰되지 않았다. |
| 2 | feature를 켰지만 text layout adapter만 빠지는가 | `spinon-runtime/Cargo.toml`이 layout feature를 전파하고, feature 전용 projection entry point가 같은 C11.4 profile만 받는다. feature suite와 app runs가 같은 profile 경로를 통과했다. |
| 3 | feature가 다른 style profile의 text 처리까지 바꾸는가 | `c11_4_text::prepare`는 `RuntimeBlockFlexGridTrackSizingV1`에서만 입력을 만들고 projection entry point도 profile을 재검사한다. Root HTML text fixture suite의 호출 분기는 유지된다. |
| 4 | 이전 viewport 또는 계산 결과 revision의 text style을 재사용하는가 | style snapshot과 현재 request snapshot에서 paragraph style을 만들고 viewport가 projection에서 일치하는지 검사한다. 저장된 app report의 input revision과 font registry revision이 함께 기록됐다. |
| 5 | OS에 설치된 Ahem 대체 폰트로 oracle을 우연히 맞추는가 | `TextLayoutEngine::new_without_system_fonts()`를 쓰고 repository fixture의 Ahem binary를 명시 등록한다. Chrome observation은 `fontReady:true`를 기록한다. |
| 6 | Ahem font registration 실패가 빈 크기로 조용히 대체되는가 | registration 오류는 구체적인 `c114_text_font_registration` layout failure로 변환된다. 성공 경로 로그에서 Ahem-derived width가 70/100이다. |
| 7 | 두 개 이상의 non-collapsible text node를 합쳐 잘못 측정하는가 | root subtree scan은 두 번째 non-collapsible node를 만나면 거부한다. 실제 scene은 text node 하나이고 HostDocument는 root·item·text 3 node다. |
| 8 | 공백 전용 text를 실제 glyph로 세어 layout을 바꾸는가 | HTML collapsible whitespace는 scan에서 제외한다. 선택한 text item은 child가 그 text node 하나여야 하므로 공백 sibling이 섞인 변형도 direct-leaf 검증에서 거부된다. |
| 9 | fixture ID가 맞아도 다른 문자열을 Ahem 입력으로 계산하는가 | payload는 정확히 `XX XXXXXXX`인지 확인한다. 불일치 payload는 `unsupported_c114_text_content`로 fail-closed한다. |
| 10 | nested descendant text를 직접 item text처럼 잘못 소유하는가 | text의 parent가 item이고 그 item만 root Grid의 직접 자식인지 검사한다. nested item이나 다른 owner는 거부된다. |
| 11 | item에 text 외 다른 child가 있어도 leaf로 취급하는가 | item element, 고정 ID, 단일 text child와 child 순서를 확인한다. element/추가 node가 존재하면 `invalid_c114_text_owner`로 종료한다. |
| 12 | 다른 Grid 안의 text가 root 계산에 끼어드는가 | 선택 item의 부모 Grid ID가 전달된 root와 같고 item이 root 직계 자식인지 두 번 검증한다. app DOM에는 root와 item이 직접 연결돼 있다. |
| 13 | Grid가 아닌 block에 track-sizing text 경로를 사용해도 통과하는가 | computed root display가 `grid`인지 확인한다. 비 Grid root는 unsupported style error로 종료한다. |
| 14 | fallback font 또는 line/wrap style 차이가 oracle width를 바꾸는가 | font family·size·weight·style, line-height, whitespace/wrap, word-break, direction, line-break, text-align, overflow-wrap, transform, spacing, indent, writing-mode를 computed longhand로 검증한다. ParagraphStyle은 현재 `line-break:auto`를 `Strict`로 표현하지만 이 ASCII 단어·공백 case의 줄바꿈에는 차이가 없다. punctuation/CJK에 결과를 일반화하지 않는 제한을 README에 추가했다. |
| 15 | app scene의 선언과 Ahem 계산이 다른 input을 쓰는가 | Android/iOS scene에 같은 문자열과 Ahem 선언을 고정했고 Chrome JSON에 앱 선언의 computed longhand를 저장했다. runtime report도 min/max 70/100을 반환했다. |
| 16 | min-content에서 공백 기준 longest word가 아닌 전체 문자열을 쓰는가 | Chrome 155에서 min-content column 70px, max-content width 100px을 관찰했고 Android/iOS trace와 Rust regression도 동일한 source values를 기록했다. |
| 17 | min-content width가 맞더라도 item block sizing이 틀리는가 | Chrome fixture와 app declarations 모두 item 70×20px이다. Simulator assertion도 양 축을 검사한다. 40px grid row의 stretch와 20px item을 별개로 기록했다. |
| 18 | tracking feature가 text를 layout하지 않고 빈 box만 그리는가 | 실제 app report에 text source·intrinsic measurements·track contributions/stages가 있으며 item frame 70×20이다. 다만 fixture renderer는 glyph paint를 건너뛰므로 화면은 box geometry만 증명한다고 제한을 README에 남겼다. |
| 19 | 새 Android/iOS expected frame은 assertion 뒤에만 검사해 trace를 잃는가 | 양 platform path는 trace chunks를 먼저 로그한 다음 status, root/item frame, text min/max를 검사한다. 세 simulator run 모두 summary와 trace를 보존했고 assertion error가 없다. |
| 20 | C11.4와 무관한 iOS C11.2.3 feature-off 빌드가 새 앱 route 때문에 깨지는가 | 첫 iOS 빌드에서 gated FFI symbol을 무조건 참조하는 기존 호출부를 재현했다. Objective-C++ create/eval/pixel call에 C11.2.3 gate를 맞추고 build-flag test를 보완했다. feature-off 재빌드와 simulator scene이 통과했다. |

**결과:** 현재 diff에서 미수정 blocker는 찾지 못했다. 검토 중 확인한 세 결함은 iOS C11.2.3 호출 feature guard 누락, 오래된 build-flag test expectation, helper의 fixtures module 경계 위반이며 모두 수정 후 다시 검증했다. 이번 검토로 확인한 제한은 단일 고정 Ahem normal-text scene과 box-only rendering이다. 일반 text/image matrix, 다른 shaping/wrapping, glyph paint 및 전체 C11.4는 별도 범위다.
