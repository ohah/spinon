# PR #487 최종 diff 적대적 검토 30개

검토 대상은 PR #487의 최종 소스 diff, 계획·실행 문서, Android/iOS 시뮬레이터 근거와 최종 PR 본문이다. 각 관점은 다른 실패 경로나 계약 경계를 확인한다.

| # | 관점 | 확인 결과 |
| ---: | --- | --- |
| 1 | 이 PR이 C11.4 전체 구현으로 읽히는가? | 계획·본문·런타임 근거 모두 네 fixed fixture slice로 한정한다. 전체 완료 주장은 없다. |
| 2 | 공식 진행 집계가 부분 구현 때문에 증가하는가? | 공식 checkbox와 72/213은 유지하고, 병합 뒤 partial progress만 별도 docs PR로 기록한다고 명시한다. |
| 3 | 근거 명세 버전이 이 작업에서 임의로 바뀌는가? | 새 API나 runtime interface가 없고 내부 0059 계약은 변경하지 않는다. |
| 4 | 기대값을 Chrome 출력에서 역산했는가? | 독립 계산 JSON은 CSS Grid 2025 CRD와 Ahem 측정으로 작성했고 Chrome 결과는 별도 observation이다. |
| 5 | cyclic percentage의 intrinsic 처리 단계가 빠졌는가? | container intrinsic sizing에서 percentage를 auto처럼 다루고 확정 뒤 calc maximum을 푼다는 §7.2.1 절차를 고정한다. |
| 6 | Ahem 환경 차이가 track/frame 기대값을 흔드는가? | font family, size, line-height 및 Ahem 파일 hash를 사전 근거에 기록한다. |
| 7 | column cyclic basis가 item min-content 등으로 대체되는가? | 50px max-content root basis에서 29px track을 기대하고 runtime trace·frame과 비교한다. |
| 8 | row cyclic basis가 inline width 또는 viewport height인가? | 20px width에서 40px intrinsic height를 얻고 final row maximum 22px을 검사한다. |
| 9 | definite controls가 cyclic 결과만 반복하는가? | 40px와 10px basis를 분리해 각각 24px과 9px을 확인한다. |
| 10 | 작은 limit scene에서 0px minimum이나 item intrinsic이 9px 결과를 오염시키는가? | track minimum과 item min-width/min-height를 0으로 고정하고 9px final frame을 확인한다. |
| 11 | typed matcher가 bare calc 또는 fit-content를 같은 입력으로 오인하는가? | ComputedCssGridTrackSize::MinMax와 max Calc 조합만 수용한다. |
| 12 | calc AST에 추가 term이나 잘못된 순서가 허용되는가? | matcher는 percentage와 length의 두 typed term을 정확히 비교한다. |
| 13 | 런타임 fixture에 다른 node 수·owner·text가 들어와도 통과하는가? | root/item/direct text 구조, ID, text 내용, node marker와 부모 관계를 고정 검증한다. |
| 14 | unknown scene 또는 style mismatch가 이전 결과로 fallback되는가? | 알려진 scene prefix에서 allowlist miss와 shape mismatch는 unsupported error로 종료한다. |
| 15 | 직접 layout test가 최종 값 대신 초기 track pass를 검사하는가? | 마지막 axis pass/stage의 track base size와 root/item frame을 검증한다. |
| 16 | 추가된 matrix가 기존 fit-content calc 회귀를 없애는가? | 같은 runtime module과 test에 기존 네 fit-content calc scene을 남기고 모두 실행했다. |
| 17 | capture matrix의 중복 ID 또는 잘못된 scene count가 조용히 허용되는가? | 전체 227 scene count와 ID uniqueness assert가 유지되고, 신규 네 ID를 별도 등록했다. |
| 18 | fixture 코드가 기본 product runtime에 섞이는가? | adapter는 기존 opt-in fixture-test route만 사용한다. |
| 19 | 정적 archive 검사 결과를 최종 앱 release 증거로 과장하는가? | 기본 archive marker absent, fixture-test positive control present만 주장하고 APK/IPA 전체 검사는 한계로 적는다. |
| 20 | Android 등록 ID와 JS capture ID가 어긋나는가? | Android scene ID set에 신규 네 ID가 있고 각 실행 로그가 동일한 scene ID를 기록한다. |
| 21 | Android root/item frame validator가 axis 결과와 다른 치수를 기대하는가? | 네 scene의 expected frames가 독립 matrix와 같고 최종 summary에서 일치했다. |
| 22 | Android PixelCopy가 미등록 scene/sample을 기존 small-limit으로 오인하는가? | 신규 scene별 sample id·좌표를 명시하고 default branch는 false를 반환한다. 후속 AVD 실행이 4/4 통과했다. |
| 23 | Android PixelCopy 표본의 node ID·색·중복 수를 느슨하게 비교하는가? | validator는 expected ID set, target item, 좌표, 중복과 총 sample 수를 검사하며 실제 9/9 exact다. |
| 24 | iOS scene allowlist 또는 expected frame이 Android/독립 matrix와 다른가? | Swift에 같은 네 ID와 root/item/node frame이 등록되어 simulator 실행 4/4 일치했다. |
| 25 | iOS Metal 표본이 올바른 scene·node·색을 비교하는가? | 네 scene switch case의 sample ID·CSS 좌표·RGBA가 capture matrix와 같고 전체 9/9 exact다. |
| 26 | surface scale 변환이나 generation mismatch가 표본 성공을 가장하는가? | 로그가 Android surface scale 2.625, iOS drawable scale 3.0과 generation을 기록하며 expected/observed sample 수가 일치한다. |
| 27 | root 바깥 red pixel을 CSS border stroke라고 주장하는가? | border 없는 fixture의 Ahem text overflow로만 해석하고 PR/근거에서 border stroke 검증을 제외한다. |
| 28 | 실패한 Android 재시도나 이전 source hash 캡처가 최종 결과로 혼입되는가? | stale APK·sample registry 오류·수정 전 실행과 최종 실행을 분리해 보존했다. 두 최종 simulator.json의 tracked diff hash가 현재 소스와 같다. |
| 29 | PR 이미지 URL이 실제 PR에서 깨지거나 잘못된 화면을 가리키는가? | 첫 링크의 잘못된 commit ref를 발견해 실제 head SHA로 수정했다. 최종 PR을 Agent Browser에서 다시 열어 두 이미지가 1080×2400 및 1206×2622로 로드됨을 확인했다. |
| 30 | 병합 전에 보고되지 않은 필수 check를 놓치는가? | live gh pr checks --required는 check 없음, main required-status endpoint는 미설정 응답, PR merge state는 CLEAN이다. 병합 직전 다시 확인한다. |

**최종 diff 검토 결과:** 미해결 blocking finding 없음. 이미지 ref는 수정 후 실제 PR 화면에서 재검증했다. Agent Browser 작업 namespace와 session은 종료했다.
