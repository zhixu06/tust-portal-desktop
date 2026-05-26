use serde::{Deserialize, Serialize};

use crate::network_info::{get_local_ipv4, get_local_ipv6};

const PORTAL_URL: &str = "http://10.10.102.50:801/eportal/portal/login";
const USER_AGENT: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/143.0.0.0 Safari/537.36";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginResult {
    pub success: bool,
    pub message: String,
}

pub(crate) async fn do_login(
    username: &str,
    password: &str,
    network_type: &str,
) -> Result<(), String> {
    let login_username = if network_type == "中国联通" {
        format!("{}@unicom", username)
    } else {
        username.to_string()
    };

    let ipv4 = get_local_ipv4().ok_or("无法获取本机IPv4地址")?;
    tracing::info!(frontend = true, message = %format!("本机IPv4: {}", ipv4));

    let ipv6 = get_local_ipv6().unwrap_or_default();
    if !ipv6.is_empty() {
        tracing::info!(frontend = true, message = %format!("本机IPv6: {}", ipv6));
    }

    let encoded_username = urlencoding::encode(&login_username);
    let encoded_account = format!("%2C0%2C{}", encoded_username);
    let encoded_password = urlencoding::encode(password);
    let url = format!(
        "{}?callback=dr1005&login_method=1&user_account={}&user_password={}&wlan_user_ip={}&wlan_user_ipv6={}&wlan_user_mac=000000000000&jsVersion=4.1.3&terminal_type=3&lang=zh-cn&v=9303",
        PORTAL_URL, encoded_account, encoded_password, ipv4, ipv6
    );

    tracing::info!(frontend = true, message = "正在发送登录请求...");

    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .map_err(|e| format!("创建HTTP客户端失败: {}", e))?;

    let resp = client
        .get(&url)
        .header("User-Agent", USER_AGENT)
        .timeout(std::time::Duration::from_secs(10))
        .send()
        .await
        .map_err(|e| format!("登录请求失败: {}", e))?;

    let body = resp
        .text()
        .await
        .map_err(|e| format!("读取响应失败: {}", e))?;

    tracing::info!(frontend = true, message = %format!("服务器响应: {}", &body[..body.len().min(200)]));

    if !body.contains("\"status\":1") && !body.contains("已经在线") {
        return Err("登录失败: 响应中未找到成功标志".to_string());
    }

    tracing::info!(frontend = true, message = "登录请求成功");

    match client
        .get("https://www.baidu.com")
        .header("User-Agent", USER_AGENT)
        .timeout(std::time::Duration::from_secs(5))
        .send()
        .await
    {
        Ok(_) => {
            tracing::info!(frontend = true, message = "连通性测试通过");
        }
        Err(_) => {
            tracing::info!(frontend = true, message = "连通性测试失败, 但登录请求已发送");
        }
    }

    Ok(())
}

pub(crate) async fn try_login(
    username: String,
    password: String,
    network_type: String,
) -> Result<LoginResult, String> {
    for attempt in 1..=3 {
        if attempt > 1 {
            tracing::info!(frontend = true, message = %format!("--- 第{}次重试 ---", attempt));
        }
        match do_login(&username, &password, &network_type).await {
            Ok(()) => {
                return Ok(LoginResult {
                    success: true,
                    message: "登录成功".to_string(),
                });
            }
            Err(e) => {
                tracing::info!(frontend = true, message = %format!("错误: {}", e));
                if attempt < 3 {
                    tracing::info!(frontend = true, message = "等待2秒后重试...");
                    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
                }
            }
        }
    }

    Ok(LoginResult {
        success: false,
        message: "登录失败, 已重试3次".to_string(),
    })
}
