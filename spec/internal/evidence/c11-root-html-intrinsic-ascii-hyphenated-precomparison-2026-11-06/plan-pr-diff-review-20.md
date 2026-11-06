# ASCII 하이픈 단어 계획 PR diff 적대 검토 20개

대상은 이 계획 PR의 staged diff 전체다: 계획, 20개 계획 검토, 상태 대장, C11.4 계획 인덱스, 공개 roadmap 후보 항목, Chrome 157 precomparison fixture와 관측·기대값·hash evidence. 구현 코드는 포함하지 않는다.

| # | 실패 관점 | 판정 |
|---:|---|---|
| 1 | 계획 후보를 구현 완료로 읽게 되는 상태 문구 | 계획·상태 대장·공개 roadmap 모두 구현 미시작을 명시한다. |
| 2 | C11/C11.4 checkbox가 후보 추가만으로 체크됨 | 공식 checkbox는 변경하지 않았다. |
| 3 | 78/214 집계가 후보를 완료로 이중 계산 | 계획과 roadmap이 공식 `78/214` 유지라고 밝힌다. |
| 4 | 유효한 hyphen grammar가 ASCII 범위를 벗어남 | `[A-Za-z0-9]+(?:-[A-Za-z0-9]+)*`로 U+002D만 허용한다. |
| 5 | 선행·후행·연속 hyphen이 비어 있는 segment를 통과 | `-A`, `A-`, `A--B` rejection을 acceptance에 고정했다. |
| 6 | Unicode lookalike나 부호가 normalization을 통해 허용됨 | U+2010·U+2212 controls와 다른 punctuation rejection을 명시했다. |
| 7 | CSS Text의 숫자 앞 hyphen guidance를 잘못 일반화 | CSS Text 3 CRD §5.2를 UA 상호운용 guidance로 표기하고 Chrome 관측을 구체적 oracle로 구분했다. |
| 8 | Grid 기대값이 독립 계산 없이 browser used track을 복사 | Canvas advance 기반 derive script가 CSSOM used track을 읽지 않으며 9 cases를 Chrome과 비교했다. |
| 9 | min-content에서 hyphen을 잘못 빼거나 segment를 잘못 나눔 | `A-A`, `12-3`, `A1-BB2`와 break segment·line 위치를 고정했다. |
| 10 | max-content·auto·fixed track을 같은 값으로 잘못 취급 | matrix에 세 sizing mode와 fixed 20px control을 따로 둔다. |
| 11 | collapsible whitespace 처리로 원문 text를 파괴 | source preservation과 기존 SPACE/TAB/LF/CR gate를 유지한다. |
| 12 | reverse DOM order에서 frame이나 contribution owner가 섞임 | reverse-order scene, item frame과 trace owner 확인을 요구한다. |
| 13 | negative style admission이 이전 revision frame을 새 결과에 섞음 | rejection atomicity와 revision identity를 acceptance에 포함했다. |
| 14 | fixture가 앱 runtime을 거치지 않아 product path를 과장 | Android API 37 ARM64 AVD와 iOS 26.2 Simulator의 기본 Root HTML route를 요구한다. |
| 15 | screenshot을 hardware/device/pixel parity 증거로 오해 | draw/present smoke와 optional PixelCopy/Metal 표본 주장을 분리한다. |
| 16 | WPT나 일반 CSS 지원을 Chrome comparison에서 추론 | WPT·일반 CSS admission·full C11.4/C11은 명시적으로 범위 밖이다. |
| 17 | fixture ABI가 default artifact에 유입됐다고 근거 없이 주장 | 새 adapter/FFI를 계획하지 않았고 경계가 변경될 때만 positive/negative artifact checks를 요구한다. |
| 18 | 상태 대장·계획 index·공개 roadmap 링크가 빠지거나 다른 범위를 가리킴 | 세 문서가 동일 plan/evidence 파일을 연결하고 diff에서 완료 수는 그대로다. |
| 19 | browser observation·독립 oracle 자산이 변조되거나 빠짐 | fixture, JSON, derivation, screenshot의 SHA-256 목록을 검증했다. |
| 20 | 저장소 문서 언어와 PR 검토 기록 기준이 어긋남 | 새 설명은 한국어이며 계획·PR diff 검토를 각각 별도 20개로 기록했다. |

판정: 계획 PR diff에서 blocking defect를 찾지 못했다. 사전 비교 독립 계산은 `derived and compared 9 cases`로 완료됐고 staged diff는 `git diff --cached --check`를 통과했다. 구현은 이 계획 PR이 병합된 뒤 별도 단계로 진행해야 한다.
