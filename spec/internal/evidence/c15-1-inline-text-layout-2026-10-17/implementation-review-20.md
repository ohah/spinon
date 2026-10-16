# C15.1 inline text layout 구현 적대 검토 20개 · 2026-10-17

**검토 범위:** `spinon-text-layout` font registry·normalization·Parley snapshot, `spinon-layout` HostDocument 수집·Taffy measure/final-layout 경계<br>
**근거:** 고정 Ahem/Roboto fixtures, Chrome 155 DOM 관찰, `spinon-text-layout`·`spinon-layout` focused tests와 전체 workspace tests<br>
**구분:** 계획 검토 20개를 재사용하지 않는다. PR 전체 diff 검토 30개도 별도 기록한다.

| # | 공격한 실패 경로 | 확인·판정 |
| ---: | --- | --- |
| 1 | registry에 기존 `Ahem`이 있을 때 Roboto bytes를 `Ahem`으로 등록해도 기존 family ID 때문에 성공 | 후보 collection에서 이번 등록의 반환 family ID를 확인하고서만 교체한다. 회귀 테스트가 잘못된 등록·revision 불변·요청 family 누락을 확인한다. 검토 중 발견해 수정. |
| 2 | 잘못된 font bytes를 등록한 뒤 부분 family/source가 live registry에 남음 | `register_font`는 clone candidate만 바꾸며 실패 시 collection과 `FontRegistryRevision`이 유지된다. malformed bytes negative control 통과. |
| 3 | `FontRegistryRevision == u64::MAX`에서 registry만 바뀌고 revision 증가가 실패 | `checked_add`를 등록 전에 계산한다. exhausted registry가 변하지 않는 negative control 통과. |
| 4 | fixture에 없는 글꼴이 OS 기본 글꼴로 조용히 대체됨 | `CollectionOptions.system_fonts=false`; `System Sans` 입력은 `MissingFontFamily`로 종료한다. fallback 성공 경로 없음. |
| 5 | 빈 family, 0·무한대 font size, NaN·음수 line-height가 Parley 입력으로 전달 | `ParagraphStyle::validate`와 전용 negative matrix가 각 property의 `InvalidStyle`을 확인한다. 통과. |
| 6 | definite width 0을 auto/indefinite로 오인하거나 문자를 0폭으로 잘라냄 | Chrome은 0px parent 안에서 Ahem `X`를 10px advance로 유지한다. Rust snapshot도 0px width와 10px advance/fragment를 보존한다. 통과. |
| 7 | `-0.0`와 `+0.0`이 measure cache와 final-layout key에서 달라져 snapshot 유실 | text engine·Taffy measure·최종 content width가 zero를 `+0.0`으로 정규화한다. Taffy `-0.0` 회귀도 snapshot bit를 검사한다. 통과. |
| 8 | 음수, NaN 또는 무한 width가 Parley/Taffy에 도달 | finite/nonnegative preflight가 `InvalidWidth`를 돌려준다. negative, NaN, positive infinity controls 통과; 음의 무한대는 동일한 `is_finite` guard에서 거부된다. |
| 9 | Taffy의 첫 measure 결과를 다른 최종 content width에 재사용 | measure 결과를 width bit key별로 보관하고 최종 width가 일치하지 않으면 실패한다. 30px wrap, 15px long word, 30px nowrap에서 width·auto-height·revision 일치. 통과. |
| 10 | text engine 부재 또는 paragraph key와 실제 Block owner가 달라도 빈 크기로 성공 | `MissingTextLayoutEngine` 및 owner 불일치 오류를 먼저 반환한다. 부분 frame map을 반환하지 않는다. 통과. |
| 11 | Normal/NoWrap 공백 collapse나 supplementary scalar가 UTF-16 owner를 잃는다 | 여러 source 공백의 원본 UTF-16 range와 inline ancestor를 유지하며, 별도 `😀` normalizer test가 UTF-16 `0..2`와 UTF-8 `0..4`를 확인한다. 통과. |
| 12 | `pre-line`의 CRLF를 공백으로 합치거나 CR/LF 중 한 code unit의 source를 버림 | 정규화된 단일 newline이 두 원본 UTF-16 range를 모두 가리키며 2 line을 만든다. 통과. |
| 13 | 좁은 parent 폭에 맞춰 unbreakable word를 자르거나 advance를 box width로 clamp | 15px 폭에서도 Ahem `XX`가 한 line·20px advance를 유지하고 0px 경계도 보존한다. geometry는 ink/clip 증거로 확대하지 않는다. 통과. |
| 14 | `break-all`, `nowrap`, `pre-line`이 기본 line-break 동작으로 뭉개짐 | Ahem `XXXXX`의 20/20/10 sequence, nowrap 한 줄 overflow, pre-line 줄 수를 고정 사례와 대조한다. 통과. |
| 15 | 같은 code-unit 수를 고정 폭으로 취급해 Roboto `iiii`와 `WWWW`가 같아짐 | Parley가 실제 고정 Roboto variable font를 shaping하고 `19.4531`/`70.9844` advance와 glyph ID를 보존한다. `office`도 고정 advance 비교. 통과. |
| 16 | 인접 text node나 span source 순서가 바뀌고 여러 줄 inline owner가 union rect 하나로 합쳐짐 | source node order, inline ancestor ownership, span의 line별 두 fragment rect를 각각 검사한다. 통과. |
| 17 | 빈 source/공백만 있는 paragraph에 가짜 line을 만들거나 고정 registry 없는 family로 성공 | 정규화 뒤 빈 문자열은 zero-length source mapping만 남기고 line/height 0을 반환한다. 없는 family는 `MissingFontFamily`. 통과. |
| 18 | generic `LayoutStyle::default()`의 `display:flex`를 plain inline span으로 평탄화 | 구현에서 기본 Flex가 허용되던 경계를 발견했다. 명시 `LayoutDisplay::Inline`과 나머지 기본 style만 허용하고 standalone Taffy Inline box는 거부한다. Flex/Block/Grid/FlowRoot·장식 회귀 통과. 검토 중 발견해 수정. |
| 19 | `display:none` subtree text가 출력에 섞이거나 숨은 element style 누락을 조용히 무시 | 숨은 text는 제외하고 source list에 나타나지 않는다. 숨은 element의 style 누락은 `MissingStyle`로 닫힌다. 통과. |
| 20 | nested/decorated/non-span inline 또는 unpaired surrogate가 일부만 버려지고 나머지 paragraph는 성공 | nested span·visible non-span·border 장식은 paragraph owner를 지목해 거부하고, invalid UTF-16은 source node를 지목한다. 통과. |

## 발견한 수정과 남은 증명 경계

- 등록 함수의 live collection 선변경과 기존 family에 대한 false-positive를 transactional candidate 등록으로 수정했다.
- span 수집기가 일반 `LayoutStyle::default()`의 Flex display를 plain inline으로 오인할 수 있어 source-only `LayoutDisplay::Inline`을 도입했다. 독립 layout box의 Inline 값은 입력 검증에서 거부한다.
- 0px definite width를 허용하고 signed zero key를 정규화했다. Chrome DOM 관찰과 geometry test는 overflow advance를 확인하며 실제 glyph ink나 부모 clip은 확인하지 않는다.
- 전체 workspace tests는 938 passed / 4 ignored, workspace Clippy와 Android/iOS Simulator target compile은 통과했다. 앱 text painter가 아직 이 API를 호출하지 않으므로 두 simulator의 앱 화면, border 바깥 pixel, clip 결과는 이 PR에서 검증하지 않았다.
