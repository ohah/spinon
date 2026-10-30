# C11.4 break-spaces 구현·runtime 실패 관점 검토 20개

검토 대상은 현재 후보의 Rust text normalization/layout, anonymous Grid fixture admission, runtime/FFI 장면 계약 및 두 모바일 simulator validator다. 결론은 이 고정 Ahem/ASCII/LTR fixture profile 안에서만 유효하다.

| # | 실패 관점 | 확인 결과 |
| --- | --- | --- |
| 1 | 새 white-space variant가 기존 public/general profile에 무심코 admission될 수 있음 | `spinon-layout`의 C11.4 anonymous paragraph 검증 경로에만 추가 허용됐다. 일반 author CSS admission은 추가되지 않았다. PASS |
| 2 | `WhiteSpaceMode` exhaustive match에서 `BreakSpaces`가 빠질 수 있음 | Parley mapping, normalization helper, text layout switch와 fixture switch를 찾아 연결했다. `cargo check`를 포함한 test/build 경로 통과. PASS |
| 3 | `break-spaces`가 `pre-wrap` 또는 `pre-line`과 잘못 동치화될 수 있음 | `WhiteSpaceCollapse::BreakSpaces`로 직접 mapping하며 각 mode의 기존 mapping은 그대로다. PASS |
| 4 | Normal/NoWrap의 collapsing 및 wrapping 회귀 | 기존 16 scene이 양 simulator의 최종 26-scene 행렬에 포함되어 통과했다. PASS |
| 5 | CRLF가 하나의 segment break가 아닌 두 개의 줄바꿈으로 계산될 수 있음 | intrinsic/layout test에서 CRLF source ranges, normalized line과 block size를 검사했다. PASS |
| 6 | CRLF가 두 source node 경계에 나뉠 때 provenance가 손실될 수 있음 | 별도 text source 두 개의 source UTF-16/paragraph UTF-8 범위를 확인했다. PASS |
| 7 | 지원하지 않는 tab-size 동작이 조용히 적용될 수 있음 | `BreakSpaces` tab source는 normalize와 intrinsic/layout 양쪽에서 전용 unsupported error로 fail-closed한다. PASS |
| 8 | PreWrap의 기존 고정 tab expansion이 변경될 수 있음 | `pre_wrap_column` 증분과 tab expansion은 `PreWrap` 조건으로 남겨 두었다. 기존 PreWrap regression matrix도 통과했다. PASS |
| 9 | max-content에서 반복·trailing spaces가 제거될 수 있음 | `XX  XX`와 `XX  `의 Ahem advances/fixture max-content를 각각 60px/40px로 고정했다. PASS |
| 10 | preserved space 뒤 wrap opportunity가 min-content에 포함되지 않을 수 있음 | `XX  XX` min-content를 30px로 고정하고 layout/track trace까지 검사했다. PASS |
| 11 | 줄 끝 보존 공백이 잘못 hang되어 inline advance가 줄 수 있음 | soft-wrap 및 trailing-space tests에서 0 hanging advance와 비-hanging line advance를 검사했다. PASS |
| 12 | 25px available width가 30px min-content fragment를 잘라낼 수 있음 | BreakSpaces만 line-break width에 min-content inline lower bound를 사용하며 25px Grid frame와 두 30px advances를 따로 검사했다. PASS |
| 13 | min-content reflow 후 block intrinsic height가 이전 0-width probe 값으로 남을 수 있음 | min-content inline width에서 다시 break/reflow한 뒤 block size를 읽고 grid row size와 비교한다. PASS |
| 14 | forced LF 뒤/앞 space가 max/min-content height에 누락될 수 있음 | max-content 40×20 및 min-content track 30×30 결과와 3-line layout을 고정했다. PASS |
| 15 | line-break lower bound가 Normal/PreWrap에 적용될 수 있음 | 해당 분기 조건은 `WhiteSpaceMode::BreakSpaces`만 대상으로 하고 기존 scene matrix를 재검증했다. PASS |
| 16 | 공백뿐인 anonymous text가 Grid item 또는 GPU box를 만들 수 있음 | source text는 validator에서 확인하지만 결과 item/box와 frame은 0; 양 simulator의 최종 matrix도 `boxes=0`을 확인했다. PASS |
| 17 | whitespace-only text가 이어지는 block의 auto-placement cell을 소비할 수 있음 | `[20px,20px]` column에서 block이 첫 cell에 남고 별도 anonymous text contribution은 없다. PASS |
| 18 | unsupported Grid/profile/style이 새 mode를 통해 우회 admission될 수 있음 | fixed scene IDs, exact child/text, computed style 및 order/profile gate를 확인했다. 일반 style projection 경로는 변경하지 않았다. PASS |
| 19 | runtime, FFI 또는 capture scene 목록 중 하나가 빠지거나 중복될 수 있음 | Rust runtime/FFI 목록은 고정 26 IDs와 중복 검사, Node module boundary는 exact new IDs를 검사한다. PASS |
| 20 | simulator test가 geometry만 확인하고 실제 app draw/present 또는 trace 누락을 놓칠 수 있음 | Android/iOS 26 observations 각각 frame/track trace 및 `drawPresented=true`; retries는 0이다. 설치본과 local app hashes가 일치한다. PASS |

## 판정과 한계

20개 관점에서 이 범위의 block finding은 없었다. 이 검토는 CSS Text 전체, WPT 적합성, 임의 font/script, Unicode line breaking, 일반 HTML/author CSS, Android 실기기/hardware GPU, iOS compositor 이후 pixels 또는 전체 C11.4를 검증하지 않는다. 첫 Android 실행의 whitespace-only box 기대값 결함은 고친 뒤 전체 양 플랫폼 matrix를 다시 실행했다. [실행 근거](README.md)와 [Chrome 사전 비교](../c11-4-anonymous-grid-text-break-spaces-precomparison-2026-10-31/README.md)를 함께 볼 것.
