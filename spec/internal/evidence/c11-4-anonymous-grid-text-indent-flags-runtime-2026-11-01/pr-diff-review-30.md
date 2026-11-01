# C11.4 anonymous Grid `text-indent` flags PR diff 실패 관점 검토 30개

**범위:** `text-indent` flags 제한 opt-in slice의 최종 코드·계약 문서·fixture·시뮬레이터 근거. 별도 계획 20개와 구현/runtime 20개 검토를 재사용하지 않았다. 이 검토는 GitHub required checks, PR 병합 또는 전체 C11 완료를 대신하지 않는다.

| # | 실패 관점 | 확인 결과 |
| ---: | --- | --- |
| 1 | 제한 slice가 공식 C11 완료 수를 부당하게 올리는가 | `spec/STATUS.md`의 C11/C11.4 확인란과 `72/213`을 유지한다. |
| 2 | 내부 contract 버전이 임의로 올라가는가 | 프로젝트 규칙에 따라 `0059`를 `0.1.0`으로 유지한다. |
| 3 | 일반 author CSS가 지원된 것으로 읽히는가 | 계약은 고정 opt-in fixture 범위와 일반 CSS admission 제외를 명시한다. |
| 4 | 예상 밖 scene ID가 유사 문자열만으로 허용되는가 | 50개 정적 scene table과 unique-ID assertion만 사용하며 미등록 ID는 실패한다. |
| 5 | 잘못된 단위·flag 순서·중복 flag가 파서를 통과하는가 | parser negative assertions가 `%`, `-0px`, unknown, duplicate 및 역순 token을 거부한다. |
| 6 | 다른 element나 중복 inline 선언에서 authored fallback을 빌리는가 | root element만 읽고 duplicate `text-indent` declaration을 error로 처리한다. |
| 7 | UTF-16 이전 문자가 진단 열 위치를 바꾸는가 | 위치 계산은 UTF-16 column을 사용하며 emoji 앞 열의 단위 테스트가 있다. |
| 8 | Stylo의 다른 parser diagnostic이 flags 예외를 여는가 | fallback은 알려진 단일 메시지와 정확한 inline source·node·위치만 허용한다. |
| 9 | 진단이 없거나 두 개 이상이어도 fallback 되는가 | 진단 없는 fallback 및 다중 진단 negative assertions가 실패를 확인한다. |
| 10 | computed serialization과 authored 값이 달라도 scene이 게시되는가 | exact scene serialization·length·flags 비교가 mismatch에서 실패한다. |
| 11 | 이전 document/style snapshot의 source가 재사용되는가 | root, generation, document/render-tree revision 검사가 fallback 전후에 적용된다. |
| 12 | fixture host 또는 scene payload가 기본 제품 경계에 섞이는가 | opt-in feature 경계의 기본/positive-control archive scan은 각각 0/20·0/2 및 20/20·2/2다. |
| 13 | 새 ParagraphStyle field가 기존 호출자의 기본 동작을 바꾸는가 | flags 기본값은 둘 다 false이고 기존 positive 18 scene이 두 플랫폼에서 재검증됐다. |
| 14 | NaN/무한 indent가 line snapshot에 들어가는가 | paragraph validation과 line metric validation이 non-finite 값을 거부한다. |
| 15 | typed flags가 Parley에 뒤바뀌어 전달되는가 | `each_line`·`hanging`을 대응 `IndentOptions` field에 직접 매핑한다. |
| 16 | `each-line`이 soft-wrap continuation에 잘못 적용되는가 | 20 scene Chrome/Ahem oracle와 Parley line-origin assertion에서 soft-wrap case를 대조한다. |
| 17 | `each-line`이 forced break 뒤 줄을 빠뜨리는가 | `pre-line` 10 scene의 두 line indent, advance, height 및 track을 확인한다. |
| 18 | `hanging` 조합이 첫 줄/continuation 줄을 뒤집는가 | `hanging`, `each-line`, 결합 flag의 soft/forced cases를 독립 기대값과 대조한다. |
| 19 | min-content가 첫 줄에만 indent를 더하거나 두 번 더하는가 | 실제 각 line의 `metrics.indent`를 한 번씩 사용하고 Ahem min-content 및 최종 track을 검사한다. |
| 20 | max-content가 forced line 이후의 indent를 누락하는가 | 모든 formatted line을 순회하며 forced-break max-content geometry를 검사한다. |
| 21 | indent가 glyph advance 안팎에 중복 계상되는가 | `TextLine.advance`와 `TextLine.indent`를 분리해 trace하고 각각 대조한다. |
| 22 | 새 `pre-line` scene이 기존 whitespace profile을 바꾸는가 | 등록된 flags forced scenes에만 `pre-line`을 선택하고 soft/기존 scene은 기존 mode를 사용한다. |
| 23 | 기존 음수 indent가 flags 확장으로 회귀하는가 | 12개 negative와 18개 positive controls를 합친 행렬이 플랫폼별 30/30이다. |
| 24 | intrinsic sizing이 line 수와 Grid row/root frame을 어긋나게 하는가 | min/max height, rows track, item/root frame을 독립 기대값과 비교한다. |
| 25 | 독립 기준이 같은 구현의 출력만 재사용하는가 | CSS Text/Sizing/Grid 규칙 및 고정 Ahem advance 계산을 Chrome 156 관측과 별도로 기록한다. |
| 26 | Android adapter가 알 수 없는 ID나 다른 flag 조합을 수용하는가 | adapter allowlist, trace booleans, line indents와 geometry를 20개 flags scene 전부에서 검사한다. |
| 27 | iOS adapter가 Android와 다른 기대값 또는 mapping을 쓰는가 | 동일 expected-values 입력으로 20 flags·18 positive·30 negative/control scene을 실행한다. |
| 28 | layout summary만으로 surface 제출까지 통과 처리하는가 | runner는 동일 process의 draw/present marker, 예상 root frame과 양수 texture 크기를 요구한다. |
| 29 | 이전 process log나 superseded retry가 성공 scene처럼 집계되는가 | Android PID/iOS processIdentifier로 로그를 좁히고 최종 여섯 matrix는 retry 0을 기록한다. |
| 30 | 실행 화면이 glyph pixel 또는 실기기 증거로 과장되는가 | 계약·README는 검은 renderer surface를 명시하고 glyph pixels, hardware GPU, iOS compositor 및 전체 bundle 제외를 주장하지 않는다. |

