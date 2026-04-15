# v1 실행 계획

## 목표

`v1`의 목적은 `nginx + Rust WASM` 조합이 실제로 gateway로 동작하는지 검증하는 것이다.

이 단계에서는 "정책 플랫폼"보다 먼저 아래를 증명해야 한다.

- `nginx`가 요청을 받는다
- WASM 필터가 요청 처리 경로에서 실행된다
- 경로 기반 라우팅이 동작한다
- upstream까지 요청이 정상 전달된다

## 범위

포함:

- `nginx` 기반 ingress gateway
- WASM 실행이 가능한 nginx 런타임 또는 모듈 구성
- Rust 기반 단일 WASM 필터 1개
- `/users/*`, `/orders/*` 경로 기반 라우팅
- mock upstream 2개
- Docker Compose 기반 실행
- 최소 실행 문서

제외:

- 인증/인가
- 다중 필터 체인
- rate limiting
- observability stack
- control plane

## 실행 단위

### 1. 개발 실행 환경 뼈대 만들기

- `v1/docker-compose.yml` 추가
- gateway 컨테이너 정의
- mock upstream 2개 정의
- 전체가 한 번에 올라오는 기본 실행 경로 정리

완료 기준:

- `docker compose up --build`로 gateway와 upstream이 기동된다

### 2. gateway 설정 만들기

- `nginx.conf` 또는 이에 준하는 gateway 설정 추가
- `/users/*` 는 `users` upstream으로 라우팅
- `/orders/*` 는 `orders` upstream으로 라우팅
- 정의되지 않은 경로는 기본적으로 `404` 처리

완료 기준:

- 두 경로가 각기 다른 upstream으로 정상 프록시된다

### 3. Rust WASM 필터 프로젝트 만들기

- Rust 크레이트 생성
- `.wasm` 산출물이 나오도록 빌드 경로 구성
- proxy-wasm 스타일의 최소 필터 구현

완료 기준:

- WASM 모듈이 빌드되고 gateway에서 로드된다

### 4. 최소 정책 동작 구현하기

- 요청 path/header 읽기
- `x-gateway-wasm: active` 같은 식별 헤더 추가
- 필요하면 디버그용 route 메타데이터 추가

완료 기준:

- 외부 요청 결과만 보고도 WASM 필터가 실행되었음을 확인할 수 있다

### 5. mock upstream 만들기

- `users` 응답용 mock 서비스
- `orders` 응답용 mock 서비스
- 전달받은 요청 정보 일부를 응답에 노출하면 디버깅에 유리

완료 기준:

- 각 upstream이 자신이 받은 요청을 구분 가능한 형태로 응답한다

### 6. 검증 시나리오 문서화하기

- `curl` 예시 추가
- `/users/*` 요청 확인
- `/orders/*` 요청 확인
- WASM 헤더 확인 방법 정리

완료 기준:

- 처음 보는 사람도 문서만 보고 end-to-end 확인이 가능하다

## 최종 완료 기준

- `nginx + Rust WASM` 조합이 실제로 동작한다
- 경로 기반 라우팅이 확인된다
- WASM 필터가 실제 요청 경로에 개입함을 확인할 수 있다
- 다음 단계에서 정책 기능을 확장할 수 있는 최소 구조가 준비된다

## 현재 레포 기준 구현 구조

- `v1/docker-compose.yml`
  - `gateway`, `users`, `orders` 세 서비스를 함께 실행
- `v1/gateway/`
  - prebuilt `wasmx` nginx 런타임을 받아 gateway 이미지 구성
  - `nginx.conf`에서 `/users/*`, `/orders/*` 라우팅과 `proxy_wasm` 필터 적용
- `v1/wasm-filter/`
  - Rust 기반 단일 `proxy-wasm` 필터
  - 요청/응답에 `x-gateway-wasm: active` 헤더 추가
  - 요청 path를 보고 `x-gateway-route` 헤더 추가
- `v1/upstreams/`
  - 공통 mock upstream 이미지
  - `SERVICE_NAME` 환경변수로 `users`, `orders`를 구분

## 실행 방법

```bash
docker compose -f v1/docker-compose.yml up --build
```

## 확인 명령

```bash
docker compose -f v1/docker-compose.yml up -d
curl -i http://localhost:8080/users/ping
curl -i http://localhost:8080/orders/ping
curl -i http://localhost:8080/unknown
```

기대 결과:

- `/users/ping` 응답 본문에 `"service": "users"` 가 보인다
- `/orders/ping` 응답 본문에 `"service": "orders"` 가 보인다
- 응답 헤더에 `x-gateway-wasm: active` 가 보인다
- 응답 본문 헤더 정보에 `x-gateway-route` 가 포함된다
- `/unknown` 은 `404` 를 반환한다
