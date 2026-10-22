# 계획 PR diff 실패 관점 30개

검토 대상은 auto-fit 후속 계획, 사전 비교 fixture·관측·캡처, 계획 review 표와 공개 roadmap preview 변경이다. 계획 자체를 검토한 20개 관점과 별도로 수행했다.

| # | 독립 실패 관점 | diff 확인 / 처리 |
|---:|---|---|
| 1 | 새 계획이 auto-fill 계획의 같은 작업으로 잘못 덮여 구현 범위를 합친다. | 별도 계획 파일로 만들고 기존 계획을 링크만 한다. |
| 2 | auto-fit named fallback이 이미 검증 완료된 것으로 읽힌다. | 상태를 plan stage로 두고 simulator evidence가 아직 없다고 적었다. |
| 3 | `77/213` 또는 C11.4 checkbox가 계획 PR에서 완료 처리된다. | 공식 상태 파일은 수정하지 않고 미완료 수를 공개 preview에도 남겼다. |
| 4 | W3C 링크가 버전 없는 최신 spec으로 바뀌어 기대값이 변한다. | 고정 2025 CRD와 §7.2.3.2/§8.3 direct anchor를 사용했다. |
| 5 | auto-fit collapse 시점이 placement 전이라고 문서가 말한다. | 독립 계산·계획 모두 placement 뒤 collapse 순서를 적었다. |
| 6 | 빈 track 정의에서 in-flow 조건이 빠진다. | 규범 설명에 in-flow item placement/span 기준을 넣었다. |
| 7 | named line fallback이 auto-fit 전용 동작인 것처럼 적힌다. | 기존 auto-fill 결과를 logical-line resolution 기준으로 재사용하고 collapse 차이만 분리했다. |
| 8 | repeat count가 collapse된 track 수로 추정된다. | 125px axis에서 배치 전 3회 반복을 기록했다. |
| 9 | implicit track sizing이 auto track으로 달라진다. | row/column `grid-auto-*`를 20px로 고정했다. |
| 10 | gap 또는 border/padding 때문에 repeat count가 3이 아니게 된다. | fixture에서 axis 125px, gap/border/padding 0을 고정했다. |
| 11 | CSS fixture hash가 README의 hash와 다르다. | fixture bytes SHA256을 다시 계산해 metadata/README와 대조했다. |
| 12 | browser screenshot이 HTML/scene 수와 다른 revision을 가리킨다. | screenshot을 oracle fixture에서 다시 생성하고 10개 scene이 모두 보이는지 확인했다. |
| 13 | browser output이 terminal-browser에서 나온다. | Agent Browser 0.31.1 + Chrome 156을 사용했고, 해당 사용 경계를 문서화했다. |
| 14 | 관측 user-agent와 Chrome patch version이 불일치한다. | `navigator.userAgent` major와 설치된 Chrome binary full version을 따로 확인했다. |
| 15 | viewport/DPR가 반복을 달리 만드는 조건으로 바뀐다. | 1280×1000 CSS px, DPR 1을 JSON 및 metadata에 저장했다. |
| 16 | positive absent-name과 insufficient-name 실패가 한 장면에 섞인다. | `ghost`와 `cell` positive cases를 분리했다. |
| 17 | negative absent-name과 insufficient-name 실패가 섞인다. | 두 negative cases를 각각 고정했다. |
| 18 | named span 방향 하나만 검증하고 반대 방향을 누락한다. | forward와 backward fallback scene을 분리했다. |
| 19 | zero-width repeat track을 사이즈 문자열에서 생략해 count 회귀를 숨긴다. | 기대값은 각 0px slot을 그대로 나열하며 Chrome JSON의 line groups도 보존한다. |
| 20 | repeat boundary의 중복 `cell` 이름이 serializer에서 병합된다. | 전체 computed track/group serialization exact 비교를 판정 기준에 포함했다. |
| 21 | occupied repeat control이 없어 모든 track이 항상 접혀도 통과한다. | `occupied-repeat`는 `[0,40,0]`을 요구한다. |
| 22 | multi-track span control이 없어 교차 track을 누락해도 알 수 없다. | `span-occupied-repeat`는 `[0,40,40]`과 80px frame을 요구한다. |
| 23 | row-axis 검증을 columns 결과로 대신한다. | row positive/negative를 별도 scene과 기대값으로 기록했다. |
| 24 | cross-axis sizing이 root-relative frame을 바꾼다. | 반대 축을 각각 고정 20px track으로 둔다. |
| 25 | intrinsic content가 auto track을 늘린다. | item을 빈 단일 in-flow block으로 두고 `min-width:0`과 고정 높이를 적용한다. |
| 26 | computed placement serialization이 resolved line index 증거로 잘못 해석된다. | README와 계획에서 CSSOM 문자열 해석 한계를 적었다. |
| 27 | 0px collapse 구간의 불가능한 pixel sample을 요구한다. | collapsed track interior sample을 제외하고 item/실재하는 neighboring pixels만 읽는다. |
| 28 | static archive 결과가 APK/IPA 전체 증거로 과장된다. | archive negative/positive controls로 범위를 한정하고 bundle exclusion을 부정했다. |
| 29 | simulator-only 결과가 실기기·hardware GPU·WPT 호환성으로 확대된다. | 제외 목록을 plan, precomparison 및 PR template에 명시했다. |
| 30 | 공개 roadmap은 계획 link를 추가하면서 검증이 끝난 듯 서술한다. | card 문구는 후속 계획만 등록됐고 runtime은 미검증이라고 명시한다. Agent Browser close 후 session info `active=false`/`pid=null`, process count 0을 보존했다. |

## 판정

이 diff에 구현 완료 주장, status 집계 변경, WPT/physical device 주장 또는 active Agent Browser process 증거는 없다. 계획과 관측 artifact 링크를 추가했고, 차후 implementation PR에는 별도 runtime review와 새로운 30-point diff review를 작성한다.

## 병합 전 재확인

종료 artifact의 첫 스냅샷에서 `close --all` 직후 background daemon이 잠시 남은 것을 찾아 즉시 종료라고 단정하지 않았다. 후속 확인에서 page count 0, session list 빈 값, `active=false`, `pid=null`과 process count 0을 확인하고 [`browser-shutdown.json`](./browser-shutdown.json)에 두 시점을 보존했다. 이 수정은 PR diff review #30의 process 수명 경계에 반영되었다.
