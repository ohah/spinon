# C11.1 Root HTML 한 축 고정 배치 계획 PR diff 실패 관점 검토 30개

대상 diff: 계획, 계획·PR diff 검토, C11 상태 대장의 미구현 계획 기록, Agent Browser precomparison fixture·oracle·evidence를 추가한다. 제품 구현은 포함하지 않는다.

| # | 독립 검토 관점 | 판정 |
| --- | --- | --- |
| 1 | 제목이 C11 전체 완료가 아닌 C11.1 제한 계획임을 표시하는가 | 통과 — `C11.1`과 parsed Root HTML 한 축 고정 범위로 한정 |
| 2 | 계획 상태가 아직 미구현임을 분명히 하는가 | 통과 — 코드 변경·앱 runtime 검증 미실시를 표시 |
| 3 | 공식 `72/213` 및 C11 parent 상태를 잘못 승격하는가 | 통과 — STATUS.md는 계획만 기록하고 완료 개수·checkbox는 유지 |
| 4 | 기존 계획 인덱스와 상위 문서를 연결하는가 | 통과 — C11, C11.2.2, internal contract 0059 연결 |
| 5 | 계획 검토와 PR diff 검토가 별도 산출물인가 | 통과 — 20개 계획 검토와 이 30개 diff 검토를 분리 |
| 6 | 비교 fixture와 observation/expected가 직접 연결되는가 | 통과 — evidence README가 각 파일을 링크 |
| 7 | Chrome 실행 환경이 재현 가능한가 | 통과 — browser, OS/arch, viewport, DPR 및 SHA-256 기록 |
| 8 | 화면 파일이 측정 JSON과 혼동되거나 측정 근거로 과장되는가 | 통과 — 화면과 JSON을 분리하고 frame 수치의 source를 설명 |
| 9 | Chrome oracle를 Spinon 결과로 서술하는가 | 통과 — browser oracle의 한계를 반복해서 구분 |
| 10 | Chrome oracle를 WPT 적합성으로 서술하는가 | 통과 — WPT 통과가 아님을 명시 |
| 11 | accepted/rejected count가 fixture와 일치하는가 | 통과 — 10 scenes, 30 frames, 5 positive, 4 rejection, 1 all-auto control |
| 12 | independent expected가 관측 뒤 임의로 정해진 것으로 보이는가 | 통과 — 고정 tracks·source order·CSS Grid algorithm에서 계산한다고 밝힘 |
| 13 | 기존 all-auto 경로가 새 candidate에 포함되는가 | 통과 — 별도 unchanged control로 둠 |
| 14 | one-axis 후보의 axis uniformity가 모호한가 | 통과 — owner 내 모든 visible item의 definite axis가 같아야 함 |
| 15 | fully definite 및 mixed input rejection이 빠졌는가 | 통과 — 각 rejection을 명시하고 expected scene이 존재 |
| 16 | spans와 negative lines가 제한 계약 밖으로 확장되는가 | 통과 — numeric span·explicit-grid negative line만 지정 |
| 17 | named placement 또는 template area가 조용히 포함되는가 | 통과 — named line/span, area를 명시적으로 미지원 처리 |
| 18 | implicit row/column 생성이 허용되는가 | 통과 — extent 생성 결과를 거부하고 expected controls가 있음 |
| 19 | explicit track 종류와 수치 범위가 불명확한가 | 통과 — fixed px, finite non-negative, 기존 cap 명시 |
| 20 | gap 조건이 reference fixture와 상충하는가 | 통과 — core fixture gap 0이며 nonzero gap은 범위 밖 |
| 21 | auto track sentinel이 implicit 생성 실패를 숨기는가 | 통과 — 지정 `grid-auto-*`에도 implicit extent 필요 시 거부 |
| 22 | generic C11.2.2 contract가 의도치 않게 넓어지는가 | 통과 — Root HTML 전용 mode 요구 |
| 23 | candidate selector가 raw CSS string을 파싱하는가 | 통과 — computed typed style와 identity 사용 |
| 24 | `display:none`과 `visibility:hidden` occupancy가 모호한가 | 통과 — 규칙은 지정했고 추가 Chrome subscenes를 code 이전 prerequisite로 둠 |
| 25 | nested/sibling Grid 격리 근거가 누락되는가 | 통과 — 추가 Agent Browser scenes와 owner별 검증을 code 이전에 요구 |
| 26 | item content 및 sizing이 intrinsic 범위로 부풀려지는가 | 통과 — empty, non-replaced direct block과 auto/fixed size로 제한 |
| 27 | browser geometry를 native surface 정확도로 혼동하는가 | 통과 — Android PixelCopy와 iOS Metal drawable은 별도 구현 gate |
| 28 | simulator/platform 차이나 iOS readback 시점이 감춰지는가 | 통과 — AVD software renderer와 iOS compositor 전 한계를 분리 기록하도록 요구 |
| 29 | fixture FFI가 기본 product artifact로 들어갈 가능성을 확인하는가 | 통과 — 기본 artifact negative와 opt-in positive control을 요구 |
| 30 | implementation 이후 문서/검증/잔여 범위가 연결되는가 | 통과 — internal contract, STATUS, 20/30 reviews 및 미지원 목록을 병합 후 반영 |

문서 diff에 대해 30개의 서로 다른 실패 경계를 확인했다. `git diff --cached --check`도 통과했다. Rust build, product tests, Android/iOS runtime은 계획 문서 PR 범위가 아니어서 실행하지 않았다.
