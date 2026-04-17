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
                "x-gateway-wasm": self.headers.get("x-gateway-wasm"),
                "x-gateway-route": self.headers.get("x-gateway-route"),
                "x-policy-profile": self.headers.get("x-policy-profile"),
                "x-request-id": self.headers.get("x-request-id"),
                "x-trace-id": self.headers.get("x-trace-id"),
                "x-organization": self.headers.get("x-organization"),
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
