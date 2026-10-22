# PR #520 로드맵 상태 보정 · 변경 적대 검토 30개

검토 범위는 `spec/public/roadmap.html`의 C11.4 replaced-image multi-span 한 문단과 이 검토 문서뿐이다. PR #520 실행 상태는 [공식 상태 대장](../../../STATUS.md), [고정 계획](../../../../plan/c11-4-cyclic-minmax-replaced-image-span.md), [runtime evidence](../c11-4-cyclic-minmax-replaced-image-span-runtime-2026-10-23/README.md), PR #520 merge metadata에서 독립 대조했다.

| # | 실패 관점 | 확인과 판정 |
|---|---|---|
| 1 | runtime이 미시작이라는 옛 계획 문장을 남기는가? | 계획, 상태표, PR #520 모두 구현·병합을 기록한다. 로드맵 항목을 runtime fixed slice로 갱신했다. 통과. |
| 2 | 이 항목이 C11.4 전체 완료처럼 보이는가? | 설명에 fixed scene 한정과 전체 C11.4 미완료를 명시했다. 통과. |
| 3 | C11.4 완료 checkbox를 바꾸거나 체크 완료로 암시하는가? | 항목은 `77/213`과 미완료 상태를 유지한다고 쓴다. 통과. |
| 4 | PR 번호가 다른 slice와 혼동되는가? | 구현 PR 링크와 제목은 #520을 가리킨다. 통과. |
| 5 | merge SHA가 PR #520과 다른가? | GitHub metadata 및 계획의 `861f69862de34858d522b380f25b52dc5310ef5d`와 대조했다. 통과. |
| 6 | 병합 시각의 UTC/KST 변환이 틀렸는가? | GitHub `2026-10-22T17:21:48Z`는 `2026-10-23 02:21:48 KST`다. 통과. |
| 7 | Rebase and merge 결과를 다른 방식으로 표현하는가? | PR merge 기록은 Rebase and merge로 명시했다. 통과. |
| 8 | empty status rollup을 CI 통과로 과장하는가? | rollup은 비었다고만 썼으며 checks 통과 주장은 없다. 통과. |
| 9 | main에 required status checks가 있었던 것처럼 읽히는가? | merge 시 설정되지 않았다고 적었다. 통과. |
| 10 | intrinsic track 기대값을 Chrome 관측에서 역산하는가? | 고정 수치는 spec-derived/runtime 계획과 일치하고 Chrome 사전 비교를 별도 링크했다. 통과. |
| 11 | intrinsic Columns `[5,8,8]`가 잘못 전사됐는가? | 계획과 runtime README의 네 scene 표를 대조했다. 통과. |
| 12 | intrinsic Rows `[7,7]`가 잘못 전사됐는가? | 계획과 runtime README를 대조했다. 통과. |
| 13 | intrinsic root `29×20px`가 border/content box 혼동인가? | 계획의 2px border와 `25×16px` content box에서 계산한 border box다. 통과. |
| 14 | definite Columns `[10,10,10]`가 잘못됐는가? | 80px content basis의 `10% + 2px` minimum 및 runtime evidence와 대조했다. 통과. |
| 15 | definite Rows `[10,10]`가 잘못됐는가? | 같은 independent expectation/runtime trace와 일치한다. 통과. |
| 16 | definite root `84×84px`에 border가 빠졌거나 중복됐는가? | 80px content box와 2px 사방 border 합산을 계획에서 확인했다. 통과. |
| 17 | image inline contribution과 block contribution이 섞였는가? | 12/18/24px 열 기여와 8/12/16px 행 기여를 별도 기술했다. 통과. |
| 18 | span-2와 span-3 topology를 잘못 읽었는가? | A/B의 span-2, C의 span-3 입력과 계획 표를 대조했다. 통과. |
| 19 | 네 scene의 forward/reverse 구성이 빠졌는가? | runtime README의 intrinsic/definite forward/reverse 네 scene과 각 4/4 결과를 연결했다. 통과. |
| 20 | Android 결과를 실기기나 hardware GPU로 확대하는가? | Android API 37 ARM64 AVD라고 한정하고 실기기/hardware GPU 미검증을 썼다. 통과. |
| 21 | iOS sample을 compositor 뒤 픽셀 검증으로 과장하는가? | iOS compositor 이후 pixels는 미검증으로 명시했다. 통과. |
| 22 | `3/3` surface sample을 전체 frame pixel parity로 확대하는가? | 선택된 세 표본의 exact 결과로만 서술했다. 통과. |
| 23 | PixelCopy와 Metal 표본을 같은 측정 단계처럼 다루는가? | Android AVD와 iOS Simulator를 분리해 설명하고 후자의 compositor 한계를 표시했다. 통과. |
| 24 | WPT 후보를 Spinon WPT 통과로 오해하는가? | WPT는 검증하지 않았다고 썼다. 통과. |
| 25 | 제한 fixed fixture를 arbitrary CSS admission으로 오해하는가? | 일반 CSS admission은 검증하지 않았다고 명시했다. 통과. |
| 26 | 계획 링크가 같은 replaced-image cyclic minmax slice를 가리키는가? | `/spinon/plan/c11-4-cyclic-minmax-replaced-image-span.html` 경로가 존재한다. 통과. |
| 27 | runtime README 링크가 실제 실행 기록을 가리키는가? | 네 scene·빌드·플랫폼 실행을 담은 README와 route를 대조했다. 통과. |
| 28 | Android 화면 링크가 다른 날짜·scene의 이미지인가? | `intrinsic-forward` Android API 37 ARM64 AVD PNG가 `internal/evidence/c11-4-cyclic-minmax-replaced-image-span-2026-10-23/` 경로에 존재하며 로드맵의 공개 경로와 일치한다. 통과. |
| 29 | iOS 화면 링크가 다른 scene 또는 다른 플랫폼인가? | `intrinsic-forward` iPhone 17 Pro/iOS 26.2 Simulator PNG가 같은 공개 evidence 경로에 존재한다. 통과. |
| 30 | 공식 상태 대장 링크가 Grid 섹션 대신 다른 문서를 여는가? | `/spinon/STATUS.html#c11-css-grid`가 C11 CSS Grid 상태 section을 가리킨다. 통과. |

**판정:** 서로 다른 상태, 규범 계산, platform/evidence 경계, archive/PR metadata와 링크 경로 30개를 확인했다. 발견된 불일치는 공개 로드맵의 stale `Runtime은 시작하지 않았` 상태와 Android/iOS 화면 링크 경로 오류였으며 둘 다 수정했다. C11.4 구현 범위나 공식 완료 집계는 변경하지 않았다.
