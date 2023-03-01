/*
 * freee人事労務 API
 *
 *  <p>freee人事労務のAPI仕様です。</p>  <hr />  <h2 id=\"start_guide\">スタートガイド</h2>  <p>freee API開発がはじめての方は<a href=\"https://developer.freee.co.jp/getting-started\">freee API スタートガイド</a>を参照してください。</p>  <hr /> <h2 id=\"specification\">仕様</h2>  <h3 id=\"api_endpoint\">APIエンドポイント</h3>  <p>https://api.freee.co.jp/hr</p>  <h3 id=\"about_authorize\">認証について</h3>  <p>OAuth2.0を利用します。<a href=\"https://developer.freee.co.jp/reference/#%e8%aa%8d%e8%a8%bc\" target=\"_blank\">詳細はリファレンスの認証に関する記載を参照してください。</a></p>  <h3 id=\"data_format\">データフォーマット</h3>  <p>リクエスト、レスポンスともにJSON形式をサポートしていますが、詳細は、API毎の説明欄（application/jsonなど）を確認してください。</p>  <h3 id=\"compatibility\">後方互換性ありの変更</h3>  <p>freeeでは、APIを改善していくために以下のような変更は後方互換性ありとして通知なく変更を入れることがあります。アプリケーション実装者は以下を踏まえて開発を行ってください。</p>  <ul> <li>新しいAPIリソース・エンドポイントの追加</li> <li>既存のAPIに対して必須ではない新しいリクエストパラメータの追加</li> <li>既存のAPIレスポンスに対する新しいプロパティの追加</li> <li>既存のAPIレスポンスに対するプロパティの順番の入れ変え</li> <li>keyとなっているidやcodeの長さの変更（長くする）</li> </ul>  <h3 id=\"common_response_header\">共通レスポンスヘッダー</h3>  <p>すべてのAPIのレスポンスには以下のHTTPヘッダーが含まれます。</p>  <ul> <li> <p>X-Request-Id</p> <ul> <li>各リクエスト毎に発行されるID</li> </ul> </li> </ul>  <h3 id=\"error_response\">共通エラーレスポンス</h3>  <p>APIリクエストでエラーが発生した場合は、エラー原因に応じたステータスコードおよびメッセージを返します。</p>    <table border=\"1\">   <tbody>     <tr>       <th style=\"padding: 10px\"><strong>ステータスコード</strong></th>       <th style=\"padding: 10px\"><strong>原因</strong></th>     </tr>     <tr><td style=\"padding: 10px\">400</td><td style=\"padding: 10px\">リクエストパラメータが不正</td></tr>     <tr><td style=\"padding: 10px\">401</td><td style=\"padding: 10px\">アクセストークンが無効</td></tr>     <tr><td style=\"padding: 10px\">403</td><td style=\"padding: 10px\">アクセス権限がない</td></tr>     <tr><td style=\"padding: 10px\">404</td><td style=\"padding: 10px\">リソースが存在しない</td></tr>     <tr><td style=\"padding: 10px\">429</td><td style=\"padding: 10px\">リクエスト回数制限を超えた</td></tr>     <tr><td style=\"padding: 10px\">503</td><td style=\"padding: 10px\">システム内で予期しないエラーが発生</td></tr>   </tbody> </table>  <p>メッセージボディ内の <code>messages</code> にはエラー内容を説明する文字列が入ります。</p> <pre><code>  {     &quot;status_code&quot; : 400,     &quot;errors&quot; : [       {         &quot;type&quot; : &quot;bad_request&quot;,         &quot;messages&quot; : [           &quot;リクエストの形式が不正です。&quot;         ]       }     ]   }  </code></pre>  </br>  <h3 id=\"api_rate_limit\">API使用制限</h3>  <p>APIリクエストは1時間で5000回を上限としています。API使用ステータスはレスポンスヘッダに付与されます。</p> <pre><code>X-Ratelimit-Limit:5000 X-Ratelimit-Remaining:4998 X-Ratelimit-Reset:2018-01-01T12:00:00.000000Z </code></pre>  <br> 各ヘッダの意味は次のとおりです。</p>   <table border=\"1\">   <tbody>     <tr>       <th style=\"padding: 10px\"><strong>ヘッダ名</strong></th>       <th style=\"padding: 10px\"><strong>説明</strong></th>     </tr>     <tr><td style=\"padding: 10px\">X-RateLimit-Limit</td><td style=\"padding: 10px\">使用回数の上限</td></tr>     <tr><td style=\"padding: 10px\">X-RateLimit-Remaining</td><td style=\"padding: 10px\">残り使用回数</td></tr>     <tr><td style=\"padding: 10px\">X-RateLimit-Reset</td><td style=\"padding: 10px\">使用回数がリセットされる時刻</td></tr>   </tbody> </table>  <p>上記に加え、freeeは一定期間に過度のアクセスを検知した場合、APIアクセスをコントロールする場合があります。<br> その際のhttp status codeは403となります。制限がかかってから10分程度が過ぎると再度使用することができるようになります。</p>
 *
 * The version of the OpenAPI document: 2022-02-01
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ApiV1EmployeeIndexSerializer {
    /// 従業員ID
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    /// 事業所ID
    #[serde(rename = "company_id", skip_serializing_if = "Option::is_none")]
    pub company_id: Option<i32>,
    /// 従業員番号
    #[serde(rename = "num", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub num: Option<Option<String>>,
    /// 従業員名（表示名）
    #[serde(rename = "display_name", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// 基礎年金番号
    #[serde(rename = "base_pension_num", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub base_pension_num: Option<Option<String>>,
    /// 被保険者番号（雇用保険）
    #[serde(rename = "employment_insurance_reference_number", skip_serializing_if = "Option::is_none")]
    pub employment_insurance_reference_number: Option<String>,
    /// 生年月日
    #[serde(rename = "birth_date", skip_serializing_if = "Option::is_none")]
    pub birth_date: Option<String>,
    /// 入社日
    #[serde(rename = "entry_date", skip_serializing_if = "Option::is_none")]
    pub entry_date: Option<String>,
    /// 退職日
    #[serde(rename = "retire_date", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub retire_date: Option<Option<String>>,
    /// ユーザーID(従業員詳細未設定の場合、nullになります。)
    #[serde(rename = "user_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<Option<i32>>,
    #[serde(rename = "profile_rule", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub profile_rule: Option<Option<Box<crate::models::ApiV1EmployeesProfileRuleSerializer>>>,
    #[serde(rename = "health_insurance_rule", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub health_insurance_rule: Option<Option<Box<crate::models::ApiV1EmployeesHealthInsuranceRuleSerializer>>>,
    #[serde(rename = "welfare_pension_insurance_rule", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub welfare_pension_insurance_rule: Option<Option<Box<crate::models::ApiV1EmployeesWelfarePensionInsuranceRuleSerializer>>>,
    /// 扶養親族
    #[serde(rename = "dependent_rules", skip_serializing_if = "Option::is_none")]
    pub dependent_rules: Option<Vec<crate::models::ApiV1EmployeesDependentRuleSerializer>>,
    #[serde(rename = "bank_account_rule", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub bank_account_rule: Option<Option<Box<crate::models::ApiV1EmployeesBankAccountRuleSerializer>>>,
    #[serde(rename = "basic_pay_rule", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub basic_pay_rule: Option<Option<Box<crate::models::ApiV1EmployeesBasicPayRuleSerializer>>>,
    /// 給与計算対象従業員の場合trueを返します
    #[serde(rename = "payroll_calculation", skip_serializing_if = "Option::is_none")]
    pub payroll_calculation: Option<bool>,
    /// 締め日支払日グループ名(給与計算対象外従業員の場合、nullを返します)
    #[serde(rename = "company_reference_date_rule_name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub company_reference_date_rule_name: Option<Option<String>>,
    /// カスタム項目のグループ
    #[serde(rename = "profile_custom_field_groups", skip_serializing_if = "Option::is_none")]
    pub profile_custom_field_groups: Option<Vec<crate::models::ApiV1ProfileCustomFieldGroupSerializer>>,
}

impl ApiV1EmployeeIndexSerializer {
    pub fn new() -> ApiV1EmployeeIndexSerializer {
        ApiV1EmployeeIndexSerializer {
            id: None,
            company_id: None,
            num: None,
            display_name: None,
            base_pension_num: None,
            employment_insurance_reference_number: None,
            birth_date: None,
            entry_date: None,
            retire_date: None,
            user_id: None,
            profile_rule: None,
            health_insurance_rule: None,
            welfare_pension_insurance_rule: None,
            dependent_rules: None,
            bank_account_rule: None,
            basic_pay_rule: None,
            payroll_calculation: None,
            company_reference_date_rule_name: None,
            profile_custom_field_groups: None,
        }
    }
}


