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
pub struct TrialPlSegment1TagsResponseTrialPlSegment1Tags {
    /// 事業所ID
    #[serde(rename = "company_id")]
    pub company_id: i32,
    /// 出力するセグメント1の指定
    #[serde(rename = "segment_1_tag_ids")]
    pub segment_1_tag_ids: String,
    /// 会計年度(条件に指定した時、または条件に月、日条件がない時のみ含まれる）
    #[serde(rename = "fiscal_year", skip_serializing_if = "Option::is_none")]
    pub fiscal_year: Option<i32>,
    /// 発生月で絞込：開始会計月(1-12)(条件に指定した時のみ含まれる）
    #[serde(rename = "start_month", skip_serializing_if = "Option::is_none")]
    pub start_month: Option<i32>,
    /// 発生月で絞込：終了会計月(1-12)(条件に指定した時のみ含まれる）
    #[serde(rename = "end_month", skip_serializing_if = "Option::is_none")]
    pub end_month: Option<i32>,
    /// 発生日で絞込：開始日(yyyy-mm-dd)(条件に指定した時のみ含まれる）
    #[serde(rename = "start_date", skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    /// 発生日で絞込：終了日(yyyy-mm-dd)(条件に指定した時のみ含まれる）
    #[serde(rename = "end_date", skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    /// 勘定科目の表示（勘定科目: account_item, 決算書表示:group）(条件に指定した時のみ含まれる）
    #[serde(rename = "account_item_display_type", skip_serializing_if = "Option::is_none")]
    pub account_item_display_type: Option<AccountItemDisplayType>,
    /// 内訳の表示（取引先: partner, 品目: item, 部門: section, 勘定科目: account_item）(条件に指定した時のみ含まれる）
    #[serde(rename = "breakdown_display_type", skip_serializing_if = "Option::is_none")]
    pub breakdown_display_type: Option<BreakdownDisplayType>,
    /// 取引先ID(条件に指定した時のみ含まれる）
    #[serde(rename = "partner_id", skip_serializing_if = "Option::is_none")]
    pub partner_id: Option<i32>,
    /// 取引先コード(条件に指定した時のみ含まれる）
    #[serde(rename = "partner_code", skip_serializing_if = "Option::is_none")]
    pub partner_code: Option<String>,
    /// 品目ID(条件に指定した時のみ含まれる）
    #[serde(rename = "item_id", skip_serializing_if = "Option::is_none")]
    pub item_id: Option<i32>,
    /// 部門ID(条件に指定した時のみ含まれる）
    #[serde(rename = "section_id", skip_serializing_if = "Option::is_none")]
    pub section_id: Option<i32>,
    /// 決算整理仕訳のみ: only, 決算整理仕訳以外: without(条件に指定した時のみ含まれる）
    #[serde(rename = "adjustment", skip_serializing_if = "Option::is_none")]
    pub adjustment: Option<Adjustment>,
    /// 配賦仕訳のみ：only,配賦仕訳以外：without(条件に指定した時のみ含まれる）
    #[serde(rename = "cost_allocation", skip_serializing_if = "Option::is_none")]
    pub cost_allocation: Option<CostAllocation>,
    /// 未承認を除く: without_in_progress (デフォルト), 全てのステータス: all(条件に指定した時のみ含まれる）
    #[serde(rename = "approval_flow_status", skip_serializing_if = "Option::is_none")]
    pub approval_flow_status: Option<ApprovalFlowStatus>,
    /// 作成日時
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "balances")]
    pub balances: Vec<crate::models::TrialPlSegment1TagsResponseTrialPlSegment1TagsBalancesInner>,
}

impl TrialPlSegment1TagsResponseTrialPlSegment1Tags {
    pub fn new(company_id: i32, segment_1_tag_ids: String, balances: Vec<crate::models::TrialPlSegment1TagsResponseTrialPlSegment1TagsBalancesInner>) -> TrialPlSegment1TagsResponseTrialPlSegment1Tags {
        TrialPlSegment1TagsResponseTrialPlSegment1Tags {
            company_id,
            segment_1_tag_ids,
            fiscal_year: None,
            start_month: None,
            end_month: None,
            start_date: None,
            end_date: None,
            account_item_display_type: None,
            breakdown_display_type: None,
            partner_id: None,
            partner_code: None,
            item_id: None,
            section_id: None,
            adjustment: None,
            cost_allocation: None,
            approval_flow_status: None,
            created_at: None,
            balances,
        }
    }
}

/// 勘定科目の表示（勘定科目: account_item, 決算書表示:group）(条件に指定した時のみ含まれる）
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AccountItemDisplayType {
    #[serde(rename = "account_item")]
    AccountItem,
    #[serde(rename = "group")]
    Group,
}

impl Default for AccountItemDisplayType {
    fn default() -> AccountItemDisplayType {
        Self::AccountItem
    }
}
/// 内訳の表示（取引先: partner, 品目: item, 部門: section, 勘定科目: account_item）(条件に指定した時のみ含まれる）
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum BreakdownDisplayType {
    #[serde(rename = "partner")]
    Partner,
    #[serde(rename = "item")]
    Item,
    #[serde(rename = "section")]
    Section,
    #[serde(rename = "account_item")]
    AccountItem,
}

impl Default for BreakdownDisplayType {
    fn default() -> BreakdownDisplayType {
        Self::Partner
    }
}
/// 決算整理仕訳のみ: only, 決算整理仕訳以外: without(条件に指定した時のみ含まれる）
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Adjustment {
    #[serde(rename = "only")]
    Only,
    #[serde(rename = "without")]
    Without,
}

impl Default for Adjustment {
    fn default() -> Adjustment {
        Self::Only
    }
}
/// 配賦仕訳のみ：only,配賦仕訳以外：without(条件に指定した時のみ含まれる）
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CostAllocation {
    #[serde(rename = "only")]
    Only,
    #[serde(rename = "without")]
    Without,
}

impl Default for CostAllocation {
    fn default() -> CostAllocation {
        Self::Only
    }
}
/// 未承認を除く: without_in_progress (デフォルト), 全てのステータス: all(条件に指定した時のみ含まれる）
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ApprovalFlowStatus {
    #[serde(rename = "without_in_progress")]
    WithoutInProgress,
    #[serde(rename = "all")]
    All,
}

impl Default for ApprovalFlowStatus {
    fn default() -> ApprovalFlowStatus {
        Self::WithoutInProgress
    }
}

