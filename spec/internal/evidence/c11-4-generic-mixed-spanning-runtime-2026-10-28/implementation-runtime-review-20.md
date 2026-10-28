# C11.4 generic mixed spanning 구현·runtime 적대 검토 20개

계획 검토와 다른 실패 가설로 구현 및 실제 시뮬레이터 입력을 확인했다. 기본 근거는 [실행 기록](./README.md), [최종 캡처 JSON·로그](./runs/), 변경 route의 Rust tests다.

| # | 공격 관점 | 확인 결과 |
|---:|---|---|
| 1 | 기존 고정 ID root가 새 generic route로 잘못 소비되는가 | generic route는 `c114-` root ID에서 `None`을 반환하고 fixed-ID route는 기존 dispatch에 남아 있다. fixed/generic Rust tests가 각각 구분한다. |
| 2 | 런처 scene allowlist까지 임의 확장되는가 | FFI allowlist는 기존 네 launcher ID 그대로이고 HTML root/item ID만 `generic-host-*`로 바뀐다. |
| 3 | 다른 namespace/tag/부모 root가 수락되는가 | HTML `div`와 document-root 직계 관계를 강제하며 다른 형태는 fail-closed한다. |
| 4 | 계산 스타일 profile 또는 document revision이 어긋나도 layout을 만드는가 | runtime profile, generation 및 document/render-tree/style revision 일치를 확인한다. stale revision·wrong profile test가 오류를 확인한다. |
| 5 | child의 추가·누락·순서 변형을 정상 fixture로 취급하는가 | 허용된 child 수와 네 고정 source order만 매치한다. extra/missing/reordered malformed 입력은 테스트에서 거부한다. |
| 6 | text item이 여러 자식 또는 nested element를 통해 텍스트를 위조하는가 | 단일 direct text leaf만 허용하고 nested/extra child를 거부한다. |
| 7 | 같은 child topology에서 text content만 변경해도 통과하는가 | 두 고정 Ahem 문자열과 정확히 일치해야 하며 changed-text negative control이 실패한다. |
| 8 | text와 image 태그를 서로 바꾸어 intrinsic source를 혼동시키는가 | 각 expected item의 tag를 확인하고 잘못된 조합은 unsupported다. |
| 9 | 올바른 `<img>` tag로 다른 URL 또는 decode 불가 bytes를 넣는가 | 고정 PNG URL, `ImageRequest` decode, resource snapshot 생성이 모두 필요하다. wrong source를 negative control로 확인했다. |
| 10 | PNG natural size와 authored transferred size가 뒤바뀌는가 | decoded natural `24×16`을 확인하고 height `48`, width `auto`에서 transferred inline contribution `72`를 검증한다. |
| 11 | image의 aspect-ratio나 width를 덧씌워 다른 계산을 유도하는가 | auto width, 고정 height, 미지정 aspect-ratio를 요구한다. 잘못된 height 테스트를 둔다. |
| 12 | root grid의 track 수·row 수·dimension·edge를 넓게 허용하는가 | 네 min-content columns, 세 `60px` rows, `220×180px` 및 0 edge/spacing 계약을 style validator가 검사한다. |
| 13 | item placement가 content와 불일치해도 같은 scene으로 계산되는가 | column start/span, row, auto end와 static position을 scene별로 검증한다. wrong-span test는 부분 입력 없이 오류를 반환한다. |
| 14 | text contribution이 system font, wrap, line-height 또는 writing mode 차이로 오염되는가 | Ahem bytes를 등록하고 text CSS 및 horizontal writing style을 검사한다. 시스템 font를 쓰지 않는 engine 경로다. |
| 15 | reverse DOM 순서가 A/B/C의 intrinsic source 또는 frame을 섞는가 | 네 scene에서 source order, node별 intrinsic source·revision, identity별 frame을 assertion하고 두 플랫폼에서 모두 실행했다. |
| 16 | track 값만 맞고 contribution 종류/span이 잘못되는가 | min-content contribution 종류와 span 2/3, 값 50/72/180을 item ID별로 추적한다. |
| 17 | 빈 네 번째 track·row pass·root width가 누락되어도 성공하는가 | 최종 track list, columns 후 rows 순서, root `220×180` frame을 테스트와 캡처 trace에서 확인했다. |
| 18 | malformed mixed 입력이 text-only 또는 image-only 성공으로 강등되는가 | 혼합에 필요한 image가 없을 때 `Ok(Some(_))`를 반환하지 않으며 malformed span cases는 fail-closed다. |
| 19 | draw/present 없는 layout trace나 이전 source artifact를 성공 근거로 쓰는가 | 최종 Android/iOS JSON 모두 네 scene에 `drawPresented: true` 및 source/app hash를 기록하고 최종 소스 수정 후 matrix를 재실행했다. |
| 20 | fixture 코드가 기본/product에 노출되거나 이 제한 slice가 공식 완료로 승격되는가 | 기본·product archive negative control과 fixture-test positive control이 통과했다. `STATUS.md`와 공식 `72/213`은 변경하지 않았다. |

**판정:** 구현·runtime 검토 20/20. 두 플랫폼은 각 4/4 scene을 통과했다. Clippy strict 실행에서 발견된 이번 코드의 두 lint는 수정했다. 변경 crate 검사는 통과했으며, 별도 두 기존 lint 때문에 전체 strict Clippy 결과를 통과로 부르지 않는다.
