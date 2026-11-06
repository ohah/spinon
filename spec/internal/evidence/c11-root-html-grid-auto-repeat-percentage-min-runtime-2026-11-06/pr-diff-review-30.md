# 구현 PR diff 적대 검토 30개

검토 대상은 implementation candidate의 staged code·fixture·실행 evidence diff다.
이 보고서는 검토 대상에 포함하지 않았다. 구현/runtime 실패 관점은
[별도 20개 검토](implementation-runtime-review-20.md)로 분리했다. 각 관점에서 실제
diff·호출 경계·테스트·빌드 근거를 대조했다.

1. **변경 파일 소유권** — staged 파일은 Root HTML Grid, Android/iOS route·asset 복사, 해당 fixture 및 전용 evidence다. 대량의 기존 untracked evidence는 stage하지 않았다.
2. **공식 상태 체크** — 이 PR은 `spec/STATUS.md` checkbox나 완료 수를 바꾸지 않는다. C11은 계속 미완료다.
3. **계약 범위** — 변경은 기본 parsed Root HTML `RootHtmlAutoRepeatV1`의 percentage minimum만 추가한다. 일반 author CSS admission이나 전체 C11 지원 주장을 추가하지 않는다.
4. **Cascade 조건** — cascade helper는 유한·비음수 min과 `1fr` max 조합으로 제한된다. 변경 helper의 callsite는 `grid-template` auto-repeat 검사 한 곳이다.
5. **원본 typed value** — count용 resolved px는 용량 산정에만 쓰고 projected `Percentage`를 대체하지 않는다. typed projection 회귀가 있다.
6. **가로 축 basis** — columns repeat은 고정 width basis를 사용하고 10%, 0%, 120%의 독립 count 테스트가 있다.
7. **세로 축 basis** — rows repeat은 고정 height basis를 사용하고 20%/160px 장면을 확인한다.
8. **비율 산술** — count 계산에 f64 중간값을 쓰고, Taffy f32 표현 한계를 넘는 결과는 거부한다. NaN·음수·f32 overflow negative controls가 있다.
9. **0%와 양수 minimum 분리** — 1px floor는 repeat count에만 적용한다. 실제 track sizing은 0% typed minimum에 남는다.
10. **120% overflow** — minimum을 240px로 clamp하지 않는다. expected track, 40px overflow, 240px runtime item을 각각 layout test와 simulator 화면에서 확인한다.
11. **반복 pattern 수** — 1~2개 fragment track을 제한하고, pair count에서 fragment 안·사이 gap을 모두 반영한다. 2-track 5회/10-track 사례가 있다.
12. **Auto-fill/auto-fit 동작** — fill은 빈 track을 남기고 fit은 placement 뒤 collapse한다. layout test 및 Agent Browser의 fixture별 computed track 관측을 확인했다.
13. **지원하지 않는 maximum** — fixed/percentage/calc/intrinsic/auto/fractional max를 percentage minimum과 조합하면 projection에서 거부하는 test를 추가했다.
14. **Indefinite 축** — Root HTML percentage minimum은 definite positive axis 없이는 layout에 진입하지 않는다. auto-width synthetic input negative control이 있다.
15. **Track ceiling** — 10,000-track 한도와 overflow 거부는 expansion 전에 검사한다. projection·layout 각각 정확 ceiling tests가 유지된다.
16. **다른 축 곱셈** — repeat track 수와 반대 축 track 수의 checked multiplication이 남아 있다. unchecked multiplication은 추가하지 않았다.
17. **부분 결과 소유권** — invalid contract와 expansion failure는 error를 반환하며 새 성공 snapshot을 만들지 않는다. 기존 failure atomicity test를 확인했다.
18. **기본 RuntimeSession 연결** — HTML source는 앱 테스트 전용 FFI가 아닌 parsed Root HTML 표준 host route에서 여섯 장면으로 계산된다. RuntimeSession geometry test가 통과한다.
19. **RuntimeSession 관측 범위** — 이 검사는 root/item frame 27 boxes를 비교한다. 빈 track count 및 Taffy used-track array 자체는 별도 layout tests가 검증하며, 이를 runtime snapshot이 직접 노출한다고 주장하지 않는다.
20. **Fixture와 precomparison 차이** — 앱 fixture는 auto-fit `width:auto`, overflow `width:240px`를 쓴다. 같은 fixture를 Agent Browser로 확인해 item 200/240px와 track list를 기록했고 precomparison의 10px item control과 구분한다.
21. **Android intent 진입** — 새 boolean은 C04.12 root-HTML route 선택에 연결되며 c11 named-scene 경로에 `c11GridCase`를 만들지 않는다. API 37 AVD에서 올바른 route가 ready 상태가 됐다.
22. **iOS argument 진입** — `--spinon-c111-root-html-auto-repeat-percentage-minimum`은 기존 C0410 runtime route에서만 fixture asset을 고른다. iPhone 17 Pro/iOS 26.2 Simulator 실행 로그가 있다.
23. **서로 배타적인 fixture** — zero-minimum과 percentage-minimum 동시 선택은 거부된다. page-scale override도 거부하며 새 iOS argument는 C0410 allowlist에 들어 있다.
24. **Android asset 복사** — fixture flag on이면 asset을 복사하고 off이면 stale asset도 제거한다. positive/negative Android build를 모두 기록했다.
25. **iOS asset 복사** — 동일 flag 조합의 iOS resource가 opt-in에서만 들어간다. positive/negative iOS Simulator build를 모두 기록했다.
26. **Fixture identity** — Android APK, iOS bundle, repository fixture SHA-256은 모두 `fec26d7a64139fc16d9c313e630932bc76d37075a65084bad49b1bcf253df1b5`다.
27. **기본 artifact 제외** — flag off Android APK와 iOS app bundle에서 해당 HTML asset이 없다. 기본 산출물 포함으로 오인할 새 FFI symbol도 추가하지 않았다.
28. **화면 증거 경계** — PR에 넣는 두 PNG는 화면 visual smoke다. 물리 기기, hardware GPU pixel readback, Android/iOS parity 또는 iOS compositor 이후 픽셀로 설명하지 않는다.
29. **검사 상태 표기** — focused tests·Clippy·fmt는 로그와 수량이 기록돼 있다. 반면 `c11_1_root_html` filter는 29개 중 27개 통과, 변경하지 않은 auto-row assertion 2개 실패다. PR 설명에서 이를 숨기거나 suite 전체를 green으로 표기하지 않는다.
30. **문서·PR 링크** — PR은 계획 PR [#1219](https://github.com/ohah/spinon/pull/1219)와 evidence를 연결한다. merge SHA 기준 계약 0059·STATUS·공개 roadmap 동기화는 별도 후속 docs PR로 수행하며 이 candidate에서 완료 수를 앞당기지 않는다.

diff whitespace check는 통과했다. 발견된 동작 결함은 없었다. 남은 제한은 29번의 두 style suite failure 및 28번의 simulator 전용 관측 범위다.
