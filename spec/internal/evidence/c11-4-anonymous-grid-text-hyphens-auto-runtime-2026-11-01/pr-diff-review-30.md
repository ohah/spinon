# C11.4 `hyphens:auto` 실제 PR diff 실패 관점 검토 30개

검토 기준은 `origin/main` `fbb26aed1acc2433e7a26072d6ae5d0e9bd9763e`와 `feat/c11-4-anonymous-grid-text-hyphens-auto`의 전체 최종 후보 diff다. 구현/runtime 검토 20개와 별도로 최종 code, plan, contract, logs, metadata, screenshot 및 artifact 문서를 대조했다. 아래 검토 파일도 동일한 PR diff에 포함되며, 항목 수·링크·결론을 별도로 확인했다.

## 최종 diff 검토

1. **변경에 다른 기능이나 이전 작업이 섞였는가? — 통과.** 201개 path는 C11.4 Auto source, 고정 fixture, simulator harness 및 해당 계획·계약·evidence로 제한된다. 기존 unrelated untracked 자료는 commit에 포함되지 않았다.
2. **미병합 candidate가 공식 완료 수에 반영됐는가? — 통과.** `spec/STATUS.md`와 C11/C11.4 checkbox는 변경하지 않았다. 공식 `72/213`은 그대로다.
3. **C11 상위 상태를 slice 통과로 잘못 승격했는가? — 통과.** 계획, 내부 계약과 evidence 모두 C11/C11.4 및 전체 CSS 지원을 미완료로 명시한다.
4. **이전 `hyphens:none|manual` 절의 Auto 거부가 새 후보와 충돌하는가? — 수정 후 통과.** 기존 절을 PR #977 병합 기준 revision이라고 명시했고, 새 세 단어 allowlist는 별도 미병합 후속 절로 분리했다.
5. **계획이 실측 결과를 public behavior로 과장하는가? — 통과.** 계획은 exact `en`, Ahem 10px, 세 단어, simulator matrix와 각 범위 밖 입력을 분리한다.
6. **후속 검증 수가 프로젝트 규칙과 맞는가? — 통과.** 계획은 구현/runtime 20개와 실제 최종 PR diff 30개를 각각 요구하고 두 검토 자료를 분리한다.
7. **내부 contract 숫자 버전이 임의로 바뀌었는가? — 통과.** `0.1.0`을 유지하고 candidate 확장은 새 절에 기록했다.
8. **CSS Text 규범이 특정 사전 알고리즘을 요구한다고 서술했는가? — 통과.** 문서는 CSS의 break opportunity와 Hypher/Chrome 관측을 별도 oracle로 기술한다.
9. **언어 fallback 범위가 계약과 구현에서 다른가? — 통과.** Auto는 exact `en`이며 missing locale, `en-US`, `fr` 및 다른 tag 거부가 source, tests와 evidence에서 일치한다.
10. **세 단어 외 일반 영어를 허용한다고 읽힐 수 있는가? — 통과.** allowlist는 구현, 계획, contract, capture oracle와 scene ID에서 동일하다.
11. **비고정 font/style/inline input이 정상 동작한다고 주장하는가? — 통과.** 제한 profile과 `UnsupportedHyphensProfile` 경계, 미지원 경우가 계획·계약·review에 드러난다.
12. **새 Rust dependency가 floating version 또는 기본 dependency인가? — 통과.** `hypher`는 exact `0.1.8`, English feature, optional dependency이며 feature graph로만 연결된다.
13. **Cargo.lock에 unrelated dependency drift가 섞였는가? — 통과.** lock diff는 Hypher package와 `spinon-text-layout` dependency entry만 추가한다.
14. **제품 archive 경계 결과가 앱 bundle 증거로 확대됐는가? — 통과.** evidence는 macOS ARM64 static archive 음성·양성 대조에 한정하고 APK/IPA 전체 exclusion을 명시적으로 제외한다.
15. **fixture feature가 Rust/runtime/FFI/platform 경로 중 하나에서 빠졌는가? — 통과.** Cargo propagation, host fixture define과 Android/iOS compile guards를 확인했다.
16. **generated marker source-map이 원문 UTF-16을 소비하는가? — 통과.** marker range는 `[i,i]`; normalized UTF-8 byte mapping은 2-byte U+00AD이며 capture oracle가 전 범위를 비교한다.
17. **layout용 사전 분절이 원문 문서 mutation으로 잘못 기록됐는가? — 통과.** code와 app logs는 raw source/document revision을 분리 검증한다.
18. **min/max-content 수치가 같은 측정 의미를 섞었는가? — 통과.** 세 단어별 min/max와 visible marker 기여가 plan, Rust tests, scene oracle 및 양 플랫폼 matrix에서 일치한다.
19. **50px track 폭을 자연 text width로 혼동한 이전 harness 결함이 남았는가? — 수정 후 통과.** `representation` line advance는 40px, track은 50px로 각각 비교한다. 수정 후 양 플랫폼 30-scene 전체를 재실행했다.
20. **line-ending discretionary hyphen 폭 또는 overflow reflow가 과장됐는가? — 통과.** 추가 폭은 줄 끝 marker 조건에만 적용하고, Parley zero-width 선택 보정은 고정 Ahem profile로 한정한다.
21. **runtime expected reject를 positive matrix로 오기록했는가? — 통과.** unsupported authored soft hyphen과 invalid keyword는 각 1건씩 expected reject이며 frame absence와 diagnostic code를 확인한다.
22. **runtime/FFI scene catalog와 JS oracle의 행렬 수가 다른가? — 통과.** 전체 30 scene 및 Auto 9 positive ID가 catalog, adapters, capture runner에서 맞는다. 최신 fixture-boundary 결과는 32/32다.
23. **Android evidence가 타겟 또는 설치 artifact를 특정하지 못하는가? — 통과.** API 37 ARM64 AVD metadata, final 30/30 matrix와 APK/build·installed SHA 일치를 고정했다.
24. **iOS evidence가 타겟 또는 설치 artifact를 특정하지 못하는가? — 통과.** iPhone 17 Pro/iOS 26.2 Simulator metadata, final 30/30 matrix와 executable/build·installed SHA 일치를 고정했다.
25. **Simulator screen을 실제 hyphen glyph/pixel 검증으로 오해할 수 있는가? — 통과.** 캡처의 검은 render surface를 확인했고 contract/evidence는 glyph raster나 compositor 이후 pixels를 증명하지 않는다고 명시한다. 해당 캡처는 PR 본문 동작 데모로 넣지 않는다.
26. **Rust·Node 결과를 기록에서 확대했는가? — 통과.** package pass/ignored 수, fixture-boundary, Clippy, formatter와 Node syntax 범위를 구분해 썼다. CI/WPT pass로 표현하지 않는다.
27. **초기 실패가 삭제되거나 최종 성공으로 혼합됐는가? — 통과.** 실패·수정 전 run은 별도 경로에 보존하고 final matrix 링크는 수정 뒤 30/30 run을 가리킨다.
28. **iOS build warning을 숨겼거나 새 변경 결함으로 오판했는가? — 통과.** build summary는 빌드가 보고한 deprecated API, cast, linker, debug-map 및 App Intents warning을 그대로 나열하고 원인을 임의 분류하지 않는다.
29. **원본 Xcode output의 후행 공백을 고쳐 raw evidence를 변조했는가? — 통과.** 원본은 보존하고 압축 복사본을 PR에 포함했다. 압축 해제 SHA-256 `f0f8179b38e28615127cf0647ab3f3a1bcf7ddda965756d87c7f956df0bedbe3`가 원본과 일치하며 `git diff --check`를 통과한다.
30. **문서 링크·artifact identity·PR review 자료가 끊겼는가? — 통과.** 계획, 이전 Chrome precomparison, contract, final run directories, summary 및 compressed raw log 경로를 확인했다. 두 platform build/install hashes와 archive hashes는 독립 metadata/근거와 일치한다.

## 발견 및 판정

최종 diff에서 기존 병합 contract와 새 Auto 후보의 역사적 범위가 혼동될 수 있음을 찾아 병합 baseline/미병합 후속 관계를 명시했다. 원본 Xcode log의 trailing whitespace 때문에 raw log를 직접 stage하지 않고 byte-identical compressed copy와 warning summary를 추가했다. 두 수정 이후 `git diff --check`와 `cargo fmt --all -- --check`가 통과했다. 이 검토에서 남은 PR diff blocker는 발견하지 못했다. GitHub CI, WPT, glyph pixels, 실기기 및 일반 CSS support는 별도 검증되지 않았다.
