# C11.4 row-span fit-content positions 4–6 계획 PR diff 적대 검토 30개

**검토 범위:** 계획·상태·로드맵·브라우저 근거 변경 16개 경로를 30개 관점으로 검토했다. 이 검토 기록이 17번째 경로로 함께 들어가며 표의 서로 다른 항목 수를 재확인했다. 제품 Rust/runtime 코드는 포함하지 않는다. 30개는 별도 계획 실패 관점 검토 20개와 중복 계수하지 않는다.

| # | 별도 실패 경로 | 확인과 판정 |
|---:|---|---|
| 1 | 다음 계획 후보를 공식 완료 ID로 잘못 표시할 수 있다. | STATUS의 C11.4 체크는 미완료 그대로이고 새 문장은 “계획 후보”라고 밝힌다. 통과. |
| 2 | 계획 선택 때문에 완료 수가 오를 수 있다. | 공식 78/214 표기는 유지됐고 칸반 completion source는 건드리지 않았다. 통과. |
| 3 | 내부 계약 숫자 버전이 계획만으로 바뀔 수 있다. | 계약 문서는 변경하지 않았고 0.1.0을 유지한다고 표시한다. 통과. |
| 4 | 기존 row-span position 2·3 구현을 새 계획의 근거로 잘못 연결할 수 있다. | 계획과 overview가 PR #1299를 해당 선행 작업으로 연결한다. 통과. |
| 5 | auto-placement position 1–6과 row-span position 4–6을 같은 topology로 취급할 수 있다. | 계획은 PR #1302 auto-placement profile과 새 row-span topology를 구분한다. 통과. |
| 6 | 계획 PR 안에 몰래 제품 코드가 포함될 수 있다. | staged diff에는 계획·상태·로드맵·Chrome fixture/evidence만 있고 제품 Rust 파일은 없다. 통과. |
| 7 | 제목의 authored position 범위와 실제 matrix가 다를 수 있다. | 계획, README, capture metadata 모두 위치 4·5·6을 사용한다. 통과. |
| 8 | list multiplicity가 설명과 fixture 사이에서 달라질 수 있다. | 한 fit-content, min-content 2개, max-content 2개, auto 1개를 fixture와 verifier가 검사한다. 통과. |
| 9 | 30 tail order가 duplicate permutation까지 세어 부풀려질 수 있다. | 관측 verifier는 tail set 30개와 key uniqueness를 확인한다. 통과. |
| 10 | explicit row가 implicit cycle의 일부로 계산될 수 있다. | fixture와 verifier는 explicit row 10px과 뒤의 여섯 implicit row를 분리한다. 통과. |
| 11 | span2-six fixture가 실제 6번째 implicit row를 만들지 않을 수 있다. | 마지막 fixed item은 row 6/span 2이고 관측 used row 수는 explicit 포함 7개다. 통과. |
| 12 | mixed span topology의 최종 row가 6보다 짧거나 길 수 있다. | row 4/span 3 및 row 5/span 3의 ending lines를 capture하고 7 used rows를 assert한다. 통과. |
| 13 | reverse topology가 item ID와 placement를 서로 바꿀 수 있다. | semantic ID별 placement table과 실제 DOM source order를 별도로 검사한다. 통과. |
| 14 | authored list position을 grid row line number로 오독할 수 있다. | 계획은 list index와 CSS row placement를 분리하고, CSSOM authored/computed order를 exact 비교한다. 통과. |
| 15 | percentage cap을 viewport나 device scale basis로 계산할 수 있다. | definite 220px block basis를 고정하고 20%와 120%의 대응 값을 기록한다. 통과. |
| 16 | cap의 경계점을 충분히 구분하지 못할 수 있다. | 0px/0%, 10px, 20px, 20%, 120% 여섯 값을 실제 matrix에 넣는다. 통과. |
| 17 | 캡 효과에 대한 주장이 실제 관측과 다를 수 있다. | 10px/20px 차이 48/48/24는 verifier summary에서 재생성되고 README도 corpus 한정 관측이라고 말한다. 통과. |
| 18 | 20px·20%·120%가 서로 다른 결과라고 과장할 수 있다. | README는 이번 corpus에서 동일했던 사실을 적고 다른 입력으로 확대하지 않는다. 통과. |
| 19 | 0% fit-content가 항상 min-content floor를 0으로 만든다고 오해할 수 있다. | 계획은 used-size ≤ cap 가정을 배제하고 raw used rows를 보존한다. 통과. |
| 20 | line count를 font metric이 아니라 문자열 단순 추정으로 고정할 수 있다. | Agent Browser Chrome 157 Range line positions를 관측하고 verifier에서 줄 수와 간격을 검사한다. 통과. |
| 21 | 가장 긴 text가 기존 32-byte grammar를 벗어날 수 있다. | A–E byte length를 계획에 고정했고 실제 fixture 최대는 31 bytes다. 통과. |
| 22 | item geometry 검사가 container bounds만 비교할 수 있다. | 각 span frame을 해당 used row 구간의 y/height 합과 비교하고 최대 오차 0px다. 통과. |
| 23 | source-order independence를 top-level rows만으로 주장할 수 있다. | verifier는 모든 semantic item frame과 row list를 forward/reverse로 비교한다. 통과. |
| 24 | stretch가 intrinsic track도 바꾸는데 검증하지 못할 수 있다. | 모든 permutation pair에서 auto 아닌 track 크기를 exact 비교한다. 통과. |
| 25 | 브라우저 관측을 CSS 규범 문서로 오인할 수 있다. | README는 W3C CRD를 독립 기준으로 두고 Chrome을 cross-check라고 한정한다. 통과. |
| 26 | W3C 링크의 section anchor가 다른 절을 가리킬 수 있다. | Agent Browser에서 공식 2025 CRD의 algo-spanning-items와 grid-align anchor를 열어 확인했다. 통과. |
| 27 | Native simulator evidence를 계획 PR에서 이미 완료했다고 말할 수 있다. | 계획은 Android/iOS 검증을 미래 구현 gate로 두며 사전 비교 README도 runtime 부재를 밝힌다. 통과. |
| 28 | opt-in fixture/FFI 경로로 default Root HTML 지원을 대신할 수 있다. | 계획은 default RuntimeSession route를 요구하고 새 fixture ABI를 추가하지 않는다. 통과. |
| 29 | public roadmap의 detailNext와 상단 문구가 서로 다른 후보를 보여줄 수 있다. | 두 위치 모두 repeated-text row-span fit-content positions 4–6으로 갱신했다. 통과. |
| 30 | Agent Browser 종료 주장에 실행 프로세스 근거가 없을 수 있다. | session-after-close는 active=false/pid=null, process search는 빈 파일, doctor는 active daemon 0개다. 통과. |

**결론:** 30개 서로 다른 PR diff 실패 관점을 확인했다. 수정할 결함은 발견하지 못했다. 제품 구현과 Android/iOS runtime 검증은 계획 PR 범위가 아니다.
