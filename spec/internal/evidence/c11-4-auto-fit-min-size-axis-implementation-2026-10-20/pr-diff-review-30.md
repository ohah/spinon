# C11.4 auto-fit definite minimum-size 최종 PR diff 검토

최종 diff를 계획 검토 및 구현 실패 경로 검토와 별도로 읽었다. 아래 30개는 서로 다른 통합 경계·회귀·증거 신뢰성 관점이며, 각 관점에서 변경 파일과 실행 결과를 대조했다.

| # | PR diff 실패 관점 | 확인 및 판정 |
|---:|---|---|
| 1 | 빈 `auto-fit` track 변경이 auto-repeat 외 일반 track까지 바꾸는가 | 새 분기는 `RepetitionCount::AutoFit`과 빈 track 조건 안에만 있다. 범위가 고정됨. |
| 2 | 직전 vector element가 track일 때 track 자체를 gutter로 접는가 | 마지막 element의 `kind`가 `Gutter`일 때만 이전 gutter를 접는다. track element는 변경하지 않음. |
| 3 | auto-fit repeat 시작 직전 leading gutter가 중복되거나 누락되는가 | 최초 repeat track 앞의 emitted gutter가 같은 조건에서 한 번 접힌다. trace가 해당 폭 0을 확인함. |
| 4 | 연속 empty repeat에서 이미 접힌 gutter/track을 다시 처리하면 값이 되살아나는가 | 이후 empty track도 같은 0-size collapse 경로를 거치며 최종 trace에서 모두 collapsed로 유지됨. |
| 5 | occupied track 앞 gutter가 occupied track 판정에 따라 과도하게 접히는가 | preceding gutter는 현재 track이 empty일 때만 접힌다. 첫 occupied repeat scene의 prefix gap은 8px로 유지됨. |
| 6 | 빈 track 다음의 occupied track이 이전 빈 track의 trailing gap을 살리는가 | 이전 iteration에서 빈 track 뒤 gutter를 이미 접는다. second-repeat trace에서 이 gap이 0임. |
| 7 | empty range 사이의 동일 gutter를 양쪽 track이 각각 기록해 폭이 두 배로 되는가 | Taffy는 track 사이에 gutter 하나를 저장한다. 그 한 element를 collapse하며 simulator는 track 쌍별 gutter 하나를 검증함. |
| 8 | explicit suffix가 있는 template에서 빈 repeat의 마지막 gutter가 suffix 앞에 남는가 | suffix fixture와 기존 fixed-neighbor scene의 trace에서 suffix는 유지되고 인접 gutter는 0임. |
| 9 | repeat가 template 끝이면 마지막 occupied track 앞까지 역방향으로 접히는가 | reverse sweep은 마지막 비collapsed track에서 중단한다. second-repeat 장면의 170px occupied track을 보존함. |
| 10 | repeat가 끝나지 않았는데 trailing special-case가 다른 component를 접는가 | `is_last` 분기가 기존처럼 `current_track_index == counts.len()`일 때만 실행됨. suffix scene 통과. |
| 11 | 고정 count repeat 경로까지 새 auto-fit logic이 퍼지는가 | 새 동작은 auto-fit branch 안에 있어 고정 `Count` 분기와 별도임. 기존 fixed-count coverage는 영향을 받지 않음. |
| 12 | auto-fill scene의 collapsed 상태가 변하는가 | collapse 조건이 `AutoFit`만 검사한다. 33개 기존 scene을 포함한 두 플랫폼 행렬에서 auto-fill controls가 통과함. |
| 13 | row/column gap을 물리 축과 반대로 읽는가 | Taffy axis별 template/gap 추출을 그대로 사용하고 새 Rows/Columns 행렬의 8px gap trace가 모두 맞음. |
| 14 | minimum-size가 definite preferred-size처럼 repeat count 최대값으로 처리되는가 | Rust regression은 `max-content` preferred-size와 definite min을 함께 두고 `ResolveWithin`으로 실행한다. 190/200/342 경계 결과는 2/3/4회임. |
| 15 | min-size 기준 계산이 fixed neighbor track과 internal gap을 누락하는가 | plan 식·capture generator·Taffy trace가 양쪽 neighbor의 38+76n을 공유하며 190/200/342 경계에서 기대 반복 수가 일치함. |
| 16 | prefix-only, suffix-only, no-neighbor 케이스가 label과 sizing formula를 공유해 혼동되는가 | generator가 위치별 track/gap 수를 독립 계산하며 각 축에서 별도 장면을 실행함. |
| 17 | `minmax()`와 fixed pattern 결과가 같은 flexible expansion으로 처리되는가 | Rust fixture에서 `Pattern`으로 source를 분기한다. fixed pattern은 빈 track collapse만 검증하고 flexible used size를 재사용하지 않음. |
| 18 | test가 track count만 검사하고 axis/frame regressions를 숨기는가 | layout test는 두 축의 min frame과 initialized repeat track count를 모두 assert하고, 앱 행렬은 final used track/collapse/frame을 추가 검사함. |
| 19 | boundary test의 expected count가 길이 검사만으로 우연히 통과하는가 | 190/200/342 각각 2/3/4회로 명시하고 AutoFill/AutoFit 및 Columns/Rows 네 조합을 비교함. |
| 20 | fixture scene ID 충돌이 Android/iOS에서 다른 장면을 실행하는가 | Rust `BTreeSet` uniqueness test, JS 53-ID uniqueness assertion, native allowlist가 같은 20개 ID를 포함함. |
| 21 | source generator가 ID나 CSS token을 임의 값으로 주입받는가 | runtime source는 정적 `SCENES` enum 값만 생성하며 4KiB bound와 source string shape를 테스트한다. |
| 22 | second-repeat in-flow item의 CSS line이 실제 두 번째 반복의 flexible track을 가리키는가 | prefix가 grid line 1을 차지하므로 `grid-column:4`/`grid-row:4`가 반복 2의 첫 track이다. 실행 trace와 item 좌표가 이를 확인함. |
| 23 | span-two CSS placement가 2 tracks를 실제로 점유하는가 | `2 / span 2`와 `2 / span 2`가 각각 논리 축 두 track을 차지하고 양 축 scene에서 두 used track을 유지함. |
| 24 | 새 runtime scenes가 앱 allowlist에는 있지만 box/frame expectation에서 거부되는가 | Android·iOS adapter가 root/item/node frame 및 box count를 ID helper에 연결하며 두 플랫폼의 53 scene capture가 모두 통과함. |
| 25 | scene 이름 suffix parsing이 row/column 축 또는 190/200/342를 잘못 읽는가 | native helper가 allowlist ID의 마지막 정수만 크기로 읽고 `columns`/`rows`를 고른다. 전체 고정 ID 실행에서 root frame이 통과함. |
| 26 | 이전 `both-fit-second-repeat` correction이 새 scenes에 가려져 검증되지 않는가 | 기존 combined-axis scene도 별도로 남아 있고 x/y 20px, track 170px가 Android/iOS 결과에 포함됨. |
| 27 | 기존 single/span-two/fixed-neighbor assertions가 새 규범 값과 불일치하는가 | Android, Swift, simulator runner 모두 162px/134px 및 updated node coordinates를 같은 값으로 기대하며 53 scene 실행이 맞음. |
| 28 | capture runner가 collapsed track만 보고 gutter 폭을 오판하는가 | auto-fit minimum-size scene은 collapsed track 양옆 gutter를 모두 0px, 양쪽이 occupied인 gutter는 8px로 확인한다. |
| 29 | 플랫폼 capture 성공 표기가 실제 draw/present나 표면 경계 검증 없이 생성되는가 | runner는 scene별 `drawPresented`, 완전한 trace chunk, frame 및 track 검사를 요구한다. overflow 장면은 각 플랫폼 pixel summary와 4개 exact 표본도 요구함. |
| 30 | evidence/roadmap 문구가 제한 slice를 전체 C11.4 완료로 과장하거나 실패 run을 숨기는가 | status checkbox와 완료 수는 그대로며 WPT, out-of-flow, physical device, hardware GPU, iOS compositor-after 한계를 기록한다. README는 오래된 154px expectation, C11.1 out-of-flow 거부, 비활성 Metal fixture, 잘못된 simulator UDID 시도와 최종 53-scene 결과를 각각 구분한다. device ID·hash·screenshots가 연결됨. |

## 판정

최종 diff 관점 30개에서 수정이 필요한 결함을 발견하지 못했다. Android APK의 ZIP 검증 및 AVD에 설치한 APK SHA-256, iOS simulator app executable SHA-256은 각 실행 `simulator.json`의 hash와 일치한다. required GitHub checks와 merge state는 PR 생성 뒤 live 조회한다.
