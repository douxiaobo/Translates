use std::io::Write;
use rand::Rng;
use crypto::md5::Md5;
use crypto::digest::Digest;
use serde::{
    Serialize,Deserialize
};

const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
abcdefghijklmnopqrstuvwxyz\
0123456789)(*^%$@!";

const LEN: i32=32;

#[derive(Serialize, Deserialize)]
pub struct Baidufanyi {
    #[serde(rename="from")]
    from: String,
    #[serde(rename="to")]
    to: String,
    #[serde(rename="trans_result")]
    trans_result: Vec<TransResult>,
}

#[derive(Serialize, Deserialize)]
pub struct TransResult {
    #[serde(rename="src")]
    src: String,
    #[serde(rename="dst")]
    dst: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let mut q,from,to,appid,salt,sign;

    print!("请输入原文：");
    let mut q=String::new();
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut q).expect("fail to read line.");
    // println!("q={}",q);

    // println!();

    let from:String="en".to_owned();
    
    let to:String="zh".to_owned();
    
    let appid:String="..........".to_owned();

    let mut rng=rand::thread_rng();
    let salt:String=(0..LEN).map(|_|{
        let idx=rng.gen_range(0..CHARSET.len());
        CHARSET[idx] as char
    })
    .collect();

    // println!("Sait:{}",salt);

    // println!();

    let key:String="............".to_owned();

    let mut sign=Md5::new();
    let text=format!("{}{}{}{}",appid,q.trim_end(),salt,key);
    // let text=(appid+&q+&salt+&key).to_string();
    sign.input_str(&text);
    // println!("{} => {}",text,sign.result_str());

    // println!();

    let url=format!("https://fanyi-api.baidu.com/api/trans/vip/translate?q={}&from={}&to={}&appid={}&salt={}&sign={}",q.trim_end(),from,to,appid,salt,sign.result_str().to_string());

    // println!("url:{}",url);

    // println!();

    let client = reqwest::Client::builder().build()?;

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("Cookie", "BAIDUID=8DC9F1CF6226539B8F14D1B7ADDBC417:FG=1".parse()?);

    let request=client.request(reqwest::Method::GET, url).headers(headers);

    let response=request.send().await?;
    let body=response.text().await?;

    // println!("{}",body);

    let model:Baidufanyi=serde_json::from_str(&body).unwrap();

    if let Some(first_result)=model.trans_result.first(){
        println!("目标翻译是：{}",first_result.dst);
    } else {
        println!("没有翻译结果。");
    }

    Ok(())


}
