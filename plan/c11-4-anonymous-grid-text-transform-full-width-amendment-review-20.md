# `text-transform: full-width` 계획 보완 실패 관점 검토 20개

대상은 [계획](./c11-4-anonymous-grid-text-transform-full-width.md)의 Parley line-breaking 보완과 [실패 관측](../spec/internal/evidence/c11-4-anonymous-grid-text-transform-full-width-precomparison-2026-11-02/implementation-discovery.md)이다. 원래 계획 검토와 다른 관점에서 내부 accommodation의 적용 경계, CSS 계약 보존, 비교 근거를 확인했다.

| # | 실패 관점 | 판정·차단 |
|---:|---|---|
| 1 | Parley workaround가 computed CSS `word-break`를 `keep-all`로 바꿈 | computed snapshot은 `normal`로 유지하고 내부 builder setting만 제한한다. |
| 2 | normal style 검증 전에 workaround가 적용됨 | runtime adapter가 exact owner와 computed `normal`을 먼저 검증한 뒤 text engine 경계에서만 적용한다. |
| 3 | 일반 text input에 FullWidth mode를 사용할 수 있음 | mode, feature, locale, source allowlist를 normalize 전에 검사한다. |
| 4 | 다른 Unicode fullwidth 문자도 내부 KeepAll 영향을 받음 | 현재 지원 source 문자열 두 개만 인정하며 나머지는 reject한다. |
| 5 | source가 인접 Text node로 쪼개짐 | text-layout profile은 `sources.len()==1`을 요구하고 runtime scene도 단일 node다. |
| 6 | `AB 12`와 `AB   12` 이외 content가 부분 변환됨 | 완전 일치 source allowlist만 받아 변환 일부 적용을 금지한다. |
| 7 | U+0020 collapse가 변환 전 수행되지 않음 | original repeated-space scene은 collapse→U+3000 순서를 검사한다. |
| 8 | KeepAll이 U+3000 뒤 줄바꿈도 막음 | 두 word 사이 break가 남아 두 줄·20px min-content를 확인한다. |
| 9 | KeepAll이 ASCII source보다 변환 결과만 대상으로 하지 않음 | internal style mapping은 transform mode 확인 뒤 transformed normalized buffer에 적용한다. |
| 10 | `AB` 글자 또는 `12` 숫자 안에서 여전히 break 가능 | 20px min-content와 두 줄을 고정 기준으로 둔다. |
| 11 | max-content width가 workaround 때문에 변함 | 50px max-content와 한 줄을 따로 검증한다. |
| 12 | source UTF-16 range와 output UTF-8 range가 섞임 | output 각 fullwidth scalar의 byte range와 원본 1-unit UTF-16 range를 대조한다. |
| 13 | repeated-space source ranges가 사라짐 | 세 source U+0020 mapping이 같은 output U+3000 byte range를 공유하는지 확인한다. |
| 14 | layout result가 internal KeepAll을 computed property처럼 노출 | result snapshot의 `word_break` field가 input computed normal을 보존하는지 확인한다. |
| 15 | browser target 결과를 product compatibility로 확대 | Chrome computed `text-transform:none`은 negative control로 남긴다. |
| 16 | candidate line count를 CSS transform support 증거로 오해 | README가 literal candidate wrapping만 관측값으로 분류한다. |
| 17 | feature-off path가 KeepAll 또는 transform을 실행 | FullWidth source profile은 opt-in feature 없이 fail-closed한다. |
| 18 | Internal KeepAll marker가 default artifact에 노출 | 신규 FFI scene ID와 source marker는 기존 fixture-only artifact gate를 사용한다. |
| 19 | Parley mismatch가 fixed tests 밖에 남음 | transform-none, uppercase, lowercase, capitalize 기존 tests를 regression gate로 유지한다. |
| 20 | workaround가 계획의 visible CSS contract로 승인됨 | 계획은 이를 adapter detail로 한정하고 일반 `word-break:keep-all` 지원을 제외한다. |

구현 단계에서는 이 계획 검토를 복사하지 않고 실제 engine, runtime snapshot, Android/iOS scene 결과를 새 관점으로 검토한다.
