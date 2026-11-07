# C11.4 `safe|unsafe flex-end` 구현/runtime 실패 관점 20개

후보 HEAD `1bc9a21fc72a1426061337e63bdb323e714b70d9`, 고정 Chrome/독립 기대값, Layout/style projection/RuntimeSession 테스트와 Android API 37 ARM64 AVD·iPhone 17 Pro/iOS 26.2 Simulator의 세 기본 Root HTML route 실행을 검토했다. 계획 실패 관점 검토 및 PR diff 검토와 구분했다.

| # | 실패 관점 | 확인 결과 |
|---:|---|---|
| 1 | CSS computed 문자열이 정확 allowlist를 벗어나거나 값을 오인하는가 | 고정 profile의 `align-content` allowlist에 `safe flex-end`와 `unsafe flex-end`만 추가했다. 별도 allowlist control 1/1 통과. |
| 2 | 기대값이 Spinon 결과에 맞춰 사후 생성됐는가 | Chrome 157 관측과 독립 계산을 사전에 고정했다. 6 scene·72 coordinates의 비교 최대 오차는 0 CSS px다. |
| 3 | 양의 free space에서 safe alignment가 plain end와 달라지는가 | positive offsets `[10,20,30]`; Layout, projection, RuntimeSession과 두 Simulator에서 유지됐다. |
| 4 | 양의 free space에서 unsafe modifier가 잘못 fallback 되는가 | 같은 positive offsets `[10,20,30]`을 유지했다. 통과. |
| 5 | safe overflow가 끝 방향으로 계속 밀려 음수 위치를 내는가 | overflow offsets `[0,56,112]`; Android/iOS screenshot bounds도 같은 좌표를 측정했다. 통과. |
| 6 | unsafe overflow가 safe fallback으로 바뀌는가 | offsets `[-82,-26,30]`; 두 Simulator의 screenshot은 viewport 안 마지막 item만 검출했다. 통과. |
| 7 | plain `end` control이 새 값 허용으로 바뀌는가 | positive·overflow에서 기존 plain `end` 기대값을 matrix에 포함하고 유지했다. 통과. |
| 8 | 정렬이 implicit auto-row track base size를 바꾸는가 | 모든 alignment에서 세 row base가 `[10,10,10]px`로 유지된다. Layout trace 통과. |
| 9 | positive case의 0 gap이 잘못 해석되는가 | positive scene의 두 gutter가 각각 0px이고 item offsets가 기대값과 일치한다. 통과. |
| 10 | overflow `%`/`calc()` basis가 viewport 또는 inline 축으로 잘못 잡히는가 | definite 40px block basis에서 `calc(120% - 2px)`가 46px로 계산되고 두 gutters에 반영된다. 통과. |
| 11 | safe/unsafe distinction이 computed style projection에서 소실되는가 | RuntimeSession matrix가 두 computed 문자열을 각각 적용하고 item frames를 비교한다. 통과. |
| 12 | 이 제한 slice가 일반 CSS admission까지 넓히는가 | 기존 `RootHtmlAutoRowStretchV1` fixed shape gate만 사용한다. parser/API/global admission은 변경하지 않았다. |
| 13 | 다른 placement profile도 새 positional 값을 무조건 허용하는가 | layout admission은 해당 고정 profile일 때만 `safe|unsafe flex-end`를 허용한다. profile 경계 및 이전 controls 통과. |
| 14 | baseline 또는 unknown computed 값이 allowlist를 우회하는가 | `baseline`, `safe baseline`, unknown values는 거부된다. Layout error attribution은 `align-content`다. 통과. |
| 15 | 기존 safe center/start/end 동작이 회귀하는가 | 인접 C11.4 회귀 filter가 각각 layout 9/9, style 6/6, runtime 9/9 통과했다. |
| 16 | unsupported cascade 변경 뒤 stale/partial snapshot이 남는가 | 지원 light 상태 → baseline이 선택된 dark 상태에서 실패하고 completed snapshot을 비운다. RuntimeSession test 통과. |
| 17 | 실패 뒤 동일 session이 다시 지원 입력을 처리할 수 있는가 | 같은 RuntimeSession을 light 상태로 복구해 `Ready` 및 completed snapshot 재발행을 확인했다. 통과. |
| 18 | default app route가 fixture-only 기능 flag 없이 실행되는가 | Android와 iOS 모두 C11 fixture flags disabled, `layout=ready`, 6 boxes, runtime text 2 records다. 세 scene 통과. |
| 19 | Simulator screenshot이 layout 수치 근거보다 강하게 표현되는가 | safe positive/overflow의 visible bounds를 별도 pixel analyzer로 측정했다. 이 값은 OS screenshot visual bounds로 한정했다. |
| 20 | 화면 밖 unsafe item이 screenshot에서 안 보이는 사실을 layout 실패로 오판하거나 pixel 확인했다고 주장하는가 | 음수 두 item은 Rust frame 검증을 사용하고 screenshot은 마지막 visible item만 기록한다. Android/iOS/app/build/bootstrap의 원상복원 hash도 세 pass 모두 통과했다. |

세 Simulator pass 모두 기존 설치 앱과 build/bootstrap 자산을 복원했다. `PixelCopy`/Metal drawable readback, 실기기, hardware GPU, WPT 또는 전체 C11.4/C11 적합성은 이 검토의 범위가 아니다.
