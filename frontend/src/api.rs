use common::{ErrorResponse, Feedback, FeedbackListResponse, FeedbackResponse};
use reqwasm::http;

pub async fn api_create_feedback(feedback_data: &str) -> Result<Feedback, String> {
    let api_url = std::env::var("API_URL").unwrap();
    let url = format!("{}/api/feedbacks/", api_url);

    let response = match http::Request::post(&url)
        .header("Content-Type", "application/json")
        .body(feedback_data)
        .send()
        .await
    {
        Ok(res) => res,
        Err(_) => return Err("Failed to make request".to_string()),
    };

    if response.status() != 200 {
        let error_response = response.json::<ErrorResponse>().await;
        if let Ok(error_response) = error_response {
            return Err(error_response.message);
        } else {
            return Err(format!("API error: {}", response.status()));
        }
    }

    let res_json = response.json::<FeedbackResponse>().await;
    match res_json {
        Ok(data) => Ok(data.data.feedback),
        Err(_) => Err("Failed to parse response".to_string()),
    }
}

pub async fn _api_fetch_single_feedback(feedback_id: &str) -> Result<Feedback, String> {
    let api_url = std::env::var("API_URL").unwrap();
    let url = format!("{}/api/feedbacks/{}", api_url, feedback_id);

    let response = match http::Request::get(&url)
    .send()
    .await
    {
        Ok(res) => res,
        Err(_) => return Err("Failed to make request".to_string()),
    };

    if response.status() != 200 {
        let error_response = response.json::<ErrorResponse>().await;
        if let Ok(error_response) = error_response {
            return Err(error_response.message);
        } else {
            return Err(format!("API error: {}", response.status()));
        }
    }

    let res_json = response.json::<FeedbackResponse>().await;
    match res_json {
        Ok(data) => Ok(data.data.feedback),
        Err(_) => Err("Failed to parse response".to_string()),
    }
}

pub async fn api_fetch_feedbacks((page, limit): (i32, i32)) -> Result<Vec<Feedback>, String> {
    let api_url = std::env::var("API_URL").unwrap();
    let url = format!("{}/api/feedbacks?page={}&limit={}", api_url, page, limit);
    let response = match http::Request::get(&url)
    .send()
    .await
    {
        Ok(res) => res,
        Err(_) => return Err("Failed to make request".to_string()),
    };

    if response.status() != 200 {
        let error_response = response.json::<ErrorResponse>().await;
        if let Ok(error_response) = error_response {
            return Err(error_response.message);
        } else {
            return Err(format!("API error: {}", response.status()));
        }
    }

    let res_json = response.json::<FeedbackListResponse>().await;
    match res_json {
        Ok(data) => Ok(data.feedbacks),
        Err(_) => Err("Failed to parse response".to_string()),
    }
}

pub async fn api_delete_feedback(feedback_id: &str) -> Result<(), String> {
    let api_url = std::env::var("API_URL").unwrap();
    let url = format!("{}/api/feedbacks/{}", api_url, feedback_id);
    let response = match http::Request::delete(&url)
    .send()
    .await
    {
        Ok(res) => res,
        Err(_) => return Err("Failed to make request".to_string()),
    };

    if response.status() != 204 {
        let error_response = response.json::<ErrorResponse>().await;
        if let Ok(error_response) = error_response {
            return Err(error_response.message);
        } else {
            return Err(format!("API error: {}", response.status()));
        }
    }

    Ok(())
}