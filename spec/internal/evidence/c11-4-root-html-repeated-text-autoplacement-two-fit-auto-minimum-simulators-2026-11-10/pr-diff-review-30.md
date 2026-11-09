# C11.4 automatic minimum 구현 PR diff 검토 30개

구현/runtime 검토와 다른 관점에서 전체 변경 diff와 첨부 증거를 확인했다. 항목마다 실제 변경 지점과 반례 또는 제출 내용을 대조했다.

1. **layout gate가 다른 track list에 적용됨:** 새 boolean은 기존 six-member auto-placement classifier와 fit 두 개를 모두 요구한다.
2. **style projection과 layout gate의 profile 정의가 다름:** 양쪽 모두 같은 six-member auto-placement/2-fit 조건으로 한정된다.
3. **기존 two-fit row-span automatic minimum 회귀:** row-span allow 조건을 유지하고 새 auto-placement 조건을 별도 OR arm으로 추가했다.
4. **다른 C11.4 profile에도 auto를 허용:** 변경은 해당 Root HTML fit-content validator 두 곳과 runtime test에만 있다.
5. **명시행 control item까지 함께 자동 최소 크기 적용:** 새 profile은 첫 item의 computed/layout min-height 0을 요구하고 이를 바꾼 negative test를 추가했다.
6. **일부 text item만 자동 최소 크기 허용:** 나머지 여덟 item 모두 auto인지 양쪽 validator가 검사한다.
7. **한 item의 잘못된 minimum을 fixture 변환이 감춤:** `source_for_auto_minimum`은 기본 `.item`만 auto로 바꾸고 #item-0은 0으로 명시한다.
8. **negative fixture가 실패 원인과 무관하게 거부:** mixed-minimum, one-fit-auto, explicit-control-auto, span topology 등 각각 독립 변형을 둔다.
9. **새 regression이 inactive color scheme을 놓침:** 한 RuntimeSession에서 Light와 Dark를 순차 요청하고 environment revision을 layout key와 비교한다.
10. **display:none Grid의 alignment parser 오류가 다시 발생:** hidden subtree alignment check skip과 justify-content default 분기가 함께 포함되고 미디어 전환 test가 통과한다.
11. **계산 cascade stale snapshot으로 회귀가 통과:** test가 요청한 environment revision과 완료 layout revision의 일치를 확인한다.
12. **Chrome auto-minimum corpus를 baseline과 혼동:** 두 관측 파일을 분리 상수로 읽고 auto 장면은 별도 test 이름으로 실행한다.
13. **scene 수 축소:** baseline과 auto-minimum 모두 각각 2,880개 scene count를 assertion한다.
14. **authored style 외 불필요한 입력이 변함:** generated source 변환은 min-height 선언과 explicit control override만 바꾼다.
15. **item style 색상 중복으로 잘못된 computed min-height 검사:** background color별 item style가 정확히 하나인지 먼저 확인한다.
16. **baseline 관측에 없는 min-height 값을 읽음:** computed min-height assertion은 해당 field가 기록된 auto-minimum corpus에서만 실행한다.
17. **floor regression이 matrix aggregate에서 숨음:** 모든 장면에서 fit-row floor를 비교하고 auto-minimum 결과는 위반 0건이어야 한다.
18. **stretch/auto occurrence 회귀:** 기존 aggregate의 list별 stretch pair와 발생 횟수 assertion을 두 matrix에서 계속 실행한다.
19. **zero-cap trace가 다른 scene으로 바뀜:** trace test는 Chrome scene index 0을 선택하고 item line count와 zero cap 바닥을 확인한다.
20. **자동 최소 크기 기여 source가 잘못 분류됨:** trace는 auto track 포함, 단일 track span, content-based인 `AutomaticMinimum` variant를 강제한다.
21. **track sizing trace가 중복·누락됨:** 단일 trace, owner node, row pass, nine used rows와 fit item별 정확히 하나의 Minimum을 검사한다.
22. **rejection atomicity가 성공한 초기 scene을 보존하지 못함:** recovery test는 initial completed key/frames를 보관하고 reject completion이 없어야 한다고 검사한다.
23. **잘못된 후속 장면이 같은 session을 영구 오염:** 거부 직후 기존 유효 color scheme으로 돌아와 Ready를 요구한다.
24. **Simulator 검증이 fixture-only screenshot으로 끝남:** iOS/Android 각각 light/dark의 app boot, layout, draw 로그와 실제 native screenshot을 함께 저장했다.
25. **Android evidence가 사용자 AVD를 변경:** 실행 인자는 기존 API 37.2 AVD에 `-read-only`를 사용했고 종료 후 shared profile을 보존했다.
26. **사용하지 않을 task resource가 남음:** 임시 iOS simulator와 실패한 task Android AVD를 삭제하고 ADB/CoreSimulator 실행 목록을 확인했다.
27. **공용 bootstrap 파일이 fixture로 남음:** `examples/bootstrap/index.html`과 `root-app.js`를 기존 SHA-256과 대조해 복원하고 기본 bundle/iOS/Android build를 다시 만들었다.
28. **기존 사용자 untracked evidence가 PR에 포함됨:** stage/commit은 구현 파일과 이 작업의 evidence directory만 명시적으로 선택한다.
29. **PR 이미지 링크가 mutable하거나 깨짐:** PR 본문에는 병합 전 candidate commit SHA의 raw.githubusercontent.com 이미지 경로를 사용한다.
30. **bounded slice를 공식 CSS 지원 완료로 표시:** PR/후속 status sync는 공식 78/214와 interface 0.1.0, 전체 C11.4/CSS Grid 미완료 경계를 유지한다.

## 판정

현재 변경에서 추가 수정이 필요한 diff 관점은 발견되지 않았다. 최종 PR diff는 stage 후 한 번 더 `git diff --cached --check`와 filename allowlist로 확인한다.

## 최종 코드 재실행 뒤 증거 변경 재검토

구현 commit `10c36104fdce8c6c99ae2dafc6df68a795524acd`의 마지막 control-minimum guard까지 포함해 iOS 26.2 Simulator와 Android API 37.2.13 AVD를 다시 빌드·실행한 뒤 기존 화면과 로그를 교체했다. 변경된 증거 diff에 대해서 20, 24–30번 관점을 다시 대조했다.

- RuntimeSession/scene behavior는 이전 5/5 test 및 2,880+2,880 matrix와 같고, 이번 UI 재실행 네 건 모두 app boot, `layout=ready`, `draw presented`를 기록한다.
- 갱신한 iOS light/dark와 Android light/dark screenshot을 직접 확인했다. 화면은 fixture layout을 보여 주며 feature 전체의 시각 parity를 주장하지 않는다.
- 공유 Android AVD는 `-read-only`로 실행했다. 이번 GPU 정보는 Goldfish GFXStream/llvmpipe software Vulkan이며 hardware GPU 측정으로 표시하지 않는다.
- 임시 iOS profile과 AVD process를 종료했고 임시 iOS profile, checkout-local `local.properties`, fixture backup을 제거했다. bootstrap source hash를 확인하고 기본 app outputs를 복원 빌드했다.
- 사용되지 않는 실패 AVD stdout artifact는 PR 변경에서 제거했다. 새로운 screenshots는 후속 evidence commit에 포함되고 PR 본문은 그 commit SHA에 고정한다.

재검토 결과: 코드 동작 변경 없음, evidence 갱신과 범위 표기에서 추가 수정할 결함 없음.
