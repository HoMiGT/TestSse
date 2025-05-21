# EventSourceResponse
> 是与服务器发送事件（Server-Sent Events, SSE）相关的一个概念    
> 主要出现在构建支持 实时推送（单向流） 的 Web 应用或 API 时
---
- SSE 是一种轻量的服务器推送机制
- 使用 HTTP 长连接
- 浏览器或客户端发起连接
- 服务端持续向客户端推送 text/event-stream 格式的数据
- 只支持服务端到客户端的单向通信
---
与 WebSocket 相比更轻量、无需复杂协议协商。

适用场景：
实时推送、日志流式传输、聊天室消息推送
