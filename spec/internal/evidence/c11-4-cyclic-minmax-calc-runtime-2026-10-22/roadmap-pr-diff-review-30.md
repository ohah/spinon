# C11.4 cyclic minmax calc 상태·로드맵 문서 PR 검토 30개

검토 대상은 `spec/STATUS.md`의 PR #487 진행 기록 1개와 `spec/public/roadmap.html`의 대응 note 1개다. 코드 동작이나 C11.4 전체 완료 판정을 검토하는 문서가 아니다.

| # | 실패 관점 | 확인 결과 |
|---:|---|---|
| 1 | C11.4 체크가 부분 fixture 추가만으로 완료될 수 있음 | 상위 C11.4 체크는 `[ ]` 그대로다. |
| 2 | 완료 집계가 한 slice로 증가할 수 있음 | 공식 `72/213`을 두 문서 모두 유지한다. |
| 3 | 하위 진행 note가 전체 지원으로 오해될 수 있음 | 네 고정 scene을 제한 slice로 표현하고 전체 C11.4 미완료를 명시한다. |
| 4 | PR 번호가 잘못 연결될 수 있음 | GitHub PR #487 URL을 추가했고 해당 PR은 `MERGED`다. |
| 5 | merge 시각이 다른 PR의 시각과 섞일 수 있음 | live PR 정보의 `2026-10-22 00:50:38 UTC`를 사용한다. |
| 6 | 일반적인 PR head SHA를 merge SHA로 적을 수 있음 | merge SHA `2a04286dba1da4bef45fb6b1a1ee9a1cace54e56`가 GitHub의 `mergeCommit.oid`와 같다. |
| 7 | PR check rollup이 통과했다고 과장될 수 있음 | PR 병합 당시 check rollup이 비어 있었음을 기록한다. |
| 8 | 필수 CI 설정 상태가 누락될 수 있음 | main required status check 미설정을 기록하고 CI 통과라고 쓰지 않는다. |
| 9 | Android AVD 결과가 실기기 결과로 보일 수 있음 | API 37 ARM64 AVD로 표기하고 Android 실기기는 미검증으로 적었다. |
| 10 | Android 모델이 바뀌거나 모호할 수 있음 | 실행 근거의 `sdk_gphone64_arm64` API 37 환경을 그대로 연결한다. |
| 11 | AVD fixture 일부만 실행한 수치가 전체로 보일 수 있음 | 네 장면 각각 4/4 실행 수를 기록한다. |
| 12 | PixelCopy 표본 수가 화면 수와 혼동될 수 있음 | 독립된 화면 표본 수 9/9 exact로 썼다. |
| 13 | iOS Simulator가 물리 iPhone으로 오해될 수 있음 | iPhone 17 Pro / iOS 26.2 Simulator라고 표기한다. |
| 14 | iOS scene 성공 수가 생략될 수 있음 | 네 장면 4/4를 명시한다. |
| 15 | Metal 표본이 compositor 이후 결과로 오해될 수 있음 | 상태와 로드맵 모두 제출 전 drawable 표본이라고 한정한다. |
| 16 | Metal 표본을 OS 화면 최종값으로 확대할 수 있음 | iOS compositor 이후 pixel은 미검증으로 명시한다. |
| 17 | fixed fixture가 임의 CSS 값 지원으로 승격될 수 있음 | 고정 scene·opt-in fixture 경로로 한정하고 일반 CSS admission은 미검증으로 둔다. |
| 18 | cyclic/definite 사례 구성이 잘못 요약될 수 있음 | cyclic column·row와 definite column control, small-limit을 네 scene으로 요약한다. |
| 19 | 9px item·10px root 크기가 뒤바뀔 수 있음 | 실행 README의 small-limit geometry와 같은 순서로 기록한다. |
| 20 | 화면 바깥 Ahem glyph를 border overflow라고 부를 수 있음 | text paint라고 쓰고 CSS border stroke가 없다고 분리한다. |
| 21 | 화면 표본 결과가 CSS border 정확성으로 확대될 수 있음 | 테두리 픽셀 검증은 아니라고 명시한다. |
| 22 | Chrome 비교가 WPT 적합성으로 오해될 수 있음 | WPT는 미검증으로 유지한다. |
| 23 | API 37 AVD 실행이 hardware GPU 성능 증거로 보일 수 있음 | hardware GPU 미검증을 두 note에 남긴다. |
| 24 | 정적 archive scan이 최종 APK/IPA 검사로 오해될 수 있음 | status는 static archive marker와 positive control만 말하며 범위를 넓히지 않는다. |
| 25 | Android 화면 링크가 이전 scene을 가리킬 수 있음 | PR #487의 최종 `definite-minmax-calc-small-limit.png` 경로가 존재한다. |
| 26 | iOS 화면 링크가 이전 실행을 가리킬 수 있음 | 최종 iOS Simulator screenshot 경로가 존재한다. |
| 27 | 계획·실행·기존 review 링크 중 경로가 깨질 수 있음 | 여섯 개 로컬 대상의 존재를 확인했고 새 문서 PR review 파일도 같은 evidence route에 둔다. |
| 28 | 공개 로드맵과 공식 상태의 판정이 달라질 수 있음 | 두 문서 모두 72/213 유지와 C11.4 미완료를 표시한다. |
| 29 | HTML note가 잘못 닫히거나 기존 문장이 바뀔 수 있음 | 새 `<p>`·`<strong>`·`<code>`·`<a>` 태그 쌍을 확인했고 추가 전 파일에서 새 단락을 제거하면 원본과 byte 단위로 같다. |
| 30 | 기존 상태 기록이나 무관한 변경이 함께 섞일 수 있음 | STATUS는 새 note 한 줄만 추가했고 roadmap은 새 note 한 단락만 추가했다. |

## 판정

차단 결함은 발견되지 않았다. 이는 문서 diff와 링크 대상의 일관성 검토이며, 전체 RSPress build, WPT, 새 앱 실행 또는 전체 CSS Grid 지원 검증을 뜻하지 않는다.
