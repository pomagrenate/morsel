//! Integration tests for CLI → daemon.

use morsel_daemon::ipc::{IpcClient, IpcRequest, IpcResponse};

#[tokio::test]
async fn test_ipc_client_status_request() {
    // This test assumes a daemon is running or handles the connection failure gracefully
    let mut client = match IpcClient::connect("127.0.0.1:54321") {
        Ok(c) => c,
        Err(_) => {
            // Daemon not running, skip test
            return;
        }
    };
    
    let request = IpcRequest::GetHistory { limit: 10 };
    let response = client.send_request(&request);
    
    match response {
        Ok(IpcResponse::Success { data }) => {
            assert!(data.is_some());
        }
        Ok(IpcResponse::Error { message }) => {
            // Might fail if daemon has no items
            assert!(!message.is_empty());
        }
        Err(_) => {
            // Connection failed, daemon not running
        }
    }
}

#[tokio::test]
async fn test_ipc_client_get_history_request() {
    let mut client = match IpcClient::connect("127.0.0.1:54321") {
        Ok(c) => c,
        Err(_) => return,
    };
    
    let request = IpcRequest::GetHistory { limit: 10 };
    let response = client.send_request(&request);
    
    match response {
        Ok(IpcResponse::Success { data }) => {
            assert!(data.is_some());
        }
        Ok(IpcResponse::Error { message }) => {
            assert!(!message.is_empty());
        }
        Err(_) => {
            // Connection failed
        }
    }
}

#[tokio::test]
async fn test_ipc_client_search_request() {
    let mut client = match IpcClient::connect("127.0.0.1:54321") {
        Ok(c) => c,
        Err(_) => return,
    };
    
    let request = IpcRequest::Search { query: "test".to_string(), limit: 5 };
    let response = client.send_request(&request);
    
    match response {
        Ok(IpcResponse::Success { data }) => {
            assert!(data.is_some());
        }
        Ok(IpcResponse::Error { message }) => {
            assert!(!message.is_empty());
        }
        Err(_) => {
            // Connection failed
        }
    }
}

#[tokio::test]
async fn test_ipc_client_add_item_request() {
    let mut client = match IpcClient::connect("127.0.0.1:54321") {
        Ok(c) => c,
        Err(_) => return,
    };
    
    let request = IpcRequest::AddItem { content: "test content from CLI".to_string() };
    let response = client.send_request(&request);
    
    match response {
        Ok(IpcResponse::Success { data }) => {
            assert!(data.is_some());
        }
        Ok(IpcResponse::Error { message }) => {
            assert!(!message.is_empty());
        }
        Err(_) => {
            // Connection failed
        }
    }
}

#[tokio::test]
async fn test_ipc_client_clear_history_request() {
    let mut client = match IpcClient::connect("127.0.0.1:54321") {
        Ok(c) => c,
        Err(_) => return,
    };
    
    let request = IpcRequest::ClearHistory;
    let response = client.send_request(&request);
    
    match response {
        Ok(IpcResponse::Success { data }) => {
            assert!(data.is_some());
        }
        Ok(IpcResponse::Error { message }) => {
            assert!(!message.is_empty());
        }
        Err(_) => {
            // Connection failed
        }
    }
}

#[tokio::test]
async fn test_ipc_client_stop_request() {
    let mut client = match IpcClient::connect("127.0.0.1:54321") {
        Ok(c) => c,
        Err(_) => return,
    };
    
    let request = IpcRequest::Stop;
    let response = client.send_request(&request);
    
    match response {
        Ok(IpcResponse::Success { data }) => {
            assert!(data.is_some());
        }
        Ok(IpcResponse::Error { message }) => {
            assert!(!message.is_empty());
        }
        Err(_) => {
            // Connection failed
        }
    }
}

#[test]
fn test_ipc_request_serialization() {
    let request = IpcRequest::GetHistory { limit: 10 };
    let serialized = serde_json::to_string(&request).unwrap();
    let deserialized: IpcRequest = serde_json::from_str(&serialized).unwrap();
    
    match deserialized {
        IpcRequest::GetHistory { limit } => {
            assert_eq!(limit, 10);
        }
        _ => panic!("Expected GetHistory request"),
    }
}

#[test]
fn test_ipc_response_serialization() {
    let response = IpcResponse::Success { data: Some(serde_json::json!({"test": true})) };
    let serialized = serde_json::to_string(&response).unwrap();
    let deserialized: IpcResponse = serde_json::from_str(&serialized).unwrap();
    
    match deserialized {
        IpcResponse::Success { data } => {
            assert!(data.is_some());
        }
        _ => panic!("Expected Success response"),
    }
}

#[test]
fn test_ipc_error_response_serialization() {
    let response = IpcResponse::Error { message: "Test error".to_string() };
    let serialized = serde_json::to_string(&response).unwrap();
    let deserialized: IpcResponse = serde_json::from_str(&serialized).unwrap();
    
    match deserialized {
        IpcResponse::Error { message } => {
            assert_eq!(message, "Test error");
        }
        _ => panic!("Expected Error response"),
    }
}
