# C11 Root HTML column-flow implicit columns 계획 PR diff 실패 관점 검토 30개

검토 범위는 새 계획·계획 검토·HTML fixture·독립 기대값·Chrome 관측 JSON·screenshot과 상위 계획 링크 변경이다. 아래 각 항목은 독립된 실패/오해 경로를 대상으로 확인했다. 23번에서 발견한 계획 누락은 수정하고 다시 확인했다.

1. **C11 상태 승격 오해** — 새 문서가 C11 또는 C11.1 완료를 주장하는가? 계획 상태를 “구현 전”으로 두고 상위 완료와 분리했다. 통과.
2. **공식 대장 오염** — `spec/STATUS.md` 체크나 72/213을 변경했는가? 상태 대장 diff가 없다. 통과.
3. **계획 인덱스 누락** — 상위 C11 계획에서 새 문서를 발견할 수 있는가? `plan/c11-css-grid.md`에 독립된 구현 전 계획 링크를 추가했다. 통과.
4. **현재 후보 문구 stale** — Root HTML runtime 통합 계획이 PR #1056 전 상태로 남는가? 병합 상태와 다음 implicit-column 단계로 문구를 교정했다. 통과.
5. **선행 구현 관계** — 계획이 기존 column-flow mode 없이 가능한 것처럼 보이는가? PR #1056 및 기존 계획/evidence를 연결한다. 통과.
6. **계약 버전 과장** — 숫자 버전을 올리거나 공개 API라고 부르는가? 내부 계약 0059의 `0.1.0` 유지와 제한 fixture/runtime 경계를 명시한다. 통과.
7. **규범 링크 혼동** — Chrome이 규범 원본처럼 제시되는가? W3C §7.6/§7.7/§8.5와 browser observation을 분리했다. 통과.
8. **규범 문장 범위** — auto-track list 반복 또는 flow 방향의 출처가 직접 연결되는가? 각 관련 W3C anchor를 별도 링크한다. 통과.
9. **기준 환경 누락** — Chrome 버전/DPR/viewport가 추적되는가? browser version·DPR 1·1280×633을 observation JSON에 저장했다. 통과.
10. **screenshot 오해** — image가 surface pixel parity를 주장하는가? README와 계획이 screenshot을 oracle 화면으로만 분류한다. 통과.
11. **브라우저 절차 충돌** — Agent Browser 대신 terminal-browser를 썼는가? 관측은 Agent Browser session으로 기록됐고 그 세션은 종료했다. 통과.
12. **fixture selector 오류** — 배치 item 수가 12개와 일치하는가? source에는 `item-00`부터 `item-11`까지 정확히 12개 direct child가 있다. 통과.
13. **CSS auto-column list 거부 누락** — 문서가 typed projection만 언급하고 layout track expansion mode를 놓치는가? 첫 검토에서 이 연결이 빠진 것을 찾아 `dense_column.rs`의 제한 profile expansion step을 명시하도록 계획을 수정했다. 수정 후 통과.
14. **explicit track order** — 기대값에서 auto columns가 explicit columns 앞에 잘못 섞이는가? 관측/기대 리스트는 explicit `[30,40]` 다음 auto `[15,25,15,25]` 순서다. 통과.
15. **패턴 시작 인덱스** — 첫 auto track이 `[25]`로 시작하는가? expected sequence 및 Chrome computed value가 `[15,25]`로 시작한다. 통과.
16. **implicit track count** — 12 items와 2 rows에서 필요한 4개 implicit columns가 과소/과다 계산되는가? 총 6 columns와 2 explicit columns를 표·JSON·oracle에서 대조했다. 통과.
17. **row count 혼동** — column-flow item이 implicit row로 배치되는가? 기대 rows `[20,40]`, y origin `[0,23]`, auto rows `auto`를 고정했다. 통과.
18. **row gap 포함** — 두 번째 row y에 gap이 빠지거나 중복됐는가? 모든 두 번째 row는 `y=23`으로 독립 JSON과 Chrome 출력에서 같다. 통과.
19. **column gap 포함** — 각 x origin에서 gap이 누락/중복됐는가? 고정 track 누적합과 `5px` gaps가 `[0,35,80,100,130,150]`을 만든다. 통과.
20. **root frame overflow** — 마지막 열이 root 밖으로 나가는가? 6 track sum 150px와 5 gaps 25px가 175px root에 맞고 마지막 frame가 x=150,width=25다. 통과.
21. **item auto-size 부수 지원** — 빈 child의 auto width/height를 우연히 새 지원 범위에 넣는가? 첫 diff에서 README의 auto-size 설명이 fixture와 다름을 찾아 고쳤고, 각 item의 column/row class가 고정 px 폭/높이를 지정함을 재확인했다. 통과.
22. **CSS child-size mapping** — class group이 source-order column/row에 맞는가? 여섯 column class와 두 row class 값이 기대 track/item 크기와 일치한다. 통과.
23. **allocation failure 계약** — plan에 quota 이전/expansion reserve 실패가 없었다. 계획 검토에서 제기된 대로 expansion과 placement reservation failure injection, fallible propagation 및 scene 미발행을 검증 기준에 추가했다. 재검토 통과.
24. **10,000 cap 합산 우회** — explicit와 implicit columns를 따로 10,000까지 허용해 총량이 초과되는가? plan은 각 축의 최종 track count quota를 expansion 전에 검사하도록 요구한다. 통과.
25. **입력 크기 폭주** — 긴 auto-track list를 먼저 복사하는가? nonempty/list-length cap와 expansion 전 bounded allocation 조건이 명시돼 있다. 통과.
26. **invalid typed 값 진단** — negative/nonfinite/unsupported typed track이 px로 변환되는가? positive px와 rejection matrix를 분리하고 property 진단을 요구한다. 통과.
27. **placement semantics 우회** — line/span/area/order가 silent auto placement로 바뀌는가? 해당 입력을 rejection/regression matrix에 포함한다. 통과.
28. **platform 증거 범위** — simulator draw/present를 PixelCopy/Metal readback으로 확대하는가? 계획이 시각 smoke와 표면 readback을 분리한다. 통과.
29. **WPT/전체 CSS 주장** — fixture 한 건을 CSS Grid 적합성으로 확대하는가? WPT 및 일반 CSS admission은 제외한다고 명시했다. 통과.
30. **검토 게이트 분리** — 계획 20개와 diff 30개가 서로 대체되는가? 계획에 두 검토를 별도 게이트로 기록했고 이 파일은 PR diff 관점 기록이다. 통과.

계획 검토는 [별도 20개 기록](./c11-root-html-grid-column-flow-implicit-columns-review-20.md)이다. 이 30개 검토는 구현/runtime 20개 및 구현 PR diff 30개를 대신하지 않는다.
