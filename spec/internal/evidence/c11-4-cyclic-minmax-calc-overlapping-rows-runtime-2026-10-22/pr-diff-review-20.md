# C11.4 cyclic `minmax(calc())` 겹치는 row span PR diff 실패 관점 20개

PR #503의 최종 코드·fixture·플랫폼 검증기·실행 근거·상태/로드맵 변경을 서로 다른 실패 경계로 검토했다. 기준 diff는 `b4fa4725500b3a081f93174be547257659c76f3e...f1bbdcbd`다. 계획 리뷰와 구현/runtime 검토를 반복하지 않고 PR diff에 들어온 연결부와 외부에서 판정할 근거를 확인했다.

| # | 독립 실패 관점 | 확인 결과 |
|---:|---|---|
| 1 | 새 scene ID가 FFI registry·runtime root matcher·Android·iOS 중 일부에서만 등록됨 | 여섯 ID가 네 경계와 simulator scene 생성기에 동일하게 연결되고 최종 양 플랫폼 run에 모두 나타난다. |
| 2 | 유사하거나 임의인 ID가 고정 fixture로 수락됨 | FFI와 runtime whitelist가 정확한 여섯 ID만 반환하며 unlisted/arbitrary span ID negative assertion이 있다. |
| 3 | fixture-test 코드가 기본 runtime/제품 API로 노출됨 | 기존 `c04-runtime-gpu-fixture-test` 경계 아래 연결했다. 기본/positive-control release archive byte scan은 신규 ID·error marker의 부재/포함을 각각 기록한다. 전체 APK/IPA 제외로 확대하지 않았다. |
| 4 | 생성 HTML의 root/child shape가 계획 입력과 달라짐 | root 한 개, direct `div` 세 개, text node 세 개, 정확한 DOM 순서와 ID를 FFI source assertion 및 runtime adapter가 검사한다. |
| 5 | row 장면이 column axis sizing을 검증하거나 두 축 값을 섞음 | root는 고정 10px Columns와 세 calc Rows를 요구하고 gap도 Rows 2px/Columns 0px로 분리한다. capture 결과에서 두 축 trace가 별도다. |
| 6 | calc track expression이 순서·기준값·max function이 다른데 허용됨 | typed runtime 검증은 세 `minmax(calc(10% + 2px), max-content)` track을 해당 axis에서 요구하고 다른 track matrix는 거부한다. |
| 7 | row placement line 또는 span 길이가 DOM source order와 어긋남 | 세 item의 line 범위 `1/3`, `2/4`, `1/4`와 span 2/2/3을 row 축에서 검사하며 reverse는 item 순서만 바꾼다. |
| 8 | 10px inline width에서 Ahem text가 예상 block contribution을 만들지 않음 | row fixture는 `word-break:break-all`, 10px Ahem/line-height를 고정하고 intrinsic input에서 70/80/140px block size를 비교한다. |
| 9 | 호스트 system font fallback이 계산 결과를 바꿈 | runtime text engine은 system font 없이 저장소 Ahem font를 등록하며 양 플랫폼 intrinsic source가 font registry revision 1로 기록된다. |
| 10 | 동일 span 그룹의 contribution kind/order mapping이 달라짐 | simulator verifier는 span 2와 3 순서, 각 item의 `Minimum`, `MinContent`, `MaxContent` 및 값 0/70/80/140을 전체 배열로 대조한다. |
| 11 | cyclic와 definite 140px 결과 또는 definite 100px control이 뒤섞임 | 각 scene은 독립 ID와 기대값을 가진다. 첫 두 조건은 Rows `[42,47,47]`, small controls는 `[32,32,32]`이며 전체 run 결과가 이를 기록한다. |
| 12 | grid frame과 text intrinsic height가 동일한 값으로 비교됨 | span item frame은 66/66/100px small control에서 따로 확인하고 text intrinsic은 70/80/140px로 별도 assertion한다. |
| 13 | 역순 scene에서 source-order node index를 item identity로 오인함 | Android·iOS frame builder와 capture 결과는 실제 DOM index를 보존하면서 kind별 frame을 forward/reverse 양쪽에 대조한다. |
| 14 | 100px root 밖 red text가 item fill 또는 root 내부 색으로 잘못 판정됨 | small scene 표본은 node ID와 CSS 좌표를 함께 고정한다: root 안 `(7,7)`, frame/root 밖 text `(7,126)`, text 뒤 clear `(7,150)`. 네이티브 표본이 각 좌표에서 기대 RGBA를 만족한다. |
| 15 | Android/iOS 표본 좌표 변환이나 surface 완료 상태가 누락됨 | AVD PixelCopy와 iOS Metal drawable readback을 각각 기록하고 `drawPresented`를 확인했다. iOS 표본의 compositor 이후 상태는 검증했다고 주장하지 않는다. |
| 16 | runtime capture의 신규 scene 수·ID 중복 검사가 갱신되지 않음 | CSS reference scene count가 245에서 251로 증가했고 전체 scene ID uniqueness assertion을 유지한다. |
| 17 | 새 row scene이 기존 column overlapping-span 검증을 잘못 통과함 | capture verifier는 `scene.axis`와 scene별 row span mapping을 이용하며 Columns fixed pass와 Rows contribution pass를 각각 판정한다. |
| 18 | 한 플랫폼 코드에서 root/item/frame 또는 pixel expectations가 빠짐 | Java와 Swift 모두 여섯 ID, root/frame 기대값 및 두 small control의 three-sample matrix를 별도 정의하고 final run 6/6으로 확인했다. |
| 19 | 캡처가 다른 소스 revision 또는 dirty checkout에서 만들어짐 | `simulator.json`은 소스 commit `938261090f303e1efa989f225641d4e132c21ee0`와 tracked dirty path 부재를 기록한다. 증거 manifest 56개 hash가 일치하며 iOS build log는 gzip 해제 시 원본 byte와 같다. |
| 20 | 공개 상태가 제한 fixture 결과를 제품 완료로 부풀리거나 PR 링크가 빠짐 | plan, `spec/STATUS.md`, 공개 roadmap은 PR #503과 실행 근거를 연결하고 C11.4 unchecked 및 `77/213`을 유지하며 미검증 플랫폼/범위를 함께 쓴다. |

## 검토 결론

최종 diff에서 추가 수정이 필요한 blocker를 찾지 못했다. 확인 범위는 제한된 row-span fixture와 기록된 Android AVD/iOS Simulator 경로다. 일반 CSS admission, WPT, Android 실기기/hardware GPU, CSS border stroke, iOS compositor 이후 pixels 및 전체 C11.4는 이 diff에서 증명하지 않는다.
