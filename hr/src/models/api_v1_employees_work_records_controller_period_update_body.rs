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
pub struct ApiV1EmployeesWorkRecordsControllerPeriodUpdateBody {
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
    /// 勤務パターン（所定労働日: normal_day, 所定休日: prescribed_holiday, 法定休日: legal_holiday）  prescribed_holiday、legal_holidayを指定すると、以下のパラメータについて、指定した値が反映されず無視されます。 - early_leaving_mins - lateness_mins - paid_holiday
    #[serde(rename = "day_pattern", skip_serializing_if = "Option::is_none")]
    pub day_pattern: Option<DayPattern>,
    /// 早退分の時間（分単位）
    #[serde(rename = "early_leaving_mins", skip_serializing_if = "Option::is_none")]
    pub early_leaving_mins: Option<i32>,
    /// 欠勤かどうか  is_absenceにtrueを指定すると、以下のパラーメータについて、指定した値が反映されず無視されます。 - break_records   - clock_in_at   - clock_out_at - clock_in_at - clock_out_at - early_leaving_mins - lateness_mins - normal_work_clock_in_at - normal_work_clock_out_at - normal_work_mins - normal_work_mins_by_paid_holiday - paid_holiday
    #[serde(rename = "is_absence", skip_serializing_if = "Option::is_none")]
    pub is_absence: Option<bool>,
    /// 遅刻分の時間（分単位）
    #[serde(rename = "lateness_mins", skip_serializing_if = "Option::is_none")]
    pub lateness_mins: Option<i32>,
    /// 所定労働開始時刻。指定しない場合はデフォルト設定が使用されます。（デフォルト設定は従業員に設定した勤務賃金設定の出退勤時刻と労働時間の設定を参照して値が決まります。）
    #[serde(rename = "normal_work_clock_in_at", skip_serializing_if = "Option::is_none")]
    pub normal_work_clock_in_at: Option<String>,
    /// 所定労働終了時刻。指定しない場合はデフォルト設定が使用されます。（デフォルト設定は従業員に設定した勤務賃金設定の出退勤時刻と労働時間の設定を参照して値が決まります。）
    #[serde(rename = "normal_work_clock_out_at", skip_serializing_if = "Option::is_none")]
    pub normal_work_clock_out_at: Option<String>,
    /// 所定労働時間。指定しない場合はデフォルト設定が使用されます。（デフォルト設定は従業員に設定した勤務賃金設定の出退勤時刻と労働時間の設定を参照して値が決まります。）
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

impl ApiV1EmployeesWorkRecordsControllerPeriodUpdateBody {
    pub fn new(company_id: i32) -> ApiV1EmployeesWorkRecordsControllerPeriodUpdateBody {
        ApiV1EmployeesWorkRecordsControllerPeriodUpdateBody {
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

/// 勤務パターン（所定労働日: normal_day, 所定休日: prescribed_holiday, 法定休日: legal_holiday）  prescribed_holiday、legal_holidayを指定すると、以下のパラメータについて、指定した値が反映されず無視されます。 - early_leaving_mins - lateness_mins - paid_holiday
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

