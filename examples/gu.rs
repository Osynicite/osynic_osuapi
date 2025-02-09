use osynic_osuapi::error::Result;
use osynic_osuapi::v2::client::request::client::OsynicOsuApiV2Client;
use osynic_osuapi::v2::model::oauth::structs::o_token::OToken;
use osynic_osuapi::v2::interface::users::IUsers;


#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    let client = OsynicOsuApiV2Client::new(OToken{
        access_token: "eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiJ9.eyJhdWQiOiIzNjEwNCIsImp0aSI6IjljNTdiMjA5YmIzYTY3NjY1M2IzZWE2Mzc5ODg3M2NlMmM0YjgxOWM1MzhmZGMzMGFlNzc1ZDFlMzRkMTRlZDM4OWQzOTdjYTRhNTE3ODc4IiwiaWF0IjoxNzM3NjA3MDQ5LjM2MzIxOCwibmJmIjoxNzM3NjA3MDQ5LjM2MzIyMSwiZXhwIjoxNzM3NjkzNDQ5LjM0Mjg2MSwic3ViIjoiMzExNzU4NDIiLCJzY29wZXMiOlsiaWRlbnRpZnkiLCJwdWJsaWMiXX0.g_UudFcYZ7nDO2V0I1gnj_oGjaSJVoPsT9Oxx-_gPsYBrNeXLCsHIaDfMR-7n6DonIvrzRY1a3kxx2MJ1mhwIGLePW1WIWdAJ4wWurpngtw-Pnvn4cnL8ZC8qHE6oduMaqiTyd8XjGMUrHuTKU1JHN0h-4Uj_kao3rF6YMZSv2O_PG2hb0wkwGUOdZza9IBHXu5MymfYGF1WOn_dVqa6TJoyC9bbRh8ueeZ5gp-nMCN3e2Bqrr4dHftns8_rpj56Li_vJ0WRY8AdZV-_DlxBOCx1bR2eeAIm0ThrqkRBAKg6Eh1wdP1ox0ikXZ8yz51n-45BBh8Wk4DXld-9yhug3oCJDWPd0deiINWo-a5s7toSGhNaoQkATJ6V44H4mHH3qSksOdux1W49CGDixnJUd_9H9M_0dHp-1zuVmmcArDudJAC0NPC2U3iNScfRks0g3ix9imT_YCf98VTiZBokRexeFJH4lFoayrxzf0E5ROPQ12Sp7KuK0LyPB65zWknHqSI8vGrh31d9xEByBWg7OQy9qrzoK437CZ3tWeosqThD4W5vXhHwHs8qrjxNUWTd36ahIC43_x3At7b-ykzeMlPr4hrvVJyBI4LowuDBR2HT7lssgakki_O6SSm9JNFJh_4h1c-q8vKEfPdnxVWutFjb0Fvs3IjsJQSMN7eF8OA".to_string(),
        refresh_token: Some("def50200867ec4c08a0c0e23a06eccc7ee18f99b20094f958922a74a25147176034653212728b9af2584e9911177132cbb366dc10497ac06761601e8a99ead41b5d84b3233ba3d27754aeae8c065e2a1c0d6b551ac345d788abe503f1934751017edee8e1de42e7ecc1cfe3790b5005c4d36653deae03bcb325400d71a6660f7576bd4bdd684b82fbf58ec57fd56266b1931da5abc44e290619d5cd49040b7d9a71632bbe4f9f5064e3d12b4864084dcf093c35ebf14b1bfcd2b886395b1c1ab9c1743709c9c4a78fcda452fa8e36ed429555557a114d6be48765b9da49f104a13071c746a3bfa7d61505d7a25fe2528440639b23a0c6db5d207aafb4af316252ebf41196b6d4edd539cbc62a49de87c7f52b32fdf4240126ef6f7a89773eef3c814433f5de298f58f9c5be8cab3f7990f29f765b5d159b5ca00c708eef38501017a5490ee1be4054ffe33508558478abd9399e03c76f66ed82729bd5738d10fc9dd849d17f05786d0030e3ac28631f100b8daa8c6a005ffef3f17f951fc84".to_string()),
        expires_in: 86400,
        token_type: "Bearer".to_string(),
    });
    let me = client.users.get_user("SisypheOvO",None).await?;
    println!("{:?}", me);
    println!("osu_account_id: {}", me.id);
    println!("username: {}", me.username);
    println!("join_date: {}", me.join_date);
    println!("country_code: {}", me.country.code);
    println!("country_name: {}", me.country.name);
    println!("cover_url: {}", me.cover_url);
    Ok(())
}