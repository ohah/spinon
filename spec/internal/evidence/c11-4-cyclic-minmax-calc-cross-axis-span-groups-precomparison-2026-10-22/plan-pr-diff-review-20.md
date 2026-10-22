# 교차 축 span groups 계획 PR 변경분 적대 검토 20개

대상: 집중 계획, 부모 계획, 상태 대장, 공개 로드맵 요약, Chrome fixture 및 precomparison evidence. 각 항목은 변경 파일에서 별도의 오류 경로를 확인한다.

| # | 독립 실패 관점 | 검토 결과 |
|---:|---|---|
| 1 | C11.4 실행 기록 순서를 건너뛰거나 PR #506 이전 항목을 다음 slice로 고르는가? | 부모 계획과 STATUS에 #506 다음 후보로 기록하고 #503 완료 slice를 반복하지 않는다. |
| 2 | 집중 계획이 single-item fixture와 구별되지 않는가? | 두 item의 겹치는 columns span, 같은 rows span 및 cross-axis wrapping을 고정했다. |
| 3 | 처음 실패한 `XX XXXXXXX`가 최종 oracle에도 남는가? | 최종 fixture와 계획은 `AA A A`를 쓰고 실패 초안은 계획 검토에서만 설명한다. |
| 4 | item A의 content width와 line threshold 설명이 다르게 읽히는가? | Ahem hash와 20/60px min/max, 두 개·세 개 line rect를 JSON, README와 계획에서 대조했다. |
| 5 | item B의 explicit 40×10 size를 놓치거나 contribution으로 과장하는가? | fixture 선언, independent JSON 및 plan table에 40/40px inline contribution과 fixed preferred box를 함께 기록했다. |
| 6 | reverse fixture가 실제 DOM order-only인지 보장되지 않는가? | `data-case` 네 ID와 HTML child 순서를 비교했다. |
| 7 | screenshot이 다른 font revision에서 캡처됐는가? | Ahem SHA-256 `b719ecb31c5b21fc573c03f6421c74ac63c271a5a3ff841e34f9705fb94b8448`을 로컬 asset과 대조했다. |
| 8 | fixture가 runtime 미지원 alignment declarations를 필수로 두는가? | start alignment 선언을 제거하고 B dimensions를 고정한 Chrome oracle을 다시 관측했다. |
| 9 | min-content constraint bases와 final intrinsic columns가 혼합되는가? | normative JSON에 두 단계 값을 별도 필드로 기록하고 Chrome JSON은 관측된 final CSSOM 값만 담는다. |
| 10 | independent formula에서 gap이나 absolute calc term이 틀리는가? | A/B planned increase, per-track max, 2px bases와 gutters를 각각 다시 계산했다. |
| 11 | definite root `40px`가 border-box로 해석되는가? | 40px content width, root 44px border-box와 12px B overflow가 JSON, README, plan에 일치한다. |
| 12 | rows expectation이 columns 결과를 무시하는가? | A span width 33.34375/26px, line widths 20/30 및 20/10/10, Rows 9/9 및 14/14를 맞췄다. |
| 13 | browser sample target이 root 밖 item이 아닌가? | Agent Browser `elementFromPoint`는 `(50,4)`에서 item B red, `(60,4)`에서 item 외부를 반환했다. |
| 14 | Chrome transparent background를 native surface clear로 기록하는가? | Chrome JSON은 transparent DOM probe만 기록하고 native `#000000`은 계획된 mobile gate로 구분한다. |
| 15 | screenshot dimensions/hash가 README와 맞지 않는가? | 1280×633 PNG와 SHA-256 `3bffa3fd80c04607e5a591f7739ace0fad4df90a097bdcbfdbcef79dd54a58f5`를 manifest에 기록했다. |
| 16 | public roadmap 요약이 runtime 완료로 보이는가? | `계획`, `Runtime 미시작`, C11.4 미완료와 `77/213` 유지 문구 및 plan/evidence links를 넣었다. |
| 17 | STATUS 또는 parent plan의 official count/checkbox가 바뀌는가? | 후보 설명만 추가했으며 C11.4 checkbox와 `77/213`은 그대로다. |
| 18 | 모바일 실행·artifact separation을 이미 통과했다고 주장하는가? | 계획 단계임을 명시하고 AVD/iOS, PixelCopy/Metal, artifact 검사는 미래 gate로 둔다. |
| 19 | 계획 검토 20개가 PR diff 검토 20개를 대신하는가? | 서로 다른 대상·파일의 별도 20개 목록을 유지했다. |
| 20 | 다른 사용자의 untracked evidence가 변경에 섞이는가? | stage 전 target file allowlist로만 검토하며 기존 untracked evidence는 건드리지 않는다. |

**결론:** 이 계획 PR은 oracle/fixture와 상태 기록만 추가하며 runtime 결과를 주장하지 않는다. Chrome screenshot은 precomparison 화면으로만 PR 본문에 연결한다.
