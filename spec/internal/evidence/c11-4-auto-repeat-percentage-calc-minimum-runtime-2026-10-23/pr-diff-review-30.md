# C11.4 definite auto-repeat percentage `calc()` PR diff 실패 관점 30개

구현 diff를 cascade·layout·Taffy·fixture/host·증거·상태 대장 경계로 나눠 30가지 실패 경로에서 검토했다. 검토 중 0px calc minimum이 cascade/layout preflight를 통과할 수 있음을 발견해 양쪽에서 positive length 항을 요구하도록 수정했다. 0px 거부 테스트와 focused Rust suite를 실행하고 수정된 source 기준 Android/iOS simulator 6/6을 다시 확인했다.

| # | 실패 관점 | 검토 결과 |
| ---: | --- | --- |
| 1 | 임의 계산 함수가 calc minimum으로 승인됨 | Stylo와 layout이 `Sum`만 받는다. `max()`는 cascade에서 거부된다. |
| 2 | 합계 항이 세 개 이상인 값이 통과함 | 두 항 slice만 destructure하므로 추가 항은 거부된다. layout 거부 케이스가 있다. |
| 3 | `LengthPx + Percentage` 이외 순서가 거부됨 | 두 항 순서를 모두 match한다. Stylo cascade test는 percentage-first tree를 보존한다. |
| 4 | `calc(0px + 0%)`가 auto-repeat 0px minimum으로 통과함 | 첫 diff에서 확인했다. 길이 항을 `> 0`으로 제한하고 cascade/layout 거부 사례를 추가했다. |
| 5 | 길이 항의 NaN/infinity 또는 음수가 통과함 | cascade/layout에서 finite와 positive 조건을 적용한다. |
| 6 | percentage 계수의 NaN/infinity 또는 음수가 통과함 | cascade/layout에서 finite와 nonnegative 조건을 적용한다. |
| 7 | 0%가 positive absolute minimum과 결합될 때 잘못 거부됨 | percentage 0은 허용하고 positive length minimum은 보존한다. |
| 8 | 산술 부정이나 중첩 함수가 우연히 Sum으로 축약되어 승인됨 | 허용되는 양 끝 term type이 `LengthPx`와 `Percentage`뿐이다. |
| 9 | calc ID가 없거나 stale해 다른 식이 사용됨 | layout은 ID가 있는 math binding을 요구하고 기존 binding validator가 입력 ID 무결성을 확인한다. |
| 10 | 다른 node의 binding을 사용함 | binding `node_id`가 현재 grid node와 일치해야 한다. |
| 11 | columns 식에 rows property binding이 연결됨 | property를 axis에 대응시켜 일치 여부를 검사한다. |
| 12 | indefinite/cyclic 축에서 percentage가 임의 resolve됨 | 해당 축의 definite basis flag가 없으면 거부한다. indefinite rows 거부 테스트가 있다. |
| 13 | column percentage가 높이에서, row percentage가 너비에서 계산됨 | columns는 width definite, rows는 height definite flag를 확인한다. 양 축 positive scene에서 track/frame을 확인했다. |
| 14 | auto-repeat minimum이 아닌 max calc가 fixed minimum으로 허용됨 | preflight는 `MinMax`의 min calc만 허용하며 max는 LengthPx/Fraction으로 제한한다. |
| 15 | fixed neighbor 제한이 확장됨 | repeat 바깥 track은 계속 nonnegative finite `LengthPx`만 허용한다. |
| 16 | 잘못된 repeat 시작/끝이 허용됨 | 기존 `start < end`, `end <= tracks.len()` 및 line-name metadata 검사를 그대로 거친다. |
| 17 | Taffy가 basis 없는 calc를 확정값으로 resolve함 | Taffy calc `definite_value`는 `parent_size: Some`에만 값을 반환하며 layout gate도 definite 축을 확인한다. |
| 18 | calc support가 비활성 feature에서도 참으로 켜짐 | Taffy fixed-component calc 분기는 `feature = "calc"`로 cfg 처리된다. |
| 19 | fit-content calc pointer를 plain calc처럼 읽음 | `is_plain_calc()`만 분류해 fit-content calc 변형은 일반 fixed component로 취급하지 않는다. |
| 20 | repeat count가 1 또는 4/5로 계산됨 | Taffy callback unit test는 112px axis와 31.2px minimum에서 3 repetition을 assert한다. |
| 21 | percentage를 버린 구현이 같은 결과를 냄 | 20px-only라면 112px에서 5 tracks가 되므로 31.2px fixed control과 세 반복이 oracle 역할을 한다. |
| 22 | 1fr 사용 폭과 반복 수가 혼동됨 | 31.2px로 반복 수 3을 정하고 used tracks는 각각 112/3px다. independent expectation와 simulator 결과를 비교했다. |
| 23 | auto-fill에서 item/frame 누락 또는 순서 뒤바뀜 | columns/rows 각 세 item frame과 node count 4를 양 플랫폼에서 기록한다. |
| 24 | auto-fit collapse가 반복 전 count를 덮어씀 | trace와 최종 `[112, 0, 0]` tracks 및 collapsed flags를 따로 확인했다. |
| 25 | 마지막 item 잔여폭 처리로 auto-fit frame이 틀어짐 | capture에서 발견한 이전 host expectation 오류를 auto-fill residual 분기로 한정했다. 최종 auto-fit item은 112px다. |
| 26 | float 문자열 포맷 차이가 frame 검사를 깨거나 틀린 값을 통과시킴 | Android/Swift formatter가 정수와 분수를 구분하며 simulator geometry tolerance는 1/64 CSS px다. |
| 27 | 새 scene이 host allowlist 또는 catalog count에서 빠짐 | 양 플랫폼 allowlist, 12-scene ID 검사, 348-scene catalog boundary expectation을 갱신했다. |
| 28 | row auto-placement가 columns 방향을 따라 배치됨 | rows scenes는 `autoFlow: column`과 고정 48px columns를 명시하고 row tracks/frames를 확인한다. |
| 29 | stale simulator binary 또는 화면 해시가 최종 소스로 오인됨 | 최종 두 run의 `sourceRevision`, APK/installed APK 일치, app hash, 12 screenshot SHA-256을 새로 기록했다. |
| 30 | 내부 fixture slice가 제품 API나 전체 C11.4 완료로 표현됨 | product/fixture static archive 차이만 주장한다. `72/213`, C11.4 미완료, 계약 0059 `0.1.0`, 실기기·GPU·compositor pixel 한계를 유지한다. |

최종 실행은 API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator 각 6/6 통과다. [실행 근거](./README.md)와 [문서 빌드 결과](./docs-build-result.md)를 참조한다. docs build는 이 slice 페이지를 제외한 기존 dead-link group 811개 때문에 실패로 남았다.
