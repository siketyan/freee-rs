# DealCreateResponseDeal

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** | 取引ID | 
**company_id** | **i32** | 事業所ID | 
**issue_date** | **String** | 発生日 (yyyy-mm-dd) | 
**due_date** | Option<**String**> | 支払期日 (yyyy-mm-dd) | [optional]
**amount** | **i64** | 金額 | 
**due_amount** | Option<**i32**> | 支払残額 | [optional]
**r#type** | Option<**String**> | 収支区分 (収入: income, 支出: expense) | [optional]
**partner_id** | **i32** | 取引先ID | 
**partner_code** | Option<**String**> | 取引先コード | [optional]
**ref_number** | Option<**String**> | 管理番号 | [optional]
**status** | **String** | 決済状況 (未決済: unsettled, 完了: settled) | 
**details** | Option<[**Vec<crate::models::DealCreateResponseDealDetailsInner>**](dealCreateResponse_deal_details_inner.md)> | 取引の明細行 | [optional]
**payments** | Option<[**Vec<crate::models::DealCreateResponseDealPaymentsInner>**](dealCreateResponse_deal_payments_inner.md)> | 取引の支払行 | [optional]
**receipts** | Option<[**Vec<crate::models::DealCreateResponseDealReceiptsInner>**](dealCreateResponse_deal_receipts_inner.md)> | ファイルボックス（証憑ファイル） | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


