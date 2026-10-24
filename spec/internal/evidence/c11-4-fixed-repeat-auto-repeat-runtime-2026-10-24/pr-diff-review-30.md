# C11.4 fixed-repeat + auto-repeat PR diff 적대적 검토 30개

구현/runtime 20개 검토와 별도로, PR에 들어갈 최종 코드·fixture·계약·계획·상태 대장·public roadmap·실행 자료의 변경 경계를 검토했다.

1. **변경 단위:** cascade adapter, 테스트, fixture, app harness 및 관련 문서만 포함한다. vendored Taffy 알고리즘은 수정하지 않았다.
2. **일반 admission:** 문서와 코드가 이 제한 opt-in runtime profile을 일반 CSS 지원으로 확대하지 않는다.
3. **계약 버전:** 내부 계약 0059는 0.1.0을 유지하고 새 공개 API를 주장하지 않는다.
4. **상위 상태:** C11.4 intrinsic track sizing checkbox가 미완료로 남아 있다.
5. **완료 수:** 72/213은 그대로다. 현재 slice는 별도의 공식 checkbox ID가 아니라 C11.4 진행 근거로 기록했다.
6. **완료 수 의미:** STATUS와 roadmap 설명은 이 값이 checked work-item 수이지 공개 지원 API 수나 merged PR 수가 아님을 구분한다.
7. **계획 경계:** plan status가 구현/runtime 검증 완료 및 PR 진행 중으로 바뀌었고 이전 “구현 시작 전” 표현은 제거됐다.
8. **계획 검토:** 독립 plan review 20, revision plan review 20, plan PR diff review 30 링크가 유지된다.
9. **비교 근거:** Chrome 156 precomparison 링크와 fixture 경로가 사용되며 Chrome 결과를 Spinon 동작으로 오인하지 않는다.
10. **규범 링크:** W3C CSS Grid 2025 CRD의 repeat notation 절을 바로 가리킨다.
11. **분리한 코드 책임:** extracted auto-repeat module을 명시 경로로 등록하고 기존 adapter 소스 책임을 유지한다.
12. **computed boundary:** fixed breadth 수용 기준이 authored token이 아닌 finite nonnegative computed LengthPx로 기술됐다.
13. **calc claim:** absolute calc의 authored spelling을 보존하거나 복구한다고 하지 않고 Stylo normalization 경계를 명시한다.
14. **repeat cardinality:** 하나의 auto-repeat 및 하나의 fixed-repeat이라는 제약이 계획·계약·runtime 결과에서 서로 일치한다.
15. **pattern cardinality:** fixed pattern 1–2 track 제한과 10,000 expanded descriptor cap이 문서와 tests에서 맞는다.
16. **ordering:** prefix/suffix order와 auto-repeat start/end index update를 source 및 tests가 확인한다.
17. **failure behavior:** overflow/allocation 실패가 node/property 오류를 반환하고 부분 typed output이 없음을 테스트한다.
18. **unsupported shapes:** percentage, unresolved calc, intrinsic/flex/minmax, names, areas, 추가 repeat은 제한 범위 밖이라고 일관되게 적었다.
19. **compatibility guard:** 기존 auto-repeat + template-area acceptance를 유지하는 회귀 테스트가 있다.
20. **cross-axis guard:** mixed repeat을 다른 axis auto-repeat과 조합하는 입력은 계획에 명시한 profile 경계로 거부한다.
21. **scene matrix:** 고정 JSON fixture에 고유 scene ID 9개가 있고 C11.4 scene catalog count도 449로 갱신됐다.
22. **oracle mapping:** runtime fixture의 각 scene이 Chrome oracle ID를 지시하며 capture validator가 9개 기대 track/frame을 확인한다.
23. **axis coverage:** columns/rows, prefix/suffix, auto-fill/auto-fit, empty/occupied, constant calc가 계획과 evidence에 맞게 배치됐다.
24. **Android gate:** 신규 Android scene 및 PixelCopy validation은 opt-in runtime GPU fixture-test build를 요구한다.
25. **iOS gate:** 신규 scene ID는 Swift conditional compilation으로 opt-in build에서만 들어가며 Xcode compile과 boundary test가 통과했다.
26. **frame/trace evidence:** 최종 AVD·Simulator result JSON 각각 9 scene의 ready frame, track trace와 draw/present를 기록한다.
27. **pixel evidence:** 각 platform의 원본 log에서 9 scene matched summary와 11/11 exact sample을 확인했다.
28. **screenshot choice:** PR용 occupied auto-fit screenshots는 최신 source-diff runs에서 선택했고 Android/API 37 AVD와 iPhone 17 Pro/iOS 26.2 Simulator label이 분명하다.
29. **artifact claims:** 9개 marker의 Debug artifact negative/positive 결과를 JSON과 markdown에 기록하되 Release APK/IPA 전체 exclusion으로 과장하지 않는다.
30. **환경 및 실패 공개:** 초기 AVD/iOS bring-up 실패와 Xcode linker/debug-map warning을 남기고, final runs/builds가 성공한 사실과 limitation을 함께 적었다.

**검토 결과:** 30개 관점에서 차단 결함은 발견하지 못했다. 이 검토는 전체 CSS Grid, WPT, Android hardware GPU, iOS real-device/compositor 이후 pixels 검증을 뜻하지 않는다.
