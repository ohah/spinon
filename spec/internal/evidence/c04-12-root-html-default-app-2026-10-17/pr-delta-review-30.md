# C04.12 기본 앱 루트 HTML PR 변경 적대 검토 30개

검토 범위는 구현, 기본 asset bundling, Android/iOS 빌드 안내, 내부 계약·계획·상태 대장·공개 roadmap, Simulator 근거와 PR 제출 내용이다. 앞선 20개 구현 검토와 달리 이 표는 diff 간 사실 일치·공개 설명·산출물 추적성을 확인한다.

| # | PR 변경 실패 관점 | 결과 |
| --- | --- | --- |
| 1 | C04.12 부분 구현이 공식 완료 checkbox를 바꾸는가 | `spec/STATUS.md`의 C04.12 checkbox가 미완료로 남는다. 통과. |
| 2 | 이 PR이 공식 완료 수를 수동으로 올리는가 | STATUS 상위 체크 수를 수정하지 않았고 public roadmap counter는 STATUS를 읽는다. 통과. |
| 3 | 기존 계획에 이미 viewport/color-scheme 앱 의미가 있었다고 소급 서술하는가 | plan·STATUS·public roadmap 모두 R01의 meta 분류와 신규 C04.12 계획을 구별한다. 통과. |
| 4 | plan의 현재 상태가 구현 전 문구로 남아 있는가 | 기본 `index.html`→V8 session 연결은 확인, metadata 의미 적용은 미완료로 갱신했다. 통과. |
| 5 | contract가 default loader와 opt-in fixture를 혼동하는가 | 0077에서 기본 asset 경로와 과거 opt-in fixture를 별도로 설명한다. 통과. |
| 6 | 계약의 existing `RuntimeSession::new()` 보장이 바뀐다고 오해되는가 | JavaScript-only 생성 경로는 유지되고 `new_with_root_html`이 별도 입력으로 명시된다. 통과. |
| 7 | plan/contract가 viewport meta 보존을 계산 결과로 표현하는가 | 현재 assertion은 element/content 접근만 확인한다고 명시했다. 통과. |
| 8 | `color-scheme: light dark` 예제가 OS 색상 반영을 뜻하는가 | plan·contract·STATUS는 OS preference, media, Canvas 효과가 아직 없다고 분리한다. 통과. |
| 9 | 빈 Simulator 화면이 HTML renderer 성공 화면처럼 게시되는가 | 근거 README는 두 캡처를 빈 native host라고 설명한다. 통과. |
| 10 | screenshot 링크가 실행 로그를 대신하는가 | boot status와 V8 report를 원본 text로 저장하고 이미지 목적을 별도로 제한한다. 통과. |
| 11 | Android 실기기를 사용했다는 암시가 있는가 | 실행 근거는 AVD name, API, serial을 고정하고 실기기 미사용을 선언한다. 통과. |
| 12 | iOS simulator와 iOS 기기를 혼용하는가 | iPhone 17 Pro / iOS 26.2 Simulator와 UDID를 명시한다. 통과. |
| 13 | 한 플랫폼만 확인했는데 공통 경로라고 표현하는가 | Android API 37과 iOS 26.2에서 각각 빌드·launch·`status=0` marker를 기록한다. 통과. |
| 14 | asset 경로 대소문자가 iOS와 Android에서 다르게 실패하는가 | `index.html`, `root-app.js` 이름이 Bun output·Android assets·iOS bundle에서 동일하다. 통과. |
| 15 | package script가 app.js 번들을 제거하는가 | 기존 `app.js` 번들을 유지하고 두 root asset 번들을 뒤에 추가한다. 통과. |
| 16 | iOS copy script가 일부 asset만 오래된 build에서 복사하는가 | 세 필수 bundle 산출물 존재를 확인한 다음 app.js와 root assets를 함께 복사한다. 통과. |
| 17 | Android Gradle asset source가 새 파일을 포함하지 않는가 | APK에서 index와 root-app bytes를 추출해 빌드 asset hash와 대조했다. 통과. |
| 18 | iOS app bundle이 새 자산 없이 성공 build만 보고되는가 | `.app`에서 두 파일을 확인했고 source/bundle SHA를 각각 기록했다. 통과. |
| 19 | HTML source와 실제 APK/iOS bundle의 내용이 다르게 게시되는가 | index hash는 두 bundle에서 동일하며 root-app bundle hash도 양쪽 동일하다. 통과. |
| 20 | build가 다른 V8 revision을 사용해 결과를 재현할 수 없는가 | 고정 V8 revision `7b50b62cb18f28617959e8452e2cd18195b38bcf`를 기록한다. 통과. |
| 21 | 최초 실패한 iOS build를 숨기고 최종 성공만 기재하는가 | Swift selector/import 오류를 구현 검토에서 설명하고 수정 후 최종 build pass를 기록한다. 통과. |
| 22 | simulator 로그의 기존 `app.js` report가 root session 검사로 오인되는가 | 별도 smoke marker와 `SPINON_ROOT_HTML_BOOT` marker를 분리해 수록한다. 통과. |
| 23 | root app JS가 실제 HTML script로 parser가 실행한 것처럼 서술되는가 | 문서는 host가 별도 JS asset을 같은 V8 session에서 평가한다고 설명한다. 통과. |
| 24 | 성공 marker가 JS assertion 실패를 포함하는가 | `root-app.js`는 assertion 실패 시 throw하고 runtime report는 `status=0`에만 success marker를 낸다. 통과. |
| 25 | 문서 revision이 root attribute mutation 이전 초기화만 증명하는가 | source assertion 뒤 mutation을 되읽으며 log report의 document revision 2를 보존한다. 통과. |
| 26 | 종료 뒤 Android root session leak가 누락되는가 | AVD back 종료 로그에서 session free start/done을 실행 근거에 기록한다. 통과. |
| 27 | 새 PR 전용 evidence 경로가 index나 contract에 연결되지 않는가 | STATUS·내부 README·계약 0077·public roadmap가 새 실행 README로 연결된다. 통과. |
| 28 | public roadmap만 다른 완료 상태 또는 counter를 수동으로 보이는가 | roadmap는 C04.12 미완료와 STATUS가 counter source인 사실을 명시한다. 통과. |
| 29 | Tailnet에서 아직 확인하지 않은 페이지를 이미 동기화했다고 주장하는가 | 이 PR의 로컬 source 수정은 공개하지 않았으며 merge 후 Tailnet sync를 별도 실행 근거로 검증한다. 통과. |
| 30 | PR 본문 screenshot이 기능을 오해시키거나 존재하지 않는 화면을 표현하는가 | 앱 화면은 비어 있으므로 UI 시연 이미지로 embed하지 않는다. 링크된 캡처는 빈 native host라는 범위를 함께 표시한다. 통과. |

## 결론

현재 변경은 기본 app root HTML asset을 실제 V8 초기 document로 전달하는 제한 slice다. viewport/page scale, OS color scheme·media query, UA Canvas 색과 HTML rendering을 완료했다고 표시하지 않는다. Tailnet served-content 검증은 merge 후 최종 문서 빌드와 함께 별도로 기록한다.
