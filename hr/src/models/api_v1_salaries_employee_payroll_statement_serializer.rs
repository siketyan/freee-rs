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
pub struct ApiV1SalariesEmployeePayrollStatementSerializer {
    /// 給与明細ID
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    /// 事業所ID
    #[serde(rename = "company_id", skip_serializing_if = "Option::is_none")]
    pub company_id: Option<i32>,
    /// 従業員ID
    #[serde(rename = "employee_id", skip_serializing_if = "Option::is_none")]
    pub employee_id: Option<i32>,
    /// 従業員の姓名
    #[serde(rename = "employee_name", skip_serializing_if = "Option::is_none")]
    pub employee_name: Option<String>,
    /// 従業員の表示名
    #[serde(rename = "employee_display_name", skip_serializing_if = "Option::is_none")]
    pub employee_display_name: Option<String>,
    /// 従業員番号
    #[serde(rename = "employee_num", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub employee_num: Option<Option<String>>,
    /// 支払日
    #[serde(rename = "pay_date", skip_serializing_if = "Option::is_none")]
    pub pay_date: Option<String>,
    /// 給与計算開始日（固定給）
    #[serde(rename = "start_date", skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    /// 給与計算締日（固定給）
    #[serde(rename = "closing_date", skip_serializing_if = "Option::is_none")]
    pub closing_date: Option<String>,
    /// 給与計算開始日（変動給） 残業手当、遅刻早退・欠勤などの計算に使われる期間
    #[serde(rename = "variable_pay_start_date", skip_serializing_if = "Option::is_none")]
    pub variable_pay_start_date: Option<String>,
    /// 給与計算締日（変動給）
    #[serde(rename = "variable_pay_closing_date", skip_serializing_if = "Option::is_none")]
    pub variable_pay_closing_date: Option<String>,
    /// 給与明細が確定されているかどうか
    #[serde(rename = "fixed", skip_serializing_if = "Option::is_none")]
    pub fixed: Option<bool>,
    /// 計算状況ステータス calculating: 計算中, calculated: 計算完了, overwritten: 直接編集, imported: インポート, error: エラー
    #[serde(rename = "calc_status", skip_serializing_if = "Option::is_none")]
    pub calc_status: Option<String>,
    /// 計算状況ステータスの更新日
    #[serde(rename = "calculated_at", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub calculated_at: Option<Option<String>>,
    /// 給与形態 monthly: 月給, daily: 日給, hourly: 時給, (空文字列): 計算中
    #[serde(rename = "pay_calc_type", skip_serializing_if = "Option::is_none")]
    pub pay_calc_type: Option<PayCalcType>,
    /// 基本給
    #[serde(rename = "basic_pay_amount", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub basic_pay_amount: Option<Option<String>>,
    /// 労働日数
    #[serde(rename = "work_days", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub work_days: Option<Option<String>>,
    /// 労働時間のうち、所定労働時間に該当するもの
    #[serde(rename = "normal_work_time", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub normal_work_time: Option<Option<String>>,
    /// 所定労働出勤日数
    #[serde(rename = "normal_work_days", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub normal_work_days: Option<Option<String>>,
    /// 有給休暇時間数
    #[serde(rename = "work_mins_by_paid_holiday", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub work_mins_by_paid_holiday: Option<Option<String>>,
    /// 有給日数
    #[serde(rename = "num_paid_holidays", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub num_paid_holidays: Option<Option<String>>,
    /// 役員かどうか
    #[serde(rename = "is_board_member", skip_serializing_if = "Option::is_none")]
    pub is_board_member: Option<bool>,
    /// 勤怠控除額合計
    #[serde(rename = "total_attendance_deduction_amount", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub total_attendance_deduction_amount: Option<Option<String>>,
    /// 支給手当額合計
    #[serde(rename = "total_allowance_amount", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub total_allowance_amount: Option<Option<String>>,
    /// 控除額合計
    #[serde(rename = "total_deduction_amount", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub total_deduction_amount: Option<Option<String>>,
    /// 差引支給額(手取り額)
    #[serde(rename = "net_payment_amount", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub net_payment_amount: Option<Option<String>>,
    /// 総支給額(額面)
    #[serde(rename = "gross_payment_amount", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub gross_payment_amount: Option<Option<String>>,
    /// 平日と休日の合計労働日数（日給用）
    #[serde(rename = "total_worked_days_count", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub total_worked_days_count: Option<Option<String>>,
    /// 課税対象支給額
    #[serde(rename = "total_taxable_payment_amount", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub total_taxable_payment_amount: Option<Option<String>>,
    /// 総経費精算額
    #[serde(rename = "total_expense_amount", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub total_expense_amount: Option<Option<String>>,
    /// 総振込額
    #[serde(rename = "total_transfer_amount", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub total_transfer_amount: Option<Option<String>>,
    /// 課税支給累計額
    #[serde(rename = "total_annual_payment_amount", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub total_annual_payment_amount: Option<Option<String>>,
    /// 支給項目（基本給、残業代、通勤手当等）
    #[serde(rename = "payments", skip_serializing_if = "Option::is_none")]
    pub payments: Option<Vec<crate::models::ApiV1EmployeePayrollStatementsEmployeePayrollStatementItemSerializer>>,
    /// 控除項目（所得税、住民税、社会保険料等）
    #[serde(rename = "deductions", skip_serializing_if = "Option::is_none")]
    pub deductions: Option<Vec<crate::models::ApiV1EmployeePayrollStatementsEmployeePayrollStatementItemSerializer>>,
    /// 勤怠控除項目（遅刻早退控除、欠勤控除等）
    #[serde(rename = "attendances", skip_serializing_if = "Option::is_none")]
    pub attendances: Option<Vec<crate::models::ApiV1EmployeePayrollStatementsEmployeeAttendanceItemSerializer>>,
    /// 時間外労働項目(法定内残業、時間外労働、休日労働、深夜労働等)
    #[serde(rename = "overtime_pays", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub overtime_pays: Option<Option<Vec<crate::models::ApiV1EmployeePayrollStatementsEmployeeOvertimePayItemSerializer>>>,
    /// 備考
    #[serde(rename = "remark", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub remark: Option<Option<String>>,
}

