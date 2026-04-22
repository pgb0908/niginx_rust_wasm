import json
import os
from http.server import BaseHTTPRequestHandler, HTTPServer


SERVICE_NAME = os.environ.get("SERVICE_NAME", "unknown")


class Handler(BaseHTTPRequestHandler):
    def do_GET(self):
        payload = {
            "service": SERVICE_NAME,
            "method": self.command,
            "path": self.path,
            "headers": {
                "x-api-key": self.headers.get("x-api-key"),
                "x-tenant-id": self.headers.get("x-tenant-id"),
                "x-request-id": self.headers.get("x-request-id"),
                "x-trace-id": self.headers.get("x-trace-id"),
                "x-gateway-route": self.headers.get("x-gateway-route"),
                "x-gateway-policy-profile": self.headers.get("x-gateway-policy-profile"),
                "x-gateway-plugin-chain": self.headers.get("x-gateway-plugin-chain"),
                "x-gateway-filter-version": self.headers.get("x-gateway-filter-version"),
                "x-gateway-decision": self.headers.get("x-gateway-decision"),
                "x-observe-span": self.headers.get("x-observe-span"),
                "x-forwarded-for": self.headers.get("x-forwarded-for"),
                "host": self.headers.get("host"),
            },
        }

        body = json.dumps(payload, indent=2).encode("utf-8")

        self.send_response(200)
        self.send_header("Content-Type", "application/json")
        self.send_header("Content-Length", str(len(body)))
        self.end_headers()
        self.wfile.write(body)

    def log_message(self, format, *args):
        return


if __name__ == "__main__":
    server = HTTPServer(("0.0.0.0", 8080), Handler)
    server.serve_forever()
