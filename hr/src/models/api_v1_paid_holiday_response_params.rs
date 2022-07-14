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
pub struct ApiV1PaidHolidayResponseParams {
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
    /// 取得単位。（full:全休、half:半休、hour:時間休）
    #[serde(rename = "holiday_type")]
    pub holiday_type: HolidayType,
    /// 取得予定開始時間
    #[serde(rename = "start_at", skip_serializing_if = "Option::is_none")]
    pub start_at: Option<String>,
    /// 取得予定終了時間
    #[serde(rename = "end_at", skip_serializing_if = "Option::is_none")]
    pub end_at: Option<String>,
    /// 申請日
    #[serde(rename = "issue_date")]
    pub issue_date: String,
    /// 申請理由
    #[serde(rename = "comment", skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    /// 申請ステータス。（draft:下書き、in_progress:申請中、approved:承認済、feedback:差戻し）
    #[serde(rename = "status")]
    pub status: Status,
    /// 取消申請ステータス。（null:取消申請されてない、revoking:取消中、revoked:取消済）
    #[serde(rename = "revoke_status")]
    pub revoke_status: Option<RevokeStatus>,
    /// 自動チェック結果
    #[serde(rename = "passed_auto_check")]
    pub passed_auto_check: bool,
    /// 申請経路ID
    #[serde(rename = "approval_flow_route_id")]
    pub approval_flow_route_id: i32,
    /// 申請経路名
    #[serde(rename = "approval_flow_route_name")]
    pub approval_flow_route_name: String,
    /// 承認履歴
    #[serde(rename = "approval_flow_logs")]
    pub approval_flow_logs: Vec<crate::models::ApiV1ApprovalFlowLogsParams>,
    /// 現在承認ステップID<br> 申請を差戻した場合、nullになります。
    #[serde(rename = "current_step_id", skip_serializing_if = "Option::is_none")]
    pub current_step_id: Option<i32>,
    /// 現在のround。差戻し等により申請がstepの最初からやり直しになるとroundの値が増えます。
    #[serde(rename = "current_round")]
    pub current_round: i32,
}

impl ApiV1PaidHolidayResponseParams {
    pub fn new(id: i32, company_id: i32, application_number: i32, applicant_id: i32, target_date: String, holiday_type: HolidayType, issue_date: String, status: Status, revoke_status: Option<RevokeStatus>, passed_auto_check: bool, approval_flow_route_id: i32, approval_flow_route_name: String, approval_flow_logs: Vec<crate::models::ApiV1ApprovalFlowLogsParams>, current_round: i32) -> ApiV1PaidHolidayResponseParams {
        ApiV1PaidHolidayResponseParams {
            id,
            company_id,
            application_number,
            applicant_id,
            approver_ids: None,
            target_date,
            holiday_type,
            start_at: None,
            end_at: None,
            issue_date,
            comment: None,
            status,
            revoke_status,
            passed_auto_check,
            approval_flow_route_id,
            approval_flow_route_name,
            approval_flow_logs,
            current_step_id: None,
            current_round,
        }
    }
}

/// 取得単位。（full:全休、half:半休、hour:時間休）
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum HolidayType {
    #[serde(rename = "full")]
    Full,
    #[serde(rename = "half")]
    Half,
    #[serde(rename = "hour")]
    Hour,
}

impl Default for HolidayType {
    fn default() -> HolidayType {
        Self::Full
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
