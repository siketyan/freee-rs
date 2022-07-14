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
pub struct ApiV1EmployeesWorkRecordsControllerUpdateBody {
    /// 事業所ID（必須）
    #[serde(rename = "company_id")]
    pub company_id: i32,
    /// 休憩時間のリスト
    #[serde(rename = "break_records", skip_serializing_if = "Option::is_none")]
    pub break_records: Option<Vec<crate::models::ApiV1EmployeesWorkRecordTimeRangeSerializer>>,
    /// 出勤時刻
    #[serde(rename = "clock_in_at", skip_serializing_if = "Option::is_none")]
    pub clock_in_at: Option<String>,
    /// 退勤時刻
    #[serde(rename = "clock_out_at", skip_serializing_if = "Option::is_none")]
    pub clock_out_at: Option<String>,
    /// 勤務パターン（所定労働日: normal_day, 所定休日: prescribed_holiday, 法定休日: legal_holiday）  prescribed_holiday、legal_holidayを指定すると、以下のパラメータについて、指定した値が反映されず無視されます - early_leaving_mins - lateness_mins - paid_holiday
    #[serde(rename = "day_pattern", skip_serializing_if = "Option::is_none")]
    pub day_pattern: Option<DayPattern>,
    /// 早退分の時間（分単位）
    #[serde(rename = "early_leaving_mins", skip_serializing_if = "Option::is_none")]
    pub early_leaving_mins: Option<i32>,
    /// 欠勤かどうか
    #[serde(rename = "is_absence", skip_serializing_if = "Option::is_none")]
    pub is_absence: Option<bool>,
    /// 遅刻分の時間（分単位）
    #[serde(rename = "lateness_mins", skip_serializing_if = "Option::is_none")]
    pub lateness_mins: Option<i32>,
    /// 所定労働開始時刻。指定しない場合はデフォルト設定が使用されます。
    #[serde(rename = "normal_work_clock_in_at", skip_serializing_if = "Option::is_none")]
    pub normal_work_clock_in_at: Option<String>,
    /// 所定労働終了時刻。指定しない場合はデフォルト設定が使用されます。
    #[serde(rename = "normal_work_clock_out_at", skip_serializing_if = "Option::is_none")]
    pub normal_work_clock_out_at: Option<String>,
    /// 所定労働時間。指定しない場合はデフォルト設定が使用されます。
    #[serde(rename = "normal_work_mins", skip_serializing_if = "Option::is_none")]
    pub normal_work_mins: Option<i32>,
    /// 有給によって計上される所定労働時間（分）
    #[serde(rename = "normal_work_mins_by_paid_holiday", skip_serializing_if = "Option::is_none")]
    pub normal_work_mins_by_paid_holiday: Option<i32>,
    /// 勤怠メモ
    #[serde(rename = "note", skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    /// この日の有休取得数。0.5日単位で指定します。
    #[serde(rename = "paid_holiday", skip_serializing_if = "Option::is_none")]
    pub paid_holiday: Option<f32>,
    /// 欠勤・遅刻・早退を控除対象時間に算入するかどうか
    #[serde(rename = "use_attendance_deduction", skip_serializing_if = "Option::is_none")]
    pub use_attendance_deduction: Option<bool>,
    /// デフォルトの勤務設定を使うかどうか。  trueを指定した場合、以下のパラメータについて、指定した値に関係なく、従業員に設定した勤務賃金設定の休日の設定を参照して値が決まります - day_pattern  trueを指定した場合、以下のパラメータについて、指定した値に関係なく、従業員に設定した勤務賃金設定の出退勤時刻と労働時間の設定を参照して値が決まります。 - normal_work_clock_in_at - normal_work_clock_out_at - normal_work_mins
    #[serde(rename = "use_default_work_pattern", skip_serializing_if = "Option::is_none")]
    pub use_default_work_pattern: Option<bool>,
}

impl ApiV1EmployeesWorkRecordsControllerUpdateBody {
    pub fn new(company_id: i32) -> ApiV1EmployeesWorkRecordsControllerUpdateBody {
        ApiV1EmployeesWorkRecordsControllerUpdateBody {
            company_id,
            break_records: None,
            clock_in_at: None,
            clock_out_at: None,
            day_pattern: None,
            early_leaving_mins: None,
            is_absence: None,
            lateness_mins: None,
            normal_work_clock_in_at: None,
            normal_work_clock_out_at: None,
            normal_work_mins: None,
            normal_work_mins_by_paid_holiday: None,
            note: None,
            paid_holiday: None,
            use_attendance_deduction: None,
            use_default_work_pattern: None,
        }
    }
}

/// 勤務パターン（所定労働日: normal_day, 所定休日: prescribed_holiday, 法定休日: legal_holiday）  prescribed_holiday、legal_holidayを指定すると、以下のパラメータについて、指定した値が反映されず無視されます - early_leaving_mins - lateness_mins - paid_holiday
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DayPattern {
    #[serde(rename = "normal_day")]
    NormalDay,
    #[serde(rename = "prescribed_holiday")]
    PrescribedHoliday,
    #[serde(rename = "legal_holiday")]
    LegalHoliday,
}

impl Default for DayPattern {
    fn default() -> DayPattern {
        Self::NormalDay
    }
}

