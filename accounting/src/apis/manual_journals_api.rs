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


/// struct for typed errors of method [`create_manual_journal`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateManualJournalError {
    Status400(crate::models::BadRequestError),
    Status401(crate::models::UnauthorizedError),
    Status403(crate::models::ForbiddenError),
    Status500(crate::models::InternalServerError),
    Status503(crate::models::ServiceUnavailableError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`destroy_manual_journal`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DestroyManualJournalError {
    Status400(crate::models::BadRequestError),
    Status401(crate::models::UnauthorizedError),
    Status403(crate::models::ForbiddenError),
    Status404(crate::models::BadRequestNotFoundError),
    Status500(crate::models::InternalServerError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_manual_journal`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetManualJournalError {
    Status400(crate::models::BadRequestError),
    Status401(crate::models::UnauthorizedError),
    Status403(crate::models::ForbiddenError),
    Status404(crate::models::BadRequestNotFoundError),
    Status500(crate::models::InternalServerError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_manual_journals`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetManualJournalsError {
    Status400(crate::models::BadRequestError),
    Status401(crate::models::UnauthorizedError),
    Status403(crate::models::ForbiddenError),
    Status500(crate::models::InternalServerError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_manual_journal`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateManualJournalError {
    Status400(crate::models::BadRequestError),
    Status401(crate::models::UnauthorizedError),
    Status403(crate::models::ForbiddenError),
    Status404(crate::models::BadRequestNotFoundError),
    Status500(crate::models::InternalServerError),
    UnknownValue(serde_json::Value),
}


///  <h2 id=\"\">概要</h2>  <p>指定した事業所の振替伝票を作成する</p>  <h2 id=\"_2\">定義</h2>  <ul> <li> <p>issue_date : 発生日</p> </li>  <li> <p>adjustment : 決算整理仕訳フラグ（true: 決算整理仕訳, false: 日常仕訳）</p> </li>  <li> <p>txn_number : 仕訳番号</p> </li>  <li> <p>details : 振替伝票の貸借行</p> </li>  <li> <p>entry_side : 貸借区分</p>  <ul> <li>credit : 貸方</li>  <li>debit : 借方</li> </ul> </li>  <li> <p>amount : 金額</p> </li> </ul>  <h2 id=\"_3\">注意点</h2>  <ul> <li>振替伝票は売掛・買掛レポートには反映されません。債権・債務データの登録は取引(Deals)をお使いください。</li> <li>事業所の仕訳番号形式が有効な場合のみ、レスポンスで仕訳番号(txn_number)を返します。</li> <li>貸借合わせて100行まで仕訳行を登録できます。</li> <li>セグメントタグ情報は法人向けのプロフェッショナルプラン以上で利用可能です。利用可能なセグメントの数は、法人向けのプロフェッショナルプランの場合は1つ、エンタープライズプランの場合は3つです。</li> <li>partner_codeを利用するには、事業所の設定から取引先コードの利用を有効にする必要があります。またpartner_codeとpartner_idは同時に指定することはできません。</li></ul>  
pub async fn create_manual_journal(configuration: &configuration::Configuration, manual_journal_create_params: Option<crate::models::ManualJournalCreateParams>) -> Result<crate::models::ManualJournalResponse, Error<CreateManualJournalError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/1/manual_journals", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&manual_journal_create_params);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CreateManualJournalError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

///  <h2 id=\"\">概要</h2>  <p>指定した事業所の振替伝票を削除する</p>
pub async fn destroy_manual_journal(configuration: &configuration::Configuration, id: i32, company_id: i32) -> Result<(), Error<DestroyManualJournalError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/1/manual_journals/{id}", local_var_configuration.base_path, id=id);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

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
        Ok(())
    } else {
        let local_var_entity: Option<DestroyManualJournalError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

///  <h2 id=\"\">概要</h2>  <p>指定した事業所の振替伝票を取得する</p>  <h2 id=\"_2\">定義</h2>  <ul> <li> <p>issue_date : 発生日</p> </li> <li> <p>adjustment : 決算整理仕訳フラグ（true: 決算整理仕訳, false: 日常仕訳）</p> </li> <li> <p>txn_number : 仕訳番号</p> </li> <li> <p>details : 振替伝票の貸借行</p> </li> <li> <p>entry_side : 貸借区分</p> <ul> <li>credit : 貸方</li> <li>debit : 借方</li> </ul> </li> <li> <p>amount : 金額</p> </li> </ul>  <h2 id=\"_3\">注意点</h2>  <ul> <li>振替伝票は売掛・買掛レポートには反映されません。債権・債務データの登録は取引(Deals)をお使いください。</li> <li>事業所の仕訳番号形式が有効な場合のみ、レスポンスで仕訳番号(txn_number)を返します。</li> <li>セグメントタグ情報は法人向けのプロフェッショナルプラン以上で利用可能です。利用可能なセグメントの数は、法人向けのプロフェッショナルプランの場合は1つ、エンタープライズプランの場合は3つです。</li> </ul>
pub async fn get_manual_journal(configuration: &configuration::Configuration, company_id: i32, id: i32) -> Result<crate::models::ManualJournalResponse, Error<GetManualJournalError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/1/manual_journals/{id}", local_var_configuration.base_path, id=id);
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
        let local_var_entity: Option<GetManualJournalError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

///  <h2 id=\"\">概要</h2>  <p>指定した事業所の振替伝票一覧を取得する</p>  <h2 id=\"_2\">定義</h2>  <ul> <li> <p>issue_date : 発生日</p> </li>  <li> <p>adjustment : 決算整理仕訳フラグ（true: 決算整理仕訳, false: 日常仕訳）</p> </li>  <li> <p>txn_number : 仕訳番号</p> </li>  <li> <p>details : 振替伝票の貸借行</p> </li>  <li> <p>entry_side : 貸借区分</p>  <ul> <li>credit : 貸方</li>  <li>debit : 借方</li> </ul> </li>  <li> <p>amount : 金額</p> </li> </ul>  <h2 id=\"_3\">注意点</h2>  <ul> <li>振替伝票は売掛・買掛レポートには反映されません。債権・債務データの登録は取引(Deals)をお使いください。</li> <li>事業所の仕訳番号形式が有効な場合のみ、レスポンスで仕訳番号(txn_number)を返します。</li> <li>セグメントタグ情報は法人向けのプロフェッショナルプラン以上で利用可能です。利用可能なセグメントの数は、法人向けのプロフェッショナルプランの場合は1つ、エンタープライズプランの場合は3つです。</li> <li>partner_codeを利用するには、事業所の設定から取引先コードの利用を有効にする必要があります。またpartner_codeとpartner_idは同時に指定することはできません。</li></ul>
pub async fn get_manual_journals(configuration: &configuration::Configuration, company_id: i32, start_issue_date: Option<&str>, end_issue_date: Option<&str>, entry_side: Option<&str>, account_item_id: Option<i32>, min_amount: Option<i64>, max_amount: Option<i64>, partner_id: Option<i32>, partner_code: Option<&str>, item_id: Option<i32>, section_id: Option<i32>, segment_1_tag_id: Option<i64>, segment_2_tag_id: Option<i64>, segment_3_tag_id: Option<i64>, comment_status: Option<&str>, comment_important: Option<bool>, adjustment: Option<&str>, txn_number: Option<&str>, offset: Option<i64>, limit: Option<i32>) -> Result<crate::models::GetManualJournals200Response, Error<GetManualJournalsError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/1/manual_journals", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("company_id", &company_id.to_string())]);
    if let Some(ref local_var_str) = start_issue_date {
        local_var_req_builder = local_var_req_builder.query(&[("start_issue_date", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = end_issue_date {
        local_var_req_builder = local_var_req_builder.query(&[("end_issue_date", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = entry_side {
        local_var_req_builder = local_var_req_builder.query(&[("entry_side", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = account_item_id {
        local_var_req_builder = local_var_req_builder.query(&[("account_item_id", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = min_amount {
        local_var_req_builder = local_var_req_builder.query(&[("min_amount", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = max_amount {
        local_var_req_builder = local_var_req_builder.query(&[("max_amount", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = partner_id {
        local_var_req_builder = local_var_req_builder.query(&[("partner_id", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = partner_code {
        local_var_req_builder = local_var_req_builder.query(&[("partner_code", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = item_id {
        local_var_req_builder = local_var_req_builder.query(&[("item_id", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = section_id {
        local_var_req_builder = local_var_req_builder.query(&[("section_id", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = segment_1_tag_id {
        local_var_req_builder = local_var_req_builder.query(&[("segment_1_tag_id", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = segment_2_tag_id {
        local_var_req_builder = local_var_req_builder.query(&[("segment_2_tag_id", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = segment_3_tag_id {
        local_var_req_builder = local_var_req_builder.query(&[("segment_3_tag_id", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = comment_status {
        local_var_req_builder = local_var_req_builder.query(&[("comment_status", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = comment_important {
        local_var_req_builder = local_var_req_builder.query(&[("comment_important", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = adjustment {
        local_var_req_builder = local_var_req_builder.query(&[("adjustment", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = txn_number {
        local_var_req_builder = local_var_req_builder.query(&[("txn_number", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = offset {
        local_var_req_builder = local_var_req_builder.query(&[("offset", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = limit {
        local_var_req_builder = local_var_req_builder.query(&[("limit", &local_var_str.to_string())]);
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
        let local_var_entity: Option<GetManualJournalsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

///  <h2 id=\"\">概要</h2>  <p>指定した事業所の振替伝票を更新する</p>  <h2 id=\"_2\">定義</h2>  <ul> <li> <p>issue_date : 発生日</p> </li>  <li> <p>adjustment : 決算整理仕訳フラグ（true: 決算整理仕訳, false: 日常仕訳）</p> </li>  <li> <p>txn_number : 仕訳番号</p> </li>  <li> <p>details : 振替伝票の貸借行</p> </li>  <li> <p>entry_side : 貸借区分</p>  <ul> <li>credit : 貸方</li>  <li>debit : 借方</li> </ul> </li>  <li> <p>amount : 金額</p> </li> </ul>  <h2 id=\"_3\">注意点</h2>  <ul> <li>振替伝票は売掛・買掛レポートには反映されません。債権・債務データの登録は取引(Deals)をお使いください。</li>  <li>事業所の仕訳番号形式が有効な場合のみ、レスポンスで仕訳番号(txn_number)を返します。</li> <li>貸借合わせて100行まで仕訳行を登録できます。</li>  <li>detailsに含まれない既存の貸借行は削除されます。更新後も残したい行は、必ず貸借行IDを指定してdetailsに含めてください。</li>  <li>detailsに含まれる貸借行IDの指定がある行は、更新行として扱われ更新されます。</li>  <li>detailsに含まれる貸借行IDの指定がない行は、新規行として扱われ追加されます。</li> <li>セグメントタグ情報は法人向けのプロフェッショナルプラン以上で利用可能です。利用可能なセグメントの数は、法人向けのプロフェッショナルプランの場合は1つ、エンタープライズプランの場合は3つです。</li> <li>partner_codeを利用するには、事業所の設定から取引先コードの利用を有効にする必要があります。またpartner_codeとpartner_idは同時に指定することはできません。</li></ul>  
pub async fn update_manual_journal(configuration: &configuration::Configuration, id: i32, manual_journal_update_params: Option<crate::models::ManualJournalUpdateParams>) -> Result<crate::models::ManualJournalResponse, Error<UpdateManualJournalError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/1/manual_journals/{id}", local_var_configuration.base_path, id=id);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&manual_journal_update_params);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<UpdateManualJournalError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

