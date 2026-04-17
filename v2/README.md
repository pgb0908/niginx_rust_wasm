# v2 예제

이 디렉터리는 `plan/v2/README.md`를 최소 동작 예제로 옮긴 것이다.

포함된 내용:

- `users` 라우트에 대한 API key 인증
- `orders` 라우트는 인증 없이 통과하는 다른 정책 조합
- `x-request-id`, `x-trace-id`, `x-organization` 공통 메타데이터 부여
- 인증 실패 시 gateway에서 표준 `401` JSON 응답 반환
- `policy.conf` 파일로 주요 정책값 외부화
- Wasm 내부 모듈 분리

## 실행

```bash
cd /home/bong/CLionProjects/niginx_rust_wasm/v2
docker compose up --build
```

gateway는 `http://localhost:8081`로 노출된다.

## 테스트

인증 실패:

```bash
curl -i http://localhost:8081/users/10
```

인증 성공:

```bash
curl -i -H 'x-api-key: users-secret' http://localhost:8081/users/10
```

인증 없이 허용되는 orders:

```bash
curl -i http://localhost:8081/orders/99
```

trace ID 전달 확인:

```bash
curl -s -H 'x-api-key: users-secret' -H 'x-trace-id: demo-trace-1' http://localhost:8081/users/10 | jq
```

## 기대 결과

- `/users/*` 는 `x-api-key`가 없거나 틀리면 upstream으로 가지 않고 `401`을 반환한다.
- `/users/*` 성공 요청은 `x-policy-profile: protected` 를 받는다.
- `/orders/*` 요청은 인증 없이 통과하며 `x-policy-profile: public` 을 받는다.
- upstream 응답 JSON에서 `x-request-id`, `x-trace-id`, `x-organization` 값을 확인할 수 있다.
