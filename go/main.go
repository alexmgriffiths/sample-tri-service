package main

import (
	"context"
	"log"
	"net"
	"strconv"
	"time"

	timev1 "github.com/alexmgriffiths/sample-service-protos/gen/go/time/v1"
	"google.golang.org/grpc"
)

type TimeServer struct {
	timev1.UnimplementedTimeServiceServer
}

func (t *TimeServer) GetServerTime(ctx context.Context, req *timev1.TimeRequest) (*timev1.TimeResponse, error) {
	unixTimesatmp := time.Now().Unix()
	timestampString := strconv.FormatInt(unixTimesatmp, 10)
	return &timev1.TimeResponse{IsoUtc: timestampString}, nil
}

func main() {
	lis, _ := net.Listen("tcp", ":50053")
	s := grpc.NewServer()
	timev1.RegisterTimeServiceServer(s, &TimeServer{})
	log.Fatal(s.Serve(lis))
}
