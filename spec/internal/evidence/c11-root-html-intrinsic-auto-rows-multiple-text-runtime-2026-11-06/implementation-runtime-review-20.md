# C11.4 여러 direct text item 구현·runtime 적대 검토 20개

**검토 대상:** 현재 working-tree projection, layout diagnostic handling, 기본 RuntimeSession assertion, Android/iOS simulator evidence. **계획의 20개 관점 표는 재사용하지 않았다.**

1. **직접 자식 목록 순회:** preflight와 admission이 같은 수집 목록을 사용해 두 번째 loop에서 text item이 빠지지 않는지 확인했다. 두 text source 모두 기여와 paint assertion에 남아 통과.
2. **hidden item 선처리:** hidden direct node는 row 수 preflight와 item style gate 양쪽에서 제외된다. 기존 hidden-item runtime regression 통과.
3. **공백 전용 direct text:** CSS whitespace만 row/cardinality에서 제외하고 NBSP는 visible invalid node로 남긴다. helper 3개 검사 통과.
4. **불완전한 tree handle:** `tree.node` 조회 실패는 visible 수에서 보수적으로 세고 admission에서 inspectability 오류로 거부한다. 알 수 없는 자식을 정상 item으로 삼지 않아 통과.
5. **빈 열 목록:** 0 columns에서 `div_ceil`을 호출하지 않고 contextual `grid-auto-rows` rejection으로 종료한다. helper 경계 assertion 통과.
6. **필요 행 산식:** visible item 수를 explicit column 수로 올림한 뒤 explicit row 수와 비교한다. 10,001/2 column 계산이 5,001이며 explicit rows 아래로 내려가지 않는지 helper에서 확인해 통과.
7. **기존 10,000 상한 적용:** 10,000 direct item 문서가 실제 layout되어 마지막 item frame `[0,10060,1,1]`을 냈다. 10,001은 같은 profile에서 사전 rejection했다.
8. **대형 trace의 사용자 frame 보존:** 10,000행에서 trace budget exceeded가 `LayoutError`로 바뀌지 않고 해당 trace만 생략된다. 계산된 마지막 frame은 유지되고 trace 일부만 반환되지 않아 통과.
9. **다른 trace 실패 분리:** trace allocation failure branch는 그대로 layout error로 남겼고, budget 초과만 생략한다. 에러 범위를 자의적으로 넓히지 않아 통과.
10. **item/style join identity:** computed style index는 동일 snapshot의 `NodeId`로 만들고 각 direct child id로 조회한다. id를 생략하거나 DOM 순서로 결합하지 않는다.
11. **다중 item lookup 비용:** child마다 전체 `snapshot.elements`를 다시 선형 검색하던 경로를 한 번 만든 `BTreeMap` 조회로 교체해 증가한 item 수에서 반복 O(n²) 검색을 없앴다.
12. **문자 토큰 parser:** source 검사는 literal ASCII `A` token과 single U+0020 separator만 허용한다. `A `, ` A`, double-space, newline, `AA`, 비 ASCII `Ａ`가 거부된다.
13. **두 번째 item gate 재사용:** content, child topology, dimensions, placement, position, font/whitespace/writing-mode 계약이 두 번째 item에도 똑같이 적용된다. short item에 `B`, `nowrap`, span, 추가 child를 넣은 네 atomic rejection이 통과.
14. **고정 줄 수 추정 우회:** row contribution은 item별 기존 text measurement result에서 계산된다. long/short text가 각각 20px/10px min-content contribution으로 trace에 남는다.
15. **역전 시 identity 유지:** short item을 long item 앞으로 이동해도 contribution이 source 순서에 따라 `[10,20]`으로 이동한다. owner frame과 item source identity가 보존된다.
16. **revision 혼합:** RuntimeSession 결과에서 cascade/layout key가 같고 두 paint source의 generation·document/tree/style revisions가 완료 key와 일치한다.
17. **paint ordering:** 각 background box 뒤에 같은 item의 text command가 있고 그 다음 item box 앞에 그 text가 놓이는지 확인했다. 전체 box sequence는 DOM order와 같다.
18. **atomic layout rejection:** 두 번째 item 실패 시 cascade는 ready 상태지만 layout은 failed이고 completed frame/render snapshot은 없다. 앞 item만 게시되는 경우를 거부해 통과.
19. **simulator asset/source 혼동:** 첫 실행 캡처가 기본 HTML로 나온 것을 app asset hash와 화면에서 발견했다. Gradle/Xcode native build가 bootstrap asset을 다시 생성한다는 원인을 확인하고, 후보 fixture를 최종 APK와 `.app` 리소스에 넣은 뒤 양쪽 bundle hash를 기준 fixture SHA와 일치시켰다.
20. **증거 범위 오표시:** AVD API 37 ARM64와 iPhone 17 Pro/iOS 26.2 simulator screenshot만 기록한다. glyph wrap 차이는 Android 4+2, iOS 3+3으로 보존하고 screenshot을 PixelCopy/Metal readback이나 실기기 증거로 부르지 않는다.

## 검토 결과

발견 사항은 trace-budget 처리, simulator asset injection 순서, nested runtime test의 include 경로였다. trace 초과는 frame과 trace를 분리해 처리하고 plan/API 설명을 갱신했다. simulator 재실행 후 설치물 fixture SHA와 `layout=ready boxes=8`을 확인했으며 test 경로를 수정한 뒤 runtime 7/7을 다시 통과시켰다. 변경 파일 hash는 [`candidate-source.sha256`](./candidate-source.sha256)에 고정했다.
