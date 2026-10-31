# 병합 후 상태 문서 diff 검토 30개

대상은 구현 PR #920 병합 뒤의 `spec/STATUS.md`, C11 Grid 내부 계약 0059, C11 내부 인덱스, 계획 및 runtime evidence README 동기화다. GitHub PR metadata, main branch required-check 설정, 현재 문서의 checkbox와 완료 수를 대조했다.

| # | 검토 관점 | 판정 |
|---:|---|---|
| 1 | 잘못된 PR 번호를 링크하는가 | **통과.** 기록은 구현 PR #920을 가리킨다. |
| 2 | 계획 PR #919와 구현 PR #920을 혼동하는가 | **통과.** #919는 계획 merge, #920은 구현 merge로 구분한다. |
| 3 | PR이 실제 merged 상태가 아닌데 병합으로 기록하는가 | **통과.** GitHub PR metadata의 state는 `MERGED`다. |
| 4 | base가 main이 아니거나 다른 repo의 PR을 기록하는가 | **통과.** #920 base는 `main`, repo는 `ohah/spinon`이다. |
| 5 | 잘못된 merge timestamp를 기록하는가 | **통과.** GitHub `mergedAt`는 `2026-10-31T11:22:59Z`다. |
| 6 | merge SHA가 PR head나 로컬 main을 가리키지 않는가 | **통과.** merge SHA는 `15888d08fbbc171815fb77ce249d1fafa11cb9cd`; fetched `origin/main`과 로컬 `main`이 이 commit을 가리킨다. |
| 7 | merge method가 project rule과 다른가 | **통과.** PR은 Rebase and merge됐고 저장소 설정은 rebase 허용, merge commit·squash 비허용이다. |
| 8 | PR checks를 green 또는 passed로 허위 기록하는가 | **통과.** `gh pr checks --required`는 checks 미보고를 반환했고 문서는 성공을 주장하지 않는다. |
| 9 | main에 required status check이 켜져 있는데 생략하는가 | **통과.** GitHub API는 required checks가 활성화되지 않았음을 반환했다. |
| 10 | check rollup 부재를 CI 통과로 해석하는가 | **통과.** plan, STATUS, runtime README가 모두 “미보고”로 남기고 CI 성공으로 세지 않는다. |
| 11 | C11 완료 checkbox를 부분 구현으로 체크하는가 | **통과.** C11 상위 checkbox는 미완료다. |
| 12 | C11.4 완료 checkbox를 제한 profile로 체크하는가 | **통과.** C11.4 전체 지원/완료를 선언하지 않는다. |
| 13 | 공식 완료 수를 72에서 올리는가 | **통과.** STATUS와 내부 인덱스는 `72/213`을 유지한다. |
| 14 | scene matrix 결과를 통합지원 비율에 더하는가 | **통과.** 각 플랫폼 16/16은 고정 runtime candidate로만 기술된다. |
| 15 | Android 대상 정보를 잘못 기록하는가 | **통과.** 근거는 Android API 37 ARM64 AVD다. |
| 16 | iOS 대상 정보를 잘못 기록하는가 | **통과.** 근거는 iPhone 17 Pro / iOS 26.2 Simulator다. |
| 17 | 두 platform 결과가 합쳐져 한 플랫폼만 검증한 것처럼 보이는가 | **통과.** Android와 iOS 각각 16/16을 표기한다. |
| 18 | 설치 artifact hash 불일치를 숨기는가 | **통과.** runtime README에 설치 APK/executable hash가 build hash와 일치한다고 기록한다. |
| 19 | AVD software rendering을 hardware GPU로 부르는가 | **통과.** Android 경로를 software-rendered emulator로 한정한다. |
| 20 | 화면 캡처를 glyph pixel 증거로 쓰는가 | **통과.** renderer 영역이 검고 route/status 화면임을 명시한다. |
| 21 | PixelCopy 또는 drawable readback을 실행했다고 서술하는가 | **통과.** 두 방법을 검사하지 않았다고 기록한다. |
| 22 | compositor 이후 또는 물리 device 결과를 주장하는가 | **통과.** compositor, 실기기, hardware GPU는 미검증으로 남긴다. |
| 23 | WPT를 통과했다고 오해할 수 있는가 | **통과.** WPT 미검증·미지원 경계를 유지한다. |
| 24 | 모든 font, script, bidi 또는 writing mode를 지원한다고 확대하는가 | **통과.** 고정 Ahem/LTR profile의 제한을 유지한다. |
| 25 | relative unit이나 `calc()` 입력을 지원한다고 하는가 | **통과.** 해당 입력은 거부되는 범위로 기록한다. |
| 26 | Chrome 관측 차이를 감춰 규범 parity를 주장하는가 | **통과.** Chrome 156은 관찰 비교이고 일부 값이 독립 기대값과 다른 점을 유지한다. |
| 27 | 기본 product와 fixture archive 경계를 모바일 전체 bundle 보증으로 확대하는가 | **통과.** archive 근거는 static `spinon-ffi`로만 범위를 한정한다. |
| 28 | 문서 링크가 병합된 C11 evidence로 연결되는가 | **통과.** plan, contract, runtime/archive evidence, review 파일이 같은 candidate directory를 가리킨다. |
| 29 | 상태 문서에 merge metadata를 넣으며 다른 C11 상태를 실수로 바꾸는가 | **통과.** 변경 목적은 C11.4 letter-spacing merge metadata 반영이고 checkbox/count는 그대로다. |
| 30 | 이후 Tailnet road map 화면과 repository ledger를 구분하지 않는가 | **통과.** 이 검토는 main의 source documents만 확인한다. Tailnet 반영은 별도 publish 후 Agent Browser로 검증한다. |

**판정:** 30개 병합 후 상태 동기화 관점을 확인했다. 기록한 merge metadata와 제한·미검증 범위가 GitHub, main branch 및 runtime evidence와 일치한다. Tailnet publish 검증은 별도 단계다.
