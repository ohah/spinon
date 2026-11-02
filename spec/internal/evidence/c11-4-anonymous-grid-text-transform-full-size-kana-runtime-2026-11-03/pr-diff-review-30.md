# PR #1082 변경 diff 실패 관점 검토 30개

검토 대상은 PR [#1082](https://github.com/ohah/spinon/pull/1082)의 `main...feat/c11-4-full-size-kana-20261103` 변경이다. 검토 기준 head는 `4ec04c228b9046a52d7aec8170671b287dd613ef`다. 구현/runtime 검토 20개와는 별도로, Rust layout/runtime/FFI, Android/iOS fixture adapter, 테스트, 계약, 상태 대장과 실행 기록의 경계를 확인했다.

| # | 실패 관점 | 검토 결과 |
| ---: | --- | --- |
| 1 | CSS computed `text-transform`이 scene 이름으로부터 임의 추론되는가 | 실제 computed property를 scene의 exact expected value와 비교한다. scene 이름만으로 속성 성공을 대체하지 않는다. |
| 2 | Grid style owner가 아닌 조상 `lang`만으로 일본어 profile에 진입하는가 | owner element의 직접 `lang="ja"`를 요구한다. 상속값은 통과하지 않는다. |
| 3 | `xml:lang`, 중복 `lang`, 또는 대소문자가 다른 값이 경계를 우회하는가 | XML namespace를 거부하고 정확히 하나의 unqualified exact `ja`만 허용한다. |
| 4 | stale computed style snapshot이 최신 HostDocument에 재사용되는가 | generation, document revision, render-tree revision 검사와 기존 C11.4 revision 회귀를 유지한다. |
| 5 | `TextTransformMode` 추가로 기존 mode가 재해석되거나 누락되는가 | 변환 dispatch와 scene adapter에 variant를 명시적으로 추가했고 기존 none/case/full-width 경로 회귀가 보존된다. |
| 6 | 일반 빌드가 새 mode를 조용히 처리하는가 | feature-off text-layout 회귀는 `UnsupportedTextTransformProfile` 거부를 확인한다. |
| 7 | Android/iOS가 fixture opt-in과 Cargo feature를 다르게 설정하는가 | 두 fixture build log에 같은 `SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST=1` opt-in을 기록했다. |
| 8 | source text 범위가 임의 Japanese 문장으로 넓어지는가 | 단일 source와 고정 9개 source string 외에는 fail-closed한다. |
| 9 | CSS mapping의 58개 source에 누락 또는 중복이 있는가 | table length와 source uniqueness test가 58개 전부를 검사한다. 독립 JSON과 mapping 비교 근거도 연결했다. |
| 10 | 작은 Kana 외 큰 Kana, ASCII 또는 다른 문자가 바뀌는가 | unmapped `あ`, `A` lookup과 mixed-script pass-through control에서 identity를 확인한다. |
| 11 | Unicode normalization이나 호환 분해가 mapping에 섞이는가 | 고정 mapping 외 Unicode normalization을 호출하지 않는다. 변환표의 source code point만 바꾼다. |
| 12 | normalized text offset을 UTF-8 byte와 Unicode scalar index 사이에서 혼동하는가 | mapping은 `char_indices()` 경계에서 source span을 만들고 output UTF-8 range를 별도 산출한다. |
| 13 | supplementary Kana의 2-unit UTF-16 origin range가 변환 뒤 손실되는가 | 단위 행렬이 원문 UTF-16 span과 transformed UTF-8 span을 character별로 검증한다. HostDocument text 자체는 유지된다. |
| 14 | replacement 적용이 source run 또는 빈 range를 손상시키는가 | 단일-node allowlist, source-range bounds 확인, empty range의 boundary remap 및 sorted output source mapping을 검사했다. |
| 15 | white-space normalization 앞이나 line breaking 뒤에 변환되어 intrinsic sizing에 반영되지 않는가 | pipeline은 white-space normalize 이후, shaping/intrinsic 및 line breaking 전에 변환을 적용한다. |
| 16 | `word-break`, `line-break`, 방향, white-space의 다른 조합이 승인되는가 | `normal`, `strict`, LTR, Japanese locale 조건을 고정하고 각 범위 밖 값을 거부한다. |
| 17 | CSS Text at-risk 초안의 매핑을 안정화된 공용 규약처럼 표기하는가 | 계획·내부 계약·README는 CRD Appendix G, at-risk 및 Chrome 미지원 oracle 제한을 명시한다. |
| 18 | Chrome Range 관찰을 font advance 또는 positive transform 증거로 과대 해석하는가 | Chrome은 변환 지원을 거부한다. literal geometry만 비교하고 Katakana line-advance 차이를 문서에 남겼다. |
| 19 | matrix scene의 source, computed value, expected output이 서로 다른 파일에서 어긋나는가 | Rust text-layout, runtime scene, FFI catalog, Android/iOS adapter, independent expected-values의 10개 scene을 대조했다. |
| 20 | min/max-content와 row 기대값이 transformed literal의 sizing을 반영하지 않는가 | 각 fixed scene의 columns, rows, line advances와 node frames를 플랫폼 로그의 trace와 비교했다. 두 플랫폼 matrix가 각각 10/10이다. |
| 21 | Android frame 비교가 root-only 결과나 node ID가 바뀐 결과를 통과시키는가 | full-size-kana matcher는 두 frame의 개수, index, node ID, 좌표와 크기를 각각 검사한다. |
| 22 | iOS frame 비교가 Android와 다른 기준 또는 identity를 사용해 false positive를 내는가 | iOS matcher도 동일한 2-frame 구조, 순서, node ID와 좌표·크기 tolerance를 검사한다. |
| 23 | tolerance가 line advance 차이를 감추거나 모든 Grid scene에 확장되는가 | `1/32 CSS px` tolerance와 `0.03125` track tolerance를 full-size-kana scene에만 적용했다. Katakana line advances는 별도로 표시한다. |
| 24 | simulator 설치본이 실행된 artifact와 다른데 동일 run으로 합쳐지는가 | Android APK/install SHA와 iOS executable/install SHA가 각각 일치하고 matrix receipt에 source head를 고정했다. |
| 25 | iOS diagnostic chunking이 UTF-8 byte count와 `String.Index`를 혼용하는가 | 실제 pass01 실패 뒤 character count 기반 chunk 수로 수정했고 최종 iOS pass02에서 10/10 scene을 확인했다. |
| 26 | 이전 Android/iOS 실패가 최종 통과 수치에 합쳐지거나 숨겨지는가 | 실패 pass는 별도 폴더와 실패 원인으로 보존하고 성공 cohort는 Android pass05/iOS pass02로 분리했다. |
| 27 | fixture 코드를 기본 artifact에서도 사용할 수 있다고 잘못 주장하는가 | macOS ARM64 default archive의 0 marker/fixture export 없음과 opt-in positive archive의 10 marker/export 포함을 짝지었다. 앱 전체 APK/IPA exclusion으로 확대하지 않았다. |
| 28 | fixture catalog scene 추가 뒤 module-boundary test가 낡은 count로 잘못 green 되는가 | scene 수 133→143 변경을 boundary test에 반영하고 최종 로그 32/32를 보존했다. FFI catalog의 별도 고정 count는 유지했다. |
| 29 | 화면 캡처를 glyph raster, GPU surface 또는 pixel parity로 오인시키는가 | 대표 캡처와 README는 `layout=ready`/빈 render surface, glyph paint disabled 한계를 명시한다. PR 본문에는 시각 동작 이미지로 첨부하지 않았다. |
| 30 | 제한 후보를 C11.4/C11 완료 또는 CI 통과로 승격시키는가 | PR, contract, STATUS 모두 체크박스와 `72/213`을 유지한다. PR에는 check run이 없고 main required checks도 없어 CI 통과로 세지 않는다. |

## 판정

30개 관점에서 현재 제한 opt-in 후보를 병합 차단할 diff 결함은 찾지 못했다. 이 판정은 일반 CSS admission, WPT, 임의 Japanese segmentation, Katakana line-advance 동등성, glyph pixel, 실기기/hardware GPU 또는 전체 APK/IPA artifact 제외를 승인하지 않는다. PR check rollup은 비어 있으며 CI 통과 주장은 없다.

실행별 hash와 실패/성공 cohort는 [runtime README](./README.md) 및 [SHA-256 manifest](./SHA256SUMS.txt)에 있다.

## 검토 기록 추가 확인

위 기능 diff 검토 뒤 PR head `4ec04c2`에서 `5abe9cd`로 추가된 변경은 STATUS/0059의 검토 링크, 이 30개 검토표, manifest digest 갱신뿐이다. 구현·런타임·플랫폼 코드는 바뀌지 않았다. 세 evidence 문서의 상대 링크와 manifest 120개 digest를 다시 검증했고, 공식 C11 미완료 및 `72/213`, PR의 no-check 상태가 유지됨을 확인했다.
