# C11.1 intrinsic track + template-area PR 변경분 적대 검토 30개

**검토 범위:** `origin/main` 기준 source diff, 추가 projection/runtime tests, 고정 fixture, simulator evidence. 이 검토는 C11 전체나 broad crate regression을 통과 처리하지 않는다.

| # | 별도 실패 관점 | 실제 변경분 대조 결과 |
|---:|---|---|
| 1 | intrinsic selector가 일반 area-only Grid 경로를 탈취함 | selector가 `root_html_has_intrinsic_explicit_tracks`일 때만 새 mode를 선택한다. fixed-track area-only path는 그대로다. 통과. |
| 2 | intrinsic selector가 `grid-auto-flow:dense` 우선순위를 덮음 | dense route가 intrinsic branch보다 먼저 선택된다. intrinsic validator는 dense flow를 reject한다. 통과. |
| 3 | intrinsic selector가 column-flow placement를 허용함 | track projection gate가 column/dense flow 조합을 거부한다. 통과. |
| 4 | direct-text aspect-ratio owner가 새 mode로 잘못 분류됨 | selector는 aspect-ratio route가 아닐 때만 intrinsic mode를 고른다. 별도 validator도 조합을 닫는다. 통과. |
| 5 | intrinsic row/column과 implicit track을 동시 admission | auto rows/columns는 `Auto`만 허용하고 layout contract도 확인한다. 통과. |
| 6 | intrinsic track list의 빈 입력·10,000 초과·named line | projection/layout 양쪽에서 non-empty, cap, line metadata를 확인한다. 통과. |
| 7 | `NaN`, 무한대, 음수 길이의 track이 Taffy까지 전달됨 | Root HTML track allowlist가 finite non-negative 값만 통과시킨다. 통과. |
| 8 | invalid area map 때문에 intrinsic track map이 버려짐 | cascade selection과 projection이 같은 typed `template_areas`를 layout input까지 전달한다. 통과. |
| 9 | intrinsic area가 track 수보다 작거나 커도 implicit line 생성 | intrinsic mode의 area dimension은 두 explicit track list와 같아야 한다. 통과. |
| 10 | layout validator가 새 mode에서 dimension mismatch를 허용 | area dimension mismatch allowlist는 기존 template-grid mode만 포함한다. 통과. |
| 11 | template area + auto-repeat가 track sizing cap을 우회 | intrinsic track validator에서 auto-repeat을 reject하고 layout area validator도 intrinsic mode에 auto-repeat 허용을 주지 않는다. 통과. |
| 12 | invalid track map에서 stale repeat/line-name metadata가 남음 | per-axis grammar와 projection reservation이 repeat 및 line-name metadata를 막는다. 통과. |
| 13 | fixed/percentage maximum의 auto minimum이 기존 unsupported 범위를 넓힘 | 새 named-area helper가 `minmax(auto, px|%)`를 거부한다. runtime negative 1개가 atomic failure를 확인한다. 통과. |
| 14 | `minmax(min-content,max-content)`와 `minmax(0,1fr)` 경계가 바뀜 | 고정 minmax fixture는 positive이며 intrinsic layout 8/8·runtime area 15/15가 통과한다. 통과. |
| 15 | named placement를 받으려고 partial/fully-auto item까지 허용 | `RootHtmlIntrinsicTrackSizingV1`의 area 예외는 fully definite placement에만 적용된다. 통과. |
| 16 | named area의 reversed endpoint나 span line 정보가 사라짐 | computed `NamedLine` four-edge를 확인하고 resolved row/column spans를 그대로 남겨 sizing에 전달한다. 통과. |
| 17 | `grid-area`의 네 endpoint가 서로 다른 이름을 가진다 | `root_html_named_area_name`은 네 endpoint가 같은 occurrence 없는 `NamedLine`인지 확인한다. 통과. |
| 18 | invalid area CSS 후 named-line fallback이 no-area route에서 성공 | explicit/one-axis owner의 named-line/span fallback을 명시 거부하고 cascade fallback negative를 유지한다. 통과. |
| 19 | named area 정렬용 string table에서 동일 이름이 다른 area index로 덮임 | duplicate set 검증 뒤 `(name,index)` 정렬/binary search를 쓰며 이름과 입력 area index의 대응을 보존한다. 통과. |
| 20 | NUL/빈 area 이름, zero-size 또는 bounds 밖 rectangle | projection helper와 layout core 양쪽이 독립적으로 거부한다. 통과. |
| 21 | 행이 다른 두 area의 일부 겹침이 sweep에서 누락됨 | row event + active column interval 이웃 검사가 가로·세로 겹침을 차단한다. overlap runtime unit 통과. |
| 22 | 같은 경계에서 끝나는 rectangle과 새 rectangle이 false-positive로 충돌 | sort key가 end event를 start보다 먼저 처리하고 interval 비교는 strict overlap만 검출한다. 통과. |
| 23 | 동일 `column_start`를 가진 active rectangles가 BTreeMap entry를 덮음 | 중복 시작은 next interval의 시작이 기존 end보다 작아서 insert 전 reject된다. 통과. |
| 24 | area cell/metadata 곱셈이나 event 수 계산 overflow | `checked_mul`/`checked_add`와 1 MiB metadata cap으로 allocation 전에 reject한다. 통과. |
| 25 | allocator reserve 실패가 panic/partial area scene으로 노출 | 이름·events·direct items·used bitmap reserve를 `try_reserve`로 처리하고 구체적인 allocation error를 반환한다. 통과. |
| 26 | extra/missing/duplicate Grid item이 이름 map 밖에서 paint | visible item owner set을 named-area direct item set과 정확히 비교하고 매 map area를 한 번씩만 사용한다. 통과. |
| 27 | whitespace text와 anonymous visible text의 DOM 차이가 사라짐 | CSS whitespace만 skip하고 non-whitespace text를 거부한다. 통과. |
| 28 | hidden, nested, non-HTML element가 item 수로 계산됨 | hidden, nested non-text child, non-HTML/non-div를 각각 거부한다. 통과. |
| 29 | simulator screenshot이 다른 fixture 또는 잘못된 app entry에서 나옴 | APK 및 iOS bundle의 `index.html` SHA가 고정 harness와 일치하고 `source=index.html`, app marker, draw status 0을 확인했다. 통과. |
| 30 | simulator screenshot을 pixel parity로 과장하거나 실행 프로세스를 남김 | OS screenshots는 visual smoke로만 분류하고 PixelCopy/Metal readback을 하지 않았다. 두 앱과 Agent Browser는 종료했다. 통과. |

focused test, Clippy, rustfmt 및 두 simulator 실행 근거는 같은 evidence directory의 [README](README.md)에 연결했다. broad Root HTML suite 실패는 분리해 보존했으며 구현 통과로 표현하지 않았다.
