# C11.4 `grid-auto-fit-columns-001.html` 구현 PR #695 diff 적대 검토 32개

검토 대상은 PR #695의 구현 diff (`main` base `e37e0764483022d8587d832f4f384fbc5d4f8c0c`부터 implementation commit `d4e9cd7e37a973fd68360d8ac29f801df18416a8`까지)와 연결된 고정 WPT/runtime 근거다. final PR head `6fa3b18d64c1773c4661879614f17c9af5714032`의 후속 문서 commit은 이 review와 상태 문구만 추가했으며 32행 수, local links, `git diff --check`를 별도로 확인했다. 각 행은 서로 다른 입력·계산·native host·증거 실패 경로 하나를 다룬다. 기존 20개 implementation/runtime review와 별도로 수행했다.

| # | 실패 가설 | diff에서 확인한 근거·결과 |
|---:|---|---|
| 1 | fixture 데이터가 다른 WPT revision에서 왔거나 source가 바뀌어도 유지된다. | source manifest와 fixture가 revision `c999c58338ee1d223df5ad62e48be1202ced0d37` 및 pinned input hash를 기록하고, Rust adapter test가 revision을 직접 대조한다. 불일치 없음. |
| 2 | WPT root 또는 direct item 일부가 빠졌는데 전체 통과로 표시된다. | fixture JSON 수와 adapter 테스트가 root 35개·direct item 37개를 각각 세고, Android/iOS summary도 각 35개 scene의 성공을 기록한다. 누락 없음. |
| 3 | duplicate, 순서 변경, 잘못 채운 scene ID가 다른 scene 데이터로 연결된다. | adapter 테스트가 index별 canonical `grid-auto-fit-columns-01`…`35`와 중복 없는 set을 확인한다. 입력과 WPT index의 어긋남 없음. |
| 4 | `;throw 1` 같은 scene ID가 allowlist를 우회한다. | 고정 JSON에서 정확히 일치하는 ID만 찾고 FFI test가 injection 문자열·`00`·`36`을 거부한다. 두 simulator의 `36` 거부 control도 eval 전 중단한다. |
| 5 | root/item CSS의 따옴표나 escape가 생성 JavaScript를 탈출한다. | ID와 style literal은 `serde_json::to_string`으로 인코딩되고, code path는 embedded allowlist의 값만 source로 직렬화한다. 임의 CSS 문자열은 FFI에 전달되지 않는다. |
| 6 | DOM 생성이 item을 중첩하거나 WPT 순서를 뒤바꾼다. | 생성기는 root의 direct child로 JSON 순서대로 append하고, adapter test가 element 및 `appendChild` 수를 item 개수와 대조한다. |
| 7 | scene JSON의 필수 key/타입이 빠지면 부분 document를 렌더한다. | `runtime_source`가 필수 field를 모두 `Option`으로 검증하고 실패 시 `None`; FFI dispatcher는 eval 전에 error 반환한다. 지원 데이터는 고정 compile-time JSON이다. |
| 8 | geometry 기대값이 Chrome 구현 관측에서 역산된다. | WPT-authored expected input과 독립 repeat expectation 생성기가 별도 파일로 유지되고, runtime report는 이 값을 비교 oracle로 싣는다. Chrome CSSOM 관측은 used track 대조 자료로 구분한다. |
| 9 | WPT `background` shorthand 정규화가 이미지나 position 효과를 임의로 제거한다. | adapter는 이미지·position 등 추가 shorthand effect가 없는 고정 cyan/green 선언만 `background-color`로 투영한다. 일반 CSS admission은 변경하지 않았고 paint samples가 두 플랫폼에서 일치한다. |
| 10 | definite percentage auto-repeat의 minimum을 repeat count에 반영하지 않는다. | layout 및 style projection에 percentage repeat 최소 크기 경로와 columns/rows positive control을 추가했다. layout·style tests 및 두 simulator의 35 scene matrix가 통과했다. |
| 11 | width/height의 border-box나 padding을 percentage basis로 잘못 쓴다. | repeat trace는 definite content basis와 200px authored basis를 대조한다. WPT case 2 padding/outer frame control과 양 플랫폼 scene 02가 통과했다. |
| 12 | indefinite content axis에서 퍼센트 auto-repeat을 임의의 0px/초기값으로 계산한다. | definite basis가 없으면 repeat minimum은 거부된다. 전용 layout test가 이 경우 error를 요구하고, FFI runtime은 오직 definite WPT scene만 allowlist한다. |
| 13 | 비정상 percentage 또는 곱셈 overflow가 유한한 fixed breadth로 취급된다. | percentage와 resolved basis는 finite 및 범위 조건을 검사하고, neighbor resolve helper도 유한한 `f64`와 `f32::MAX` 경계를 확인한다. |
| 14 | `minmax()` neighbor에서 max/min floor를 뒤집어 auto-repeat 수가 달라진다. | px·percentage min/max를 resolved breadth로 비교하며 max가 minimum 아래일 때 floor를 적용한다. 새 track-area 사례와 WPT repeat-count matcher가 통과했다. |
| 15 | intrinsic/auto/flex neighbor 형태가 fixed size인 것처럼 무리하게 허용된다. | projection 및 layout은 명시 whitelist만 받으며, 나머지는 `InvalidGridTrackList`/unsupported computed value로 거부한다. 기존 fixed-repeat provenance 검사도 남아 있다. |
| 16 | placement로 생긴 implicit column이 explicit auto-repeat count에 더해진다. | final trace는 explicit·negative implicit·positive implicit count를 따로 기록한다. overflow cases의 used list와 각 count를 비교하고 35 scene matrix가 통과했다. |
| 17 | item span이 점유한 auto-fit 반복 track까지 empty로 collapse한다. | matcher는 scene 15/18/19의 contribution kind·item·span을 확인하고, 최종 collapsed indices를 used tracks와 대조한다. 양 플랫폼 trace matcher가 통과했다. |
| 18 | zero-size authored track과 actual collapsed track을 혼동한다. | auto-fit collapse 기대값은 final used track size 0에만 연결하고 auto-fill은 collapse를 허용하지 않는다. auto-fill/auto-fit layout controls 및 35 scene trace가 통과했다. |
| 19 | collapsed track 양쪽 gutter 또는 trailing collapsed run의 gap이 남는다. | 각 pass의 track/gutter index, adjacency, gap base size와 collapsed state를 확인한다. nonzero gap 및 empty prefix/suffix scene들이 양 플랫폼에서 통과했다. |
| 20 | repeated line-name groups의 join 순서나 동일 이름 occurrence가 깨진다. | matcher는 CSSOM used track list의 각 line name set과 occurrence를 parse해 순서별로 비교한다. repeated/fixed boundary 장면의 Android/iOS trace가 통과했다. |
| 21 | negative numeric line, named occurrence 또는 missing-name fallback이 엉뚱한 track을 가리킨다. | pinned scene의 authored placement와 WPT geometry를 비교하고 named/negative occurrence가 포함된 scene 14, 26–31에서 양 플랫폼 item frame이 맞았다. |
| 22 | axis pass 또는 final resolved list가 누락돼 stage trace 일부만으로 성공한다. | matcher는 grid trace 수, Columns/Rows sequence, repeat count, 두 used axes와 각 stage track array를 검사한다. 모든 35 scene에서 통과했다. |
| 23 | null, 과장된 길이 또는 잘못된 UTF-8 scene ID가 FFI 경계를 넘어간다. | FFI가 output/scene pointer, 128-byte 길이, UTF-8 및 allowlist를 검사한다. invalid ID/null argument tests와 두 simulator rejection control이 통과했다. |
| 24 | pixel FFI의 `sample_count * 2` overflow/null pointer가 slice 생성 전 통과한다. | `checked_mul`, 1–64 count, null 검사 후에만 slice를 만들고 `try_reserve_exact` 실패도 처리한다. invalid pointer/count test가 통과했다. |
| 25 | fixture FFI symbol/string이 일반 product archive에 들어간다고 오판하거나 누락을 놓친다. | 기본/opt-in macOS ARM64 archive를 순차 비교했다. product 0 symbol·0/35 marker·expected marker 없음, fixture positive control 2 symbols·35/35 marker다. APK/IPA 제외를 증명한다고 주장하지 않는다. |
| 26 | Android summary만 성공하고 실제 render draw 전의 splash를 scene screenshot으로 쓴다. | 이전 runner defect를 수정해 summary PID와 동일한 process의 `status=0 presented boxes=N` marker를 기다린다. 최종 Android matrix는 35/35 draw/present다. |
| 27 | 이전 splash screenshot을 최종 PR 화면으로 계속 사용한다. | superseded `141144Z` screenshot과 summary를 별도 이름으로 보존하며 runtime README에서 사용 금지를 명시한다. PR 본문에는 이후 `141559Z` scene 07을 넣었다. |
| 28 | Android Surface recreation 후 overflow viewport가 320px로 되돌아가 표본이 틀린 위치를 읽는다. | pixel-only scene에서 viewport 480×240을 설정하고 surface sizing·request·sample bounds를 같은 폭으로 확인한다. 보존된 초기 mismatch 이후 최신 6 scene PixelCopy는 24/24 exact다. |
| 29 | iOS capture가 pixel-only 480px viewport와 drawable sizing을 혼합한다. | sample scene에만 480×240 CSS viewport를 적용하고 drawable readback request도 해당 폭을 요구한다. 6 scene의 iOS pre-compositor Metal readback은 24/24 exact다. |
| 30 | iOS drawable readback을 compositor 이후 pixels나 일반 화면 결과로 과장한다. | runtime README 및 PR 본문은 측정 지점을 WGPU Metal drawable의 compositor 제출 전으로 한정하고, post-compositor/physical-device 범위를 제외한다. |
| 31 | root border 밖 item 위치를 geometry나 screenshot만으로 주장한다. | 여섯 overflow scene 각각에서 border magenta, border 밖 cyan item, item 끝 다음 배경, 별도 control을 surface에서 표본화한다. Android PixelCopy와 iOS drawable 각각 24/24 exact다. |
| 32 | PR 이미지 링크가 깨졌거나, 테스트·고정 WPT slice를 공식 C11.4 완료로 읽게 된다. | Agent Browser에서 PR #695의 두 `<img>`가 `complete=true`와 natural size Android 1080×2400/iOS 1206×2622인 것을 확인하고 세션·프로세스를 종료했다. 문서와 본문은 C11.4 미완료, 공식 `72/213` 유지, checks 미보고 상태를 적는다. |

**구현 PR diff 적대 검토:** 서로 다른 실패 가설 32개를 PR #695 실제 diff와 연결 증거에 대조했다. 이 범위에서 미해결 코드 결함은 찾지 못했다. PR #695는 Rebase and merge로 `d6dd9fd15d2897804142e6bb119149a633952a9b`에 병합됐다. GitHub `gh pr checks 695`는 병합 전 `no checks reported`를 반환했으므로 CI pass로 해석하지 않는다. 기존 20개 implementation/runtime review와 이 PR diff review는 계획 PR #694의 리뷰 기록과 구분한다.
