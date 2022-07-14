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
pub struct ApiV1EmployeesBankAccountRuleUpdateRequestSerializer {
    /// 金融機関名
    #[serde(rename = "bank_name", skip_serializing_if = "Option::is_none")]
    pub bank_name: Option<String>,
    /// 金融機関名カナ 英字カナのみ
    #[serde(rename = "bank_name_kana", skip_serializing_if = "Option::is_none")]
    pub bank_name_kana: Option<String>,
    /// 金融機関コード 数値文字列4桁
    #[serde(rename = "bank_code", skip_serializing_if = "Option::is_none")]
    pub bank_code: Option<String>,
    /// 支店名
    #[serde(rename = "branch_name", skip_serializing_if = "Option::is_none")]
    pub branch_name: Option<String>,
    /// 支店名カナ　英字カナのみ
    #[serde(rename = "branch_name_kana", skip_serializing_if = "Option::is_none")]
    pub branch_name_kana: Option<String>,
    /// 支店コード 数値文字列3桁
    #[serde(rename = "branch_code", skip_serializing_if = "Option::is_none")]
    pub branch_code: Option<String>,
    /// 口座番号 数値文字列7桁
    #[serde(rename = "account_number", skip_serializing_if = "Option::is_none")]
    pub account_number: Option<String>,
    /// 口座名義カナ　英字カナのみ
    #[serde(rename = "account_name", skip_serializing_if = "Option::is_none")]
    pub account_name: Option<String>,
    /// 預金種類 ordinary: 普通預金, current: 当座預金, saving: 貯蓄預金
    #[serde(rename = "account_type", skip_serializing_if = "Option::is_none")]
    pub account_type: Option<AccountType>,
}

impl ApiV1EmployeesBankAccountRuleUpdateRequestSerializer {
    pub fn new() -> ApiV1EmployeesBankAccountRuleUpdateRequestSerializer {
        ApiV1EmployeesBankAccountRuleUpdateRequestSerializer {
            bank_name: None,
            bank_name_kana: None,
            bank_code: None,
            branch_name: None,
            branch_name_kana: None,
            branch_code: None,
            account_number: None,
            account_name: None,
            account_type: None,
        }
    }
}

/// 預金種類 ordinary: 普通預金, current: 当座預金, saving: 貯蓄預金
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AccountType {
    #[serde(rename = "ordinary")]
    Ordinary,
    #[serde(rename = "current")]
    Current,
    #[serde(rename = "saving")]
    Saving,
}

impl Default for AccountType {
    fn default() -> AccountType {
        Self::Ordinary
    }
}

