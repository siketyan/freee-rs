# ExpenseApplicationCreateParams

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**company_id** | **i32** | 事業所ID | 
**title** | **String** | 申請タイトル (250文字以内) | 
**issue_date** | Option<**String**> | 申請日 (yyyy-mm-dd)<br> 指定しない場合は当日の日付が登録されます。  | [optional]
**description** | Option<**String**> | 備考 (10000文字以内) | [optional]
**section_id** | Option<**i32**> | 部門ID | [optional]
**tag_ids** | Option<**Vec<i32>**> | メモタグID | [optional]
**expense_application_lines** | [**Vec<crate::models::ExpenseApplicationCreateParamsExpenseApplicationLines>**](expenseApplicationCreateParams_expense_application_lines.md) |  | 
**approval_flow_route_id** | Option<**i32**> | 申請経路ID<br> <ul>     <li>経費申請のステータスを申請中として作成する場合は、必ず指定してください。</li>     <li>指定する申請経路IDは、申請経路APIを利用して取得してください。</li>     <li>         未指定の場合は、基本経路を設定している事業所では基本経路が、基本経路を設定していない事業所では利用可能な申請経路の中から最初の申請経路が自動的に使用されます。         <ul>            <li>意図しない申請経路を持った経費申請の作成を防ぐために、使用する申請経路IDを指定することを推奨します。</li>         </ul>     </li>     <li>         ベーシックプランの事業所では以下のデフォルトで用意された申請経路のみ指定できます         <ul>         <li>指定なし</li>         <li>承認者を指定</li>         </ul>     </li> </ul>  | [optional]
**approver_id** | Option<**i32**> | 承認者のユーザーID<br> 「承認者を指定」の経路を申請経路として使用する場合に指定してください。<br> 指定する承認者のユーザーIDは、申請経路APIを利用して取得してください。  | [optional]
**draft** | Option<**bool**> | 経費申請のステータス<br> falseを指定した時は申請中（in_progress）で経費申請を作成します。<br> trueを指定した時は下書き（draft）で経費申請を作成します。<br> 未指定の時は下書きとみなして経費申請を作成します。  | [optional]
**parent_id** | Option<**i32**> | 親申請ID(法人向けプロフェッショナル, 法人向け エンタープライズプラン)<br> <ul>   <li>承認済みの既存各種申請IDのみ指定可能です。</li>   <li>各種申請一覧APIを利用して取得してください。</li> </ul>  | [optional]
**segment_1_tag_id** | Option<**i64**> | セグメント１ID(法人向けプロフェッショナル, 法人向けエンタープライズプラン)<br> セグメントタグ一覧APIを利用して取得してください。<br> <a href=\"https://support.freee.co.jp/hc/ja/articles/360020679611\" target=\"_blank\">セグメント（分析用タグ）の設定</a><br>  | [optional]
**segment_2_tag_id** | Option<**i64**> | セグメント２ID(法人向け エンタープライズプラン)<br> セグメントタグ一覧APIを利用して取得してください。<br> <a href=\"https://support.freee.co.jp/hc/ja/articles/360020679611\" target=\"_blank\">セグメント（分析用タグ）の設定</a><br>  | [optional]
**segment_3_tag_id** | Option<**i64**> | セグメント３ID(法人向け エンタープライズプラン)<br> セグメントタグ一覧APIを利用して取得してください。<br> <a href=\"https://support.freee.co.jp/hc/ja/articles/360020679611\" target=\"_blank\">セグメント（分析用タグ）の設定</a><br>  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


