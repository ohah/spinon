# C11.4 `hyphens:auto` 구현·런타임 실패 관점 검토 20개

2026-11-01 구현 후보의 text-layout, runtime/FFI fixture, Android/iOS adapter와 저장된 실행 근거를 서로 다른 경계에서 검토했다. 항목은 이전 계획 검토를 재사용하지 않는다. 현재 작업 브랜치의 제한 후보 검토이며 PR 최종 diff 검토나 병합 승인을 대신하지 않는다.

## 검토 항목

1. **제품 기본 feature에서 의존성이 켜지는가? — 통과.** `hypher = 0.1.8`은 optional이며 Auto 전용 feature가 의존성을 활성화한다. 기본 product archive의 의존성 그래프·심볼·문자열 음성 대조는 [artifact 근거](artifacts/README.md)에 있다.
2. **영어 pattern payload가 기본 product archive로 새어 나오는가? — 통과.** 기본 `c04-runtime-gpu` archive에 Hypher 및 세 고정 단어의 양성 문자열이 없고, fixture-test archive에는 positive control이 있다. archive 한정 결과이며 APK/IPA 전체 bundle 주장은 하지 않는다.
3. **feature 전파가 text-layout에서 runtime/FFI까지 일관적인가? — 통과.** Auto 전용 feature는 C11.4 runtime fixture-test와 C04 Runtime GPU fixture-test에 연결되고 scene/host harness는 기존 opt-in fixture 경계를 사용한다.
4. **다른 글꼴·크기·방향·spacing·paragraph style을 실수로 받는가? — 통과.** Ahem 10px, 고정 line-height, LTR horizontal, normal whitespace/break/spacing과 기본 indent만 허용한다. 이 profile 밖 입력은 `UnsupportedHyphensProfile`로 실패한다.
5. **언어 미지정 또는 유사하지만 다른 language tag가 영어 사전으로 처리되는가? — 통과.** Auto는 locale이 정확히 `en`이어야 한다. `None`, `en-US`, `fr` 거부 테스트가 있다.
6. **고정 allowlist 밖 문장이나 대소문자 변형이 자동 분절되는가? — 통과.** 세 개의 소문자 ASCII 원문만 정확히 허용하며 고정 문자열 외 입력은 사전 호출 전 profile에서 차단된다.
7. **사전 출력이 바뀌거나 분절 결과가 빈/불완전할 때 조용히 다른 break를 쓰는가? — 통과.** `hypher` 결과는 빈 segment, 누적 길이 불일치, 세 기대 segmentation 불일치를 거부한다. 입력 byte 길이도 Hypher 최대 크기 전에 검사한다.
8. **source 자체 U+00AD, 복수 source 또는 inline run과 자동 기회가 섞이는가? — 통과.** 단일 direct source 및 빈 inline style/run만 받고 authored U+00AD를 거부한다. runtime reject scene과 text-layout negative tests가 있다.
9. **다중 marker를 앞에서부터 삽입해 뒤쪽 source byte offset이 밀리는가? — 통과.** byte boundary를 확인한 뒤 marker를 역순 삽입한다. 허용 단어가 ASCII이므로 breakpoint byte offset과 UTF-16 index가 같다.
10. **생성 marker가 원본 source 문자로 잘못 귀속되는가? — 통과.** 각 marker는 원문 UTF-16의 빈 range `[i,i]` 및 normalized text의 marker byte range를 갖고, 원문 source mapping은 이동된 normalized byte 범위로 유지된다. capture runner가 모든 normalized mapping을 비교한다.
11. **삽입 후 mapping 순서가 비결정적이거나 소스 범위를 가로지르는가? — 통과.** 삽입 후 `(paragraph_utf8.start, end)`로 정렬하고 겹침을 감지한다. 최종 Android/iOS scene log의 mapping이 고정 oracle과 일치한다.
12. **HostDocument 또는 저장된 source가 layout용 하이픈으로 변이되는가? — 통과.** marker는 normalized layout text에만 추가한다. 두 simulator adapter의 capture가 원문 `sources`와 document revision을 별도로 비교한다.
13. **min-content가 선택될 discretionary hyphen의 advance를 누락하는가? — 통과.** 별도 measurement 복사본에서 생성 marker를 U+2010으로 바꿔 intrinsic min-content를 계산한다. 세 단어 기대 min-content는 50/50/40px이고 Rust·runtime·simulator oracle과 일치한다.
14. **max-content에 선택되지 않은 discretionary hyphen advance가 추가되는가? — 통과.** max-content는 normalized U+00AD text의 무제한 줄 폭에서 계산하고, 세 고정 기대값 110/200/140px와 일치한다.
15. **줄 끝이 아닌 marker 또는 마지막 줄에 visible hyphen advance를 중복 더하는가? — 통과.** 현재 line range가 U+00AD로 끝나고 마지막 줄이 아닐 때만 한 em을 더한다. 세 단어별 line count/advance와 기존 `manual` regression이 이를 확인한다.
16. **Parley가 U+00AD 폭을 무시해 그릴 hyphen이 track 바깥으로 넘치는가? — 통과.** 선택 line의 visible advance가 사용 가능 폭보다 클 때만 한 번 줄 폭을 한 em 줄여 재배치한다. 50px definite-track 세 단어 행렬이 각 line advance를 정확히 확인한다.
17. **computed cascade 값과 언어 owner가 fixture matrix 바깥에서 Auto를 허용하는가? — 통과.** runtime scene matcher는 명시된 scene ID와 예상 computed 값만 진행시키며 `en` owner language를 검증한다. 공개 author CSS admission 경로는 이번 변경에 추가되지 않았다.
18. **unsupported source 또는 invalid computed value가 frame publication 전에 실패하는가? — 통과.** authored soft-hyphen `auto`는 고정 profile 오류, `balanced`는 unsupported computed value로 거절되고 두 경우 모두 frame 없는 reject 결과를 기대한다.
19. **FFI scene catalog와 runtime catalog/adapter 사이에 누락·중복이 있는가? — 통과.** Auto 9개와 기존 회귀/reject를 포함해 각 플랫폼 30-scene matrix이며 scene ID 유일성·catalog 경계 테스트가 이를 확인한다.
20. **플랫폼별 실행이 서로 다른 artifact나 느슨한 검증을 사용했는가? — 통과.** Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator가 각각 30/30이며 설치 APK/executable hash가 build 결과와 같다. capture는 raw source, UTF-16/UTF-8 mapping, intrinsic sizes, line advances, frame과 Grid tracks를 검사한다. 세부 환경·hash·로그는 [runtime evidence](README.md)에 있다.

## 발견 및 판정

초기 검증에서 발견한 Android capture의 `hyphens:initial` 기본값 분류 오류와 50px Grid track을 자연 text width로 기대한 오류를 Android/iOS 및 capture oracle에서 수정했다. 수정본 양 플랫폼 행렬을 다시 통과시켰고 실패 run은 삭제하지 않고 별도 경로에 보존했다. 구현 중 Clippy의 빈 문자열 비교도 `is_empty()`로 수정했다.

이 검토에서 남은 구현 blocker는 발견하지 못했다. 판정은 이 고정 opt-in candidate 범위에만 해당한다. 별도 [PR 최종 diff review 30개](pr-diff-review-30.md)도 완료했다. GitHub check 상태와 병합은 아직 남아 있으며, 일반 CSS admission, 다른 language/word/font, 실기기·hardware GPU, compositor 이후 pixel, WPT 및 C11.4/C11 완료를 의미하지 않는다.
