# C11.4 auto-repeat intrinsic 재계산 PR diff 검토 · 30개 관점

구현·fixture·플랫폼 matcher·capture validator·계획·실행 evidence 변경분을 병합 전 별도로 검토했다. 각 관점은 서로 다른 경계 또는 실패 경로를 확인한다.

| # | 실패 관점 | 검토 결과 |
| ---: | --- | --- |
| 1 | 내부 fixture slice가 공개 API를 추가하거나 내부 contract version을 바꾼다. | 공개 API와 `0059` contract version은 변경하지 않는다. 변경은 opt-in fixture adapter, 검증, 계획/evidence로 제한된다. |
| 2 | 공식 완료 숫자가 구현 후보를 완료로 센다. | `spec/STATUS.md`는 수정하지 않았다. C11.4 checkbox와 공식 `72/213`을 미완료로 유지한다. |
| 3 | feature-off app이 신규 scene ID를 실행한다. | native allowlist와 Rust route는 기존 C04 runtime GPU fixture-test gate 안에 있다. default FFI archive에 새 scene string이 없고 fixture archive에는 positive control이 있다. |
| 4 | default Rust layout에서 전용 absolute-positioned probe exception이 활성화된다. | helper 모듈·호출·static-position correction이 전용 C11.4 fixture-test feature로 cfg 처리된다. default rlib에는 symbol이 없다. |
| 5 | 열 auto-repeat의 새로운 neighbor 규칙이 행에도 적용된다. | projection과 layout 모두 `grid-template-columns`만 허용한다. 행 negative control은 거부한다. |
| 6 | `auto-fit`이 auto-fill 전용 WPT workaround를 통과한다. | 양 계층에서 `AutoFill`을 명시적으로 검사하고 auto-fit negative control을 둔다. |
| 7 | intrinsic neighbor가 repeat prefix 위치에서 허용된다. | style-to-layout은 suffix 바로 다음 track만, layout은 `neighbor_index == prefix.len()`만 intrinsic으로 허용한다. prefix negative control이 있다. |
| 8 | trailing track이 여러 개여도 fixed-max 합계로 repeat 개수를 계산한다. | 각 계층에서 suffix 길이 1을 강제한다. 복수 suffix negative control이 거부된다. |
| 9 | 음수·무한대 maximum이 repeat preflight에 들어간다. | projection/layout에서 finite·nonnegative를 검사하고 음수·무한대 controls를 거부한다. |
| 10 | `auto`, `max-content` minimum 또는 percentage/flex maximum이 우회 허용된다. | 허용 variant는 `MinContent` minimum + 유한 비음수 `LengthPx` maximum뿐이다. 다른 breadth controls는 거부한다. |
| 11 | repeat count 계산 편의를 위해 min-content track sizing이 fixed maximum으로 덮인다. | count 계산에는 fixed maximum을 사용하지만 projected `GridTrackSize`와 Taffy input에는 `MinContent` minimum과 max를 유지한다. positive test가 typed output을 확인한다. |
| 12 | 지원 track 합산 중 누적 overflow를 놓친다. | 기존 layout neighbor sum의 finite/range 검사에 기존 overflow 경로가 그대로 적용된다. 새 branch는 finite 비음수 max만 합산한다. |
| 13 | root가 일반 grid인데 absolute children이면 모두 통과한다. | 예외는 content-box root, 정확한 intrinsic width/min-width, border/padding/margin, flow, repeat/list 및 line-name profile을 확인한다. |
| 14 | root가 relative containing block이 아니거나 inset이 달라도 예외가 적용된다. | root `position:relative`/auto inset과 자식 auto inset을 일치 검사하며 negative control이 있다. |
| 15 | in-flow item의 spanning placement가 바뀌어도 허용된다. | 단일 in-flow child가 정확히 row auto/column `1 / -1`인 경우만 통과한다. |
| 16 | probe line pairs가 중복·누락·재정렬된다. | 고정 `1/2`, `2/3`, `3/4`, `4/5` line pair와 child order를 검사한다. reorder negative control이 거부된다. |
| 17 | probe width·height·margin·padding·border·display가 WPT와 다르다. | absolute probe helper가 해당 layout style fields를 확인하고 inset·width·placement·root gap 회귀 controls가 거부된다. |
| 18 | fixture helper가 표시되지 않는 probe를 레이아웃 예외로 받는다. | hidden map의 child는 즉시 거부하고 마지막 hidden probe negative control이 있다. |
| 19 | generic static-position 보정이 grid-area 기반 probe frame을 덮는다. | C11.4 fixture feature의 TrackSizing Grid child에만 generic flow adjustment를 생략한다. 완성된 end-to-end frame matrix가 네 위치를 검증한다. |
| 20 | in-flow item 폭 제한 검증이 실제 WPT 입력을 차단하거나 scope를 넓힌다. | helper는 15·80·100·120px만 허용한다. positive matrix와 90px reject 대조가 통과한다. |
| 21 | V8 fixture source의 CSS가 Rust HostDocument sequence와 다르다. | 둘 다 같은 auto-fill/min-content suffix, 30px min-width, 2px border, 10px top padding, 20px auto-columns 및 probe styles를 사용한다. 최종 app frames/tracks는 고정 oracle과 일치한다. |
| 22 | route 추가가 기존 C11.4 scene count나 scene ID를 덮어쓴다. | JS capture matrix는 총 508 scene과 신규 8 scene 및 전체 ID 유일성을 assertion한다. |
| 23 | capture oracle에서 WPT state가 누락되거나 바뀐다. | capture script는 고정 `expected.json`을 읽고 states 길이 8을 검사한다. 최종 결과 ID와 순서는 runtime sequence 표와 같다. |
| 24 | repeat 수만 맞고 track·frame geometry가 달라도 성공 처리된다. | native matchers와 JS runner는 root/item/node frame, Columns track sizes, trace contribution 및 draw/present를 검사한다. 두 OS 최종 실행이 각각 8/8이다. |
| 25 | 허용오차가 과도하거나 플랫폼 좌표계를 혼합한다. | Rust tolerance는 1/64 CSS px다. JS 비교도 1/64 CSS px를 설정하고 root border/frame 및 child node frames를 별도 비교한다. |
| 26 | 폭 0 absolute probe를 frame 목록에서 제거하거나 그려진 box로 세어 false failure를 만든다. | frame 비교는 6 node 모두 유지한다. painted box count는 root/item과 positive-width probe만 계산하고 80px·15px states에서 확인한다. |
| 27 | runtime state sequence 성공을 독립 앱 scene으로 대체한다. | README와 계획은 HostDocument 8-state mutation test와 플랫폼의 독립 입력 8 scene을 구분한다. 두 증거를 서로 대체하지 않는다. |
| 28 | 실패한 구현 시도가 성공 근거와 혼합되거나 삭제된다. | bring-up 실패 로그를 보존하고 최종 Android/iOS run ID와 디렉터리를 별도 지정한다. 초기 실행은 final 8/8 수치에 포함하지 않는다. |
| 29 | PR 화면이 다른 실행의 screenshot이거나 설치물 hash가 달라진다. | 최종 screenshot은 최종 result 디렉터리에 보관되고, simulator metadata에서 설치 APK/app과 빌드 산출물 hash 일치를 확인한다. |
| 30 | 문서 또는 PR 문구가 범위를 전체 WPT/C11.4, 실기기, GPU pixel 증거로 확대한다. | 계획/evidence는 미지원 CSS, 실기기, hardware GPU, iOS compositor 이후 pixels 및 C11.4 전체 완료를 제외한다고 적는다. |

검토 중 찾아 수정한 90px scope escape는 #20에 반영했고, 수정 후 focused test·Clippy·Android/iOS rebuild와 8/8 captures를 다시 실행했다. 현재 blocking diff finding은 남지 않았다. 실행 근거는 [README](README.md)에 연결한다.
