# C11.4 Unicode text-transform PR 변경 적대 검토 · 30개 관점

검토일: 2026-11-02
대상 PR head: `b3dadbc6ae28712b656a5ff24157cb4939d16252`
비교 기준: `origin/main` `f037a3c50...`
검토 범위: PR #991 구현·FFI·Android/iOS adapter·fixture·명세·runtime evidence diff. 각 관점은 별개의 실패 경로를 확인한다. 코드 리뷰 결과는 저장된 focused test/build/simulator evidence와 대조했으며, 새 통합 실행을 CI 성공으로 대신하지 않는다.

## 판정

**PASS 30 / PARTIAL 0 / FAIL 0.** 최초 검토에서 attached root cache가 scene ID를 확인하지 않아 동일 host의 다른 scene을 재사용할 수 있는 경계를 발견했다. `b3dadbc6`에서 저장·비교 scene ID를 추가하고 FFI source test와 Android/iOS simulator 행렬을 다시 실행해 수정했다. GitHub PR Checks에는 보고된 check run이 없어 CI green으로 주장하지 않는다.

## 적대적 검토 30개

| # | 실패 관점 | 결과와 근거 |
|---:|---|---|
| 1 | Unicode case mapping dependency가 제품 기본 dependency graph에 들어가는가 | **PASS** — ICU4X dependency는 `c11-4-text-transform-unicode-fixture-test` feature에서만 활성화된다. 실행 근거의 기본·fixture dependency/archive 비교를 확인했다. |
| 2 | Unicode scene이 fixture feature를 끈 상태에서도 노출되는가 | **PASS** — runtime·FFI fixture 경로와 Unicode tests가 opt-in feature에 연결된다. feature-off 기본 text-layout 96/96 근거를 유지했다. |
| 3 | `text-transform:none`이 새 normalization 경로에서 바뀌는가 | **PASS** — None은 profile validation을 통과하고 replacement 목록은 비어 있다. Android/iOS matrix의 기존 None 장면이 모두 통과했다. |
| 4 | 기존 ASCII uppercase/lowercase/capitalize 입력과 결과가 회귀하는가 | **PASS** — 기존 ASCII 12 scene은 그대로 두었고 새 Unicode 11 scene과 함께 양 simulator에서 23/23 통과했다. |
| 5 | ASCII-only uppercase/lowercase가 의도치 않게 Unicode 전체 casing으로 넓어지는가 | **PASS** — 비 ASCII uppercase negative test는 `UnsupportedTextTransformProfile`을 확인한다. candidate 지원은 Unicode capitalize에 한정한다. |
| 6 | allowlist가 부분 문자열·접두사만 확인해 임의 입력을 허용하는가 | **PASS** — Unicode admission은 정확한 concatenated source 문자열 전체를 고정 목록과 비교한다. `ßeta!` negative test가 거부된다. |
| 7 | 임의 locale이 English mapping으로 처리되는가 | **PASS** — Unicode admission은 정확한 `en` locale만 허용하고 mapper도 English identifier를 쓴다. `de` negative test가 거부된다. |
| 8 | `white-space:pre` 등 collapse와 충돌하는 입력이 들어오는가 | **PASS** — Unicode profile은 `white-space:normal`만 허용한다. `pre` negative test가 layout 전에 거부된다. |
| 9 | inline style이 paragraph-wide 변환으로 잘못 합쳐지는가 | **PASS** — `inline_styles`가 비어 있지 않으면 profile 검증이 거부한다. |
| 10 | inline-run 또는 styled ancestor 경계를 평탄화하는가 | **PASS** — `inline_run_styles`와 source `inline_ancestors`를 각각 검사한다. inline source negative test가 거부된다. |
| 11 | 연속 공백 collapse 전에 titlecase해 원문 whitespace가 결과에 영향을 주는가 | **PASS** — transform은 normalized text 생성 뒤 적용된다. `ßeta   éclair`는 `Sseta Éclair`이며 세 원문 공백 mapping을 보존한다. |
| 12 | Unicode 단어 경계를 ASCII 공백이나 scalar 경계만으로 추정하는가 | **PASS** — ICU4X UAX #29 `WordSegmenter`를 사용한다. hyphen·apostrophe 장면이 양 platform matrix에 포함됐다. |
| 13 | apostrophe 내부를 새 단어로 처리해 `o'neill`을 잘못 대문자화하는가 | **PASS** — UAX #29 scene 기대값은 `O'neill`이고 양 simulator가 통과했다. |
| 14 | 하이픈을 단어 경계로 놓쳐 `well-known` 결과가 달라지는가 | **PASS** — 하이픈 경계 기대값 `Well-Known`이 Rust와 양 simulator에서 확인됐다. |
| 15 | 여는 문장부호 뒤 첫 lowercase letter를 찾지 못하는가 | **PASS** — `«élan»` scene은 `«Élan»`을 기대하며 양 simulator가 통과했다. |
| 16 | 숫자로 시작하는 segment에서 뒤의 글자를 대문자로 바꾸는가 | **PASS** — `49ers` capitalize와 None control의 폭·텍스트를 별도 scene으로 비교했다. |
| 17 | 첫 글자 뒤의 원문 casing을 lowercase나 uppercase로 강제하는가 | **PASS** — `TrailingCase::Unchanged`를 지정한다. `Sseta`와 `O'neill` 기대값을 확인했다. |
| 18 | full titlecase mapping의 `ß` 확장 scalar 하나를 버리는가 | **PASS** — `ß`→`Ss`, `ßeta`→`Sseta`를 확인하고 두 output scalar 모두 원문 UTF-16 span을 가리킨다. |
| 19 | supplementary-plane scalar의 UTF-16 길이를 1로 가정하는가 | **PASS** — 기존 source normalization은 `char.len_utf16()` 단위이며 Deseret `𐐨`→`𐐀` mapping test를 포함한다. |
| 20 | combining mark를 제거하거나 독립 문자처럼 이동시키는가 | **PASS** — `e + U+0301` 입력은 `E + U+0301`로 남는다. source mapping test와 양 simulator scene을 확인했다. |
| 21 | 대소문자 변환 뒤 source UTF-16 offset이 원문 대신 출력 길이를 따르는가 | **PASS** — replacement는 기존 source mapping을 복제한다. `ß` output 두 scalar가 같은 원문 span을 가진다. |
| 22 | 확장된 output의 UTF-8 byte ranges가 중첩되거나 stale offset을 쓰는가 | **PASS** — output scalar마다 새 byte range를 만들고 source list를 새 범위로 정렬한다. `Sseta` source assertions가 경계를 확인한다. |
| 23 | 인접 direct Text node가 별개 단어로 변환되거나 node identity가 합쳐지는가 | **PASS** — `ß`·`eta` 두 source node가 `Sseta`를 만들며 각 output mapping은 해당 원래 node ID를 유지한다. |
| 24 | collapsed whitespace 뒤의 여러 원문 source range 중 일부가 사라지는가 | **PASS** — 세 원문 공백 range가 한 normalized output range를 공유함을 Rust test와 runtime trace에서 확인했다. |
| 25 | replacement 뒤 tab advance index가 이전 UTF-8 offset에 남는가 | **PASS** — `apply_replacements`가 tab advance index를 boundary offset map으로 재계산한다. 유효하지 않은 boundary는 typed error로 거부한다. |
| 26 | auto-hyphenation marker가 transform보다 먼저 붙어 byte/source boundary를 오염시키는가 | **PASS** — normalization 순서는 text transform 후 auto-hyphenation marker 삽입이다. Unicode input은 별도 고정 allowlist로 범위를 제한한다. |
| 27 | FFI scene 목록에 누락·중복 ID가 있거나 split-source 장면이 합쳐지는가 | **PASS** — 23 ID 수와 고유성을 assert하고 `source-split-max-content`는 두 direct text node를 생성한다. |
| 28 | superseded retry 때 다른 scene의 attached root/node IDs를 재사용하는가 | **PASS** — 첫 검토에서 scene ID 확인 누락을 발견했다. `b3dadbc6`는 cache state에 scene ID를 저장하고 동일 ID일 때만 attached root를 재사용하며 FFI source test 1/1이 통과했다. |
| 29 | Android/iOS validator가 새 Unicode 허용 오차를 기존 ASCII 장면까지 넓히는가 | **PASS** — CSSOM tolerance 증가는 Unicode scene predicate에만 한정한다. 양 platform의 12개 ASCII 회귀 장면도 최종 행렬에서 통과했다. |
| 30 | 최종 근거가 이전 HEAD의 app 또는 화면 이미지라 현재 diff를 검증하지 못하는가 | **PASS** — pass05/pass06 metadata는 모두 `b3dadbc6`과 일치하고 설치 artifact SHA-256도 각 빌드 hash와 같다. 두 행렬은 23/23이다. 화면은 layout/status 및 Chrome oracle로만 설명하며 glyph paint 증거로 주장하지 않는다. |

## 결과와 한계

- final simulator evidence: [Android pass05](c114-android-23-scenes-final-pass05/matrix.tsv) · [iOS pass06](c114-ios-23-scenes-final-pass06/matrix.tsv) · [runtime README](README.md).
- scene-scoped retry cache 수정은 FFI source test와 양쪽 simulator app에서 확인했다.
- GitHub PR Checks에는 check run이 보고되지 않았다. 이는 CI green을 뜻하지 않으며, 기록된 local Rust/Node/build/simulator 결과와 별도다.
- 일반 CSS admission, arbitrary locale/font/text, inline formatting, hardware/physical device, GPU glyph raster, full WPT 및 C11.4/C11 completion은 이 PR diff 검토 범위 밖이다.
