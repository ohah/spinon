# C11.4 anonymous Grid text `letter-spacing` 계획 PR diff 적대 검토 20개

이 검토는 계획 PR의 실제 변경 경계만 대상으로 한다. 계획 20개 실패 관점과 목적을 겹치지 않게 문서 정확성, 근거 연결, 범위 통제와 재현 가능성을 본다.

| # | PR diff 실패 관점 | 확인 결과 | 판정 |
| ---: | --- | --- | --- |
| 1 | 계획이 C11 상위 또는 C11.4 전체 완료를 약속 | 두 checkbox와 공식 집계의 승격을 제외한다고 명시 | 통과 |
| 2 | 계획 제목이 계획과 구현 상태를 혼동 | 상태에 사전 비교·계획 검토만 기록하고 구현은 plan merge 후로 명시 | 통과 |
| 3 | fixture가 다른 C11.4 anonymous text 입력을 덮어씀 | 별도 precomparison HTML이며 기존 fixture 경로와 분리 | 통과 |
| 4 | font asset 경로가 실제 Ahem 파일과 다름 | 상대 경로가 C15 Ahem asset으로 연결 | 통과 |
| 5 | Chrome 관찰을 Spinon 실행 결과로 표시 | README와 JSON에서 Chrome-only observation임을 분리 | 통과 |
| 6 | browser version 또는 viewport를 생략해 관찰을 재현 못함 | Chrome 156.0.8078.80, viewport, DPR, host와 Ahem load를 기록 | 통과 |
| 7 | min-content 사례가 공백/단일 단어를 분리하지 못함 | 두 단어 행렬에 single-word min/max controls를 추가 | 통과 |
| 8 | normal과 zero가 같은 cascade 처리인지 확인 불가 | zero computed value의 `normal` 직렬화와 같은 폭을 원본에 기록 | 통과 |
| 9 | Chrome의 일반 block 대조를 Grid oracle로 오해 | block/range 측정은 Grid case와 별도 probe property로 기록 | 통과 |
| 10 | 규범 계산이 브라우저 출력에 역산됨 | adjacent-unit 수식과 CSS Text/Sizing 링크를 README에서 독립 제시 | 통과 |
| 11 | CSS Text Level 3과 4의 값 문법을 혼동 | 계획은 Level 3 px-only absolute value subset으로 한정 | 통과 |
| 12 | Chrome와 규범값 차이를 조용히 숨기거나 parity 주장 | 차이 표, 계획 판정, 향후 PR 보고 요구를 모두 명시 | 통과 |
| 13 | Parley support를 이미 줄 끝 규칙을 충족한 것으로 오해 | atom spacing 존재만 기록하고 line-edge 처리를 검증 과제로 남김 | 통과 |
| 14 | ParagraphStyle 추가가 기존 callers를 바꿈 | 기존 explicit initializers를 zero로 고정하고 회귀 요구 | 통과 |
| 15 | unsupported CSS units가 자체 계산으로 수용됨 | normal/finite bounded px 외 입력을 fail-closed로 제한 | 통과 |
| 16 | runtime fixture 결과가 공개 CSS 지원을 주장 | 제한 opt-in profile 및 public admission 제외가 문서에 반복 명시 | 통과 |
| 17 | Android AVD를 Android 실기기로 표현 | AVD와 iOS Simulator만 계획하고 hardware/device 주장을 제외 | 통과 |
| 18 | surface sample을 geometry와 혼합 | 표면 픽셀은 track/frame proof와 분리하도록 정의 | 통과 |
| 19 | fixture ABI가 default artifact에 포함될 위험을 계획에서 누락 | default negative와 opt-in positive artifact gate를 모두 요구 | 통과 |
| 20 | 무관한 사전 untracked 파일을 PR에 함께 올림 | PR은 계획·review·전용 fixture·전용 evidence 경로만 stage하도록 제한 | 통과 |

최종 diff에는 구현 코드가 없다. 구현/runtime review는 실제 변경 코드와 simulator 결과가 생긴 뒤 새로운 실패 관점으로 작성해야 한다.
