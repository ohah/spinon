# C11.4 anonymous Grid negative `text-indent` 계획 PR diff 검토 30개

## 검토 경계

계획 PR diff의 항목은 negative indent 집중 계획·부모 계획 index·구현 전 계획 검토·고정 HTML 입력·Chrome observation/expected data/화면이다. `spec/STATUS.md`와 제품 소스는 변경하지 않는다. 이 PR diff 검토는 계획 자체의 20개 검토를 재사용하지 않는다.

| # | PR diff를 공격하는 실패 관점 | 대조 결과 |
|---:|---|---|
| 1 | 2026-11-01 plan date가 증거 시점과 맞지 않을 수 있다 | Chrome 156 관측과 CSS Text 2026 CRD 확인일을 이 날짜로 연결했다. 통과. |
| 2 | 후속 계획이 기존 #930 완료 상태를 미병합으로 표현할 수 있다 | 계획은 #930의 0/5/10px positive profile을 직전 slice로 특정한다. 통과. |
| 3 | 계획 후보가 구현 완료로 오인될 수 있다 | 상태를 “구현 전 계획 후보”라고 적었다. 통과. |
| 4 | 부모 C11.4 계획이 실제 신규 계획 파일을 찾지 못할 수 있다 | index의 새 항목이 집중 계획 경로를 링크한다. 통과. |
| 5 | plan review 20개가 parent index에서 빠져 검증 단계를 생략할 수 있다 | 계획과 parent index 모두 별도 20개 review를 연결한다. 통과. |
| 6 | plan review와 PR diff review가 동일 검토를 재사용할 수 있다 | 두 검토 파일이 분리되고 서로의 역할을 명시한다. 통과. |
| 7 | 계획 PR merge 전 구현을 시작해 사용자/프로젝트 순서를 위반할 수 있다 | parent index와 plan에서 구현은 계획 PR 병합 후 시작한다고 적었다. 통과. |
| 8 | C11.4 작업 기록만으로 공식 체크가 닫힐 수 있다 | `spec/STATUS.md`는 변경 목록에 없고 공식 count는 후보 계획에만 언급된다. 통과. |
| 9 | 내부 계약 0059의 공개 숫자 버전을 올릴 수 있다 | 계약 파일을 수정하지 않고 다음 구현에서도 버전 0.1.0을 유지하도록 적었다. 통과. |
| 10 | CSS Text URL이 다른 문서 버전이나 섹션을 열 수 있다 | 링크가 2026-08-14 CRD §8.1 `text-indent` section을 가리킨다. 통과. |
| 11 | signed length가 아닌 값이 negative candidate로 적힐 수 있다 | scope 값은 `-5px`와 `-10px`로 고정했다. 통과. |
| 12 | fixture value가 arbitrary negative length로 확대될 수 있다 | exact allowlist 이외의 -7px·-30px는 거부 경계에 둔다. 통과. |
| 13 | `-30px` 관측이 지원 계약처럼 parent plan에 올라갈 수 있다 | boundary는 oracle-only라고 두 번 명시했고 implementation scope에서 제외했다. 통과. |
| 14 | -30px에서 max<min invariant와 Grid track floor가 감춰질 수 있다 | `X XXX`의 30px/30px Chrome track과 단순 20px raw formula의 상호작용을 문서화했다. 통과. |
| 15 | negative indent가 모든 줄에 적용된다는 잘못된 기대를 넣을 수 있다 | first formatted line만 indent하며 min-content word breaks는 독립 계산한다. 통과. |
| 16 | text sample 순서가 첫 word 길이 차이를 검증하지 않을 수 있다 | `XXX X`, `X XXX`, `XX` 세 input을 양방향 길이 대조와 단일 단어 control로 사용한다. 통과. |
| 17 | U+0020 collapsing을 두 word 사이에서 무시할 수 있다 | Ahem space advance 10px와 normal whitespace collapse를 고정했다. 통과. |
| 18 | Ahem fallback 또는 다른 파일 hash가 결과를 바꿀 수 있다 | source fixture와 README에 pinned Ahem SHA, `fontLoaded:true`를 기록했다. 통과. |
| 19 | HTML fixture가 다른 font 자원을 상대경로로 열 수 있다 | `../../resources/c15/Ahem.ttf`는 `tests/fixtures/resources/c15/Ahem.ttf`에 해석된다. 통과. |
| 20 | Browser observation의 requested value와 computed value가 다를 수 있다 | observation JSON은 각 scene의 `specifiedTextIndent`와 `computedTextIndent`를 모두 보존한다. 통과. |
| 21 | 기대값 JSON이 Chrome observation과 어긋날 수 있다 | candidate 12개와 -30 boundary 6개를 Python으로 교차 대조해 track width/height가 일치했다. 통과. |
| 22 | expected JSON이 plan의 12-scene count를 모호하게 표현할 수 있다 | `candidateScenes`에 min-/max-content별 12개 scene ID·width·height를 열거했다. 통과. |
| 23 | Oracle-only control이 구현 candidate 수로 잘못 더해질 수 있다 | -30 관측은 별도 `oracleOnlyBoundary` array로 저장한다. 통과. |
| 24 | screenshot에 JSON dump가 보여 입력 비교가 읽히지 않을 수 있다 | screenshot은 grid case label과 Ahem block만 표시하며 JSON output은 hidden이다. 통과. |
| 25 | screenshot만으로 glyph/surface 정확성을 주장할 수 있다 | README와 plan은 screenshot을 observation UI로만 사용하고 glyph pixels를 제외한다. 통과. |
| 26 | Agent Browser 캡처가 terminal-browser 결과일 수 있다 | Chrome 156 Headless Agent Browser 세션에서 캡처하고 종료 후 process 0개를 확인했다. 통과. |
| 27 | Android/iOS screenshot이 계획 PR에서 이미 검증된 것처럼 적힐 수 있다 | 사전 비교는 browser-only이며 mobile runtime은 계획의 future implementation gate다. 통과. |
| 28 | opt-in fixture가 일반 CSS admission이나 full WPT 지원으로 과장될 수 있다 | 계획과 README가 일반 admission/WPT를 scope 밖으로 둔다. 통과. |
| 29 | review 문서 자체가 20개 계획 검토 외 구현 검증을 했다고 보일 수 있다 | 계획 review는 코드·simulator 미실행을 적고 PR diff review 30개와 분리한다. 통과. |
| 30 | PR diff 검토 보고서가 변경/검증 파일 목록을 누락할 수 있다 | diff 대상 8개 plan/index/evidence/fixture file과 image를 대조했으며, 이 보고서가 30개 검토 결과를 기록한다. 통과. |

## 판정

30개 PR diff 실패 관점에서 계획·기준입력·증거 링크·공식 상태 경계에 불일치는 발견되지 않았다. 계획 PR 단계의 사전 비교 확인이며, 제품 코드나 시뮬레이터 결과를 뜻하지 않는다.
