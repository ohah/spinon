# C11.4 `line-break: loose` 구현 PR diff 실패 관점 검토 30개

## 검토 기준

계획 diff와 구현/runtime 20개 검토를 반복 집계하지 않았다. 코드, simulator harness, evidence 및 상태 문서가 최종 PR diff에서 일으킬 수 있는 별도 실패를 한 항목씩 확인했다. 수정 사항이 있으면 영향받은 구현/runtime 관점도 다시 확인한다.

| # | PR diff 실패 관점 | 확인과 판정 |
| ---: | --- | --- |
| 1 | 좁은 opt-in 작업이 일반 CSS 지원으로 설명되는가 | Rust runtime, 내부 계약, plan, status와 README가 제한 fixture로 명시하며 일반 author admission을 제외한다. |
| 2 | C11.4/C11 checkbox나 완료 수가 부당하게 증가하는가 | 공식 상태는 미완료이며 `72/213`을 유지한다. |
| 3 | 출시 전 내부 interface version이 오르는가 | `spec/internal/0059-c11-grid-layout.md`는 `0.1.0`을 유지한다. |
| 4 | 기존 normal/strict 행렬이나 증거가 덮이는가 | 기존 expected JSON·runner 기본 variant·evidence directory는 그대로고 신규 variant는 별도 directory를 쓴다. |
| 5 | 계획 PR #961이 구현 PR로 잘못 서술되는가 | 계획 PR link/SHA와 구현 후보 기록이 분리되어 있다. |
| 6 | 기준 구현과 사전 기대값이 실행 뒤에 정해지는가 | 기존 Chrome 156 precomparison과 독립 기대값 JSON을 사전 oracle로 참조한다. |
| 7 | 신규 scene 수가 설명/코드/expected JSON 사이에서 다른가 | 4 신규 + 5 회귀 = 9로 runtime, FFI, Android, iOS, JSON과 test가 동일하다. |
| 8 | 일본어 loose 결과에만 의존해 locale 동작을 과장하는가 | English loose와 Japanese normal prefix control을 함께 넣어 20px 대비를 고정했다. |
| 9 | fullwidth prefix가 일반 currency/dollar 처리로 오해되는가 | source는 정확한 U+FF04 `＄`이고 fixture text와 source trace를 비교한다. |
| 10 | glyph advance를 OS font fallback에 맡기는가 | 고정 Noto Sans KR resource/hash와 test font registry를 유지한다. |
| 11 | max-content를 min-content 결과에서 유추해 잘못 고정하는가 | 별도 Japanese loose max-content scene은 30px/10px이다. |
| 12 | row height, line count와 line advance가 서로 모순되는가 | Rust/runtime/app expectation/JSON/trace는 10px line-height와 2·3 line geometry를 대조한다. |
| 13 | English scene에서 locale이나 computed keyword가 누락되는가 | `lang=en`, `Loose`, `文＄文`이 platform trace와 JSON에서 확인된다. |
| 14 | normal/strict fallback의 회귀가 새 code로 숨는가 | 기존 다섯 scene이 신규 matrix와 각 simulator build에서 재실행된다. |
| 15 | `LineBreakMode::Loose`가 enum default나 public CSS parser에 노출되는가 | default는 Strict로 유지되고 새 `Loose` input은 existing fixture-test route에서만 만들어진다. |
| 16 | FFI generator가 caller-provided source text를 허용하는가 | scene allowlist가 source text를 선택하고 exact generator test는 direct text node 하나를 요구한다. |
| 17 | unknown scene이나 invalid locale/mode가 부분 layout을 만들 수 있는가 | allowlist, direct owner locale, exact mode/text mismatch rejection checks를 유지한다. |
| 18 | Android와 iOS가 서로 다른 line-break enum을 전달하는가 | Java/Swift mode resolver와 4 scene expectation을 비교했고 양 simulator 9/9다. |
| 19 | Android expected line advances가 source order와 다른가 | 첫 실행에서 `[20,10]` 오류를 발견해 `[10,20]`으로 수정했고 실패 원본을 보존했다. |
| 20 | 기존 capture 명령에 새 인자를 요구해 사용법이 깨지는가 | variant 기본값은 `baseline`; 기존 두 인자 호출을 유지하고 optional `loose`만 추가한다. |
| 21 | 임의 variant 문자열이 기본 evidence에 써지는가 | variant를 `baseline|loose`로 제한하고 각각 고정 evidence root를 선택한다. |
| 22 | candidate 캡처가 merged baseline evidence를 수정하는가 | loose expected JSON, logs, screenshots, matrices는 새 directory 아래 생성된다. |
| 23 | ADB에 다른 기기/실기기를 대상으로 실행하는가 | runner는 명시 serial과 API 37 ARM64 AVD `qemu=1`을 확인한다. |
| 24 | iOS가 다른 runtime/model/booted device를 대상으로 실행하는가 | runner는 명시 UDID, iOS 26.2 boot state, `iPhone18,1`을 검사한다. |
| 25 | stale build가 설치된 채 성공으로 기록되는가 | Android/iOS built-vs-installed artifact SHA-256이 각각 일치한다. |
| 26 | truncated runtime trace를 정상 결과로 오인하는가 | trace chunk index·total과 JSON을 검사하고 최종 track, intrinsic source, frame, paragraph style을 대조한다. |
| 27 | iOS superseded run을 숨기거나 무제한 재시도하는가 | startup `status=-12`만 한 번 재시도하고 원본 log와 attempt table을 보존한다. |
| 28 | 앱 화면 screenshot을 glyph/pixel proof로 설명하는가 | README와 contract는 route/status 캡처이며 glyph, hardware GPU, drawable 및 compositor pixels는 미검증이라고 적는다. |
| 29 | feature fixture 코드가 default FFI archive에 포함되는가 | default/positive-control archive에서 scene string, host symbol, font bytes가 각각 없음/있음을 확인한다. |
| 30 | 주변 문서 검증을 위해 기존 static assertion을 과도하게 약화하는가 | boundary test의 font-size assertion만 실제 multiline call·indent field로 맞췄고 전체 boundary suite 30/30이 통과한다. |

## 결론

30개 diff 관점에서 병합을 막는 미해결 항목을 찾지 못했다. PR check rollup 상태는 GitHub에서 별도 확인하고, check run이 보고되지 않으면 성공으로 기록하지 않는다. 일반 CSS line-break, WPT, glyph painting, 실기기/hardware GPU, iOS compositor 이후 pixels 및 전체 C11.4/C11 지원은 포함하지 않는다.
