# C11.4 fixed-repeat + auto-repeat 계획 수정 PR diff 검토 · 실패 관점 30개

**검토 범위:** computed `LengthPx`로 좁힌 수정 계획, 9-scene Chrome 156 사전 비교, Stylo computed-AST 관찰, 계획 수정 검토 20개 및 `STATUS`·0059·conformance·public roadmap의 연결 상태. 현재 PR은 계획·문서·oracle 자료만 바꾼다. 기능 구현, Spinon layout/runtime, Android/iOS Simulator, WPT 또는 공식 완료 판정을 주장하지 않는다.

| # | 독립 실패 관점 | diff 검토 결과 |
| --- | --- | --- |
| 1 | 계획 보정이 기능 완료로 읽혀 C11.4 checkbox가 바뀔 수 있음 | `spec/STATUS.md`의 C11.4 상태와 checkbox를 유지하고 변경은 계획 후보로만 기록했다. |
| 2 | 계획 PR이 공식 완료 수를 늘릴 수 있음 | 문서의 현재 공식 집계 `72/213`을 유지했다. |
| 3 | 내부 contract가 범위 수정만으로 새 숫자 버전이 될 수 있음 | `0059` 숫자 버전 `0.1.0`이 그대로다. |
| 4 | mixed repeat 범위를 두 축 동시 지원으로 넓힐 수 있음 | 계획과 oracle은 한 grid의 한 축만 다룬다. |
| 5 | 한 list의 두 auto-repeat을 실수로 허용할 수 있음 | 한 list당 기존 profile의 auto-repeat 하나 제한을 유지한다. |
| 6 | fixed-repeat을 여러 개 허용한다고 오해할 수 있음 | fixed-repeat 하나만 후보 범위로 정했다. |
| 7 | 0 또는 음수 repeat count를 허용할 수 있음 | positive integer count만 계획에 남겼다. |
| 8 | 구현 경계가 source spelling을 구분한다고 과장할 수 있음 | computed AST에서 원 authored spelling/unit을 복원할 수 없음을 명시했다. |
| 9 | constant absolute `calc()`가 자동 지원으로 읽힐 수 있음 | Stylo가 finite nonnegative `LengthPx`로 정규화한 경우만 후보이며, 아직 구현 전이라고 명시했다. |
| 10 | percentage가 포함된 `calc()`를 absolute `calc()`와 혼동할 수 있음 | percentage 또는 unresolved `Calc`는 계속 제외한다. |
| 11 | negative computed length를 허용할 수 있음 | finite와 nonnegative 조건을 함께 둔다. |
| 12 | NaN/infinity를 finite length로 처리할 수 있음 | 구현 단계에서 finite 검사와 fail-closed 오류를 요구한다. |
| 13 | flex fixed track을 허용해 auto-repeat sizing 계약을 넓힐 수 있음 | fixed-repeat breadth에 flex를 제외했다. |
| 14 | intrinsic sizing 또는 `minmax()`를 fixed group에 포함할 수 있음 | intrinsic breadth와 `minmax()`를 범위 밖으로 뒀다. |
| 15 | named line metadata가 버려지며 조용히 수용될 수 있음 | outer/inner line-name groups가 비어 있는 경우만 계획한다. |
| 16 | nested repeat이나 둘째 fixed group이 통과할 수 있음 | 둘 다 명시적으로 거부할 구현 검사를 계획했다. |
| 17 | checked arithmetic·track cap 없이 확장될 수 있음 | 10,000 descriptor cap, checked multiply/add, reservation 실패 시 no partial output을 통과조건에 뒀다. |
| 18 | auto-repeat index가 fixed prefix 전개 뒤 어긋날 수 있음 | prefix/suffix authored order 및 전개된 start/end를 단위·layout 판정에 포함했다. |
| 19 | component 경계 line metadata 크기를 검증하지 않을 수 있음 | 확장 경계 `N+1`과 metadata reservation/failure 관찰을 계획에 포함했다. |
| 20 | CSS parser가 입력을 버린 것을 adapter rejection으로 오인할 수 있음 | parser diagnostic과 typed adapter 오류를 분리하도록 명시했다. |
| 21 | Stylo AST 기록이 현재 구현이 허용한다고 오해될 수 있음 | probe는 기존 adapter의 `RepeatCount::Number` 거부를 포함하고 mixed 구현을 주장하지 않는다. |
| 22 | 새 calc 입력이 Chrome oracle에서 직접 px 입력과 다를 수 있음 | 9번째 scene의 used tracks를 literal px control과 대조하며 관찰 JSON을 연결했다. |
| 23 | scene 수와 계획/fixture/README의 표가 어긋날 수 있음 | fixture·계획·conformance·evidence 표를 모두 9 scenes로 맞췄다. |
| 24 | row axis 값이나 case ordering이 누락될 수 있음 | rows prefix/suffix와 auto-fill/auto-fit 점유·빈 장면을 기존 oracle에 보존했다. |
| 25 | 독립 산식이 Chrome 값만 재진술할 수 있음 | 반복 최소 크기, non-repeated 합, gap 수를 별도 식으로 제시했다. |
| 26 | Chrome DOM 관찰을 Spinon 결과나 WPT 통과로 확대할 수 있음 | README와 계획은 oracle 관찰을 Spinon/WPT 증거와 분리한다. |
| 27 | simulator·physical-device 검증이 끝난 것처럼 보일 수 있음 | Android/iOS runtime은 미실행으로 분명히 기록했다. |
| 28 | 수정 계획 검토 20개를 PR 변경 검토 30개로 재사용할 수 있음 | 이번 PR diff에 맞춘 30개의 별도 실패 관점을 작성했다. |
| 29 | relative link 또는 roadmap 경로가 수정으로 끊길 수 있음 | RSPress 2.0.22 선택 문서 build가 완료됐다. Agent Browser에서 계획, evidence README와 20/30 검토 페이지를 열어 제목, 9-case 내용과 각각 20/30 행을 확인했다. 로드맵은 별도 HTML route로 제공되는 것을 확인하고 Tailnet 동기화 뒤 해당 route도 다시 점검한다. |
| 30 | 캡처·입력 hash가 수정 후 stale할 수 있음 | fixture, observation, screenshot, AST JSON, 계획·검토·상태 문서 hash를 SHA256SUMS에서 재생성했고 전체 `shasum -c` 확인이 통과했다. |

## 결론

이 문서 검토는 계획-only diff의 상태·범위·oracle 출처와 링크/hash 경계를 확인한다. 29번 선택 문서 build 및 Agent Browser 표시와 30번 최종 SHA 검증을 완료했다. Tailnet은 병합 뒤 sync 단계에서 별도 확인한다. 남은 diff 결함은 발견되지 않았다. 코드 구현이나 simulator 증거는 이 PR에 포함하지 않는다.
