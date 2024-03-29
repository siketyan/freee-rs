/*
 * freee API
 *
 *  <h1 id=\"freee_api\">freee API</h1> <hr /> <h2 id=\"start_guide\">スタートガイド</h2>  <p>freee API開発がはじめての方は<a href=\"https://developer.freee.co.jp/getting-started\">freee API スタートガイド</a>を参照してください。</p>  <hr /> <h2 id=\"specification\">仕様</h2>  <h3 id=\"api_endpoint\">APIエンドポイント</h3>  <p>https://api.freee.co.jp/ (httpsのみ)</p>  <h3 id=\"about_authorize\">認証について</h3> <p>OAuth2.0を利用します。<a href=\"https://developer.freee.co.jp/reference/#%e8%aa%8d%e8%a8%bc\" target=\"_blank\">詳細はリファレンスの認証に関する記載を参照してください。</a></p>  <h3 id=\"data_format\">データフォーマット</h3>  <p>リクエスト、レスポンスともにJSON形式をサポートしていますが、詳細は、API毎の説明欄（application/jsonなど）を確認してください。</p>  <h3 id=\"compatibility\">後方互換性ありの変更</h3>  <p>freeeでは、APIを改善していくために以下のような変更は後方互換性ありとして通知なく変更を入れることがあります。アプリケーション実装者は以下を踏まえて開発を行ってください。</p>  <ul> <li>新しいAPIリソース・エンドポイントの追加</li> <li>既存のAPIに対して必須ではない新しいリクエストパラメータの追加</li> <li>既存のAPIレスポンスに対する新しいプロパティの追加</li> <li>既存のAPIレスポンスに対するプロパティの順番の入れ変え</li> <li>keyとなっているidやcodeの長さの変更（長くする）</li> </ul>  <h3 id=\"common_response_header\">共通レスポンスヘッダー</h3>  <p>すべてのAPIのレスポンスには以下のHTTPヘッダーが含まれます。</p>  <ul> <li> <p>X-Freee-Request-ID</p> <ul> <li>各リクエスト毎に発行されるID</li> </ul> </li> </ul>  <h3 id=\"common_error_response\">共通エラーレスポンス</h3>  <ul> <li> <p>ステータスコードはレスポンス内のJSONに含まれる他、HTTPヘッダにも含まれる</p> </li> <li> <p>一部のエラーレスポンスにはエラーコードが含まれます。<br>詳細は、<a href=\"https://developer.freee.co.jp/tips/faq/40x-checkpoint\">HTTPステータスコード400台エラー時のチェックポイント</a>を参照してください</p> </li> <p>type</p>  <ul> <li>status : HTTPステータスコードの説明</li>  <li>validation : エラーの詳細の説明（開発者向け）</li> </ul> </li> </ul>  <p>レスポンスの例</p>  <pre><code>  {     &quot;status_code&quot; : 400,     &quot;errors&quot; : [       {         &quot;type&quot; : &quot;status&quot;,         &quot;messages&quot; : [&quot;不正なリクエストです。&quot;]       },       {         &quot;type&quot; : &quot;validation&quot;,         &quot;messages&quot; : [&quot;Date は不正な日付フォーマットです。入力例：2019-12-17&quot;]       }     ]   }</code></pre>  </br>  <h3 id=\"api_rate_limit\">API使用制限</h3>    <p>freeeは一定期間に過度のアクセスを検知した場合、APIアクセスをコントロールする場合があります。</p>   <p>その際のhttp status codeは403となります。制限がかかってから10分程度が過ぎると再度使用することができるようになります。</p>  <h4 id=\"reports_api_endpoint\">/reportsと/receipts/{id}/downloadエンドポイント</h4>  <p>freeeはエンドポイント毎に一定頻度以上のアクセスを検知した場合、APIアクセスをコントロールする場合があります。その際のhttp status codeは429（too many requests）となります。</p> <ul>   <li>/reports:1秒に10回まで</li>   <li>/receipts/{id}/download:1秒に3回まで</li> </ul>  <p>http status codeが429となった場合、API使用ステータスはレスポンスヘッダに付与されます。</p> <pre><code>x-ratelimit-limit:10 x-ratelimit-remaining:1 x-ratelimit-reset:2023-01-13T10:22:29+09:00 </code></pre>  <br> 各ヘッダの意味は次のとおりです。</p>  <table border=\"1\">   <tbody>     <tr>       <th style=\"padding: 10px\"><strong>ヘッダ名</strong></th>       <th style=\"padding: 10px\"><strong>説明</strong></th>     </tr>     <tr><td style=\"padding: 10px\">x-ratelimit-limit</td><td style=\"padding: 10px\">使用回数の上限</td></tr>     <tr><td style=\"padding: 10px\">x-ratelimit-remaining</td><td style=\"padding: 10px\">残り使用回数</td></tr>     <tr><td style=\"padding: 10px\">x-ratelimit-reset</td><td style=\"padding: 10px\">使用回数がリセットされる時刻</td></tr>   </tbody> </table>  </br>  <h3 id=\"plan_api_rate_limit\">プラン別のAPI Rate Limit</h3>   <table border=\"1\">     <tbody>       <tr>         <th style=\"padding: 10px\"><strong>freee会計プラン名</strong></th>         <th style=\"padding: 10px\"><strong>事業所とアプリケーション毎に1日でのAPIコール数</strong></th>       </tr>       <tr>         <td style=\"padding: 10px\">エンタープライズ</td>         <td style=\"padding: 10px\">10,000</td>       </tr>       <tr>         <td style=\"padding: 10px\">プロフェッショナル</td>         <td style=\"padding: 10px\">5,000</td>       </tr>       <tr>         <td style=\"padding: 10px\">ベーシック</td>         <td style=\"padding: 10px\">3,000</td>       </tr>       <tr>         <td style=\"padding: 10px\">ミニマム</td>         <td style=\"padding: 10px\">3,000</td>       </tr>       <tr>         <td style=\"padding: 10px\">上記以外</td>         <td style=\"padding: 10px\">3,000</td>       </tr>     </tbody>   </table>  <h3 id=\"webhook\">Webhookについて</h3>  <p>詳細は<a href=\"https://developer.freee.co.jp/docs/accounting/webhook\" target=\"_blank\">会計Webhook概要</a>を参照してください。</p>  <hr /> <h2 id=\"contact\">連絡先</h2>  <p>ご不明点、ご要望等は <a href=\"https://support.freee.co.jp/hc/ja/requests/new\">freee サポートデスクへのお問い合わせフォーム</a> からご連絡ください。</p> <hr />&copy; Since 2013 freee K.K.
 *
 * The version of the OpenAPI document: v1.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Receipt {
    /// ファイルボックス（証憑ファイル）ID
    #[serde(rename = "id")]
    pub id: i32,
    /// ステータス(confirmed:確認済み、deleted:削除済み、ignored:無視)
    #[serde(rename = "status")]
    pub status: Status,
    /// メモ
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// MIMEタイプ
    #[serde(rename = "mime_type")]
    pub mime_type: String,
    /// 発生日
    #[serde(rename = "issue_date", skip_serializing_if = "Option::is_none")]
    pub issue_date: Option<String>,
    /// アップロード元種別
    #[serde(rename = "origin")]
    pub origin: Origin,
    /// 作成日時（ISO8601形式）
    #[serde(rename = "created_at")]
    pub created_at: String,
    /// ファイルのダウンロードURL（freeeにログインした状態でのみ閲覧可能です。） <br> <br> file_srcは廃止予定の属性になります。<br> file_srcに替わり、証憑ファイルのダウンロード APIをご利用ください。<br> 証憑ファイルのダウンロードAPIを利用することで、以下のようになります。 <ul>   <li>アプリケーション利用者はfreee APIアプリケーションにログインしていれば、証憑ダウンロード毎にfreeeに改めてログインすることなくファイルが参照できるようになります。</li> </ul>
    #[serde(rename = "file_src")]
    pub file_src: String,
    #[serde(rename = "user")]
    pub user: Box<crate::models::DealCreateResponseDealReceiptsInnerUser>,
    #[serde(rename = "receipt_metadatum", skip_serializing_if = "Option::is_none")]
    pub receipt_metadatum: Option<Box<crate::models::ReceiptUpdateParamsReceiptMetadatum>>,
    /// この項目はインボイス制度で利用する項目です。2023年4月頃から利用できる予定です。 適格請求書等（qualified: 該当する、not_qualified: 該当しない） 
    #[serde(rename = "qualified_invoice", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub qualified_invoice: Option<Option<QualifiedInvoice>>,
    /// この項目はインボイス制度で利用する項目です。2023年4月頃から利用できる予定です。 インボイス制度適格請求書発行事業者登録番号 - 先頭T数字13桁の固定14桁の文字列 <a target=\"_blank\" href=\"https://www.invoice-kohyo.nta.go.jp/index.html\">国税庁インボイス制度適格請求書発行事業者公表サイト</a> 
    #[serde(rename = "invoice_registration_number", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub invoice_registration_number: Option<Option<String>>,
    /// この項目はインボイス制度で利用する項目です。2023年4月頃から利用できる予定です。 書類の種類（receipt: 領収書、invoice: 請求書、other: その他） 
    #[serde(rename = "document_type", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub document_type: Option<Option<DocumentType>>,
}

impl Receipt {
    pub fn new(id: i32, status: Status, mime_type: String, origin: Origin, created_at: String, file_src: String, user: crate::models::DealCreateResponseDealReceiptsInnerUser) -> Receipt {
        Receipt {
            id,
            status,
            description: None,
            mime_type,
            issue_date: None,
            origin,
            created_at,
            file_src,
            user: Box::new(user),
            receipt_metadatum: None,
            qualified_invoice: None,
            invoice_registration_number: None,
            document_type: None,
        }
    }
}

/// ステータス(confirmed:確認済み、deleted:削除済み、ignored:無視)
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "confirmed")]
    Confirmed,
    #[serde(rename = "deleted")]
    Deleted,
    #[serde(rename = "ignored")]
    Ignored,
}

impl Default for Status {
    fn default() -> Status {
        Self::Confirmed
    }
}
/// アップロード元種別
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Origin {
    #[serde(rename = "unknown")]
    Unknown,
    #[serde(rename = "web")]
    Web,
    #[serde(rename = "mobile_camera")]
    MobileCamera,
    #[serde(rename = "mobile_album")]
    MobileAlbum,
    #[serde(rename = "scansnap")]
    Scansnap,
    #[serde(rename = "scannable")]
    Scannable,
    #[serde(rename = "dropbox")]
    Dropbox,
    #[serde(rename = "mail")]
    Mail,
    #[serde(rename = "safety_contact_file")]
    SafetyContactFile,
    #[serde(rename = "public_api")]
    PublicApi,
}

impl Default for Origin {
    fn default() -> Origin {
        Self::Unknown
    }
}
/// この項目はインボイス制度で利用する項目です。2023年4月頃から利用できる予定です。 適格請求書等（qualified: 該当する、not_qualified: 該当しない） 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum QualifiedInvoice {
    #[serde(rename = "qualified")]
    Qualified,
    #[serde(rename = "not_qualified")]
    NotQualified,
}

impl Default for QualifiedInvoice {
    fn default() -> QualifiedInvoice {
        Self::Qualified
    }
}
/// この項目はインボイス制度で利用する項目です。2023年4月頃から利用できる予定です。 書類の種類（receipt: 領収書、invoice: 請求書、other: その他） 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DocumentType {
    #[serde(rename = "receipt")]
    Receipt,
    #[serde(rename = "invoice")]
    Invoice,
    #[serde(rename = "other")]
    Other,
}

impl Default for DocumentType {
    fn default() -> DocumentType {
        Self::Receipt
    }
}

