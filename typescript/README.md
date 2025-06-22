
# Test with

    grpcurl -plaintext -import-path ./protos/proto -proto ./protos/proto/echo/v1/echo.proto -d '{"message": "Boo!"}' '[::1]:8080' echo.v1.EchoService/Echo
