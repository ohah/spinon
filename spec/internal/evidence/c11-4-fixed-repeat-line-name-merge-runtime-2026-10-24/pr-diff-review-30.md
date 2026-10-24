# C11.4 fixed-repeat named-line merge · PR #600 diff 실패 관점 검토 30개

**검토 대상:** PR #600의 구현 커밋 `2cf963a08cc2267fcd43ecb107ce808814c6c05d`와 그 diff에 포함된 Rust, Android/iOS fixture adapter, plan/status/contract, runtime evidence 및 PR 본문. 이 review artifact는 검토 기록용 후속 commit이며 아래 항목 30은 기능·플랫폼·문서 diff를 서로 다른 실패 경로로 나눠 확인한다.

| # | 실패 관점 | 결과 |
| ---: | --- | --- |
| 1 | 일반 `TrackList` outer names가 누락되거나 `values + 1`과 길이가 달라도 인덱싱 | cascade 진입에서 길이를 검사하고 expansion에서 각 group과 trailing group의 존재를 확인한다. |
| 2 | 두 번째 auto-repeat 또는 auto-repeat 부재에서 named boundaries 모호성 | 기존 single auto-repeat gate를 유지한다. 두 번째 auto-repeat은 unsupported value로 종료된다. |
| 3 | fixed-repeat count 0·음수·변환 overflow가 반복 수로 사용 | positive `usize` 변환만 승인하고 checked conversion 실패는 contextual cascade error다. |
| 4 | pattern이 비었거나 2 tracks 초과, `N+1` inner groups 불일치 | fixed group별 1–2개 track과 `track_sizes + 1` line groups를 검사한다. |
| 5 | percentage, unresolved calc, flex/intrinsic 또는 `minmax()` fixed pattern이 잘못 허용 | computed finite nonnegative `LengthPx`만 받아 unsupported breadth를 거부한다. |
| 6 | authored component 수·반복 횟수·expanded track 수의 overflow나 >10,000 | checked multiply/add와 기존 component/track caps를 유지한다. |
| 7 | auto-repeat 위치 전 fixed tracks가 잘못 expand되거나 start/end가 틀림 | `auto_repeat_start/end`는 실제 expanded `tracks.len()`로 계산되고 16-track runtime trace가 기대 순서와 맞는다. |
| 8 | fixed-repeat 내부 line names의 authored 순서·iteration이 섞임 | 각 repeat iteration과 inner group을 authored order로 순회한다. exact cascade group assertion이 통과한다. |
| 9 | repeat 내부 iteration seam에서 end와 다음 start가 다른 physical line로 분리 | index를 한 번만 증가시켜 양 group을 같은 output line에 붙이고 duplicate token을 보존한다. |
| 10 | 서로 다른 fixed-repeat와 ordinary track 사이 seam names 누락 | outer groups를 각 value 앞에 append하고 fixed inner trailing group과 다음 value leading group을 seam에 합친다. |
| 11 | auto-repeat 전/후의 fixed names가 auto component 경계에 잘못 복사 | outer vector는 non-auto track 경계 `N+2`개로 만들고 auto inner `N+1` groups는 분리한다. |
| 12 | auto-repeat 자체의 authored line names가 사라지거나 outer names와 중복 복사 | cascade test는 inner groups를 별도 필드에서 exact 비교하며 기존 named auto-repeat regression도 통과한다. |
| 13 | 동일 physical line의 duplicate identifier가 조기 제거·재정렬 | snapshot은 duplicate `join` token을 그대로 보존하며 named occurrence는 layout resolver 결과로 검증한다. |
| 14 | positive named occurrence가 고정/auto seam에서 엉뚱한 track을 선택 | positive `start`, `join`, `mid`, `seam`, `repeat-end`, `tail` placement의 rectangles가 고정값과 일치한다. |
| 15 | negative line index가 explicit grid의 끝이 아닌 시작 기준으로 계산 | negative column seam 및 negative row tail placement가 expected CSS px frames와 일치한다. |
| 16 | named span이 같은 이름의 중복 occurrence를 잘못 세거나 방향을 잃음 | `span 2 mid` layout frame과 precomparison occurrence validator가 통과한다. |
| 17 | auto-fit이 named placement 전에 collapse되어 target line이 이동 | placement item frame을 먼저 확정하고 비점유 repeated tracks 다섯 개가 이후 collapse된 trace를 양 플랫폼에서 확인한다. |
| 18 | row axis에서 columns용 boundary/name mapping을 재사용 | rows auto-fill의 16-track trace, y=96 item frame, row 방향 pixel samples가 Android/iOS에서 통과한다. |
| 19 | metadata budget에서 group headers, `String` headers 또는 UTF-8 bytes 누락 | checked 합산은 outer/auto groups, token headers와 bytes를 포함한다. exact 1 MiB 통과, +1 byte 거부다. |
| 20 | huge name list/multiplier가 metadata budget 산술을 overflow | checked name-count/name-byte multiplier가 overflow 시 node/property 오류를 반환한다. |
| 21 | outer group Vec 예약 실패 후 `resize_with` 또는 push가 fallible하지 않음 | `try_reserve_exact`가 성공한 뒤 resize/push하며 injected `LineGroups` failure는 contextual allocation error다. |
| 22 | nested name Vec 또는 owned String 예약 실패가 panic/부분 snapshot을 만듦 | `LineNames`/`LineNameText` 주입 오류를 검사하고 각 실패 다음 cascade를 다시 실행해 recovery를 확인한다. |
| 23 | track reservation 실패와 metadata 실패가 서로 다른 오류 계약을 사용 | track과 line metadata 예약 모두 같은 node/property `GridAllocationFailure`로 이어진다. |
| 24 | 실패한 cascade 결과의 일부 Grid metadata가 후속 호출에 남음 | 결과는 `Result::Err`로 반환되고 같은 fixture의 다음 호출은 성공한다. Taffy 내부 infallible allocation은 명시적으로 제외한다. |
| 25 | fixed-repeat 없는 기존 named auto-repeat/무명 fixed repeat 결과가 회귀 | 기존 fixed-repeat 관련 style suite 16/16 통과, named auto-repeat metadata가 이전 expected value를 보존한다. |
| 26 | platform fixture argument allowlist, expected root/item frames 또는 scene count 불일치 | Rust scene set 31개, capture catalog 464개 및 Android/iOS allowlists의 3 신규 ID가 focused fixture test와 simulator runs에서 통과한다. |
| 27 | Android/iOS axis pixel probes가 item 경계·fixture 밖 anchor를 잘못 표본 | Android AVD PixelCopy와 iOS drawable 각각 4 sample/scene, 12/12 exact다. frame·app hash가 run JSON에 기록됐다. |
| 28 | opt-in fixture data나 C ABI가 default product archive에 유입 | release archive default는 marker 0/3, ABI와 Rust symbol absent; fixture-test positive control은 marker 3/3 및 두 defined symbols present다. |
| 29 | PR 화면 URL이 404거나 이미지가 렌더되지 않음 | Agent Browser에서 PR #600을 열어 6개 screenshot image 모두 `complete=true`; Android 1080×2400, iOS 1206×2622 natural dimensions를 확인했다. 브라우저 session을 닫았고 후속 process scan에서 Agent Browser/HeadlessChrome가 남지 않았다. |
| 30 | 상태/계약/PR이 제한 fixture 후보를 C11 완료나 CI 통과로 과장 | plan, contract 0059, `spec/STATUS.md`는 제한 범위·미완료 C11.3.3/C11.4 및 `72/213`을 유지한다. GitHub PR checks는 현재 보고되지 않았고 C11 완료 또는 required CI 통과를 주장하지 않는다. |

## 최종 판정

이 30개 관점에서 PR 변경분의 차단 결함을 찾지 못했다. GitHub PR #600은 `CLEAN` merge state이나 check rollup과 required status checks가 없으므로 이는 CI 성공 근거가 아니다. Merge 전 실제 `gh pr checks`/merge 상태를 다시 확인한다. AVD는 software-rendered simulator이며 iOS Metal 표본은 compositor 제출 전이다.

## 최종 head 문서 delta 확인

검토 기록을 추가한 후속 PR commit `39a796c689645319eeb35ca5d11a70de6901c054`는 review 파일과 plan/status/contract/evidence 사이의 링크·상태 문구만 바꾼다. 해당 문구를 reread해 72/213, 미완료 C11.3.3/C11.4, 빈 GitHub check rollup과 merge 대기 상태가 실제 PR/GitHub 조회와 일치하는지 확인했다. 이 후속 delta에는 source, fixture, app adapter 또는 runtime artifact 변경이 없다.
