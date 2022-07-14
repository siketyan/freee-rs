/*
 * freee API
 *
 *  <h1 id=\"freee_api\">freee API</h1> <hr /> <h2 id=\"start_guide\">スタートガイド</h2>  <p>freee API開発がはじめての方は<a href=\"https://developer.freee.co.jp/getting-started\">freee API スタートガイド</a>を参照してください。</p>  <hr /> <h2 id=\"specification\">仕様</h2>  <h3 id=\"api_endpoint\">APIエンドポイント</h3>  <p>https://api.freee.co.jp/ (httpsのみ)</p>  <h3 id=\"about_authorize\">認証について</h3> <p>OAuth2.0を利用します。詳細は<a href=\"https://developer.freee.co.jp/docs\" target=\"_blank\">ドキュメントの認証</a>パートを参照してください。</p>  <h3 id=\"data_format\">データフォーマット</h3>  <p>リクエスト、レスポンスともにJSON形式をサポートしていますが、詳細は、API毎の説明欄（application/jsonなど）を確認してください。</p>  <h3 id=\"compatibility\">後方互換性ありの変更</h3>  <p>freeeでは、APIを改善していくために以下のような変更は後方互換性ありとして通知なく変更を入れることがあります。アプリケーション実装者は以下を踏まえて開発を行ってください。</p>  <ul> <li>新しいAPIリソース・エンドポイントの追加</li> <li>既存のAPIに対して必須ではない新しいリクエストパラメータの追加</li> <li>既存のAPIレスポンスに対する新しいプロパティの追加</li> <li>既存のAPIレスポンスに対するプロパティの順番の入れ変え</li> <li>keyとなっているidやcodeの長さの変更（長くする）</li> </ul>  <h3 id=\"common_response_header\">共通レスポンスヘッダー</h3>  <p>すべてのAPIのレスポンスには以下のHTTPヘッダーが含まれます。</p>  <ul> <li> <p>X-Freee-Request-ID</p> <ul> <li>各リクエスト毎に発行されるID</li> </ul> </li> </ul>  <h3 id=\"common_error_response\">共通エラーレスポンス</h3>  <ul> <li> <p>ステータスコードはレスポンス内のJSONに含まれる他、HTTPヘッダにも含まれる</p> </li> <li> <p>一部のエラーレスポンスにはエラーコードが含まれます。<br>詳細は、<a href=\"https://developer.freee.co.jp/tips/faq/40x-checkpoint\">HTTPステータスコード400台エラー時のチェックポイント</a>を参照してください</p> </li> <p>type</p>  <ul> <li>status : HTTPステータスコードの説明</li>  <li>validation : エラーの詳細の説明（開発者向け）</li> </ul> </li> </ul>  <p>レスポンスの例</p>  <pre><code>  {     &quot;status_code&quot; : 400,     &quot;errors&quot; : [       {         &quot;type&quot; : &quot;status&quot;,         &quot;messages&quot; : [&quot;不正なリクエストです。&quot;]       },       {         &quot;type&quot; : &quot;validation&quot;,         &quot;messages&quot; : [&quot;Date は不正な日付フォーマットです。入力例：2019-12-17&quot;]       }     ]   }</code></pre>  </br>  <h3 id=\"api_rate_limit\">API使用制限</h3>    <p>freeeは一定期間に過度のアクセスを検知した場合、APIアクセスをコントロールする場合があります。</p>   <p>その際のhttp status codeは403となります。制限がかかってから10分程度が過ぎると再度使用することができるようになります。</p>  <h4 id=\"reports_api_endpoint\">/reportsと/receipts/{id}/downloadエンドポイント</h4>  <p>freeeはエンドポイント毎に一定頻度以上のアクセスを検知した場合、APIアクセスをコントロールする場合があります。その際のhttp status codeは429（too many requests）となります。</p>  <ul>   <li>/reports:1秒に10回まで</li>   <li>/receipts/{id}/download:1秒に3回まで</li> </ul>  <p>レスポンスボディのmetaプロパティに以下を含めます。</p>  <ul>   <li>設定されている上限値</li>   <li>上限に達するまでの使用可能回数</li>   <li>（上限値に達した場合）使用回数がリセットされる時刻</li> </ul>  <h3 id=\"plan_api_rate_limit\">プラン別のAPI Rate Limit</h3>   <table border=\"1\">     <tbody>       <tr>         <th style=\"padding: 10px\"><strong>freee会計プラン名</strong></th>         <th style=\"padding: 10px\"><strong>事業所とアプリケーション毎に1日でのAPIコール数</strong></th>       </tr>       <tr>         <td style=\"padding: 10px\">エンタープライズ</td>         <td style=\"padding: 10px\">10,000</td>       </tr>       <tr>         <td style=\"padding: 10px\">プロフェッショナル</td>         <td style=\"padding: 10px\">5,000</td>       </tr>       <tr>         <td style=\"padding: 10px\">ベーシック</td>         <td style=\"padding: 10px\">3,000</td>       </tr>       <tr>         <td style=\"padding: 10px\">ミニマム</td>         <td style=\"padding: 10px\">3,000</td>       </tr>       <tr>         <td style=\"padding: 10px\">上記以外</td>         <td style=\"padding: 10px\">3,000</td>       </tr>     </tbody>   </table>  <h3 id=\"webhook\">Webhookについて</h3>  <p>詳細は<a href=\"https://developer.freee.co.jp/docs/accounting/webhook\" target=\"_blank\">会計Webhook概要</a>を参照してください。</p>  <hr /> <h2 id=\"contact\">連絡先</h2>  <p>ご不明点、ご要望等は <a href=\"https://support.freee.co.jp/hc/ja/requests/new\">freee サポートデスクへのお問い合わせフォーム</a> からご連絡ください。</p> <hr />&copy; Since 2013 freee K.K.
 *
 * The version of the OpenAPI document: v1.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PartnerUpdateParams {
    /// 事業所ID
    #[serde(rename = "company_id")]
    pub company_id: i32,
    /// 取引先名 (255文字以内)
    #[serde(rename = "name")]
    pub name: String,
    /// ショートカット１ (255文字以内)
    #[serde(rename = "shortcut1", skip_serializing_if = "Option::is_none")]
    pub shortcut1: Option<String>,
    /// ショートカット２ (255文字以内)
    #[serde(rename = "shortcut2", skip_serializing_if = "Option::is_none")]
    pub shortcut2: Option<String>,
    /// 事業所種別（null: 未設定、1: 法人、2: 個人）
    #[serde(rename = "org_code", skip_serializing_if = "Option::is_none")]
    pub org_code: Option<OrgCode>,
    /// 地域（JP: 国内、ZZ:国外）、指定しない場合JPになります。
    #[serde(rename = "country_code", skip_serializing_if = "Option::is_none")]
    pub country_code: Option<CountryCode>,
    /// 正式名称（255文字以内）
    #[serde(rename = "long_name", skip_serializing_if = "Option::is_none")]
    pub long_name: Option<String>,
    /// カナ名称（255文字以内）
    #[serde(rename = "name_kana", skip_serializing_if = "Option::is_none")]
    pub name_kana: Option<String>,
    /// 敬称（御中、様、(空白)の3つから選択）
    #[serde(rename = "default_title", skip_serializing_if = "Option::is_none")]
    pub default_title: Option<String>,
    /// 電話番号
    #[serde(rename = "phone", skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    /// 担当者 氏名 (255文字以内)
    #[serde(rename = "contact_name", skip_serializing_if = "Option::is_none")]
    pub contact_name: Option<String>,
    /// 担当者 メールアドレス (255文字以内)
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// 振込元口座ID（一括振込ファイル用）:（walletableのtypeが'bank_account'のidのみ指定できます。また、未設定にする場合は、nullを指定してください。）
    #[serde(rename = "payer_walletable_id", skip_serializing_if = "Option::is_none")]
    pub payer_walletable_id: Option<i32>,
    /// 振込手数料負担（一括振込ファイル用）: (振込元(当方): payer, 振込先(先方): payee)、指定しない場合payerになります。
    #[serde(rename = "transfer_fee_handling_side", skip_serializing_if = "Option::is_none")]
    pub transfer_fee_handling_side: Option<TransferFeeHandlingSide>,
    #[serde(rename = "address_attributes", skip_serializing_if = "Option::is_none")]
    pub address_attributes: Option<Box<crate::models::PartnerCreateParamsAddressAttributes>>,
    #[serde(rename = "partner_doc_setting_attributes", skip_serializing_if = "Option::is_none")]
    pub partner_doc_setting_attributes: Option<Box<crate::models::PartnerCreateParamsPartnerDocSettingAttributes>>,
    #[serde(rename = "partner_bank_account_attributes", skip_serializing_if = "Option::is_none")]
    pub partner_bank_account_attributes: Option<Box<crate::models::PartnerCreateParamsPartnerBankAccountAttributes>>,
    #[serde(rename = "payment_term_attributes", skip_serializing_if = "Option::is_none")]
    pub payment_term_attributes: Option<Box<crate::models::PartnerCreateParamsPaymentTermAttributes>>,
    #[serde(rename = "invoice_payment_term_attributes", skip_serializing_if = "Option::is_none")]
    pub invoice_payment_term_attributes: Option<Box<crate::models::PartnerCreateParamsInvoicePaymentTermAttributes>>,
}

impl PartnerUpdateParams {
    pub fn new(company_id: i32, name: String) -> PartnerUpdateParams {
        PartnerUpdateParams {
            company_id,
            name,
            shortcut1: None,
            shortcut2: None,
            org_code: None,
            country_code: None,
            long_name: None,
            name_kana: None,
            default_title: None,
            phone: None,
            contact_name: None,
            email: None,
            payer_walletable_id: None,
            transfer_fee_handling_side: None,
            address_attributes: None,
            partner_doc_setting_attributes: None,
            partner_bank_account_attributes: None,
            payment_term_attributes: None,
            invoice_payment_term_attributes: None,
        }
    }
}

/// 事業所種別（null: 未設定、1: 法人、2: 個人）
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum OrgCode {
    #[serde(rename = "1")]
    _1,
    #[serde(rename = "2")]
    _2,
}

impl Default for OrgCode {
    fn default() -> OrgCode {
        Self::_1
    }
}
/// 地域（JP: 国内、ZZ:国外）、指定しない場合JPになります。
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CountryCode {
    #[serde(rename = "JP")]
    JP,
    #[serde(rename = "ZZ")]
    ZZ,
}

impl Default for CountryCode {
    fn default() -> CountryCode {
        Self::JP
    }
}
/// 振込手数料負担（一括振込ファイル用）: (振込元(当方): payer, 振込先(先方): payee)、指定しない場合payerになります。
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TransferFeeHandlingSide {
    #[serde(rename = "payer")]
    Payer,
    #[serde(rename = "payee")]
    Payee,
}

impl Default for TransferFeeHandlingSide {
    fn default() -> TransferFeeHandlingSide {
        Self::Payer
    }
}

