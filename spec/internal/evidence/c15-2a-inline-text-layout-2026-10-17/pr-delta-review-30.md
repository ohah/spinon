# C15.2a PR 변경 실패 관점 검토 30개

검토 대상은 이 PR의 실제 staged delta 전체다. 구현 실패 검토와 별도로 변경 경계·소비자·명세·배포 영향에서 재검토했다. 표 작성 뒤 review 문서와 실행 README의 링크/수치도 다시 확인한다.

| # | PR 변경 실패 관점 | 변경 대조와 결과 |
| --- | --- | --- |
| 1 | C15.2a 구현이 parent C15를 완료로 표시하는가 | `spec/STATUS.md` C15 체크는 `[ ]`이며 C15.2b/C15.3 및 U04/S07/X16 잔여가 유지된다. |
| 2 | 내부 계약 버전이 기능 추가만으로 증가하는가 | 계약 0070은 pre-release `0.1.0`을 유지한다. |
| 3 | `ParagraphInput` 새 field로 기존 내부 호출자가 깨지는가 | workspace의 모든 literal/call site를 함께 갱신했고 full workspace test가 통과한다. |
| 4 | inline style 숫자 타입이 플랫폼 의존성을 끌어들이는가 | CSS px `f32` 값과 기존 `LayoutLengthPercentage`만 사용하며 새 dependency는 없다. |
| 5 | public re-export가 의도하지 않은 app CSS API를 만들었다고 보이는가 | 타입은 `spinon-text-layout` 내부 geometry input이며 contract와 roadmap이 public API 아님을 밝힌다. |
| 6 | HostDocument source collection에서 DFS child order가 역전되는가 | reverse stack traversal과 source-order fixture가 직접 대조된다. |
| 7 | 빈 text node가 visible source나 decorated owner를 만들어내는가 | 빈 DOM text는 건너뛰고 zero visible range decoration은 `InlineBoxWithoutText`다. |
| 8 | hidden subtree가 unsupported span 판정보다 먼저 빠지는가 | `display:none` branch가 subtree collect 후 skip하며 hidden style 누락은 별도 오류다. |
| 9 | paragraph 밖의 text/알 수 없는 style key가 섞이는가 | 기존 unknown/source-node validation이 유지되고 전체 layout suite가 통과한다. |
| 10 | visible non-HTML span이나 다른 display가 조용히 평탄화되는가 | adapter가 namespace/tag/display와 다른 layout fields를 확인해 진단한다. |
| 11 | margin·percentage·negative border·overflowing style sum을 기본값 처리하는가 | 각각 host negative test에서 `UnsupportedTextParagraph`로 거부한다. |
| 12 | 세 단계 중첩이 C15.2a 지원 범위로 새어 나오는가 | 두 단계 상한 검사와 명세 표가 일치하고 depth negative test가 통과한다. |
| 13 | 동일 NodeId의 owner path가 끊겨 중간 text를 덮는가 | source-order re-entry guard와 `[A] → [] → [A]` regression이 포함된다. |
| 14 | 동일 owner ID가 다른 parent/depth에서 재사용되는가 | NodeId별 depth와 prefix path 불일치에서 error를 낸다. |
| 15 | source boundary가 UTF-8 interior byte에서 inline box를 생성하는가 | input source endpoint 검사와 output line/run/cluster UTF-8 boundary 검사 모두 존재한다. |
| 16 | normalized collapsed whitespace가 UTF-16 source를 잃는가 | existing CRLF/collapsed-space source-map tests가 유지되고 full suite가 통과한다. |
| 17 | inline start/end가 wrong byte index나 역순으로 전달되는가 | owner range는 normalized byte endpoints를 사용하고 event ordering은 end→start, nested start outer-first/end inner-first다. |
| 18 | zero horizontal edge가 inline marker를 불필요하게 추가하는가 | width 0 event는 생략하고 vertical edge는 text fragment 후처리만 한다. |
| 19 | nested box가 line breaking에 기여하지 않아 wrapping이 틀리는가 | Parley inline marker width를 measure input에 넣으며 고정 `nested-wrap` oracle를 통과한다. |
| 20 | decorated span의 edge width가 두 줄 모두에서 중복되는가 | 첫/마지막 edge marker의 line 위치와 각 줄 rect를 별도로 비교한다. |
| 21 | line fragments가 union되어 줄 사이 영역을 덮는가 | map key는 `(NodeId, line_index)`이고 테스트가 두 line rect를 개별 확인한다. |
| 22 | vertical padding이 CSS line height나 text content rect를 늘리는가 | vertical edge는 owner fragment 후처리에만 적용하며 plan geometry가 일치한다. |
| 23 | Parley aggregate overflow가 성공 snapshot으로 노출되는가 | nested `f32::MAX` negative case와 line/run/cluster range fail-closed guard를 포함한다. |
| 24 | unexpected/out-of-flow Parley inline item이 owner로 오인되는가 | `InFlow`, ID boundary, finite positioned geometry와 width equality를 확인한다. |
| 25 | text baseline을 위해 raw font metrics가 사라지는가 | `quantize=true`에서 Roboto raw ascent/descent를 별도 보존하고 line baseline `19px`를 확인한다. |
| 26 | Roboto 사전 fixture의 1px baseline marker가 빠져 폭을 1px 작게 보는가 | test-only anchor를 통해 `93.4063px` parent, `46.2031px` 두 text ranges, `[46.2031,19,1,0]` marker를 확인한다. 실제 `inline-block` 제품 지원은 주장하지 않는다. |
| 27 | Taffy measure snapshot이 final width/revision과 불일치해 재사용되는가 | 기존 exact width·revision gate를 변경하지 않았고 integration suite가 통과한다. |
| 28 | evidence 수치가 현재 로그와 다르거나 platform 실행을 과장하는가 | text `17`, C15 layout `9`, workspace `947/0/4`를 재실행 로그와 대조한다. Android/iOS는 compile-only로 쓴다. |
| 29 | Rspress dead-link 실패와 Tailnet 경로 실패가 C15 링크 결함을 숨기는가 | 전체 docs build의 기존 dead-link 472개와 9-route isolated build 결과를 분리했다. Agent Browser에서 roadmap의 기존 `/spec/` C15 경로가 404임을 재현하고 제공되는 `/internal/` 미러로 고친 뒤 README, implementation review, PR review route가 열리는 것을 확인했다. |
| 30 | Tailnet이나 PR screenshot이 Spinon 화면처럼 오해되거나 배포 경로를 넓히는가 | Chrome 이미지는 oracle 전용, Agent Browser 캡처는 로드맵 문서 미리보기로 구분한다. 최종 SHA manifest가 동기화한 page/evidence/asset을 열거하고 모든 교체본은 Tailnet root 밖에 백업했다. 경로 삭제와 Pages/`gh-pages` 배포는 없다. |

남은 검증 경계는 runtime text projection/painter, Android/iOS Simulator 앱 화면, actual glyph/overflow pixels다. C15는 미완료다.
