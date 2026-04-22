# v3 예제

이 디렉터리는 [plan/v3/README.md](/home/bong/CLionProjects/niginx_rust_wasm/plan/v3/README.md:1) 의 요구사항을 기준으로 만든 "플러그인 기반 gateway 플랫폼 초안" 예제다.

`v2`가 단일 Wasm 필터 중심이었다면, `v3`는 아래를 구조로 드러내는 데 집중한다.

- 다중 Wasm 플러그인 체인
- 플러그인 계약과 공통 메타데이터 키
- route, service, tenant 단위의 외부 정책 파일
- structured logging 초안
- 배포, 롤백, 호환성 문서

## 디렉터리

- `gateway/`
  - nginx 및 wasmx 런타임 이미지
  - 여러 Wasm 플러그인을 로드하는 설정
- `plugins/`
  - `auth-filter`, `header-filter`, `observe-filter`
- `config/`
  - route, tenant, plugin, compatibility 설정 예제
- `docs/`
  - 플러그인 계약과 운영 문서
- `tests/`
  - 수동 검증용 curl 예제

## 실행

```bash
cd /home/bong/CLionProjects/niginx_rust_wasm/v3
docker compose up --build
```

gateway는 `http://localhost:8082`로 노출된다.

## 예제 요청

보호된 users 라우트:

```bash
curl -i -H 'x-api-key: users-secret' -H 'x-tenant-id: team-a' http://localhost:8082/users/10
```

공개 orders 라우트:

```bash
curl -i -H 'x-tenant-id: team-b' http://localhost:8082/orders/99
```

정의되지 않은 경로:

```bash
curl -i http://localhost:8082/unknown
```

## 의도된 결과

- 요청은 `auth-filter -> header-filter -> observe-filter` 순서로 처리된다고 가정한다.
- 각 플러그인은 공통 헤더 계약을 통해 메타데이터를 전달한다.
- nginx access log는 구조화된 JSON 한 줄로 남는다.
- 정책 파일과 운영 문서를 통해 배포, 롤백, 호환성 규칙을 설명할 수 있다.

## 주의

이 예제는 `plan/v3`의 운영 구조를 보여주기 위한 초안이다. 즉시 제품 수준의 control plane을 제공하지는 않는다. 설정 파일은 외부화되어 있지만, 실제 동적 반영 API 대신 운영 절차와 배포 단위 중심으로 설계했다.
