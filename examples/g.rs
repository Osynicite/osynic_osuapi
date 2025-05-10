use wasm_bindgen_futures::spawn_local;
use osynic_osuapi::v1::client::gloo::client::OsynicOsuApiV1GlooClient;
use osynic_osuapi::v1::model::beatmap::GetBeatmapsParams;
use osynic_osuapi::v1::interface::beatmap::IBeatmap;

// 在WASM环境中使用
fn main() {
    dotenvy::dotenv().ok();
    let key = std::env::var("API_KEY").expect("Please set the API_KEY environment variable to a valid osu! API v1 API key");
    let client = OsynicOsuApiV1GlooClient::new(key);
    
    // 创建查询参数
    let params = GetBeatmapsParams::default()
        .sid("114514".to_string());
    
    // 在WASM中使用spawn_local来处理异步任务
    spawn_local(async move {
        match client.beatmap.get_beatmaps(params).await {
            Ok(beatmaps) => {
                // 处理返回的beatmaps数据
                for beatmap in beatmaps {
                    web_sys::console::log_1(&format!("谱面: {}", beatmap.title).into());
                }
            },
            Err(e) => {
                web_sys::console::error_1(&format!("错误: {:?}", e).into());
            }
        }
    });
}