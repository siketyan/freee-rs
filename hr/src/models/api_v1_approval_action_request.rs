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
pub struct ApiV1ApprovalActionRequest {
    /// 事業所ID
    #[serde(rename = "company_id")]
    pub company_id: i32,
    /// 申請操作。（approve:承認、cancel:取り消し、feedback:差戻し、force_feedback:承認取り消し）
    #[serde(rename = "approval_action")]
    pub approval_action: ApprovalAction,
    /// 対象round。差戻し等により申請がstepの最初からやり直しになるとroundの値が増えます。取得APIレスポンス.current_roundを送信してください。
    #[serde(rename = "target_round")]
    pub target_round: Option<i32>,
    /// 対象承認ステップID。取得APIレスポンス.current_step_idを送信してください。
    #[serde(rename = "target_step_id")]
    pub target_step_id: Option<i32>,
    /// 次のステップの承認者のユーザーID
    #[serde(rename = "next_approver_id", skip_serializing_if = "Option::is_none")]
    pub next_approver_id: Option<i32>,
}

impl ApiV1ApprovalActionRequest {
    pub fn new(company_id: i32, approval_action: ApprovalAction, target_round: Option<i32>, target_step_id: Option<i32>) -> ApiV1ApprovalActionRequest {
        ApiV1ApprovalActionRequest {
            company_id,
            approval_action,
            target_round,
            target_step_id,
            next_approver_id: None,
        }
    }
}

/// 申請操作。（approve:承認、cancel:取り消し、feedback:差戻し、force_feedback:承認取り消し）
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ApprovalAction {
    #[serde(rename = "approve")]
    Approve,
    #[serde(rename = "cancel")]
    Cancel,
    #[serde(rename = "feedback")]
    Feedback,
    #[serde(rename = "force_feedback")]
    ForceFeedback,
}

impl Default for ApprovalAction {
    fn default() -> ApprovalAction {
        Self::Approve
    }
}

