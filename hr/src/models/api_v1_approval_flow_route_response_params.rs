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
pub struct ApiV1ApprovalFlowRouteResponseParams {
    /// 申請経路ID
    #[serde(rename = "id")]
    pub id: i32,
    /// 申請経路名
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 申請経路の説明
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 更新したユーザーのユーザーID
    #[serde(rename = "user_id", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i32>,
    /// システム作成の申請経路かどうか
    #[serde(rename = "definition_system", skip_serializing_if = "Option::is_none")]
    pub definition_system: Option<bool>,
    /// 最初の承認ステップのID
    #[serde(rename = "first_step_id", skip_serializing_if = "Option::is_none")]
    pub first_step_id: Option<i32>,
    /// 申請種別（申請経路を使用できる申請種別を示します。例えば、AttendanceWorkflow の場合は、勤怠申請で使用できる申請経路です。） - AttendanceWorkflow - 勤怠申請 - PersonalDataWorkflow - 身上変更申請
    #[serde(rename = "usages", skip_serializing_if = "Option::is_none")]
    pub usages: Option<Vec<Usages>>,
    /// 承認ステップ（配列）
    #[serde(rename = "steps", skip_serializing_if = "Option::is_none")]
    pub steps: Option<Vec<crate::models::ApiV1FlowRouteStepSrializerInner>>,
}

impl ApiV1ApprovalFlowRouteResponseParams {
    pub fn new(id: i32) -> ApiV1ApprovalFlowRouteResponseParams {
        ApiV1ApprovalFlowRouteResponseParams {
            id,
            name: None,
            description: None,
            user_id: None,
            definition_system: None,
            first_step_id: None,
            usages: None,
            steps: None,
        }
    }
}

/// 申請種別（申請経路を使用できる申請種別を示します。例えば、AttendanceWorkflow の場合は、勤怠申請で使用できる申請経路です。） - AttendanceWorkflow - 勤怠申請 - PersonalDataWorkflow - 身上変更申請
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Usages {
    #[serde(rename = "AttendanceWorkflow")]
    AttendanceWorkflow,
    #[serde(rename = "PersonalDataWorkflow")]
    PersonalDataWorkflow,
}

impl Default for Usages {
    fn default() -> Usages {
        Self::AttendanceWorkflow
    }
}

