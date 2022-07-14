/*
 * freee人事労務 API
 *
 * freee人事労務のAPI仕様です。  ## [重要] freee人事労務 APIの新バージョンについて  このリファレンスはfreee人事労務 APIの新バージョンのリファレンスです。  2022年7月まで[旧バージョン](https://developer.freee.co.jp/docs/hr/pre-reference)と合わせて2つのバージョンが利用できる状態です。  新しいAPIを利用するにはリクエストヘッダーに以下を指定します。  ``` FREEE-VERSION: 2022-02-01 ```  指定がない場合には2022年7月に廃止予定のAPIを利用することになります。  詳細な変更やスケジュールは[【重要】freee人事労務APIの仕様変更について](https://developer.freee.co.jp/news/5418)をご覧ください。  ## 認証について  OAuth2.0を利用します。詳細は[ドキュメントの認証](https://developer.freee.co.jp/docs)パートを参照してください。  ## エンドポイント  https://api.freee.co.jp/hr  ## 後方互換性ありの変更  freeeでは、APIを改善していくために以下のような変更は後方互換性ありとして通知なく変更を入れることがあります。アプリケーション実装者は以下を踏まえて開発を行ってください。  - 新しいAPIリソース・エンドポイントの追加 - 既存のAPIに対して必須ではない新しいリクエストパラメータの追加 - 既存のAPIレスポンスに対する新しいプロパティの追加 - 既存のAPIレスポンスに対するプロパティの順番の入れ変え - keyとなっているidやcodeの長さの変更（長くする）  ## エラーレスポンス  APIリクエストでエラーが発生した場合は、エラー原因に応じたステータスコードおよびメッセージを返します。  |ステータスコード|原因| |---|---| |400|リクエストパラメータが不正| |401|アクセストークンが無効| |403|アクセス権限がない| |404|リソースが存在しない| |429|リクエスト回数制限を越えた| |503|システム内で予期しないエラーが発生|  メッセージボディ内の `messages` にはエラー内容を説明する文字列が入ります。  ``` {     \"status_code\": 400,     \"errors\": [         {             \"type\": \"bad_request\",             \"messages\": [                 \"リクエストの形式が不正です。\"             ]         }     ] } ```  ## API使用制限  APIリクエストは1時間で5000回を上限としています。API使用ステータスはレスポンスヘッダに付与されます。  ``` X-Ratelimit-Limit:5000 X-Ratelimit-Remaining:4998 X-Ratelimit-Reset:2018-01-01T12:00:00.000000Z ```  各ヘッダの意味は次のとおりです。  |ヘッダ名|説明| |---|---| |X-Ratelimit-Limit|使用回数の上限| |X-Ratelimit-Remaining|残り使用回数| |X-Ratelimit-Reset|使用回数がリセットされる時刻|  上記に加え、freeeは一定期間に過度のアクセスを検知した場合、APIアクセスをコントロールする場合があります。 その際のhttp status codeは403となります。制限がかかってから10分程度が過ぎると再度使用することができるようになります。  
 *
 * The version of the OpenAPI document: v2022-02-01
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
    #[serde(rename = "employee_num", skip_serializing_if = "Option::is_none")]
    pub employee_num: Option<String>,
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
    #[serde(rename = "calculated_at", skip_serializing_if = "Option::is_none")]
    pub calculated_at: Option<String>,
    /// 給与形態 monthly: 月給, daily: 日給, hourly: 時給, (空文字列): 計算中
    #[serde(rename = "pay_calc_type", skip_serializing_if = "Option::is_none")]
    pub pay_calc_type: Option<PayCalcType>,
    /// 基本給
    #[serde(rename = "basic_pay_amount", skip_serializing_if = "Option::is_none")]
    pub basic_pay_amount: Option<String>,
    /// 労働日数
    #[serde(rename = "work_days", skip_serializing_if = "Option::is_none")]
    pub work_days: Option<String>,
    /// 労働時間のうち、所定労働時間に該当するもの
    #[serde(rename = "normal_work_time", skip_serializing_if = "Option::is_none")]
    pub normal_work_time: Option<String>,
    /// 所定労働出勤日数
    #[serde(rename = "normal_work_days", skip_serializing_if = "Option::is_none")]
    pub normal_work_days: Option<String>,
    /// 有給休暇時間数
    #[serde(rename = "work_mins_by_paid_holiday", skip_serializing_if = "Option::is_none")]
    pub work_mins_by_paid_holiday: Option<String>,
    /// 有給日数
    #[serde(rename = "num_paid_holidays", skip_serializing_if = "Option::is_none")]
    pub num_paid_holidays: Option<String>,
    /// 役員かどうか
    #[serde(rename = "is_board_member", skip_serializing_if = "Option::is_none")]
    pub is_board_member: Option<bool>,
    /// 勤怠控除額合計
    #[serde(rename = "total_attendance_deduction_amount", skip_serializing_if = "Option::is_none")]
    pub total_attendance_deduction_amount: Option<String>,
    /// 支給手当額合計
    #[serde(rename = "total_allowance_amount", skip_serializing_if = "Option::is_none")]
    pub total_allowance_amount: Option<String>,
    /// 控除額合計
    #[serde(rename = "total_deduction_amount", skip_serializing_if = "Option::is_none")]
    pub total_deduction_amount: Option<String>,
    /// 差引支給額(手取り額)
    #[serde(rename = "net_payment_amount", skip_serializing_if = "Option::is_none")]
    pub net_payment_amount: Option<String>,
    /// 総支給額(額面)
    #[serde(rename = "gross_payment_amount", skip_serializing_if = "Option::is_none")]
    pub gross_payment_amount: Option<String>,
    /// 平日と休日の合計労働日数（日給用）
    #[serde(rename = "total_worked_days_count", skip_serializing_if = "Option::is_none")]
    pub total_worked_days_count: Option<String>,
    /// 課税対象支給額
    #[serde(rename = "total_taxable_payment_amount", skip_serializing_if = "Option::is_none")]
    pub total_taxable_payment_amount: Option<String>,
    /// 総経費精算額
    #[serde(rename = "total_expense_amount", skip_serializing_if = "Option::is_none")]
    pub total_expense_amount: Option<String>,
    /// 総振込額
    #[serde(rename = "total_transfer_amount", skip_serializing_if = "Option::is_none")]
    pub total_transfer_amount: Option<String>,
    /// 課税支給累計額
    #[serde(rename = "total_annual_payment_amount", skip_serializing_if = "Option::is_none")]
    pub total_annual_payment_amount: Option<String>,
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
    #[serde(rename = "overtime_pays", skip_serializing_if = "Option::is_none")]
    pub overtime_pays: Option<Vec<crate::models::ApiV1EmployeePayrollStatementsEmployeeOvertimePayItemSerializer>>,
    /// 備考
    #[serde(rename = "remark", skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,
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
