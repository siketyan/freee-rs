/*
 * freee API
 *
 *  <h1 id=\"freee_api\">freee API</h1> <hr /> <h2 id=\"start_guide\">スタートガイド</h2>  <p>freee API開発がはじめての方は<a href=\"https://developer.freee.co.jp/getting-started\">freee API スタートガイド</a>を参照してください。</p>  <hr /> <h2 id=\"specification\">仕様</h2>  <h3 id=\"api_endpoint\">APIエンドポイント</h3>  <p>https://api.freee.co.jp/ (httpsのみ)</p>  <h3 id=\"about_authorize\">認証について</h3> <p>OAuth2.0を利用します。<a href=\"https://developer.freee.co.jp/reference/#%e8%aa%8d%e8%a8%bc\" target=\"_blank\">詳細はリファレンスの認証に関する記載を参照してください。</a></p>  <h3 id=\"data_format\">データフォーマット</h3>  <p>リクエスト、レスポンスともにJSON形式をサポートしていますが、詳細は、API毎の説明欄（application/jsonなど）を確認してください。</p>  <h3 id=\"compatibility\">後方互換性ありの変更</h3>  <p>freeeでは、APIを改善していくために以下のような変更は後方互換性ありとして通知なく変更を入れることがあります。アプリケーション実装者は以下を踏まえて開発を行ってください。</p>  <ul> <li>新しいAPIリソース・エンドポイントの追加</li> <li>既存のAPIに対して必須ではない新しいリクエストパラメータの追加</li> <li>既存のAPIレスポンスに対する新しいプロパティの追加</li> <li>既存のAPIレスポンスに対するプロパティの順番の入れ変え</li> <li>keyとなっているidやcodeの長さの変更（長くする）</li> </ul>  <h3 id=\"common_response_header\">共通レスポンスヘッダー</h3>  <p>すべてのAPIのレスポンスには以下のHTTPヘッダーが含まれます。</p>  <ul> <li> <p>X-Freee-Request-ID</p> <ul> <li>各リクエスト毎に発行されるID</li> </ul> </li> </ul>  <h3 id=\"common_error_response\">共通エラーレスポンス</h3>  <ul> <li> <p>ステータスコードはレスポンス内のJSONに含まれる他、HTTPヘッダにも含まれる</p> </li> <li> <p>一部のエラーレスポンスにはエラーコードが含まれます。<br>詳細は、<a href=\"https://developer.freee.co.jp/tips/faq/40x-checkpoint\">HTTPステータスコード400台エラー時のチェックポイント</a>を参照してください</p> </li> <p>type</p>  <ul> <li>status : HTTPステータスコードの説明</li>  <li>validation : エラーの詳細の説明（開発者向け）</li> </ul> </li> </ul>  <p>レスポンスの例</p>  <pre><code>  {     &quot;status_code&quot; : 400,     &quot;errors&quot; : [       {         &quot;type&quot; : &quot;status&quot;,         &quot;messages&quot; : [&quot;不正なリクエストです。&quot;]       },       {         &quot;type&quot; : &quot;validation&quot;,         &quot;messages&quot; : [&quot;Date は不正な日付フォーマットです。入力例：2019-12-17&quot;]       }     ]   }</code></pre>  </br>  <h3 id=\"api_rate_limit\">API使用制限</h3>    <p>freeeは一定期間に過度のアクセスを検知した場合、APIアクセスをコントロールする場合があります。</p>   <p>その際のhttp status codeは403となります。制限がかかってから10分程度が過ぎると再度使用することができるようになります。</p>  <h4 id=\"reports_api_endpoint\">/reportsと/receipts/{id}/downloadエンドポイント</h4>  <p>freeeはエンドポイント毎に一定頻度以上のアクセスを検知した場合、APIアクセスをコントロールする場合があります。その際のhttp status codeは429（too many requests）となります。</p> <ul>   <li>/reports:1秒に10回まで</li>   <li>/receipts/{id}/download:1秒に3回まで</li> </ul>  <p>http status codeが429となった場合、API使用ステータスはレスポンスヘッダに付与されます。</p> <pre><code>x-ratelimit-limit:10 x-ratelimit-remaining:1 x-ratelimit-reset:2023-01-13T10:22:29+09:00 </code></pre>  <br> 各ヘッダの意味は次のとおりです。</p>  <table border=\"1\">   <tbody>     <tr>       <th style=\"padding: 10px\"><strong>ヘッダ名</strong></th>       <th style=\"padding: 10px\"><strong>説明</strong></th>     </tr>     <tr><td style=\"padding: 10px\">x-ratelimit-limit</td><td style=\"padding: 10px\">使用回数の上限</td></tr>     <tr><td style=\"padding: 10px\">x-ratelimit-remaining</td><td style=\"padding: 10px\">残り使用回数</td></tr>     <tr><td style=\"padding: 10px\">x-ratelimit-reset</td><td style=\"padding: 10px\">使用回数がリセットされる時刻</td></tr>   </tbody> </table>  </br>  <h3 id=\"plan_api_rate_limit\">プラン別のAPI Rate Limit</h3>   <table border=\"1\">     <tbody>       <tr>         <th style=\"padding: 10px\"><strong>freee会計プラン名</strong></th>         <th style=\"padding: 10px\"><strong>事業所とアプリケーション毎に1日でのAPIコール数</strong></th>       </tr>       <tr>         <td style=\"padding: 10px\">エンタープライズ</td>         <td style=\"padding: 10px\">10,000</td>       </tr>       <tr>         <td style=\"padding: 10px\">プロフェッショナル</td>         <td style=\"padding: 10px\">5,000</td>       </tr>       <tr>         <td style=\"padding: 10px\">ベーシック</td>         <td style=\"padding: 10px\">3,000</td>       </tr>       <tr>         <td style=\"padding: 10px\">ミニマム</td>         <td style=\"padding: 10px\">3,000</td>       </tr>       <tr>         <td style=\"padding: 10px\">上記以外</td>         <td style=\"padding: 10px\">3,000</td>       </tr>     </tbody>   </table>  <h3 id=\"webhook\">Webhookについて</h3>  <p>詳細は<a href=\"https://developer.freee.co.jp/docs/accounting/webhook\" target=\"_blank\">会計Webhook概要</a>を参照してください。</p>  <hr /> <h2 id=\"contact\">連絡先</h2>  <p>ご不明点、ご要望等は <a href=\"https://support.freee.co.jp/hc/ja/requests/new\">freee サポートデスクへのお問い合わせフォーム</a> からご連絡ください。</p> <hr />&copy; Since 2013 freee K.K.
 *
 * The version of the OpenAPI document: v1.0
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};


/// struct for typed errors of method [`download_journal`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DownloadJournalError {
    Status400(crate::models::BadRequestError),
    Status401(crate::models::UnauthorizedError),
    Status403(crate::models::ForbiddenError),
    Status404(crate::models::BadRequestNotFoundError),
    Status500(crate::models::InternalServerError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_journal_status`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetJournalStatusError {
    Status400(crate::models::BadRequestError),
    Status401(crate::models::UnauthorizedError),
    Status403(crate::models::ForbiddenError),
    Status404(crate::models::BadRequestNotFoundError),
    Status500(crate::models::InternalServerError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_journals`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetJournalsError {
    Status400(crate::models::BadRequestError),
    Status401(crate::models::UnauthorizedError),
    Status403(crate::models::ForbiddenError),
    Status500(crate::models::InternalServerError),
    UnknownValue(serde_json::Value),
}


///  <h2 id=\"\">概要</h2>  <p>仕訳帳をダウンロードする</p>  <p>＊このAPIは無料プランのアカウントではご利用になれません</p>  <h2 id=\"_2\">定義</h2>  <ul> <li>id : 受け付けID</li> </ul>
pub async fn download_journal(configuration: &configuration::Configuration, id: i32, company_id: i32) -> Result<String, Error<DownloadJournalError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/1/journals/reports/{id}/download", local_var_configuration.base_path, id=id);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("company_id", &company_id.to_string())]);
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<DownloadJournalError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

///  <h2 id=\"\">概要</h2>  <p>仕訳帳のダウンロードリクエストのステータスを取得する</p>  <p>＊このAPIは無料プランのアカウントではご利用になれません</p>  <h2 id=\"_2\">定義</h2>  <ul> <li> <p>status</p>  <ul> <li>enqueued : 実行待ち</li>  <li>working : 実行中</li>  <li>uploaded : 準備完了</li> </ul> </li>  <li> <p>id : 受け付けID</p> </li> </ul>
pub async fn get_journal_status(configuration: &configuration::Configuration, company_id: i32, id: i32) -> Result<crate::models::JournalStatusResponse, Error<GetJournalStatusError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/1/journals/reports/{id}/status", local_var_configuration.base_path, id=id);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("company_id", &company_id.to_string())]);
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetJournalStatusError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

///  <h2 id=\"\">概要</h2>  <p>ユーザーが所属する事業所の仕訳帳のダウンロードをリクエストします 生成されるファイルに関しては、<a href=\"https://support.freee.co.jp/hc/ja/articles/204599604#2\">ヘルプページ</a>をご参照ください</p>  <p>＊このAPIは無料プランのアカウントではご利用になれません</p>  <h2 id=\"_2\">定義</h2>  <ul>   <li>download_type     <ul>       <li>generic(freee Webからダウンロードできるものと同じ)</li>       <li>csv (yayoi形式)</li>       <li>pdf</li>     </ul>   </li>   <li>visible_tags : 指定しない場合は従来の仕様の仕訳帳が出力されます     <ul>       <li>partner : 取引先タグ</li>       <li>item : 品目タグ</li>       <li>tag : メモタグ</li>       <li>section : 部門タグ</li>       <li>description : 備考欄</li>       <li>wallet_txn_description : 明細の備考欄</li>       <li>         segment_1_tag : セグメント1(法人向けプロフェッショナル, 法人向けエンタープライズプラン)<br>         segment_2_tag : セグメント2(法人向け エンタープライズプラン)<br>         segment_3_tag : セグメント3(法人向け エンタープライズプラン)<br><br>         <a href=\"https://support.freee.co.jp/hc/ja/articles/360020679611\" target=\"_blank\">セグメント（分析用タグ）の設定</a><br>       </li>       <li>all : 指定された場合は上記の設定をすべて有効として扱いますが、セグメント1、セグメント2、セグメント3は含みません。セグメントが必要な場合はallではなく、segment_1_tag, segment_2_tag, segment_3_tagを指定してください。</li>     </ul>   </li>   <li>visible_ids : download_typeがgenericの場合のみ利用可能です     <ul>       <li>deal_id : 取引ID</li>       <li>transfer_id : 取引(振替)ID</li>       <li>manual_journal_id : 振替伝票ID</li>     </ul>   </li>    <li>id : 受け付けID</li> </ul>
pub async fn get_journals(configuration: &configuration::Configuration, download_type: &str, company_id: i32, visible_tags_left_square_bracket_right_square_bracket: Option<Vec<String>>, visible_ids_left_square_bracket_right_square_bracket: Option<Vec<String>>, start_date: Option<&str>, end_date: Option<&str>) -> Result<crate::models::JournalsResponse, Error<GetJournalsError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/1/journals", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("download_type", &download_type.to_string())]);
    local_var_req_builder = local_var_req_builder.query(&[("company_id", &company_id.to_string())]);
    if let Some(ref local_var_str) = visible_tags_left_square_bracket_right_square_bracket {
        local_var_req_builder = match "multi" {
            "multi" => local_var_req_builder.query(&local_var_str.into_iter().map(|p| ("visible_tags[]".to_owned(), p.to_string())).collect::<Vec<(std::string::String, std::string::String)>>()),
            _ => local_var_req_builder.query(&[("visible_tags[]", &local_var_str.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
        };
    }
    if let Some(ref local_var_str) = visible_ids_left_square_bracket_right_square_bracket {
        local_var_req_builder = match "multi" {
            "multi" => local_var_req_builder.query(&local_var_str.into_iter().map(|p| ("visible_ids[]".to_owned(), p.to_string())).collect::<Vec<(std::string::String, std::string::String)>>()),
            _ => local_var_req_builder.query(&[("visible_ids[]", &local_var_str.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
        };
    }
    if let Some(ref local_var_str) = start_date {
        local_var_req_builder = local_var_req_builder.query(&[("start_date", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = end_date {
        local_var_req_builder = local_var_req_builder.query(&[("end_date", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetJournalsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

