# C11.4 auto-fit named-line implicit fallback PR diff 검토 30개

계획 검토 20개와 구현/runtime 검토 20개에서 다룬 항목을 재사용하지 않고, PR 전체 diff의 통합 경계·계약·증거 게시 경로를 따로 점검했다. 판정은 최종 source diff, 두 simulator 성공 run, 계약·상태 문서 및 PR #537의 게시 결과를 기준으로 한다.

| # | 실패 관점 | 관측 및 판정 |
| ---: | --- | --- |
| 1 | 기능 slice 때문에 제품 layout algorithm이 의도치 않게 바뀌는가 | 제품 layout source 변경은 없다. 추가 파일은 fixture data, platform fixture harness, capture runner, 상태·계약·증거 문서다. |
| 2 | 새 scene이 fixture-test 전용 경계 밖에서 앱에 노출되는가 | scene data는 기존 C11.4 fixture 경로에 추가됐고 제품 런타임 진입점이나 일반 CSS admission 변경은 없다. 기본/fixture-test static archive 대조는 별도 artifact 근거와 README에 범위를 한정했다. |
| 3 | 전체 scene 수 갱신이 ID 충돌을 가리는가 | JSON은 28개이며 고유 ID assertion을 유지한다. Rust focused test와 capture runner도 같은 개수를 확인한다. |
| 4 | 새 matrix를 추가하면서 기존 named auto-repeat 장면이 삭제되는가 | 기존 18개 scene을 보존하고 10개 auto-fit scene을 더했다. 두 simulator 성공 결과에 auto-fill 10개와 auto-fit 10개가 모두 들어 있다. |
| 5 | 신규 기대값이 요청 범위를 빠뜨리는가 | integer의 positive/negative 및 missing/insufficient, forward/backward missing-name span, occupied controls, row-axis 양 방향의 10개 조건이 명시되어 있다. |
| 6 | positive missing integer가 collapse된 leading repeat 이후 선을 잘못 고르는가 | 기대 track `[0,0,0,20,20,20,20,20,20]` 및 item x=100이 fixture·Android/iOS 기대값·성공 결과에서 일치한다. |
| 7 | positive insufficient occurrence가 authored 이름이 부족한 조건을 시험하지 못하는가 | `5 cell` 장면은 collapsed repeat 뒤의 x=20, 20px item과 5-track 기대값을 사용한다. |
| 8 | negative missing integer에서 trailing repeat collapse와 implicit fallback 방향이 뒤집히는가 | `-5/-4 ghost`는 trailing 세 repeat track이 collapse된 `[20,20,20,20,20,0,0,0]` 및 x=0 frame으로 고정됐다. |
| 9 | negative insufficient occurrence에서 같은 이름의 authored occurrence 수를 잘못 계산하는가 | `-5/-4 cell` 장면의 `[20,0,0,0]` track 및 x=0 frame이 기대값과 맞는다. |
| 10 | forward named span이 검색 기준 방향을 잃는가 | line 4부터 `span ghost`까지의 frame/track 기대값이 `[0,0,0,20]`으로 명시되고 양 플랫폼 결과와 일치한다. |
| 11 | backward named span이 forward 사례를 잘못 재사용하는가 | `span ghost`부터 line 1까지 별도 ID와 `[20,0,0,0]` track 기대값이 있으며 두 플랫폼에서 구별해 통과했다. |
| 12 | 빈 repeat과 점유된 repeat을 구분하지 못하는가 | 단일 점유 control은 `[0,40,0]`이고 item 40px다. collapse 결과가 미점유 사례와 분리되어 있다. |
| 13 | spanning item이 점유 track 두 개를 하나로 합쳐 버리는가 | span control은 `[0,40,40]`, item 80px로 독립 고정했고 두 플랫폼에서 일치했다. |
| 14 | row-axis fallback이 column 결과에 우연히 의존하는가 | 양수·음수 row controls가 `20×125` root와 Rows 전용 track 배열을 사용하며 column fixture와 별도 frame을 가진다. |
| 15 | serialized collapsed 표시와 track 크기가 서로 모순되는가 | 신규 auto-fit fixture에서 0px track에는 `collapsed=true`, 20px implicit/occupied track에는 `false`가 대응한다. README의 track 요약 및 simulator 결과와 일치한다. |
| 16 | 예상 배치가 gap이나 indefinite sizing 변화에 의해 모호해지는가 | 모든 새 scene은 definite 125px axis, 40px authored track, 20px implicit track, gap 0 profile로 고정돼 line-resolution 변수를 분리한다. |
| 17 | Android root metadata가 row scene을 가로 축으로 보고하는가 | Android root 기대값은 column `125×20`, row `20×125`로 분기한다. 최종 20개 결과가 맞고 앞서 기록된 row-axis 보정도 README에 설명돼 있다. |
| 18 | Android item frame 문자열이 scene 종류별 폭·좌표를 혼동하는가 | 정수 positive, row positive, occupied 40px, spanning occupied 80px가 별도 기대 분기를 가진다. 설치 APK hash가 빌드 산출물과 일치한 성공 run으로 확인됐다. |
| 19 | Android scene allowlist에는 있으나 PixelCopy 경로에 빠지는가 | 신규 10 ID가 Grid 및 named auto-repeat allowlist와 surface sample 매핑에 포함된다. 성공 run은 20/20 scene과 60/60 표본이다. |
| 20 | Android auto-fit 표본이 기존 named-auto-repeat 색상 분기에서 회색 sentinel을 받는가 | `auto-fit-named-*` RGB prefix 경로를 추가했고, 최초 sentinel 실패 수정 후 최종 PixelCopy 표본 60/60 exact를 기록했다. |
| 21 | iOS scene allowlist와 Metal 고정 sample selector가 서로 어긋나는가 | Grid/named allowlist와 `auto-fit-named-*` selector가 함께 갱신됐다. 최종 Simulator 결과는 20/20 scene 및 compositor 전 Metal 60/60 exact다. |
| 22 | iOS root/item frame 기대값이 Android와 달라 플랫폼별 성공처럼 보이는가 | 각 플랫폼 expected mapping을 fixture 기대값과 대조했고 20개 root/item/node frame 결과가 동일하다. simulator 모델·OS·앱 hash도 별도 metadata로 연결됐다. |
| 23 | row-axis pixel sample이 column 축 끝점이나 item owner를 읽는가 | row 표본은 x=4에서 y 방향 item 시작/끝을 읽고 root 내부 표본을 별도로 둔다. fixture node ID는 root/item owner와 일치한다. |
| 24 | 0px collapsed track 내부를 실제로 존재하는 surface 표본처럼 판정하는가 | collapsed track interior 표본은 없다. 관측 가능한 root background와 item 내부만 읽어 60개 exact 결과를 집계했다. |
| 25 | root 바깥 item 끝 표본을 root surface 소유로 오인하는가 | sample ID가 item end가 root 밖인지 계산하며 node owner는 item으로 유지된다. README는 border stroke와 root clip 일반 동작을 검증 범위에 넣지 않는다. |
| 26 | capture runner가 기존 전체 C11.4 matrix count나 새 fixture count를 놓치는가 | named fixture count를 28, 전체 C11.4 scene count를 307로 갱신했다. 중복 선택 및 요청 ID 누락을 잡는 기존 assertion도 보존했다. |
| 27 | 독립 oracle 결과와 runtime 기대값이 서로 다른 source를 사용하는가 | Chrome 156 사전 비교 JSON/입력과 CSS Grid 2025 CRD 근거가 연결돼 있다. track/group은 exact, frame은 `1/64 CSS px` 허용으로 사전 판정 기준이 기록돼 있다. |
| 28 | 실패한 초기 실행을 성공 수에 섞거나 다른 source revision을 검증한 것처럼 보이는가 | 최초 Android/iOS harness 실패는 별도 run directory에 남고 README가 제외했다고 설명한다. 성공 run에는 source diff hash, 앱/APK hash와 simulator metadata가 있다. |
| 29 | 제한된 simulator·archive 결과를 CSS 완성 또는 전체 bundle 제외로 확대 게시하는가 | 상태 대장과 계약 0059는 C11.4 checkbox 및 공식 `77/213`을 유지하고 계약 숫자 `0.1.0`도 그대로 둔다. 문서와 PR은 실기기, hardware GPU, WPT, iOS compositor 이후 pixels, APK/IPA 전체 제외를 명시적으로 범위 밖에 둔다. |
| 30 | PR 본문 링크가 올바른 파일·이미지로 렌더링되고 라벨 및 남은 범위가 유지되는가 | `enhancement`와 `area: layout` 라벨이 있다. 화면 이미지는 `refs/pull/537/head` raw PNG를 사용한다. 최초 점검에서 PR 상대 파일 링크가 `/pull/spec/...`로 잘못 연결되는 것을 찾아 최종 HEAD SHA의 GitHub `blob` 링크로 고쳤으며, 수정 본문은 Agent Browser에서 링크 대상 및 두 이미지의 natural size 로딩을 확인했다. |

병합 전 live GitHub 상태에서는 PR이 `CLEAN`이었고 check rollup은 비어 있었다. GitHub API는 `main`에 required status checks가 설정되지 않았다고 반환했다. Rebase and merge 전 이 상태를 다시 조회한다.
