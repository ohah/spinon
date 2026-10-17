# 선택 Rspress 빌드 결과

## 실행 정보

- 기준 커밋: `f43a04ff61f9148ebf16047f069dd8280353feda`
- Rspress: `2.0.22`
- 산출물: `/private/tmp/spinon-c04-12-tailnet-roadmap-20261017/out`
- 결과: 프로세스 종료 코드 `0`, Rspress가 web/node 출력 렌더링 완료
- 요약: web 산출물 1,397.8 kB (gzip 438.3 kB), node 산출물 623.0 kB (gzip 160.1 kB)
- 원본 출력: [선택 빌드 로그](selected-build.log), 원래 바이트를 보존한 gzip 로그 `selected-build.log.raw.gz`

## 빌드 경고

Rspress는 dist 경로가 앱 root 밖에 있어 자동으로 비우지 않는다는 경고를 출력했다. 빌드 전용 출력 경로를 사용했고, 최종 68개 파일의 사전 검사 manifest가 각 파일의 길이와 SHA-256을 기록한다. 복사 전 출력 파일이 manifest와 일치하는지 확인했으며, Tailnet 목적지의 파일 삭제는 수행하지 않았다. 전체 저장소 strict build나 전체 링크 검사는 이 결과에 포함하지 않았다.

로그 보존을 위해 줄 끝 공백과 탭만 제거한 읽기용 로그를 제공한다. 원본 gzip과 `selected-build-log-normalization.json`에 원본·정규화된 크기와 해시를 함께 기록했다.
