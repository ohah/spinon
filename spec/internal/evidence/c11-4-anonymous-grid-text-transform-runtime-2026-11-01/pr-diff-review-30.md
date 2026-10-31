# C11.4 text-transform PR diff 실패 관점 검토 30개

- 검토 대상 PR: [#952](https://github.com/ohah/spinon/pull/952)
- 구현 diff head: `c75b76cc2b5dbd2f75aadca188c6d0ce016e11ee`
- 기준: `main`의 PR base `b77964973b40534c03a43a5e1fa86993fb6f7f42`
- 범위: 실제 구현·adapter·고정 fixture·근거·상태 변경. 이 결과 문서는 검토 완료 후 추가했으며 자체 문구는 30개 코드 관점에 포함하지 않는다.
- 판정: 아래 검토에서 수정이 필요한 결함은 발견하지 못했다. glyph painting과 일반 문서 CSS 경로는 이번 diff가 지원한다고 주장하지 않는다.

1. **scene allowlist 중복·누락** — FFI와 runtime의 6개 ID가 `none|uppercase|lowercase` × min/max-content의 완전한 조합인지 확인했다. 각 목록의 길이·순서·고유성 테스트가 빠진 scene과 중복을 거부한다.
2. **알 수 없는 `text-transform-*` ID의 fallback** — 등록되지 않은 transform scene이 기존 일반 anonymous-text scene으로 흘러가지 않는지 확인했다. prefix 미등록은 `unknown anonymous text scene` 오류로 닫히며 fixed source 생성기도 `None`을 돌려준다.
3. **computed-style owner의 종류** — style owner가 사라졌거나 Text node일 때 Element로 잘못 캐스팅되지 않는지 확인했다. snapshot Element lookup 실패는 unsupported profile로 끝난다.
4. **transform property 누락·값 불일치** — 계산 style에 기대 속성이 없거나 다른 scene 값이 들어오면 전달되지 않는지 확인했다. scene별 값 불일치와 누락을 거부한다.
5. **지원하지 않는 keyword·표기 변형** — `capitalize`, `full-width`, 대소문자 변경, 끝 공백을 허용하지 않는지 확인했다. 허용 문자열과 완전히 일치해야 통과한다.
6. **부모 `lang`으로 direct `lang` 누락을 보충** — 조상만 `lang=en`이고 Grid owner에는 속성이 없는 경우가 거부되는 테스트를 확인했다.
7. **직접 언어 값의 과잉 허용** — `en-US`가 `en`으로 간주되지 않는 테스트를 확인했다. 이 fixture는 owner의 정확한 `lang="en"`만 허용한다.
8. **`xml:lang`과 HTML `lang` 우선순위 혼동** — owner에 XML namespace의 `lang`이 함께 있으면 명시적으로 profile에서 거부한다.
9. **다른 HostDocument generation의 style 재사용** — adapter가 snapshot generation identity를 비교해 stale style을 거부하는 분기를 확인했다.
10. **다른 document revision의 style 재사용** — 이전 document revision 입력은 거부된다. 전용 회귀가 이를 호출한다.
11. **다른 render-tree revision의 style 재사용** — document revision이 같아도 render-tree revision이 다르면 거부된다. 전용 회귀가 이를 호출한다.
12. **Grid owner style 대신 child style 사용** — computed `text-transform`은 정확한 Grid root style에서 확인되고 paragraph에 전달된다. 다른 노드 style을 찾아 보정하지 않는다.
13. **고정 직접 자식 구조 우회** — scene별 direct child 개수와 예상 text source를 검증하는 경로를 확인했다. 추가 node, block, 다른 text는 matrix에 들어가지 않는다.
14. **원문 HostDocument mutation** — 대문자/소문자 변환이 DOM/HostDocument text를 바꾸지 않는 단위·runtime assertion을 확인했다. 원문은 계속 `IIII ii`다.
15. **UTF-16 source offset drift** — ASCII 변환 후 각 source node와 UTF-16 범위가 일대일로 보존되는 Rust text-layout 및 Grid runtime assertion을 확인했다.
16. **UTF-8 paragraph range drift** — 모든 fixture 문자가 ASCII라 변환 전후 byte 길이가 유지되고, paragraph UTF-8 range도 source 위치와 일치함을 확인했다.
17. **공백 축약과 원문 범위 합치기** — `II   ii` 변형에서 축약된 세 공백이 하나의 paragraph 위치를 공유하더라도 각각의 source UTF-16 범위를 유지하는 회귀를 확인했다.
18. **ASCII 밖의 locale-sensitive casing** — `İi`에 lowercase를 적용하면 실패하고 `UnsupportedTextTransformProfile`을 반환하는 테스트를 확인했다. 비ASCII casing을 조용히 근사하지 않는다.
19. **`none`의 비ASCII 호환성** — 같은 `İi` source는 `text-transform:none`일 때 기존 text 경로에서 원문 그대로 layout되는 테스트를 확인했다.
20. **inline descendant의 부분 변환** — source에 inline ancestor가 있으면 변환 profile 전체를 거부한다. 일부 run만 바꾸고 나머지를 조용히 유지하지 않는다.
21. **inline style/run-style 혼합** — 변환이 켜진 상태에서 inline style map 또는 inline-run style이 비어 있지 않으면 normalization 단계가 profile을 거부한다.
22. **uppercase의 ASCII 변환 방향** — `IIII ii`가 layout snapshot에서 `IIII II`가 되고 intrinsic min/max 폭이 independent font advance와 맞는 테스트를 확인했다.
23. **lowercase의 ASCII 변환 방향** — `IIII ii`가 `iiii ii`가 되며 lowercase min/max 폭이 별도로 계산되는 테스트를 확인했다.
24. **intrinsic 측정과 실제 paragraph layout 간 mode 불일치** — engine의 intrinsic 및 normal layout snapshot 생성 모두 같은 `TextTransformMode`를 복사한다. 통합 fixture에서 text, advances, intrinsic inputs를 함께 비교한다.
25. **변환이 달라도 결과 문자가 같은 경우 cache alias** — `TextLayoutSnapshot`의 equality가 `text_transform`을 비교해 mode가 다른 결과를 동일 snapshot으로 취급하지 않는다.
26. **min-content/max-content scene 혼동** — 각 mode마다 별도 track ID를 만들고 selected track, contribution kind와 line count를 검증한다. 독립 Noto Sans KR advance를 기준으로 0.03125 CSS px 허용치를 적용한다.
27. **row·root geometry 회귀** — min-content scene 두 줄/20px, max-content scene 한 줄/10px의 expected root height와 row track을 검증한다. 각 mode에서 Android/iOS matrix가 모두 6/6 통과했다.
28. **fixture ABI 기본 빌드 누출** — 기본 `c04-runtime-gpu` archive의 raw scan에서 새 scene ID 0/6, export-name 0/2를 확인하고 fixture-test archive positive control에서 6/6, 2/2를 확인했다. 이 결과를 APK/IPA 전체 bundle 증명으로 확대하지 않는다.
29. **플랫폼 harness의 일반 빌드 노출** — Android scene helper는 fixture BuildConfig가 꺼지면 `null`을 반환하는 guard와 boundary test가 있다. iOS runtime scene 분기는 fixture-test compile condition 아래 연결된다. 사용 가능한 build 설정도 실행 근거에 적었다.
30. **runtime 상태·화면 근거 과장** — capture는 설치 APK/executable hash와 여섯 장면 matrix를 함께 보존한다. representative screenshot은 `layout=ready boxes=1` 상태만 보여주며 glyph pixels나 iOS compositor 후 readback으로 설명하지 않는다. 실패·superseded 캡처는 별도 기록하고 성공 matrix에 합산하지 않았다.
