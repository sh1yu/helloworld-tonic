use hello_world::greeter_client::GreeterClient;
use hello_world::HelloRequest;
use tonic::transport::Channel;
use tower::discover::Change;

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 直接连接地址
    // let mut client = GreeterClient::connect("http://[::1]:50051").await?;

    // 使用balancer
    let (channel, rx) = Channel::balance_channel(10);
    let mut client = GreeterClient::new(channel);

    // 动态添加endpoint
    let endpoint1 = Channel::from_static("http://[::1]:50051");
    let endpoint2 = Channel::from_static("http://[::1]:50052");
    let res = rx.send(Change::Insert("1", endpoint1)).await;
    println!("ep1: {:?}", res);
    let res = rx.send(Change::Insert("2", endpoint2)).await;
    println!("ep2: {:?}", res);

    for i in 0..10 {
        let request = tonic::Request::new(HelloRequest {
            name: format!("Tonic {}", i + 1).into(),
        });
        let response = client.say_hello(request).await?;
        println!("RESPONSE={:?}", response);
    }

    // 动态删除endpoint
    let res = rx.send(Change::Remove("2")).await;
    println!("remove ep2: {:?}", res);

    for i in 0..10 {
        let request = tonic::Request::new(HelloRequest {
            name: format!("Tonic bye {}", i + 1).into(),
        });
        let response = client.say_hello(request).await?;
        println!("RESPONSE={:?}", response);
    }

    Ok(())
}
