#![allow(unused)]

fn main() {
    // 使用占位符表示文本
    // let text = "这是一个很长的文本，包含了很多信息，我们暂时不需要展示它的全部内容，所以我们用占位符来表示它。";

    // 读取本地的response.txt文件
    let text = std::fs::read_to_string("response.txt").expect("无法读取文件！");

    let position = 177047;

    let range = 1000;
    // 检查文本长度是否足够
    if text.len() < position {
        println!("文本长度不足 {} 个字符！", position);
        return;
    }

    // 获取第 5810 个字符（索引从 0 开始）
    let target_char = text.chars().nth(position - 1).unwrap();

    // 获取上下各 100 个字符的范围
    let start = if position - 1 >= range {
        position - 1 - range
    } else {
        0
    };
    let end = if position - 1 + range < text.len() {
        position - 1 + range
    } else {
        text.len() - 1
    };

    // 提取上下文
    let context = &text[start..end];

    // 找到目标字符在上下文中的位置
    let target_index_in_context = position - 1 - start;

    // 打印上下文，并在目标字符处换行
    println!(
        "{}[{}]\n{}",
        &context[..target_index_in_context],
        target_char,
        &context[target_index_in_context + 1..]
    );
}

/*alse,"is_supporter":true,"last_visit":"2026-04-08T10:02:12+00:00","pm_friends_only":false,"profile_colour":null,"username":"TheShadowOfDark","country":{"code":"CL","name":"Chile"},"cover":{"custom_url":"https:\/\/assets.ppy.sh\/user-profile-covers\/5795337\/07be4e881e4152e8127f183c0d76baf416a3e4105f5fc9c01b571e39ddce3748.jpeg","url":"https:\/\/assets.ppy.sh\/user-profile-covers\/5795337\/07be4e881e4152e8127f183c0d76baf416a3e4105f5fc9c01b571e39ddce3748.jpeg","id":null},"groups":[],"statistics":{"count_100":4015366,"count_300":63481251,"count_50":730973,"count_miss":4023488,"level":{"current":107,"progress":20},"global_rank":27968,"global_rank_percent":0.00876217377601192,"global_rank_exp":null,"pp":7449,"pp_exp":0,"ranked_score":139121240908,"hit_accuracy":98.4443,"accuracy":0.984443,"play_count":454941,"play_time":26903576,"total_score":747826161171,"total_hits":68227590,"maximum_combo":31406,"replays_watched_by_others":506129,"is_ranked":true,"grade_counts":{"ss":1,"ssh":38343,"s":-1,["]
sh":2818,"a":3735}},"support_level":2,"team":{"flag_url":"https:\/\/assets.ppy.sh\/teams\/flag\/482\/2420acc2280fac0ac17afc53a1396138103f6990003fdf5a486688a8c15c4aff.gif","id":482,"name":"The Guys","short_name":"477"}}},{"target_id":5992551,"relation_type":"friend","mutual":true,"target":{"avatar_url":"https:\/\/a.ppy.sh\/5992551?1773540073.jpeg","country_code":"CL","default_group":"default","id":5992551,"is_active":true,"is_bot":false,"is_deleted":false,"is_online":false,"is_supporter":true,"last_visit":null,"pm_friends_only":false,"profile_colour":null,"username":"[Bau]","country":{"code":"CL","name":"Chile"},"cover":{"custom_url":"https:\/\/assets.ppy.sh\/user-profile-covers\/5992551\/6cb08d97877b8b5708873ef2b77861e7828fc737a99cf478473ed36db8ab0653.jpeg","url":"https:\/\/assets.ppy.sh\/user-profile-covers\/5992551\/6cb08d97877b8b5708873ef2b77861e7828fc737a99cf478473ed36db8ab0653.jpeg","id":null},"groups":[],"statistics":{"count_100":1439670,"count_300":24101491,"count_50":196107,"c*/
