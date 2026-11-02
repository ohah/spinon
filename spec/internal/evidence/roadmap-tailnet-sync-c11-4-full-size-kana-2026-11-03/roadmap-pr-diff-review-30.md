# C11.4 `full-size-kana` 로드맵 변경 PR diff 검토 30개

검토 대상은 `spec/public/roadmap.html`에 PR #1082의 병합된 제한 후보 한 건을 추가한 변경이다. 공식 상태 대장과 구현 계약은 앞선 PR에서 이미 병합됐으며, 이 변경은 공개 로드맵 카드의 내용과 링크만 갱신한다. 기존 카드 및 완료 집계는 비교 대상으로 유지했다.

| # | 실패 관점 | 확인 결과 |
| ---: | --- | --- |
| 1 | 구현 PR을 아직 열린 상태로 표시하는가 | 카드에 PR #1082 `Rebase and merge`를 표시한다. |
| 2 | 계획 PR과 구현 PR을 하나로 합치는가 | 계획 문서 링크와 구현 PR #1082 링크를 분리한다. |
| 3 | 병합 시각을 local time으로 바꾸거나 틀리게 적는가 | GitHub 병합 시각 `2026-11-02T20:18:40Z`를 그대로 적었다. |
| 4 | 병합 commit 대신 source head를 merge SHA로 쓰는가 | GitHub PR의 merge SHA `3c34f3b8c71eb9a960414942cecab4179d62dbb8`를 표기한다. |
| 5 | `full-size-kana`와 `full-width`가 혼동되는가 | CSS property value를 정확한 `text-transform:full-size-kana`로 쓴다. |
| 6 | CSS Text CRD와 at-risk 의미를 보편 안정 표준처럼 과장하는가 | CSS Text Level 3 CRD Appendix G와 Unicode 15.0 mapping만 명시한다. |
| 7 | 58개라는 mapping 수가 구현 자료와 맞지 않는가 | 공개 문구의 58은 내부 계약/runtime 근거의 고정 mapping 수와 일치한다. |
| 8 | 모든 Japanese text에 변환을 적용한다고 읽히는가 | direct `lang=ja`와 고정 source allowlist에 한정한다고 표시했다. |
| 9 | opt-in fixture 한계를 누락해 공개 CSS 지원으로 오해되는가 | “제한 opt-in fixture”와 “일반 CSS admission 미완료”를 함께 적었다. |
| 10 | simulator를 물리 Android 기기라고 부르는가 | Android API 37 ARM64 AVD라고 적었다. |
| 11 | iOS Simulator를 iPhone 실기기라고 부르는가 | iPhone 17 Pro / iOS 26.2 Simulator를 명시했다. |
| 12 | 두 플랫폼 scene 수를 합산해 20/20 한 번의 기기 결과로 표현하는가 | AVD와 iOS Simulator 각각 10/10이라고 나눠 썼다. |
| 13 | Android AVD의 software rendering을 hardware GPU 결과로 확대하는가 | hardware GPU 검증은 미완료 범위에 둔다. |
| 14 | 실기기 검증 여부를 빠뜨리는가 | 실기기는 미완료 범위에 명시했다. |
| 15 | WPT 적합성 결과가 있는 것처럼 표현하는가 | WPT 미완료라고 표시했다. |
| 16 | Chrome 156이 positive transform oracle로 기술되는가 | Chrome이 property를 지원하지 않아 positive oracle이 아니라고 적었다. |
| 17 | literal geometry 대조가 transform semantic parity처럼 읽히는가 | Chrome의 unsupported 상태와 transform oracle 한계를 분리했다. |
| 18 | `layout=ready` 화면을 glyph pixel 증거로 잘못 표시하는가 | 캡처가 glyph pixel 근거가 아니라고 썼다. |
| 19 | 비어 있는 render surface가 텍스트 시각 출력을 증명하는가 | 화면 동작 근거라고 부르지 않고 render 상태 보고로 제한한다. |
| 20 | katakana line-advance 차이를 일반 parity 주장으로 덮는가 | 카드에서 line-advance parity를 주장하지 않는다. 상세 차이는 linked runtime evidence에 남는다. |
| 21 | 일본어 일반 단어 분절이 지원되는가 | 일본어 일반 단어 분절은 미완료 범위에 명시했다. |
| 22 | static archive scan을 APK/IPA 전체 제외 근거로 오해하는가 | 기본 static archive 경계만 확인했고 전체 bundle 제외는 미검증이라고 쓴다. |
| 23 | fixture archive positive control을 제품 기능 admission과 혼동하는가 | opt-in fixture 한계가 문장에 남아 있다. |
| 24 | C11.4 전체가 완료된 것으로 표시되는가 | 전체 C11.4 미완료를 명시한다. |
| 25 | 상위 C11이 완료된 것으로 표시되는가 | 전체 C11 미완료를 명시한다. |
| 26 | 공식 완료 수가 이번 후보 때문에 올라가는가 | 공식 `72/213`을 유지한다고 명시한다. |
| 27 | 카드 삽입으로 기존 공백/PR #995 문구가 손상되는가 | 새 card는 유일한 PR #995 카드 바로 뒤에 삽입하며, 기존 다음 hyphens 카드와 구분된다. |
| 28 | 사이트 base `/spinon/` 아래 local link가 잘못 resolve되는가 | 신규 review link를 포함해 모든 local route를 `/spinon/` 기준으로 검사한다. |
| 29 | 계획, 사전 비교, 계약, runtime/review 또는 상태 링크가 끊기는가 | source Markdown/계약 파일과 공개 상태 route가 모두 존재함을 확인한다. |
| 30 | 화면을 확인하지 않고 로드맵 카드가 실제 served content라고 주장하는가 | Tailnet 검증은 별도 Agent Browser 및 served SHA 기록 후에만 성공으로 보고한다. |

## 판정

로드맵 문구는 구현 PR #1082의 제한 범위와 병합 상태를 요약하며 C11/C11.4 완료 또는 공개 CSS 지원을 주장하지 않는다. 이 30개 관점에서 카드 source 변경을 차단할 결함은 발견하지 못했다.
