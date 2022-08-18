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
pub struct ApiV1EmployeeGroupMembershipSerializer {
    /// 従業員ID
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    /// 従業員番号
    #[serde(rename = "num", skip_serializing_if = "Option::is_none")]
    pub num: Option<String>,
    /// 従業員名（表示名）
    #[serde(rename = "display_name", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// 入社日
    #[serde(rename = "entry_date", skip_serializing_if = "Option::is_none")]
    pub entry_date: Option<String>,
    /// 退職日
    #[serde(rename = "retire_date", skip_serializing_if = "Option::is_none")]
    pub retire_date: Option<String>,
    /// ユーザーID(従業員詳細未設定の場合、nullになります。)
    #[serde(rename = "user_id", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i32>,
    /// ログイン用メールアドレス(従業員詳細未設定の場合、nullになります。)
    #[serde(rename = "login_email", skip_serializing_if = "Option::is_none")]
    pub login_email: Option<String>,
    /// 生年月日
    #[serde(rename = "birth_date", skip_serializing_if = "Option::is_none")]
    pub birth_date: Option<String>,
    /// 性別　unselected: 未選択, male: 男性, female: 女性
    #[serde(rename = "gender", skip_serializing_if = "Option::is_none")]
    pub gender: Option<Gender>,
    /// 給与計算対象従業員の場合trueを返します
    #[serde(rename = "payroll_calculation", skip_serializing_if = "Option::is_none")]
    pub payroll_calculation: Option<bool>,
    #[serde(rename = "group_memberships", skip_serializing_if = "Option::is_none")]
    pub group_memberships: Option<Vec<crate::models::ApiV1GroupMembershipSerializer>>,
}

impl ApiV1EmployeeGroupMembershipSerializer {
    pub fn new() -> ApiV1EmployeeGroupMembershipSerializer {
        ApiV1EmployeeGroupMembershipSerializer {
            id: None,
            num: None,
            display_name: None,
            entry_date: None,
            retire_date: None,
            user_id: None,
            login_email: None,
            birth_date: None,
            gender: None,
            payroll_calculation: None,
            group_memberships: None,
        }
    }
}

/// 性別　unselected: 未選択, male: 男性, female: 女性
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Gender {
    #[serde(rename = "unselected")]
    Unselected,
    #[serde(rename = "male")]
    Male,
    #[serde(rename = "female")]
    Female,
}

impl Default for Gender {
    fn default() -> Gender {
        Self::Unselected
    }
}

