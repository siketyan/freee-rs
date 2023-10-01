/*
 * freee人事労務 API
 *
 *  <p>freee人事労務のAPI仕様です。</p>  <hr />  <h2 id=\"start_guide\">スタートガイド</h2>  <p>freee API開発がはじめての方は<a href=\"https://developer.freee.co.jp/getting-started\">freee API スタートガイド</a>を参照してください。</p>  <hr /> <h2 id=\"specification\">仕様</h2>  <h3 id=\"api_endpoint\">APIエンドポイント</h3>  <p>https://api.freee.co.jp/hr</p>  <h3 id=\"about_authorize\">認証について</h3>  <p>OAuth2.0を利用します。<a href=\"https://developer.freee.co.jp/reference/#%e8%aa%8d%e8%a8%bc\" target=\"_blank\">詳細はリファレンスの認証に関する記載を参照してください。</a></p>  <h3 id=\"data_format\">データフォーマット</h3>  <p>リクエスト、レスポンスともにJSON形式をサポートしていますが、詳細は、API毎の説明欄（application/jsonなど）を確認してください。</p>  <h3 id=\"compatibility\">後方互換性ありの変更</h3>  <p>freeeでは、APIを改善していくために以下のような変更は後方互換性ありとして通知なく変更を入れることがあります。アプリケーション実装者は以下を踏まえて開発を行ってください。</p>  <ul> <li>新しいAPIリソース・エンドポイントの追加</li> <li>既存のAPIに対して必須ではない新しいリクエストパラメータの追加</li> <li>既存のAPIレスポンスに対する新しいプロパティの追加</li> <li>既存のAPIレスポンスに対するプロパティの順番の入れ変え</li> <li>keyとなっているidやcodeの長さの変更（長くする）</li> <li>エラーメッセージの変更</li> </ul>  <h3 id=\"common_response_header\">共通レスポンスヘッダー</h3>  <p>すべてのAPIのレスポンスには以下のHTTPヘッダーが含まれます。</p>  <ul> <li> <p>X-Request-Id</p> <ul> <li>各リクエスト毎に発行されるID</li> </ul> </li> </ul>  <h3 id=\"error_response\">共通エラーレスポンス</h3>  <p>APIリクエストでエラーが発生した場合は、エラー原因に応じたステータスコードおよびメッセージを返します。</p>    <table border=\"1\">   <tbody>     <tr>       <th style=\"padding: 10px\"><strong>ステータスコード</strong></th>       <th style=\"padding: 10px\"><strong>原因</strong></th>     </tr>     <tr><td style=\"padding: 10px\">400</td><td style=\"padding: 10px\">リクエストパラメータが不正</td></tr>     <tr><td style=\"padding: 10px\">401</td><td style=\"padding: 10px\">アクセストークンが無効</td></tr>     <tr><td style=\"padding: 10px\">403</td><td style=\"padding: 10px\">アクセス権限がない</td></tr>     <tr><td style=\"padding: 10px\">404</td><td style=\"padding: 10px\">リソースが存在しない</td></tr>     <tr><td style=\"padding: 10px\">429</td><td style=\"padding: 10px\">リクエスト回数制限を超えた</td></tr>     <tr><td style=\"padding: 10px\">503</td><td style=\"padding: 10px\">システム内で予期しないエラーが発生</td></tr>   </tbody> </table>  <p>メッセージボディ内の <code>messages</code> にはエラー内容を説明する文字列が入ります。</p> <pre><code>  {     &quot;status_code&quot; : 400,     &quot;errors&quot; : [       {         &quot;type&quot; : &quot;bad_request&quot;,         &quot;messages&quot; : [           &quot;リクエストの形式が不正です。&quot;         ]       }     ]   }  </code></pre>  </br>  <h3 id=\"api_rate_limit\">API使用制限</h3>  <p>APIリクエストは1時間で5000回を上限としています。API使用ステータスはレスポンスヘッダに付与されます。</p> <pre><code>X-Ratelimit-Limit:5000 X-Ratelimit-Remaining:4998 X-Ratelimit-Reset:2018-01-01T12:00:00.000000Z </code></pre>  <br> 各ヘッダの意味は次のとおりです。</p>   <table border=\"1\">   <tbody>     <tr>       <th style=\"padding: 10px\"><strong>ヘッダ名</strong></th>       <th style=\"padding: 10px\"><strong>説明</strong></th>     </tr>     <tr><td style=\"padding: 10px\">X-RateLimit-Limit</td><td style=\"padding: 10px\">使用回数の上限</td></tr>     <tr><td style=\"padding: 10px\">X-RateLimit-Remaining</td><td style=\"padding: 10px\">残り使用回数</td></tr>     <tr><td style=\"padding: 10px\">X-RateLimit-Reset</td><td style=\"padding: 10px\">使用回数がリセットされる時刻</td></tr>   </tbody> </table>  <p>上記に加え、freeeは一定期間に過度のアクセスを検知した場合、APIアクセスをコントロールする場合があります。<br> その際のhttp status codeは403となります。制限がかかってから10分程度が過ぎると再度使用することができるようになります。</p>  </br>  <h3 id=\"api_rate_limit\">プランごとの利用可能API</h3>   <p>契約プランごとに利用可能なfreee人事労務APIはfreee人事労務のWeb版でご利用できる機能と同様です。例えば、スタンダードプラン（または旧ベーシックプラン）を契約している場合、Web版では打刻機能をご利用いただけますので、APIでもタイムレコーダー(打刻)APIが利用可能です。<a href=\"https://support.freee.co.jp/hc/ja/articles/203309710\" target=\"_blank\">freee人事労務のWeb版のプラン別機能比較はfreee人事労務のプラン・料金についてのヘルプを参照してください。</a></p>  </br>  <hr />
 *
 * The version of the OpenAPI document: 2022-02-01
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApiV1OvertimeWorkIndexResponseParams {
    /// 申請ID
    #[serde(rename = "id")]
    pub id: i32,
    /// 事業所ID
    #[serde(rename = "company_id")]
    pub company_id: i32,
    /// 申請No
    #[serde(rename = "application_number")]
    pub application_number: i32,
    /// 申請者のユーザーID
    #[serde(rename = "applicant_id")]
    pub applicant_id: i32,
    /// 承認者のユーザーID配列<br> 次の場合、空配列になります。 - 指定なしの申請経路を利用した、申請ステータスが承認済み以外の申請 - 申請が差戻された
    #[serde(rename = "approver_ids", skip_serializing_if = "Option::is_none")]
    pub approver_ids: Option<Vec<i32>>,
    /// 対象日
    #[serde(rename = "target_date")]
    pub target_date: String,
    /// 取得予定開始時間
    #[serde(rename = "start_at")]
    pub start_at: String,
    /// 取得予定終了時間
    #[serde(rename = "end_at")]
    pub end_at: String,
    /// 申請日
    #[serde(rename = "issue_date")]
    pub issue_date: String,
    /// 申請理由
    #[serde(rename = "comment", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub comment: Option<Option<String>>,
    /// 申請ステータス。（draft:下書き、in_progress:申請中、approved:承認済、feedback:差戻し）
    #[serde(rename = "status")]
    pub status: Status,
    /// 取消申請ステータス。（null:取消申請されてない、revoking:取消中、revoked:取消済）
    #[serde(rename = "revoke_status", deserialize_with = "Option::deserialize")]
    pub revoke_status: Option<RevokeStatus>,
    /// 自動チェック結果
    #[serde(rename = "passed_auto_check")]
    pub passed_auto_check: bool,
}

impl ApiV1OvertimeWorkIndexResponseParams {
    pub fn new(id: i32, company_id: i32, application_number: i32, applicant_id: i32, target_date: String, start_at: String, end_at: String, issue_date: String, status: Status, revoke_status: Option<RevokeStatus>, passed_auto_check: bool) -> ApiV1OvertimeWorkIndexResponseParams {
        ApiV1OvertimeWorkIndexResponseParams {
            id,
            company_id,
            application_number,
            applicant_id,
            approver_ids: None,
            target_date,
            start_at,
            end_at,
            issue_date,
            comment: None,
            status,
            revoke_status,
            passed_auto_check,
        }
    }
}

/// 申請ステータス。（draft:下書き、in_progress:申請中、approved:承認済、feedback:差戻し）
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "draft")]
    Draft,
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "approved")]
    Approved,
    #[serde(rename = "feedback")]
    Feedback,
}

impl Default for Status {
    fn default() -> Status {
        Self::Draft
    }
}
/// 取消申請ステータス。（null:取消申請されてない、revoking:取消中、revoked:取消済）
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RevokeStatus {
    #[serde(rename = "revoking")]
    Revoking,
    #[serde(rename = "revoked")]
    Revoked,
}

impl Default for RevokeStatus {
    fn default() -> RevokeStatus {
        Self::Revoking
    }
}

