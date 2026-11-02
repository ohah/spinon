# 계획 PR 실제 diff 적대 검토 30개

검토 범위는 계획 PR에서 바뀐 C11 상위 계획, 전용 계획, 계획 review, HTML fixture 및 precomparison evidence 전체다. 이 검토는 계획 실패 관점 20개와 별개이며 구현/runtime review가 아니다.

1. **변경 파일 경계:** runtime Rust 파일이 섞여 계획 PR이 구현을 먼저 노출하지 않는지 확인했다. 계획·fixture·evidence 파일만 변경된다.
2. **상위 계획 상태:** `plan/c11-css-grid.md` 문구가 완료로 오인되지 않는지 확인했다. 다음 focused plan과 공식 미완료를 표시한다.
3. **전용 계획 연결:** 새 계획이 상위 C11, fixed/percentage row와 explicit fr plans를 링크하는지 확인했다. 링크를 포함한다.
4. **공식 상태 영향:** `spec/STATUS.md`나 체크박스/집계가 임의로 올라가지 않았는지 확인했다. 이 PR은 공식 `72/213`을 수정하지 않는다.
5. **contract version:** 계획 문서가 내부 0059의 숫자 버전을 올리는지 확인했다. 버전 변경은 제안하지 않는다.
6. **문서 언어:** 사용자 문서가 한국어인지 확인했다. 표준/코드/API 식별자는 원어를 유지한다.
7. **fixture 경로:** HTML이 CSS C11 fixture archive 아래 작업 ID로 배치됐는지 확인했다. `root-html-grid-auto-row-fr-precomparison.html`을 사용한다.
8. **완전한 문서:** doctype, html/head/body, viewport, charset, title이 있는지 확인했다. 전체 HTML fixture다.
9. **fixture 목적:** 제품 런타임 입력으로 오해될 수 있는지 확인했다. 페이지 자체 JavaScript는 browser geometry 관찰 전용이며 계획이 runtime fixture 재사용을 금한다.
10. **case 수:** expected와 raw observation, 계획 표가 같은 일곱 scene을 다루는지 확인했다. 세 파일의 ID를 대조한다.
11. **fixture 고정:** fixture SHA-256이 expected JSON과 README에 같은지 확인했다. `062f22d9e04236d9b6723717139f01211792fabccefe21d30d58bcd819479b88`다.
12. **oracle version:** user agent가 Chrome 156, DPR 1을 증명하는지 확인했다. expected와 observation이 같은 고정 문자열을 쓴다.
13. **관찰 분리:** independent expected와 Chrome observation이 별도 파일인지 확인했다. `expected.json`과 `chrome156-observation.json`이 분리돼 있다.
14. **1fr/2fr 계산:** cyclic flexible rows의 used values가 독립 결과와 브라우저 결과에 같은지 확인했다. `[20,10,20,10,20]px`다.
15. **mixed unit 계산:** px/percent/fr 목록이 같은 root height 기준과 factor distribution을 쓰는지 확인했다. mixed scene에서 flexible row는 20px다.
16. **percentage basis:** gap을 percentage basis에서 잘못 빼는 기대가 없는지 확인했다. 120px의 25%는 30px로 기록한다.
17. **row gap:** used rows 뒤 gutter를 별도 extent로 포함했는지 확인했다. 각 case의 extent 계산에 separator 수 × 5px를 반영한다.
18. **zero height:** definite `0px`과 indefinite height를 혼동하지 않는지 확인했다. `zero` scene의 used rows는 두 개 모두 0px다.
19. **zero factor:** zero factor가 실제 direct CSS value로 보존되는지 확인했다. `0fr 1fr` computed auto rows와 used rows를 raw capture한다.
20. **automatic minimum:** `0fr`의 fixed child minimum을 fixture가 보이는지 확인했다. zero-factor의 두 번째 item은 10px이며 track도 10px이다.
21. **subunit factor:** factor sum이 1 미만일 때 trailing space를 없어지는 것으로 오판하지 않는지 확인했다. `0.25fr` 두 row의 extent는 65px다.
22. **item minimum overflow:** fixed 40px child를 1fr track이 임의 축소하는지 확인했다. Chrome used row는 40px이며 root overflow는 35px다.
23. **percentage overflow:** 120% row가 root height로 clamp되지 않는지 확인했다. used row 120px, bottom overflow 50px다.
24. **frame 원점:** item frame을 viewport 좌표로 착각할 위험을 확인했다. raw DOMRect는 root-relative frame으로 변환한다.
25. **frame 목록 완전성:** 각 direct item을 빠뜨리거나 순서를 바꾸지 않는지 확인했다. raw observation에는 source order로 모든 ID가 있다.
26. **screenshot 연결:** 시각 캡처가 정확한 fixture revision을 가리키는지 확인했다. fixture SHA와 screenshot SHA를 README에 기록한다.
27. **screenshot hash:** README의 PNG checksum이 현재 파일과 일치하는지 확인했다. `b05dc8f41c5bf8f566a23940feb10dafe624939af9a94144fb54f0da48660198`다.
28. **browser cleanup:** 테스트용 브라우저를 남기지 않는지 확인했다. 종료 뒤 `session info`는 `active=false`, `pid=null`이었다.
29. **개발 상태 과장:** Chrome observation을 Spinon/device/WPT 통과라고 해석할 수 있는지 확인했다. README가 검증 한계를 명시한다.
30. **구현 관문 분리:** 계획 PR이 구현 검토 20개나 구현 PR diff review를 미리 세는지 확인했다. 각각 별도 구현 단계로 남겨 둔다.

## 발견 및 판정

- 첫 시각 캡처에서 case 배치가 위쪽에 몰리고 flex behavior를 읽기 어려웠다. fixture를 2열 case board로 구성하고 캡처를 다시 만들어, labeled scene을 확인했다.
- `0fr` case의 첫 초안은 fixed-size automatic minimum을 보여주지 못했다. 두 번째 child를 10px로 고정해 Chrome의 `0fr` track이 10px로 유지되는 것을 관찰하고 expected/raw evidence/계획 표를 갱신했다.
- 최종 7-scene observation의 used rows와 root-relative frames가 독립 기대값과 허용 오차 내에서 일치한다. 두 발견을 반영한 뒤 다시 검토해 계획 PR diff는 승인 가능하다.
