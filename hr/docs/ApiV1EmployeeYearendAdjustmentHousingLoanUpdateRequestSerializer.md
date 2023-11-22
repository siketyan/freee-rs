# ApiV1EmployeeYearendAdjustmentHousingLoanUpdateRequestSerializer

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**residence_start_date** | [**String**](string.md) | 居住開始の年月日 2000年1月1日から現在年+5の12月31日まで登録可能 | 
**remaining_balance_at_yearend** | **i32** | 住宅借入金等年末残高 | 
**category** | **String** | 住宅借入金等特別控除区分 general: 住: 一般の住宅借入金等, qualified: 認: 認定住宅の新築等, extension: 増: 特定増改築等, earthquake: 震: 震災特例法による特別控除 | 
**specific_case_type** | **String** | 特定取得/特別特定取得 not_qualified: 該当しない, specified: 特定取得, special_specified_or_special_exception: 特別特定取得または特別特例取得, exception_special_exception: 特例特別特例取得 | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


