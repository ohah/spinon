# C11.4 fixed-repeat 두 그룹 구현/runtime 실패 관점 검토 20개 · 2026-10-24

계획 검토와 분리해 실제 Rust 변경, opt-in fixture, 앱 validator와 최종 simulator 실행을 검토했다. 적용 범위는 단일 axis/list에서 두 fixed-repeat 그룹과 auto-repeat 한 개를 다루는 제한 profile이다.

| # | 실패 관점 | 검토 결과와 근거 |
| --- | --- | --- |
| 1 | 세 번째 fixed-repeat이 두 그룹 제한을 우회하는가 | fixed-group array가 두 칸이며 추가 integer repeat 발견 시 바로 `UnsupportedComputedGridValue`를 반환한다. 세 그룹 negative input은 `fixed_repeat_mixed_profile_rejects_names_non_px_tracks_and_extra_groups`에서 확인했다. |
| 2 | 같은 list의 두 번째 auto-repeat이 마지막 mode를 덮어쓰는가 | `auto_repeat_index.replace(index)`가 기존 index가 있으면 거부한다. 이번 신규 scene은 auto-repeat을 정확히 하나만 사용하며 이 기존 fail-closed gate는 변경하지 않았다. |
| 3 | zero/negative/unrepresentable integer count가 usize cast에서 통과하는가 | `usize::try_from` 뒤 양수 필터를 적용하므로 변환 실패와 0을 거부한다. CSS parser가 만들 수 없는 음수 count는 직접 computed AST fixture가 없어 별도 입력 실행 근거로 주장하지 않는다. |
| 4 | 비어 있거나 3개 이상인 fixed pattern이 열리는가 | 각 group에서 pattern 길이 `1..=2`와 line-group 개수를 확인하고 나서 확장한다. 3-track pattern negative control이 통과한다. |
| 5 | percentage, unresolved calc, flex, intrinsic 또는 `minmax()` fixed breadth가 length profile에 섞이는가 | `track_size` 결과가 finite nonnegative `LengthPx`인지 확인한다. CSS-level negative controls는 percentage, mixed calc, minmax, flex, min-content를 포함한다. |
| 6 | NaN, infinity, 음수 LengthPx가 metadata/track으로 전달되는가 | 수집 시와 실제 push 직전에 finite/nonnegative 조건을 검사한다. 확장 중 재검사로 validation/output 경계가 갈라지는 경우도 fail closed다. |
| 7 | outer/fixed/auto line names가 조용히 합쳐지거나 잘리는가 | fixed-group 경로는 outer, 각 fixed repeat, auto repeat 내부 line-name 중 하나라도 비어 있지 않으면 거부한다. named fixed와 named auto-repeat negative controls가 이를 고정한다. |
| 8 | count × pattern 길이 연산이 overflow하는가 | `checked_mul`을 사용하고 실패를 node/property가 있는 computed-grid 오류로 바꾼다. |
| 9 | 두 그룹의 expanded track 합이 10,000 한도를 넘는가 | group마다 checked cumulative addition 뒤 `MAX_COMPUTED_GRID_TRACKS`를 적용한다. 합계 10,000은 통과하고 10,001은 거부한다. |
| 10 | ordinary tracks와 auto-repeat pattern 추가로 total descriptor 한도를 넘는가 | list를 순회하며 각 component를 checked-add하고 같은 10,000 한도를 적용한다. fixed group 외 track도 검사 대상이다. |
| 11 | prefix/split/suffix 순서에서 fixed와 ordinary track의 개수가 line metadata 경계와 어긋나는가 | authored list를 순회해 확장하고 metadata group 수를 expanded fixed/ordinary tracks와 auto pattern에서 계산한다. 네 배치가 Rust trace와 두 앱 capture에서 통과했다. |
| 12 | fixed group 뒤/앞 위치에 따라 auto-repeat start/end가 원래 index에 머무는가 | auto-repeat index가 아닌 `tracks.len()`을 push 전후로 기록해 expanded index로 rebasing한다. prefix·split·suffix 및 reverse order assertions가 start/end를 확인한다. |
| 13 | 예약된 vector capacity와 실제 push 수가 다르거나 부분 output이 노출되는가 | fixed expansion 전에 checked total을 계산하고 reserve한다. 10,000 경계에서 output length와 line group length를 검사하며 10,001 입력은 layout 결과를 반환하지 않는다. |
| 14 | 두 fixed group 사이에서 pattern 내부 순서가 뒤집히거나 그룹 count가 섞이는가 | 그룹별 count와 authored index를 보존해 list 순서로 확장한다. reverse group과 multitrack pattern의 전체 track 배열을 assert했다. |
| 15 | multitrack repeat이 auto-repeat start를 한 track이 아닌 group 경계로 잡는가 | `repeat(2,20px 12px)`가 4개 fixed track으로 확장되는 scene의 auto-repeat span과 최종 track 배열이 Rust test 및 Chrome 비교에서 일치한다. |
| 16 | rows 검증이 columns에 적용되거나 두 축을 한 scene에서 동시에 확장하는가 | row scene은 `repeatAxis=rows`이고 반대 축은 단일 fixed track이다. Rust, AVD, iOS에서 row trace와 axis-specific root/item geometry를 확인했다. |
| 17 | empty `auto-fit`이 반복 횟수 계산 전에 사라져 인접 fixed group index가 달라지는가 | empty column/row 장면의 used tracks가 0으로 collapse되며 fixed suffix 위치와 line/group count는 유지된다. 두 simulator의 trace가 통과했다. |
| 18 | occupied auto-fit item이 fixed neighbor로 이동하거나 collapse 대상 track에 잘못 배치되는가 | expected item은 column `(110,0)`, row `(0,110)`의 10×10 frame이고 사용 auto-fit track은 186px다. Android PixelCopy와 iOS Metal sample에서 root/item 색 표본이 모두 exact다. |
| 19 | track/line-group/line-name reservation 실패 뒤 panic, partial result 또는 다음 호출 오염이 생기는가 | 세 injection site 모두 contextual `GridAllocationFailure`를 내고 guard 종료 후 동일 fixture 재계산이 성공하는 테스트를 통과했다. |
| 20 | 신규 fixture가 기본 artifact나 미등록 scene 경로로 노출되는가 | 10개 고유 ID와 unlisted ID rejection을 fixture test에서 확인했다. 기본 debug static archive에는 scene marker/fixture ABI가 없고 opt-in positive control에는 모두 있다. AVD/iOS 각각 10/10 최종 runtime capture가 통과했다. |

## 판정과 한계

검토 중 발견한 Clippy `needless_range_loop`를 array-index loop에서 mutable iterator로 바꾸고, 변경 후 format·11개 style test·FFI fixture test·scoped Clippy와 두 simulator 캡처를 다시 실행했다. 공식 C11.4 checkbox는 변경하지 않았다. 실기기, Android hardware GPU, iOS compositor 이후 pixels, WPT, 세 번째 fixed group과 일반 CSS admission은 이 검토 범위가 아니다.
