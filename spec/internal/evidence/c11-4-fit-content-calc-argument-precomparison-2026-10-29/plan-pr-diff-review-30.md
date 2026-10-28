# C11.4 definite calc `fit-content()` 계획 PR diff 적대 검토 30개

검토 대상은 계획 문서, 사전 비교 JSON/README, C11.4 parent plan 및 `spec/STATUS.md`의 변경이다. 구현 PR diff 검토와 분리한다.

| # | 실패 관점 | 판정 |
|---:|---|---|
| 1 | 계획만 만든 상태를 구현 완료로 표시하는가 | 아니다. 계획 문서와 상태 대장은 구현 전임을 명시한다. |
| 2 | 다른 pinned WPT revision을 끌어오는가 | 아니다. WPT revision `c999c58338ee1d223df5ad62e48be1202ced0d37`을 일치시켰다. |
| 3 | 원본 파일 hash가 잘못됐는가 | 아니다. pinned tree SHA-256은 WPT checkout과 일치한다. |
| 4 | 계획의 target markup이 다른 assertion인가 | 아니다. `calculatedArgument` scene과 35px assertion을 지정한다. |
| 5 | line locator가 pinned source를 가리키지 않는가 | 아니다. target markup 52행, assertion 111행을 확인했다. |
| 6 | browser 결과를 Spinon 실행으로 과장하는가 | 아니다. upstream Chrome/WPT 결과로만 분류한다. |
| 7 | 전체 WPT 13/13을 새 assertion 단독 결과로 잘못 설명하는가 | 아니다. 13/13은 upstream harness이고 target computed value는 별도 기록했다. |
| 8 | Agent Browser 관측에 viewport와 DPR이 없는가 | 아니다. 1280×633 CSS px와 DPR 1을 기록했다. |
| 9 | 계산 결과가 Chrome 관측에서 역산됐는가 | 아니다. CSS Grid 규범 항과 입력값의 산술식으로 계산한다. |
| 10 | 10%가 row 축이 아닌 inline 축에서 resolve되는가 | 아니다. §7.2.1의 inner block-size basis를 명시한다. |
| 11 | absolute length 5px이 빠지는가 | 아니다. percentage 30px과 length 5px을 분리한다. |
| 12 | definite 300px 입력을 cyclic sizing으로 오인하는가 | 아니다. intrinsic-container fallback을 이 입력에서 배제한다. |
| 13 | `fit-content()` formula에서 minimum/max-content가 뒤바뀌는가 | 아니다. `max(minimum, min(limit, max-content))`를 고정한다. |
| 14 | min-height 0이 plan input에서 누락되는가 | 아니다. browser observation과 exact style gate에 포함한다. |
| 15 | 400px child를 final row 400px의 근거로 착각하는가 | 아니다. max-content 400px과 cap 35px을 분리한다. |
| 16 | root border-box와 content-box basis가 달라지는가 | 아니다. runtime normalized root의 padding/border를 0으로 고정한다. |
| 17 | 1264px browser root width와 60px runtime width를 섞는가 | 아니다. block basis 보존 및 inline normalization 이유를 설명한다. |
| 18 | explicit row가 만들어져 auto-row assertion을 우회하는가 | 아니다. `grid-template-rows:none`과 implicit row 하나를 고정한다. |
| 19 | 여러 auto rows가 생겨 첫 track만 우연히 35px인가 | 아니다. direct item 하나와 implicit row 하나만 허용한다. |
| 20 | fit-content가 leftover space에 stretched 되는가 | 아니다. auto max-track function이 아니며 265px free space에도 35px이어야 한다. |
| 21 | stage trace의 raw growth limit을 used row로 기록하는가 | 아니다. raw diagnostic과 final base/frame을 분리한다. |
| 22 | typed calc identity가 새 layout revision과 일치하지 않는가 | 아니다. same snapshot/revision과 stale handle negative를 요구한다. |
| 23 | item child overflow가 row geometry를 바꾸는가 | 아니다. item height와 child height를 별도 frame으로 비교한다. |
| 24 | viewport height 400px을 row percentage basis로 쓰는가 | 아니다. 320×400 viewport는 paint sample에만 사용한다. |
| 25 | author overflow declaration이 initial style을 바꾸는가 | 아니다. author declaration 없이 computed initial visible을 검사한다. |
| 26 | sample 좌표가 row 경계와 root 경계를 구별하지 못하는가 | 아니다. row 안·root 안·root 밖 표본을 분리했다. |
| 27 | fixture scene이 product CSS admission을 넓히는가 | 아니다. opt-in fixture-test 경계로 제한한다. |
| 28 | static archive 부재를 APK/IPA 전체 제외로 과장하는가 | 아니다. 정적 archive의 증거 한계를 적었다. |
| 29 | 계획 단계에서 Android/iOS 결과나 CI 성공을 주장하는가 | 아니다. 이는 이후 구현 관문이며 미실행 상태로 남긴다. |
| 30 | 공식 완료 수나 상위 C11.4 checkbox를 올리는가 | 아니다. `72/213` 및 C11.4 미완료를 유지한다. |

## 판정

30개의 서로 다른 문서·입력·수치·상태 전이 실패 관점을 검토했다. 수정이 필요한 오류는 발견하지 않았다. 이 검토는 계획 PR 변경분에만 해당하며 이후 구현 diff의 리뷰를 대신하지 않는다.
