import * as http2 from "http2"
import routes from './connect'
import { connectNodeAdapter } from '@connectrpc/connect-node'

http2.createServer(
    connectNodeAdapter({ routes })
).listen(8080);