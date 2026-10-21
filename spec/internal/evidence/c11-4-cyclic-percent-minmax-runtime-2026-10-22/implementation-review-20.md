# C11.4 cyclic percentage `minmax()` 구현 실패 관점 20개

계획 검토 20개와 분리해 구현 소스·고정 runtime scene·Android/iOS 실행 근거를 살폈다. 각 관점은 별도 실패 경로 또는 경계 조건을 확인한다.

1. **fixture 이외 product 경로 유입** — C11.4 source router는 `c04-runtime-gpu-fixture-test` 아래에 있고 새 adapter도 같은 경로에서만 등록된다. 기본/fixture release rlib 대조로 신규 payload 부재/포함을 확인했다. 통과.
2. **scene allowlist 확대** — 새 adapter는 세 정적 ID만 보유하고 미등록 ID 테스트는 `None`을 확인한다. 임의 source나 CSS 문자열은 scene 입력으로 받을 수 없다. 통과.
3. **기존 bare percentage scene 충돌** — 기존 `cyclic_percentage_tracks` adapter와 별도 `cyclic_percentage_minmax` adapter를 순차 dispatch하고 ID가 겹치지 않는다. 기존 세 scene 테스트도 함께 통과했다. 통과.
4. **잘못된 CSS 함수 표기** — 세 source의 CSS 값은 `minmax(0px,50%)`이며 Chrome 156 precomparison과 같은 입력이다. 공백 정규화 결과에 기대지 않는다. 통과.
5. **percentage minimum 구현으로 잘못 해석** — `0px`는 fixed minimum이고 `50%`는 maximum이다. 결과를 percentage minimum이나 `minmax(50%,0px)`로 확대하지 않는다. 통과.
6. **cyclic column intrinsic phase** — column scene은 `width:max-content`, 24px item contribution으로 root width 24px를 얻고 final track은 12px다. AVD/iOS 양쪽 trace가 정확히 일치했다. 통과.
7. **cyclic row intrinsic phase** — row scene은 definite 24px inline size와 `height:max-content`를 사용해 root height 16px를 얻고 final row 8px를 사용한다. 두 플랫폼 결과가 일치했다. 통과.
8. **definite control 오분류** — control은 root width 40px에서 percentage maximum을 resolve해 20px column을 만든다. cyclic scenes와 다른 root frame으로 실행해 control이 살아 있다. 통과.
9. **두 축 검사 누락** — runner scene은 선택 축만 검사하지 않고 Columns와 Rows track arrays를 모두 기대값과 비교한다. 각 플랫폼 result JSON에서 두 축을 확인했다. 통과.
10. **root/item frame 경계** — Android/Swift expectation과 runner node frame은 root 및 item을 별도 node index로 검사한다. 세 scene 전부 expected geometry와 맞았다. 통과.
11. **overflow item clip 오판** — column x=18, row y=12, definite x=22는 각 track 바깥이지만 해당 red item과 root frame 안이다. 캡처 화면과 raw sample 모두 item 색을 확인한다. 통과.
12. **표본 좌표 aliasing** — 각 장면의 inside/past-track 표본은 다른 좌표와 ID를 쓰며 유일하다. Android PixelCopy와 iOS Metal sample summary가 각각 6/6 exact다. 통과.
13. **Android RGB table 누락** — 첫 시도에서 신규 six sample IDs의 기대 RGB가 없어 `-1,-1,-1` mismatch가 났다. 실패 원인을 로그로 보존하고 RGB allowlist를 추가한 뒤 재빌드·재실행해 6/6을 확인했다. 수정 후 통과.
14. **Android scene 검증 누락** — C114 scene ID array, root/item frame, node frame, box count, pixel request gate를 각각 확장했다. `emulator-5558`에서 세 scene 모두 draw/present까지 통과했다. 통과.
15. **iOS scene 검증 누락** — C114 Set, frame/count checks, node frames와 exact Metal sample tuples를 추가했다. iPhone 17 Pro Simulator에서 세 scene이 통과했다. 통과.
16. **pixel 결과를 compositor 이후로 과장** — iOS evidence는 Metal drawable readback이므로 compositor 제출 전으로 한정한다고 기록했다. compositor 이후 capture라고 주장하지 않는다. 통과.
17. **Android 실기기 범위 혼동** — 사용한 target은 API 37 ARM64 AVD이며 physical device가 아니다. result metadata와 README에서 이를 분리했다. 통과.
18. **static archive 결과 확대 해석** — byte scan은 host release `.rlib`만 대상으로 한다. APK/IPA release 크기나 제외로 일반화하지 않는다고 명시했다. 통과.
19. **화면에서 border paint 증명** — 화면은 root 배경과 red item을 보여준다. 앱 renderer가 CSS border stroke를 그린다고 주장하지 않고 border 검증은 범위에서 뺐다. 통과.
20. **진행 상태 완료 오표기** — STATUS의 C11.4 checkbox와 완료 수 `72/213`을 유지하며, 검증하지 않은 WPT·일반 CSS admission·기타 minmax 범위도 명시했다. 통과.

**판정:** 첫 Android sample-color allowlist 누락을 수정했고 영향받은 build와 전체 세 scene matrix를 다시 통과시켰다. 남은 구현 blocker는 발견하지 못했다. 이 판정은 고정 opt-in fixture 세 장면에만 적용한다.
