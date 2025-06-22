import { ConnectRouter } from '@connectrpc/connect'
import { EchoService, EchoRequest } from '../../protos/gen/ts/echo/v1/echo_pb'

export default function (router: ConnectRouter) {
    router.service(EchoService, {
        async echo(req: EchoRequest) {
            return {
                message: 'You said: ' + req.message
            }
        }
    })
}