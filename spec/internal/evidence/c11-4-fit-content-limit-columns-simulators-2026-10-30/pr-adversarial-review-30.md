# C11.4 열 축 fit-content 제한 slice PR 적대 검토 30개

**대상:** PR 후보 diff 전체와 그 diff가 주장하는 네 장면의 실행 근거.
**방법:** 별개의 실패 경로·불변식 30개를 순서대로 대조했다. 빌드·테스트 실행 여부는 근거 문서의 기록과 구분했다. 발견한 수정 사항은 기존 구현 검토와 evidence에 기록되어 있다.

| # | 적대 입력·실패 경로 | 판정 |
| ---: | --- | --- |
| 1 | 허용 목록 앞뒤에 공백·접미사를 붙인 scene ID | 미등록 ID는 exact lookup에서 거부된다. |
| 2 | root ID는 맞지만 namespace 또는 tag가 다른 host node | HTML namespace와 div identity를 확인한 뒤 제한 fixture만 선택한다. |
| 3 | 같은 ID의 두 번째 root 또는 root가 document root가 아닌 경우 | fixture 단일 identity와 parent 관계 검사가 계산 전 거부 경로에 있다. |
| 4 | root에 item이 없거나 item을 둘 이상 넣는 경우 | 정확한 직계 자식 cardinality가 요구된다. |
| 5 | item을 다른 parent로 옮기거나 다른 item의 child를 재사용 | HostParent와 direct-child 관계를 확인한다. |
| 6 | child를 0개·2개로 만들거나 expected ID를 바꾸는 경우 | child 수·ID·ownership이 고정된다. |
| 7 | style snapshot에 중복 node row를 추가하는 경우 | ID set cardinality와 snapshot 길이를 함께 비교한다. |
| 8 | root/item/child 중 하나의 style row를 누락하는 경우 | 정확한 세 node의 존재를 요구한다. |
| 9 | 다른 computed-style profile로 같은 authored CSS를 전달 | profile mismatch는 거부된다. |
| 10 | CSS declaration source와 computed property가 서로 다른 경우 | authored root contract와 typed Grid input을 각각 비교한다. |
| 11 | columns를 명시하거나 rows를 auto로 바꾸는 경우 | empty explicit columns 및 60px fixed row 조건과 달라져 거부된다. |
| 12 | flow를 row/dense로 바꾸거나 auto-columns를 다르게 설정 | column flow와 해당 fit-content limit을 exact 비교한다. |
| 13 | gap, padding, border 또는 overflow를 수정하는 경우 | 고정 spacing/box/overflow contract 검사에서 거부된다. |
| 14 | 기본 scene에 min-width:auto를 넣어 minimum contribution을 높이는 경우 | specified zero minimum 기대와 달라져 거부된다. |
| 15 | auto-minimum control에서 min-width:0으로 낮추는 경우 | automatic minimum source와 100px floor 기대를 만족하지 못한다. |
| 16 | child width를 80px로 바꾸고 track을 우연히 맞추는 경우 | child width 및 min/max-content contribution 100px을 독립 확인한다. |
| 17 | fit-content limit을 0·음수·다른 값으로 바꾸는 경우 | scene별 authored style, runtime CASES, FFI source, expected data가 서로 맞지 않아 통과하지 않는다. |
| 18 | 200px limit을 100px보다 큰 max-content와 함께 사용 | 현재 fixture가 고정한 100px child 외의 입력은 계약 밖이며 일반화 주장하지 않는다. |
| 19 | growth limit을 used base size와 동일시해 30/40px로 보고 | trace validator는 raw growth limit 100px과 used base를 분리한다. |
| 20 | trace에서 Columns pass만 남기고 Rows pass를 누락 | axis 순서와 각 pass cardinality 검사가 실패시킨다. |
| 21 | fixed row에서 intrinsic contribution을 추가 | Rows early-return와 빈 contribution list 기대가 이를 거부한다. |
| 22 | track trace에 gutter·collapsed track를 더해 대상 track을 혼동 | non-gutter/non-collapsed track cardinality를 고정한다. |
| 23 | root/item/child 중 하나의 layout frame이 누락·중복 | 정확한 frame node 집합 및 각 좌표를 비교한다. |
| 24 | layout 완료 revision은 새것이고 style snapshot revision은 오래됨 | observation·layout key·diagnostic revision의 일치가 필요하다. |
| 25 | Android 앱이 stale APK를 실행했지만 로그만 정상처럼 보임 | 기록된 installed APK와 local APK SHA-256 일치가 설치 identity 증거다. |
| 26 | Android PixelCopy를 시작했지만 일부 좌표가 생략 | 실행 기록은 설정된 14점 전부를 비교하고 scene별 표본 수를 분리한다. |
| 27 | iOS drawable readback flag가 꺼진 binary의 screenshot을 pixel proof로 사용 | 최종 build flag와 14점 drawable 결과가 기록되고 compositor 전 한계를 명시한다. |
| 28 | 이전 surface의 draw 결과를 현재 scene의 성공으로 보고 | surface generation·presentation sequence identity matcher가 일치해야 한다. |
| 29 | fixture marker가 product FFI archive에 남아 있는데 경로명만 보고 제외를 주장 | 두 archive scan을 positive/negative control로 비교하고 archive 결과만 주장한다. |
| 30 | 제한 fixture 구현을 전체 CSS Grid 완료로 문서화 | STATUS와 계약이 C11.4/C11 및 72/213을 미완료로 명시한다. |

## 판정

현재 확인한 네 scene의 제한 런타임 slice에는 위 실패 경로를 막는 검사 또는 범위 한계가 기록돼 있다. 이 표는 검토 기록이며 PR 필수 CI 성공이나 병합을 대신하지 않는다. Android는 API 37 software AVD, iOS는 Simulator drawable 경로다. 전체 WPT·실기기/hardware GPU·iOS compositor-after와 일반 CSS 사용 경로는 미검증이다. runtime PR이 병합되고 요구된 check 상태가 확인되기 전까지 공식 상태는 C11.4 미완료, 72/213으로 유지한다.
