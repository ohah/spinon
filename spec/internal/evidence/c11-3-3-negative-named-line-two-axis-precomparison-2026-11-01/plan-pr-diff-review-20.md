# 양 축 negative named-line 계획 PR 변경 검토 20개

검토 범위는 C11.3.3 candidate plan, Chrome precomparison 자료, plan index와 `spec/STATUS.md`의 후보 링크다. 제품 코드 변경은 없다. 구현·runtime 승인은 별도 작업으로 남긴다.

| # | 별도 실패 관점 | 결과 |
|---:|---|---|
| 1 | 계획 PR이 구현 완료처럼 보일 수 있다 | PASS — plan과 status 모두 구현/runtime 미시작으로 쓴다. |
| 2 | C11.3.3 또는 C11 공식 체크가 잘못 올라갈 수 있다 | PASS — checkboxes와 공식 `72/213`을 유지한다고 명시한다. |
| 3 | CSS Grid 기준 revision이 섞일 수 있다 | PASS — CSS Grid 2025-03-26 CRD와 §§7.2, 8.1.3, 8.3.1을 고정한다. |
| 4 | Chrome 결과를 규범 expected 값으로 재사용할 수 있다 | PASS — 독립 expected JSON과 실제 Chrome JSON을 분리하고 8/8을 수치 대조했다. |
| 5 | browser/device 환경이 빠지거나 추정될 수 있다 | PASS — UA, HeadlessChrome 156, Agent Browser 0.31.1, viewport, DPR를 기록한다. |
| 6 | 테스트 input의 root가 indefinite일 수 있다 | PASS — 두 축 definite 120×100px 및 explicit 20px/15px tracks를 고정한다. |
| 7 | extra child가 track contribution을 바꿀 수 있다 | PASS — layout-participating direct item을 하나로 고정한다. |
| 8 | 한 축 test를 양축 지원으로 계산할 수 있다 | PASS — one-axis rows/columns controls와 simultaneous both-axis scene을 분리한다. |
| 9 | named lines의 ident namespace가 모호할 수 있다 | PASS — 각 축 endpoint pair의 same-ident 조건과 고정 `ghost` input을 명시한다. |
| 10 | implicit track 개수 또는 item span이 하나 어긋날 수 있다 | PASS — `-4/-1`에 네 leading tracks와 세-track span을 별도로 적는다. |
| 11 | item contribution이 축 사이에서 섞일 수 있다 | PASS — 18px/12px independent contributions와 비대칭 explicit tracks를 둔다. |
| 12 | span 밖 auto track이 contribution을 받을 수 있다 | PASS — 네 번째 leading auto track의 0px expected를 명시한다. |
| 13 | fixed `grid-auto-*` pattern이 역방향에서 틀릴 수 있다 | PASS — 두 축의 full CSSOM arrays를 expected와 oracle에 보존한다. |
| 14 | fixed item이 track sizing을 늘리는 결과를 누락할 수 있다 | PASS — 17×10px spanned fixed area를 18×12px item이 1×2px 넘는 경계를 계획에 추가했다. |
| 15 | equal endpoint가 0 span으로 해석될 수 있다 | PASS — §8.3.1 end removal과 기본 1-track expected를 기록한다. |
| 16 | reversed endpoints가 축 하나에서만 정상화될 수 있다 | PASS — both-reversed와 column-only/row-only reversed controls를 둔다. |
| 17 | authored line precedence가 implicit profile에 가려질 수 있다 | PASS — precedence control과 새 mode 선택조건을 계획에 둔다. |
| 18 | malformed/unsupported input에서 partial output이 남을 수 있다 | PASS — mixed signs, spans, numeric, areas, wrong children, intrinsic sizing, stale revision, quota 및 atomic failure controls를 요구한다. |
| 19 | screenshot이 앱 실행 근거로 확대 해석될 수 있다 | PASS — README는 browser screenshot을 reference-only로 표시하며 simulator matrix는 구현 후 단계다. |
| 20 | PR 검토나 CI 상태가 과장될 수 있다 | PASS — 이 문서는 독립 20-point plan PR diff review이며 CI 성공이나 runtime을 주장하지 않는다. |

검토 결과 새 설계 결함은 발견하지 않았다. 구현에 앞서 계획 PR을 병합하고, 구현 뒤에는 별도의 20-point implementation/runtime review와 20-point PR diff review를 수행한다.
