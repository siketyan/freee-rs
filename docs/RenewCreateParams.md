# RenewCreateParams

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**company_id** | **i32** | 事業所ID | 
**update_date** | **String** | 更新日 (yyyy-mm-dd) | 
**renew_target_id** | **i64** | +更新対象行ID (details(取引の明細行), accruals(債権債務行), renewsのdetails(+更新の明細行)のIDを指定)  | 
**details** | [**Vec<crate::models::RenewCreateParamsDetails>**](renewCreateParams_details.md) | +更新の明細行 | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


