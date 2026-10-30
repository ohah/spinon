# 구현 PR diff 적대 검토 30개

대상 PR [#837](https://github.com/ohah/spinon/pull/837), base `945c9781be074231bc4163d4847be4aa78b8fc3f`, 기능·상태 변경 검토 시점 head `28a20db902d874703f350a3da45f99a12a783d3e`. 실제 test diff, 계획·상태 대장·evidence 변경, 검증 로그, PR body와 labels를 대조했다. 이 검토 파일과 실행 README 링크는 검토 기록으로 뒤이어 추가했으며, 별도 code change는 없다. 구현/runtime 20개 검토와 별도다.

| # | 적대 검토 관점 | 판정 |
|---:|---|---|
| 1 | Grid 제품 구현을 바꾸면서 test-only라고 설명하는가 | **통과.** Rust 변경은 `#[test]`이 있는 grid test module뿐이며 production path 변경은 없다. |
| 2 | 기존 하나의 ceiling test를 무심코 교체하거나 약화하는가 | **통과.** 기존 count/collapse·source-order test를 보존하고 새 test function을 추가한다. |
| 3 | 10,000이라는 내부 cap을 CSS 규범 요구로 서술하는가 | **통과.** 계획은 vendored Taffy/Spinon 내부 cap으로 한정하고 §5.4 explicit-grid oracle 오용을 명시적으로 배제한다. |
| 4 | auto-fill 대신 auto-fit의 collapse 동작을 섞는가 | **통과.** 새 matrix는 auto-fill만 실행하고 collapse를 요구하지 않는다. 기존 auto-fit 회귀 검사는 그대로 있다. |
| 5 | exact-fit 기대 반복 횟수가 source fixed tracks를 무시하는가 | **통과.** `15,002px` 계산에서 fixed prefix/suffix 합 5px를 빼고 3px pair 4,999개를 사용한다. |
| 6 | overflow 수치가 cap 초과 source list를 실제 만들지 못하는가 | **통과.** `15,005px`은 prefix·10,000 repeated·suffix의 10,002 tracks를 요청한다. |
| 7 | track count가 repeated tracks만 세고 prefix/suffix를 누락하는가 | **통과.** resolved explicit count와 gutter 제외 stage count 모두 10,000을 요구한다. |
| 8 | clamp가 source prefix를 잃거나 suffix를 보존하는가 | **통과.** 전체 vector를 비교해 prefix `2px` 보존, exact-fit suffix `3px` 유지, overflow suffix 절단을 구분한다. |
| 9 | overflow가 pattern pair 경계에서만 잘려 named-line partial case를 놓치는가 | **통과.** overflow의 마지막 track은 pair 첫 `1px`이며 전체 expected vector에 포함된다. |
| 10 | 전체 sequence 대신 마지막 size만 검사해 내부 반복 순서 오류를 놓치는가 | **통과.** 10,000 base size를 모두 expected vector와 비교한다. |
| 11 | repeat line-name group count가 두 track pattern과 불일치하는가 | **통과.** 시작·중간·끝의 세 repeat metadata group을 제공한다. |
| 12 | outer line metadata validator 요구 길이를 놓치는가 | **통과.** 실제 첫 실패를 기록했고, 네 source track과 두 repeated track에서 요구된 네 빈 group을 helper에 반영했다. |
| 13 | outer names에 repeat names를 복제해 negative occurrence를 바꾸는가 | **통과.** outer group은 비어 있고 실제 이름은 repeat metadata에만 존재한다. |
| 14 | repeated axis의 `-1` named line이 마지막 occurrence를 가리키지 않는가 | **통과.** named start/middle child frame을 exact-fit과 overflow 모두 검사한다. |
| 15 | exact-fit named-line 좌표가 완전한 마지막 pair와 맞지 않는가 | **통과.** independent offset `14,996px`, size `1px`를 assert한다. |
| 16 | overflow named-line 좌표가 partial final track과 맞지 않는가 | **통과.** independent offset `14,999px`, size `1px`를 assert한다. |
| 17 | Columns/Rows 중 하나에서 start/end placement를 뒤집는가 | **통과.** 두 axis input을 분리하고 반복 축 frame의 x/width 또는 y/height를 각각 검사한다. |
| 18 | 직교 축의 다른 grid 때문에 trace 선택이 모호해지는가 | **통과.** 직교 axis는 고정 track 하나이고 helper는 요청한 axis의 final pass만 읽는다. |
| 19 | definite basis가 viewport/root/axis 중 다르게 주어지는가 | **통과.** root와 viewport의 반복 axis size를 같은 basis로 설정한다. |
| 20 | gap 또는 edge inset이 계산에 들어가 독립 반복 수를 바꾸는가 | **통과.** zero gap과 default zero edge style을 사용하며 그 외 inset을 주지 않는다. |
| 21 | frame 계산에 f32 정밀도 오차가 생길 수 있는 큰 수를 쓰는가 | **통과.** basis, 합산 위치와 sizes는 f32의 정수 exact range 안에 있다. |
| 22 | gutter가 explicit count/size assertion에 포함되는가 | **통과.** 공통 helper가 gutter를 거른 뒤 count 및 vector를 만든다. |
| 23 | 중간 pass의 track sequence를 최종 결과로 오인하는가 | **통과.** 반복 axis의 마지막 sizing pass에서 유일한 Initialize stage를 요구한다. |
| 24 | 다른 node trace나 여러 trace가 있어도 테스트가 통과하는가 | **통과.** trace 하나와 owner node `id(1)`을 고정한다. |
| 25 | 알고리즘 trace가 과도한 크기로 늘어도 이를 놓치는가 | **통과.** 전체 pass/stage의 captured track state 합이 100,000 미만이어야 한다. |
| 26 | 필터 오선택 0개 결과를 성공 증거로 홍보하는가 | **통과.** 0-test 실행은 실패 이력으로 명시하고 성공 카운트에서 제외한다. |
| 27 | 첫 실제 입력 검증 실패를 삭제하거나 원인을 숨기는가 | **통과.** `InvalidGridTrackList`, validator 길이 계약, 수정된 helper 및 rerun을 evidence에 남겼다. |
| 28 | 검증 범위를 넘어 앱/플랫폼 결과를 주장하는가 | **통과.** focused 3/3, Grid 135/135, crate 314/314, crate Clippy/rustfmt만 주장한다. Android/iOS simulator, screenshot, WPT와 일반 CSS admission을 명시적으로 제외한다. |
| 29 | 공식 완료 수나 C11.4 checkbox를 부분 테스트로 증가시키는가 | **통과.** `spec/STATUS.md`, 계획, PR body 모두 C11.4/C11 미완료와 `72/213` 유지로 정렬되어 있다. |
| 30 | 계획·status·evidence·PR 설명의 범위와 링크가 서로 어긋나는가 | **통과.** 계획, precomparison, runtime evidence, 20-item implementation review, test logs 및 PR labels/body를 같은 test-only slice로 맞췄다. |

**판정:** 30개 독립 관점을 검토했다. 구현에서 발견한 metadata input 결함을 수정하고 focused 및 wider Rust gates를 통과시켰다. CI check rollup은 비어 있고 main branch protection에 required status checks가 없으므로 CI 성공으로 세지 않는다.
