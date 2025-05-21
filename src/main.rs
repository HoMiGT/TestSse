use std::convert::Infallible;
use tokio::{fs, sync::mpsc::unbounded_channel};
use axum::{
    response::{IntoResponse,Response,Html}, routing::get,Router,
    body::Body,
};
use futures::StreamExt;
use serde::{Serialize,Deserialize};
use tokio_stream::wrappers::{IntervalStream, UnboundedReceiverStream};
use tokio::time::interval;
use tokio::time::Duration;
use async_stream::stream;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    
    let app = Router::new()
        .route("/index",get(index))
        .route("/events",get(sse_stream))
        .route("/tick_events",get(sse_handler));
    
    println!("Server running at http://localhost:3000");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    println!("Hello, world!");
}

#[derive(Serialize,Deserialize)]
struct NavResponse{
    code: isize,
    message: Option<String>,
    data: Option<String>
}

async fn index()->Html<String>{
    match fs::read_to_string("templates/index.html").await{
        Ok(contents) => Html(contents),
        Err(_) => Html("<h1>404 Not Found</h1>".to_string())
    }
}

async fn sse_stream()->impl IntoResponse{
    let message  = vec!["你好","这是","一条","流式","消息"];
    
    let (tx,rx) = unbounded_channel::<String>();
    
    tokio::spawn(async move {
        for msg in message{
            let _  = tx.send(msg.to_string());
            tokio::time::sleep(Duration::from_secs(2)).await;
        }
        drop(tx);
    });
    
    let stream = stream!{
        let mut rx = UnboundedReceiverStream::new(rx);
        while let Some(msg) = rx.next().await{
            let formatted = format!("data: {}\n\n",msg);
            yield Ok::<_,Infallible>(formatted);
        }
        yield Ok::<_, Infallible>("event: close\ndata: bye\n\n".to_string());
    };
        
    let body = Body::from_stream(stream);
    Response::builder()
        .header("Content-Type", "text/event-stream")
        .header("Cache-Control", "no-cache")
        .header("Connection", "keep-alive")
        .body(body)
        .unwrap()
        
}

async fn sse_handler()->impl IntoResponse{
    let interval = IntervalStream::new(interval(Duration::from_secs(1)));
    let stream = interval.enumerate().map(|(i,_)|{
        let payload = format!("data:tick {} \n\n",i);
        Ok::<_,Infallible>(payload)
    });
    
    let body = Body::from_stream(stream);
    Response::builder()
        .header("Content-Type", "text/event-stream")
        .header("Cache-Control", "no-cache")
        .header("Connection", "keep-alive")
        .body(body)
        .unwrap()
    
}