/*
 * freee人事労務 API
 *
 *  <p>freee人事労務のAPI仕様です。</p>  <hr />  <h2 id=\"start_guide\">スタートガイド</h2>  <p>freee API開発がはじめての方は<a href=\"https://developer.freee.co.jp/getting-started\">freee API スタートガイド</a>を参照してください。</p>  <hr /> <h2 id=\"specification\">仕様</h2>  <h3 id=\"api_endpoint\">APIエンドポイント</h3>  <p>https://api.freee.co.jp/hr</p>  <h3 id=\"about_authorize\">認証について</h3>  <p>OAuth2.0を利用します。<a href=\"https://developer.freee.co.jp/reference/#%e8%aa%8d%e8%a8%bc\" target=\"_blank\">詳細はリファレンスの認証に関する記載を参照してください。</a></p>  <h3 id=\"data_format\">データフォーマット</h3>  <p>リクエスト、レスポンスともにJSON形式をサポートしていますが、詳細は、API毎の説明欄（application/jsonなど）を確認してください。</p>  <h3 id=\"compatibility\">後方互換性ありの変更</h3>  <p>freeeでは、APIを改善していくために以下のような変更は後方互換性ありとして通知なく変更を入れることがあります。アプリケーション実装者は以下を踏まえて開発を行ってください。</p>  <ul> <li>新しいAPIリソース・エンドポイントの追加</li> <li>既存のAPIに対して必須ではない新しいリクエストパラメータの追加</li> <li>既存のAPIレスポンスに対する新しいプロパティの追加</li> <li>既存のAPIレスポンスに対するプロパティの順番の入れ変え</li> <li>keyとなっているidやcodeの長さの変更（長くする）</li> <li>エラーメッセージの変更</li> </ul>  <h3 id=\"common_response_header\">共通レスポンスヘッダー</h3>  <p>すべてのAPIのレスポンスには以下のHTTPヘッダーが含まれます。</p>  <ul> <li> <p>X-Request-Id</p> <ul> <li>各リクエスト毎に発行されるID</li> </ul> </li> </ul>  <h3 id=\"error_response\">共通エラーレスポンス</h3>  <p>APIリクエストでエラーが発生した場合は、エラー原因に応じたステータスコードおよびメッセージを返します。</p>    <table border=\"1\">   <tbody>     <tr>       <th style=\"padding: 10px\"><strong>ステータスコード</strong></th>       <th style=\"padding: 10px\"><strong>原因</strong></th>     </tr>     <tr><td style=\"padding: 10px\">400</td><td style=\"padding: 10px\">リクエストパラメータが不正</td></tr>     <tr><td style=\"padding: 10px\">401</td><td style=\"padding: 10px\">アクセストークンが無効</td></tr>     <tr><td style=\"padding: 10px\">403</td><td style=\"padding: 10px\">アクセス権限がない</td></tr>     <tr><td style=\"padding: 10px\">404</td><td style=\"padding: 10px\">リソースが存在しない</td></tr>     <tr><td style=\"padding: 10px\">429</td><td style=\"padding: 10px\">リクエスト回数制限を超えた</td></tr>     <tr><td style=\"padding: 10px\">503</td><td style=\"padding: 10px\">システム内で予期しないエラーが発生</td></tr>   </tbody> </table>  <p>メッセージボディ内の <code>messages</code> にはエラー内容を説明する文字列が入ります。</p> <pre><code>  {     &quot;status_code&quot; : 400,     &quot;errors&quot; : [       {         &quot;type&quot; : &quot;bad_request&quot;,         &quot;messages&quot; : [           &quot;リクエストの形式が不正です。&quot;         ]       }     ]   }  </code></pre>  </br>  <h3 id=\"api_rate_limit\">API使用制限</h3>  <p>APIリクエストは1時間で5000回を上限としています。API使用ステータスはレスポンスヘッダに付与されます。</p> <pre><code>X-Ratelimit-Limit:5000 X-Ratelimit-Remaining:4998 X-Ratelimit-Reset:2018-01-01T12:00:00.000000Z </code></pre>  <br> 各ヘッダの意味は次のとおりです。</p>   <table border=\"1\">   <tbody>     <tr>       <th style=\"padding: 10px\"><strong>ヘッダ名</strong></th>       <th style=\"padding: 10px\"><strong>説明</strong></th>     </tr>     <tr><td style=\"padding: 10px\">X-RateLimit-Limit</td><td style=\"padding: 10px\">使用回数の上限</td></tr>     <tr><td style=\"padding: 10px\">X-RateLimit-Remaining</td><td style=\"padding: 10px\">残り使用回数</td></tr>     <tr><td style=\"padding: 10px\">X-RateLimit-Reset</td><td style=\"padding: 10px\">使用回数がリセットされる時刻</td></tr>   </tbody> </table>  <p>上記に加え、freeeは一定期間に過度のアクセスを検知した場合、APIアクセスをコントロールする場合があります。<br> その際のhttp status codeは403となります。制限がかかってから10分程度が過ぎると再度使用することができるようになります。</p>  </br>  <h3 id=\"api_rate_limit\">プランごとの利用可能API</h3>   <p>契約プランごとに利用可能なfreee人事労務APIはfreee人事労務のWeb版でご利用できる機能と同様です。例えば、スタンダードプラン（または旧ベーシックプラン）を契約している場合、Web版では打刻機能をご利用いただけますので、APIでもタイムレコーダー(打刻)APIが利用可能です。<a href=\"https://support.freee.co.jp/hc/ja/articles/203309710\" target=\"_blank\">freee人事労務のWeb版のプラン別機能比較はfreee人事労務のプラン・料金についてのヘルプを参照してください。</a></p>  </br>  <hr />
 *
 * The version of the OpenAPI document: 2022-02-01
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApiV1ApprovalFlowRouteResponse {
    #[serde(rename = "approval_flow_route")]
    pub approval_flow_route: Box<crate::models::ApiV1ApprovalFlowRouteResponseParams>,
}

impl ApiV1ApprovalFlowRouteResponse {
    pub fn new(approval_flow_route: crate::models::ApiV1ApprovalFlowRouteResponseParams) -> ApiV1ApprovalFlowRouteResponse {
        ApiV1ApprovalFlowRouteResponse {
            approval_flow_route: Box::new(approval_flow_route),
        }
    }
}


