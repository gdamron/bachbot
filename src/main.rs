use log::{error, info};
use rumqttc::{AsyncClient, Event, Incoming, MqttOptions, Publish, QoS, Transport};
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::time::Duration;
use tokio::select;

#[derive(Debug, Deserialize)]
#[serde(tag = "command", rename_all = "snake_case")]
enum ControlCommand {
    StartStreaming { params: Option<serde_json::Value> },
    StopStreaming,
    Ping,
}

#[derive(Debug, Serialize)]
struct StatusMessage {
    status: String,
    details: Option<String>,
}

#[tokio::main]
async fn main() {
    env_logger::init();
    let device_id = env::var("DEVICE_ID").unwrap_or_else(|_| "demo_device".to_string());
    let mqtt_host = env::var("MQTT_HOST").unwrap_or_else(|_| "localhost".to_string());
    let mqtt_port: u16 = env::var("MQTT_PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(1883);

    info!("Starting device with ID: {}", device_id);
    info!("Connecting to MQTT broker at {}:{}", mqtt_host, mqtt_port);

    let ca_path = env::var("AWS_CA_PATH").unwrap_or_else(|_| "./AmazonRootCA1.pem".into());
    let cert_path = env::var("AWS_CERT_PATH").unwrap_or_else(|_| "./device.pem.crt".into());
    let key_path = env::var("AWS_KEY_PATH").unwrap_or_else(|_| "./private.pem.key".into());
    // Read certificate/key files to Vec<u8>
    let ca = fs::read(&ca_path).expect("Could not read CA file");
    let client_cert = fs::read(&cert_path).expect("Could not read client cert");
    let client_key = fs::read(&key_path).expect("Could not read client key");

    let mut mqttoptions = MqttOptions::new(device_id.clone(), mqtt_host, mqtt_port);
    mqttoptions.set_keep_alive(Duration::from_secs(10));
    mqttoptions.set_transport(Transport::tls(ca, Some((client_cert, client_key)), None));

    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);
    let command_topic = format!("devices/{}/commands", device_id);
    let status_topic = format!("devices/{}/status", device_id);

    client
        .subscribe(&command_topic, QoS::AtMostOnce)
        .await
        .unwrap();
    info!("Subscribed to command topic: {}", command_topic);

    // Send initial status message
    let hello = StatusMessage {
        status: "online".to_string(),
        details: device_id.clone().into(),
    };
    let payload = serde_json::to_vec(&hello).unwrap();
    client
        .publish(&status_topic, QoS::AtLeastOnce, false, payload)
        .await
        .unwrap();

    loop {
        select! {
            event = eventloop.poll() => {
                match event {
                    Ok(Event::Incoming(Incoming::Publish(Publish { topic, payload, .. }))) => {
                        info!("Received message on topic {}: {:?}", topic, payload);

                        if let Ok(cmd) = serde_json::from_slice::<ControlCommand>(&payload) {
                            handle_command(cmd, &client, &status_topic, &device_id).await;
                        } else {
                            error!("Failed to parse command payload: {:?}", payload);
                        }
                    }
                    Ok(_) => {}
                    Err(e) => {
                        error!("MQTT error: {:?}", e);
                    }
                }
            }
        }
    }
}

async fn handle_command(
    cmd: ControlCommand,
    client: &AsyncClient,
    status_topic: &str,
    details: &str,
) {
    match cmd {
        ControlCommand::StartStreaming { params } => {
            info!("Start streaming: {params:?}");
            // TODO: Implement streaming logic here
            send_status(client, status_topic, "streaming_started", Some(details)).await;
        }
        ControlCommand::StopStreaming => {
            info!("Stop streaming");
            // TODO: Implement stop streaming logic here
            send_status(client, status_topic, "streaming_stopped", Some(details)).await;
        }
        ControlCommand::Ping => {
            info!("Received ping");
            send_status(client, status_topic, "pong", Some(details)).await;
        }
    }
}

async fn send_status(client: &AsyncClient, topic: &str, status: &str, details: Option<&str>) {
    let msg = StatusMessage {
        status: status.to_string(),
        details: details.map(|d| d.to_string()),
    };

    let payload = serde_json::to_vec(&msg).unwrap();
    if let Err(e) = client
        .publish(topic, QoS::AtLeastOnce, false, payload)
        .await
    {
        error!("Failed to publish status: {e:?}");
    }
}
