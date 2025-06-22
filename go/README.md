# Run

   go run ./main.go

# Test with

    grpcurl -plaintext -import-path ./protos/proto -proto ./protos/proto/time/v1/time.proto '[::1]:50053' time.v1.TimeService/GetServerTime