## 리뷰 뒤 lint 발견과 수정

Feature-scoped Clippy가 새 test의 `needless_range_loop`, 두 fixture helper의 `too_many_arguments`, FFI test 모듈의 `FLAG_SCENES` import 누락을 추가로 드러냈다. 줄별 expected indent를 iterator로 순회하고 `TextIndentFlags`/`ComputedTextIndent` typed value로 인자를 묶었으며 test import를 고쳤다. 이후 text-layout/runtime/FFI의 `--all-targets` Clippy가 `-D warnings`로 통과했고 focused tests는 각각 1/1, 9/9, 2/2 통과했다. 해당 수정 뒤 두 앱을 다시 빌드해 여섯 simulator matrix 136/136 scene을 재실행했으며 각 row의 draw/present·surface·최종 build/install hash가 일치한다. 최종 run paths와 hashes는 [실행 README](./README.md)에 연결했다. 기본/fixture archive scanner도 다시 실행해 scene/export 결과가 각각 0/0과 20/2임을 확인했다.

## 남은 경계

30개 관점과 후속 lint/실행 재검토에서 차단 결함을 남기지 않았다. 구현/runtime 검토에서 확인된 Stylo fallback 경계는 exact diagnostic gate로 수정돼 있다. PR의 check rollup·merge state는 GitHub에서 별도로 확인해야 한다. 전체 WPT, 일반 CSS admission, glyph pixel 동등성, 실기기/hardware GPU, iOS compositor 이후 화면 및 C11/C11.4 완료는 검증 범위 밖이다.
