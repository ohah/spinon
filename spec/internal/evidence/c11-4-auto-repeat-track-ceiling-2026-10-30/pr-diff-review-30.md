# Auto-repeat track 상한 구현 PR diff 검토 · 실패 관점 30개

검토 범위: test-only Rust 변경, C11.4 계획·상태 대장 동기화, 실행 로그와 구현 review. 이 diff로 제품 지원 범위를 늘리지 않는다.

| # | 독립 실패 관점 | 확인 결과 |
| --- | --- | --- |
| 1 | C11 상위 항목을 완료 처리하는가 | 아니다. C11 체크는 그대로 미완료다. |
| 2 | C11.4 체크나 공식 완료 수를 바꾸는가 | 아니다. C11.4 미완료, `72/213`을 유지한다. |
| 3 | test-only 경계를 제품 CSS 지원으로 설명하는가 | 아니다. 계획·상태·evidence가 고정 Rust test라고 구분한다. |
| 4 | Stage C 상태를 이전보다 넓게 완료 처리하는가 | 아니다. 단일 fixed-size cap 경계만 확인했다고 적는다. |
| 5 | 9,999px 기대 개수가 틀리는가 | 양 축의 resolved count와 Initialize track 수가 9,999다. |
| 6 | 10,000px 기대 개수가 틀리는가 | 양 관측값이 두 축에서 정확히 10,000이다. |
| 7 | 10,001px 입력을 실패해야 한다고 오해하는가 | 아니다. 고정 Taffy 0.15.0은 10,000으로 제한하고 layout이 성공한다. |
| 8 | Rows와 Columns를 모두 시험하는가 | 예. 두 축은 각각 독립 실행한다. |
| 9 | `auto-fill` coverage가 빠졌는가 | 아니다. 세 크기 × 양 축에서 비collapse를 확인한다. |
| 10 | 빈 `auto-fit` collapse가 빠졌는가 | 아니다. 세 크기 × 양 축에서 collapse를 확인한다. |
| 11 | auto-fit 결과에 예상치 못한 item occupancy가 있는가 | 아니다. root child와 grid item이 없다. |
| 12 | 잘못된 axis basis가 반복 수를 만들 수 있는가 | 아니다. target axis root와 viewport 크기를 같은 값으로 고정한다. |
| 13 | gap이 반복 계산에 끼어드는가 | 아니다. helper는 zero gap을 사용한다. |
| 14 | resolved count만 믿고 stage 결과를 놓치는가 | 아니다. resolved explicit count와 non-gutter Initialize count를 비교한다. |
| 15 | gutter를 CSS track으로 세는가 | 아니다. `is_gutter` 항목을 걸러낸다. |
| 16 | 중간 pass를 최종 결과로 읽는가 | 아니다. 마지막 target-axis pass의 Initialize stage를 확인한다. |
| 17 | trace 예산이 넘은 partial output을 허용하는가 | 아니다. layout 성공과 총 100,000 미만 trace-state assertion을 요구한다. |
| 18 | 두 축 최대 trace가 함께 예산을 소진하는가 | 아니다. 한 입력에서 하나의 축만 최대 크기다. |
| 19 | prefix/suffix를 같은 크기로 만들어 잘림을 구분하지 못하는가 | 아니다. 2px prefix, 1px repeat, 3px suffix다. |
| 20 | 10,003px control이 cap에 너무 일찍 닿는가 | 아니다. 9,998 repeated tracks와 suffix가 포함돼 총 10,000개다. |
| 21 | 10,004px control이 cap에 도달하지 않는가 | 아니다. prefix 후 9,999 repeated tracks로 채우고 suffix를 자른다. |
| 22 | test가 Taffy의 unpinned host 설치본을 사용한다 | 아니다. Cargo는 vendored Taffy 0.15.0을 참조한다. |
| 23 | 새 test가 production Rust 경로에 등록되는가 | 아니다. `grid_track_sizing_tests`의 test module에만 연결된다. |
| 24 | fixture ABI, JNI, Swift 또는 앱 payload가 바뀌는가 | 아니다. 해당 소스와 artifact 설정은 수정하지 않았다. |
| 25 | 보통 300–500줄 파일 제한을 넘기는가 | 아니다. 새 Rust test module은 200줄 미만이다. |
| 26 | focused run에서 실제 테스트가 선택됐는가 | 예. 정식 경로로 2/2 pass를 log에 보존했다. |
| 27 | 넓은 Grid 회귀가 있는가 | 예. Grid 선택 134/134와 crate 전체 312/312가 pass다. |
| 28 | lint나 format 결과를 생략했는가 | 아니다. crate Clippy와 workspace rustfmt 결과를 보존한다. |
| 29 | UI가 없는 test에 screenshot을 붙였는가 | 아니다. 앱 scene이 없어 screenshot은 해당 없음이다. |
| 30 | 이 결과가 WPT·전체 auto-repeat·C11 완료를 증명한다고 읽히는가 | 아니다. 남은 grammar, placement, implicit, percentage/intrinsic, WPT 범위를 명시한다. |

**판정:** 30개 독립 실패 관점에서 최종 diff를 대조했다. 발견된 범위 과장이나 test-only 경계 위반은 없다. 계획과 evidence 문서에 대한 영어 초안은 프로젝트 한글 문서 규칙에 맞춰 이 PR에서 한글로 정리했다.
