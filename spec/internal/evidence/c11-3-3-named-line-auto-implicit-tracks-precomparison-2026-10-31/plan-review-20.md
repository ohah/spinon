# C11.3.3 named-line implicit auto-track 계획 실패 관점 검토 20개

구현 전 계획 자체를 독립된 실패 관점으로 검토했다. 비교 입력과 기준은 [사전 비교](README.md), [기대값](expected-values.json), [계획](../../../../plan/c11-3-3-named-line-auto-implicit-tracks.md)에 고정했다.

| # | 실패 관점 | 계획상 방어 및 판정 |
| ---: | --- | --- |
| 1 | 실제 Chrome이 요청 버전과 다른 엔진임 | User-Agent에 `HeadlessChrome/156.0.0.0`을 저장하고 일반 Chrome 결과로 과장하지 않는다. |
| 2 | screenshot과 DOM 측정이 다른 fixture revision을 가리킴 | 입력 SHA-256, screenshot SHA-256, 기준 commit을 observation에 묶는다. |
| 3 | DPR/viewport가 바뀌어 CSS px와 image px를 혼동 | viewport와 DPR을 고정·기록하고 frame assertion은 CSS px로 비교한다. |
| 4 | parent area가 implicit grid 생성을 유발함 | 모든 auto scene은 `grid-template-areas` 없이 고정 explicit rows/columns만 선언한다. |
| 5 | explicit track size가 빠져 전체 grid가 implicit-only로 됨 | 각 scene의 explicit track list와 수를 Chrome의 computed list와 함께 기록한다. |
| 6 | 이름 없는 numerical placement가 새 auto 허용 경로를 우회함 | projection contract에서 numeric out-of-range가 `TemplateGridImplicitTracks`에 들어오지 않는지 확인한다. |
| 7 | negative/signed placement가 positive trailing gate를 상속 | `TemplateGridSignedImplicitTracks + Auto` rejection을 회귀 assertion으로 유지한다. |
| 8 | auto-placement item이 named-line positive mode와 섞임 | definite-placement guard와 실패 오류를 유지하고 item placement를 직접 검증한다. |
| 9 | columns 성공만 보고 rows 경로를 놓침 | 별도 row scene에서 `20px 0px 25px`와 y=20을 고정한다. |
| 10 | rows 성공만 보고 columns 경로를 놓침 | 별도 column scene과 fixed-px control을 함께 고정한다. |
| 11 | auto가 실제 sizing에 사용되지 않고 기본 0 처리됨 | computed track sequence와 child frame을 모두 exact 확인하고 child의 fixed size를 기여 기준으로 둔다. |
| 12 | 빈 생성 track과 child가 놓인 track을 혼동 | 기대값에서 zero-size generated track과 다음 item-contributing track을 따로 assertion한다. |
| 13 | root 크기와 content가 같아 overflow 회귀가 숨음 | columns child가 root 오른쪽에서 시작하고 rows child가 root보다 5px 더 크도록 고정한다. |
| 14 | `overflow:visible`이 fixture에서 빠져 overflow 관찰이 잘못됨 | CSS fixture의 computed overflow와 화면의 root 경계를 확인한다. |
| 15 | fixed-px control 결과가 바뀌어 placement 자체가 흔들림 | control은 `40px 40px 30px 30px`, item x=110px로 계속 확인한다. |
| 16 | Taffy change가 area-backed auto sizing을 바꿈 | 기존 C11.3.3 area auto matrix 전체 회귀를 실행하고 area behavior를 별도 기록한다. |
| 17 | `TemplateGridImplicitTracks`가 다른 용도로 확대됨 | 값이 설정되는 모든 projection 호출 경로를 코드 검토하고 new mode admission을 한 곳에서 판정한다. |
| 18 | fixture FFI/runtime coverage가 product binary에 섞임 | default와 opt-in fixture-test archive를 각각 검사해 symbol/payload 경계를 증명한다. |
| 19 | simulator surface 결과를 Chrome 규범의 대체로 봄 | Chrome oracle, app geometry, Android PixelCopy, pre-compositor Metal을 별도 증거로 보고한다. |
| 20 | 제한 slice가 전체 CSS Grid나 공식 C11 완료로 집계됨 | C11.3.3/C11.4/C11 및 공식 `72/213` 체크를 유지한다. |