impl ApiV1SalariesEmployeePayrollStatementSerializer {
    pub fn new() -> ApiV1SalariesEmployeePayrollStatementSerializer {
        ApiV1SalariesEmployeePayrollStatementSerializer {
            id: None,
            company_id: None,
            employee_id: None,
            employee_name: None,
            employee_display_name: None,
            employee_num: None,
            pay_date: None,
            start_date: None,
            closing_date: None,
            variable_pay_start_date: None,
            variable_pay_closing_date: None,
            fixed: None,
            calc_status: None,
            calculated_at: None,
            pay_calc_type: None,
            basic_pay_amount: None,
            work_days: None,
            normal_work_time: None,
            normal_work_days: None,
            work_mins_by_paid_holiday: None,
            num_paid_holidays: None,
            is_board_member: None,
            total_attendance_deduction_amount: None,
            total_allowance_amount: None,
            total_deduction_amount: None,
            net_payment_amount: None,
            gross_payment_amount: None,
            total_worked_days_count: None,
            total_taxable_payment_amount: None,
            total_expense_amount: None,
            total_transfer_amount: None,
            total_annual_payment_amount: None,
            payments: None,
            deductions: None,
            attendances: None,
            overtime_pays: None,
            remark: None,
        }
    }
}

/// 給与形態 monthly: 月給, daily: 日給, hourly: 時給, (空文字列): 計算中
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PayCalcType {
    #[serde(rename = "monthly")]
    Monthly,
    #[serde(rename = "daily")]
    Daily,
    #[serde(rename = "hourly")]
    Hourly,
    #[serde(rename = "")]
    Empty,
}

impl Default for PayCalcType {
    fn default() -> PayCalcType {
        Self::Monthly
    }
}

