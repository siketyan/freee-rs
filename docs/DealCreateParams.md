# DealCreateParams

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**issue_date** | **String** | 発生日 (yyyy-mm-dd) | 
**_type** | **String** | 収支区分 (収入: income, 支出: expense) | 
**company_id** | **i32** | 事業所ID | 
**due_date** | Option<**String**> | 支払期日(yyyy-mm-dd) | [optional]
**partner_id** | Option<**i32**> | 取引先ID | [optional]
**partner_code** | Option<**String**> | 取引先コード | [optional]
**ref_number** | Option<**String**> | 管理番号 | [optional]
**details** | [**Vec<crate::models::DealCreateParamsDetails>**](dealCreateParams_details.md) |  | 
**payments** | Option<[**Vec<crate::models::DealCreateParamsPayments>**](dealCreateParams_payments.md)> | 支払行一覧（配列）：未指定の場合、未決済の取引を作成します。 | [optional]
**receipt_ids** | Option<**Vec<i32>**> | 証憑ファイルID（ファイルボックスのファイルID）（配列） | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


