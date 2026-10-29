# C11.4 열 축 fit-content 제한 slice PR 변경 검토 20개

**대상:** C11.4 fit-content column limit의 Rust fixture, FFI source/report, Android/iOS route와 matcher, capture script, 계약·상태·실행 근거 변경.
**판정 경계:** 이 문서는 PR 변경 범위 검토다. 이미 기록한 구현/runtime 실패 관점 20개와는 다른 문서이며, C11.4 또는 C11 완료 승인이 아니다.

| # | 변경 관점 | 검토 결과 |
| ---: | --- | --- |
| 1 | 기존 행 축 fixture와 새 열 축 scene이 서로 route를 가로채는가 | scene ID 집합이 분리되어 있고, capture run-kind 분기도 열 축 ID 집합에만 적용된다. |
| 2 | scene별 HTML source와 runtime adapter의 ID가 어긋나는가 | root/item/child ID가 네 source entry와 runtime CASES에서 동일하다. |
| 3 | 네 기대 column width가 독립 입력과 맞는가 | 30/40/200px limit은 30/40/100px, 40px limit + 100px automatic minimum은 100px이다. |
| 4 | 200px limit을 root 폭 또는 max-content보다 큰 값으로 가려도 통과하는가 | root 300px, child max-content 100px를 두어 growth limit보다 content가 먼저 끝나는 경우를 분리한다. |
| 5 | 30/40px 장면의 item과 child overflow를 같은 폭으로 축소하는가 | item은 used track 폭, child는 100px이며 root 밖 viewport 내 overflow를 표본에 남긴다. |
| 6 | min-width 0과 auto 대조가 실제로 다른 minimum source를 만드는가 | 기본 장면은 specified zero minimum, control은 100px automatic minimum source를 요구한다. |
| 7 | authored root sizing 값이 CSS source와 typed Grid input 사이에서 바뀌는가 | template rows/columns, auto columns, flow, gaps와 placement mode를 strict validator가 확인한다. |
| 8 | 고정 row가 열 축 intrinsic contribution을 오염시키는가 | row trace의 fixed-track early return과 빈 contribution list를 검사한다. |
| 9 | raw growth limit과 used base size가 한 값으로 뭉개지는가 | expected width와 별도로 MaximizeTracks growth limit 100px을 보고·검사한다. |
| 10 | 새 observation 필드가 기존 runtime report의 소비자와 충돌하는가 | 전용 C11.4 variant/report 경로에 추가되고 기존 scene schema를 변경하지 않는다. |
| 11 | FFI JavaScript scene 생성에서 임의 scene 문자열이 실행되는가 | 고정 scene tuple lookup 이후 source를 만들며 미등록 suffix/입력은 반환하지 않는다. |
| 12 | fixture FFI 선언이 기본 product feature에 노출되는가 | 기존 opt-in fixture-test 경계 안에 등록되고 product archive negative control을 별도로 기록한다. |
| 13 | Android allowlist에만 등록되고 strict report matcher를 건너뛰는가 | route 등록과 별도 column trace/frame/presentation matcher가 모두 추가됐다. |
| 14 | Android 출력 숫자의 정수·소수 표기가 matcher를 불안정하게 하는가 | runtime numeric debug 형식으로 비교하도록 맞춘 수정과 네 장면 재실행이 기록돼 있다. |
| 15 | iOS scene route만 있고 Metal readback이 실제 활성화되지 않는가 | 최종 build에 전용 Metal sample flag를 켠 build log 및 14/14 샘플 결과가 기록돼 있다. |
| 16 | draw/present 결과를 이전 surface generation으로 잘못 귀속하는가 | 환경 결과와 presentation identity가 surface generation·sequence와 결합된다. |
| 17 | viewport 바깥 표본 또는 root overflow 점을 root 내부 pixel로 오해하는가 | 내부·overflow·viewport 내 root 바깥 표본을 서로 다른 위치로 구성하고 샘플 좌표를 검사한다. |
| 18 | run-kind 선택 때문에 이 실행이 auto-repeat 이웃 행렬로 섞이는가 | 네 ID 전부가 열 축 집합에 속해야 해당 run-kind와 output 경로를 선택한다. |
| 19 | 새 boundary validator 하위 파일이 fixture 경계 회귀 검사에서 빠지는가 | 검사는 신규 style/trace 하위 모듈을 함께 읽도록 수정됐고 관련 테스트 2/2가 기록돼 있다. |
| 20 | 문서가 제한 slice를 전체 Grid 또는 C11 완료로 승격하는가 | STATUS와 내부 계약은 slice 설명만 갱신했고 C11.4 checkbox 및 72/213은 유지한다. |

## 수정 뒤 확인 및 남은 경계

구현 검토 중 드러난 matcher 표기 차이를 고친 뒤 Android AVD와 iOS Simulator 각각 네 장면을 다시 실행했다. 모듈 분리 뒤 boundary test의 검사 경로가 불완전했던 문제도 새 하위 파일 두 개를 포함하도록 고치고 2/2 통과를 기록했다. 전체 diff와 실행 근거는 [시뮬레이터 evidence](./README.md)를 참고한다.

이 검토는 정적 변경 검토와 기록된 focused/simulator 증거를 연결한다. 실기기/hardware GPU, iOS compositor 이후 픽셀, 전체 WPT, 일반 author CSS admission 및 전체 C11 적합성은 입증하지 않는다. runtime PR은 아직 병합되지 않았으므로 공식 완료 수는 72/213으로 유지한다.
