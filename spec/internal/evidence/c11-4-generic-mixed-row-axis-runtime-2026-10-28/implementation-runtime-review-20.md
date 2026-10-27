# C11.4 generic mixed row-axis 구현·runtime 검토 · 20개 관점

구현 어댑터, route 경계, intrinsic/track trace, 두 플랫폼의 최종 simulator matrix를 계획 검토와 분리해 확인했다. review 대상 commit은 `0c89ffcd5043ce093bac8e4a1dfadeac062ff3c1`이다.

| # | 별도 실패 관점 | 검토 결과 |
| ---: | --- | --- |
| 1 | fixture adapter가 기본 제품 runtime 모듈에 포함된다. | 모듈은 기존 `c11-4-grid-track-sizing-fixture-test`와 `c04-runtime-gpu-fixture-test` 경계 안에 있다. 기본 archive에는 네 generic row scene ID가 없고 opt-in archive에는 전부 있어 static archive 경계가 확인됐다. |
| 2 | 새 generic row route가 fixed-ID row scene보다 먼저 가로챈다. | 기존 `SCENES` 고정 ID lookup이 먼저 실행된다. 새 matcher는 기존 route에서 찾지 못한 novel root만 받으며 기존 fixed row scene도 Android/iOS matrix에서 통과했다. |
| 3 | generic row matcher가 이전 generic column scene을 소비한다. | row matcher는 row intrinsic shape만 선택하고 `non_row_and_non_mixed_inputs_are_not_claimed`가 column control을 거부한다. generic column 네 scene은 기존 adapter와 양 플랫폼 matcher에서 계속 통과했다. |
| 4 | text-only 또는 image-only root가 mixed adapter에서 잘못 소유된다. | 공유 mixed signature가 meaningful text와 HTML img 둘 다 요구한다. runtime tests는 각 단독 root가 기존 route로 남는지 검사한다. |
| 5 | 잘못된 mixed 입력이 일부 노드만 처리한 결과로 낮아진다. | signature가 성립한 뒤 exact child count, direct parent, element kind, text topology와 image leaf/source를 검증한다. malformed row controls는 모두 fail-closed한다. |
| 6 | known HostNode ID만 우연히 성공하고 novel identity는 받지 못한다. | root/text/image ID를 기대값이나 route 조건으로 사용하지 않는다. 네 min/max·forward/reversed positive cases가 모두 novel ID로 성공한다. |
| 7 | DOM source order 변경이 intrinsic item identity나 frame 순서를 뒤섞는다. | Runtime tests는 source order 0/1과 text/image source kind를 node별로 검사한다. 플랫폼 trace matcher는 reversed 입력을 따로 대조하고 네 reversed scene이 양쪽에서 통과했다. |
| 8 | text shape 변경이나 nested text가 고정 Ahem 입력으로 오인된다. | 정확히 하나의 direct text node와 `XX XXXXX`만 허용한다. changed-text 및 nested-text negative cases가 거부된다. |
| 9 | image URL·leaf topology·decode natural size가 실제 intrinsic input과 달라진다. | 고정 URL, leaf img, PNG decode, `24×16` decoded dimensions를 확인한다. wrong/missing source와 nested child controls는 거부되고 trace는 natural size와 intrinsic revision 1을 기록한다. |
| 10 | stale document/style revision이나 다른 profile 결과를 받아들인다. | 공통 revision guard는 profile·generation·document·render-tree revision을 일치시킨다. revision/profile mismatch tests가 실패 경로를 확인한다. |
| 11 | computed initial CSS가 authored 값으로 잘못 정규화된다. | Generic row만 oracle와 같은 `content-box`, `normal` align-content/justify-content/justify-items, LTR와 horizontal writing mode를 요구한다. computed style mismatch controls는 fail-closed한다. |
| 12 | 다른 row track 수나 다른 column sizing도 같은 scene으로 허용된다. | Root는 `50px 24px` columns와 단일 min/max-content row만 받는다. 추가 row track·잘못된 columns controls가 거부된다. |
| 13 | root/item edge나 placement 변경이 고정 geometry 검증을 우회한다. | Root와 item edge, text/image column, shared row, static positioning을 확인한다. root edge, item edge, 잘못된 placement controls가 거부된다. |
| 14 | writing direction, writing mode 또는 font가 contribution 값을 바꾼다. | Root/text의 horizontal-tb LTR와 Ahem 10/10 paragraph profile을 고정한다. wrong-font control은 거부되고 양 플랫폼은 text intrinsic metrics `[50,20,80,10]`을 확인했다. |
| 15 | row `min-content`/`max-content` contribution이 column contribution과 혼동된다. | Rust runtime test와 capture verifier는 row pass에서 text 20px·image 16px을 두 track kind 모두 확인한다. 결과 row는 `max(20,16)=20px`이다. 이 네 입력은 두 algorithm 차이를 증명하지 않는다고 범위를 제한한다. |
| 16 | Columns/Rows pass 누락, 순서 역전 또는 최종 track 크기 오류가 있다. | Runtime tests와 두 native matcher는 Columns 후 Rows, columns `[50,24]`, rows `[20]`을 모두 검사한다. 32개 scene이 같은 고정 기대값으로 통과했다. |
| 17 | contribution 값은 맞지만 item/root frame이 다르다. | Root `74×20`, text `(0,0,50,20)`, image `(50,0,24,16)`을 Rust tests 및 platform frame lines에서 확인했다. |
| 18 | native scene 목록·node index·intrinsic source order가 Rust fixture와 어긋난다. | Android와 iOS matcher에 generic row 네 ID가 등록돼 있다. platform별 모든 16 scene이 frame·source kind/order·trace·draw 검증을 통과했다. |
| 19 | 예전 앱이나 다른 소스 commit의 캡처가 현재 구현을 가장한다. | 두 manifest가 같은 commit/tree를 가리키고 tracked diff hash가 empty다. Android 설치 APK hash와 build hash가 같고 iOS installed app hash와 build hash가 같다. |
| 20 | 화면 캡처를 pixel-perfect rendering 또는 전체 C11.4 완료로 확대한다. | 실제 screenshot은 host surface와 layout box를 보여주며 glyph/texel equality를 증명하지 않는다. Android real device/hardware GPU, iOS compositor-after pixels, 일반 CSS admission, 전체 WPT와 C11.4는 미검증으로 남긴다. 공식 집계는 `72/213`이다. |

Focused runtime/FFI tests, simulator evidence와 artifact 경계는 [README](README.md)에 연결돼 있다. Feature-enabled 전체 workspace Clippy는 변경되지 않은 기존 `type_complexity`와 `map_identity` lint에서 막혔다. 두 lint class만 허용한 재검사는 통과했으며, 해당 실패와 범위는 README에 함께 기록했다. 구현 변경에서 새 blocking runtime finding은 남지 않았다.
