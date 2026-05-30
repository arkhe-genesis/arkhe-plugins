import asyncio
import json
from http.server import BaseHTTPRequestHandler, HTTPServer
from urllib.parse import urlparse, parse_qs
from typing import Optional
from passport_gateway import PassportGateway

class APIGateway:
    def __init__(self, node_id: str, passport: Optional[PassportGateway] = None):
        self.node_id = node_id
        self.queries_processed = 0
        self.passport = passport
        self.server = None

    async def handle_ws_message(self, message):
        pass

    async def start_http_server(self):
        self.server = HTTPServer(("0.0.0.0", 8080), lambda *args: self.RequestHandler(*args, gateway=self))
        # Run server in a thread or asyncio executor in a real app, here we just show the structure

    def stop(self):
        if self.server:
            self.server.server_close()

    class RequestHandler(BaseHTTPRequestHandler):
        def __init__(self, *args, gateway=None, **kwargs):
            self.gateway = gateway
            super().__init__(*args, **kwargs)

        def do_GET(self):
            path = urlparse(self.path).path
            if path == "/v1/status":
                self.send_json(200, {"status": "ok", "node_id": self.gateway.node_id})
            elif path == "/v1/oracle/feeds":
                self.send_json(200, {"feeds": []})
            elif path.startswith("/v1/identity/passport"):
                params = parse_qs(urlparse(self.path).query)
                address = params.get("address", [None])[0]
                if not address:
                    self.send_error(400, "Missing address")
                    return
                proof = asyncio.run(self.gateway.passport.is_human(address))
                self.send_json(200, {
                    "address": proof.address,
                    "is_human": proof.is_human,
                    "score": proof.score,
                    "stamps": proof.stamps,
                    "orcid_verified": proof.orcid_verified,
                })
            elif path.startswith("/v1/dao/verify-voter"):
                params = parse_qs(urlparse(self.path).query)
                address = params.get("address", [None])[0]
                if not address:
                    self.send_error(400, "Missing address")
                    return
                can_vote = asyncio.run(self.gateway.passport.verify_dao_voter(address))
                self.send_json(200, {"address": address, "can_vote": can_vote})
            else:
                self.send_error(404, "Not found")

        def send_json(self, status: int, data: dict):
            self.send_response(status)
            self.send_header("Content-Type", "application/json")
            self.end_headers()
            self.wfile.write(json.dumps(data).encode())
