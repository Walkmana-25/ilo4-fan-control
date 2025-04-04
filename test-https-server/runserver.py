import ssl
from http.server import HTTPServer, SimpleHTTPRequestHandler

PORT = 8080
CERTFILE = "./localhost.pem"

Handler = SimpleHTTPRequestHandler

context = ssl.SSLContext(ssl.PROTOCOL_TLS_SERVER)
context.load_cert_chain(CERTFILE)

with HTTPServer(("127.0.0.1", PORT), Handler) as httpd:
    print("serving at address", httpd.server_address, "using cert file", CERTFILE)
    httpd.socket = context.wrap_socket(httpd.socket, server_side=True)
    httpd.serve_forever()