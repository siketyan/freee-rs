/*
 * freee人事労務 API
 *
 * freee人事労務のAPI仕様です。  ## 認証について  OAuth2.0を利用します。詳細は[ドキュメントの認証](https://developer.freee.co.jp/docs)パートを参照してください。  ## エンドポイント  https://api.freee.co.jp/hr  ## 後方互換性ありの変更  freeeでは、APIを改善していくために以下のような変更は後方互換性ありとして通知なく変更を入れることがあります。アプリケーション実装者は以下を踏まえて開発を行ってください。  - 新しいAPIリソース・エンドポイントの追加 - 既存のAPIに対して必須ではない新しいリクエストパラメータの追加 - 既存のAPIレスポンスに対する新しいプロパティの追加 - 既存のAPIレスポンスに対するプロパティの順番の入れ変え - keyとなっているidやcodeの長さの変更（長くする）  ## エラーレスポンス  APIリクエストでエラーが発生した場合は、エラー原因に応じたステータスコードおよびメッセージを返します。  |ステータスコード|原因| |---|---| |400|リクエストパラメータが不正| |401|アクセストークンが無効| |403|アクセス権限がない| |404|リソースが存在しない| |429|リクエスト回数制限を越えた| |503|システム内で予期しないエラーが発生|  メッセージボディ内の `messages` にはエラー内容を説明する文字列が入ります。  ``` {     \"status_code\": 400,     \"errors\": [         {             \"type\": \"bad_request\",             \"messages\": [                 \"リクエストの形式が不正です。\"             ]         }     ] } ```  ## API使用制限  APIリクエストは1時間で5000回を上限としています。API使用ステータスはレスポンスヘッダに付与されます。  ``` X-Ratelimit-Limit:5000 X-Ratelimit-Remaining:4998 X-Ratelimit-Reset:2018-01-01T12:00:00.000000Z ```  各ヘッダの意味は次のとおりです。  |ヘッダ名|説明| |---|---| |X-Ratelimit-Limit|使用回数の上限| |X-Ratelimit-Remaining|残り使用回数| |X-Ratelimit-Reset|使用回数がリセットされる時刻|  上記に加え、freeeは一定期間に過度のアクセスを検知した場合、APIアクセスをコントロールする場合があります。 その際のhttp status codeは403となります。制限がかかってから10分程度が過ぎると再度使用することができるようになります。  
 *
 * The version of the OpenAPI document: v2022-02-01
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ApiV1BonusesEmployeePayrollStatementSerializer {
    /// 賞与明細ID
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
    /// 確定日
    #[serde(rename = "closing_date", skip_serializing_if = "Option::is_none")]
    pub closing_date: Option<String>,
    /// 支払日
    #[serde(rename = "pay_date", skip_serializing_if = "Option::is_none")]
    pub pay_date: Option<String>,
    /// 賞与明細が確定されているかどうか
    #[serde(rename = "fixed", skip_serializing_if = "Option::is_none")]
    pub fixed: Option<bool>,
    /// 計算状況ステータス calculating: 計算中, calculated: 計算完了, error: エラー
    #[serde(rename = "calc_status", skip_serializing_if = "Option::is_none")]
    pub calc_status: Option<String>,
    /// 計算状況ステータスの更新日
    #[serde(rename = "calculated_at", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub calculated_at: Option<Option<String>>,
    /// 賞与額
    #[serde(rename = "bonus_amount", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub bonus_amount: Option<Option<String>>,
    /// 手当額合計
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
    /// 課税対象支給額
    #[serde(rename = "total_taxable_payment_amount", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub total_taxable_payment_amount: Option<Option<String>>,
    /// 手当
    #[serde(rename = "allowances", skip_serializing_if = "Option::is_none")]
    pub allowances: Option<Vec<crate::models::ApiV1EmployeePayrollStatementsEmployeePayrollStatementItemSerializer>>,
    /// 控除項目（所得税、社会保険料等）
    #[serde(rename = "deductions", skip_serializing_if = "Option::is_none")]
    pub deductions: Option<Vec<crate::models::ApiV1EmployeePayrollStatementsEmployeePayrollStatementItemSerializer>>,
    /// 備考
    #[serde(rename = "remark", skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,
}

impl ApiV1BonusesEmployeePayrollStatementSerializer {
    pub fn new() -> ApiV1BonusesEmployeePayrollStatementSerializer {
        ApiV1BonusesEmployeePayrollStatementSerializer {
            id: None,
            company_id: None,
            employee_id: None,
            employee_name: None,
            employee_display_name: None,
            employee_num: None,
            closing_date: None,
            pay_date: None,
            fixed: None,
            calc_status: None,
            calculated_at: None,
            bonus_amount: None,
            total_allowance_amount: None,
            total_deduction_amount: None,
            net_payment_amount: None,
            gross_payment_amount: None,
            total_taxable_payment_amount: None,
            allowances: None,
            deductions: None,
            remark: None,
        }
    }
}


