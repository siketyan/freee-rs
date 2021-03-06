# FiscalYears

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**use_industry_template** | **bool** | 製造業向け機能（true: 使用する、false: 使用しない） | 
**indirect_write_off_method** | **bool** | 固定資産の控除法（true: 間接控除法、false: 直接控除法） | 
**start_date** | Option<**String**> | 期首日 | [optional]
**end_date** | Option<**String**> | 期末日 | [optional]
**depreciation_record_method** | **i32** | 月次償却（0: しない、1: する） | 
**tax_method** | **i32** | 課税区分（0: 免税、1: 簡易課税、2: 本則課税（個別対応方式）、3: 本則課税（一括比例配分方式）、4: 本則課税（全額控除）） | 
**sales_tax_business_code** | **i32** | 簡易課税用事業区分（0: 第一種：卸売業、1: 第二種：小売業、2: 第三種：農林水産業、工業、建設業、製造業など、3: 第四種：飲食店業など、4: 第五種：金融・保険業、運輸通信業、サービス業など、5: 第六種：不動産業など | 
**tax_fraction** | **i32** | 消費税端数処理方法（0: 切り捨て、1: 切り上げ、2: 四捨五入） | 
**tax_account_method** | **i32** | 消費税経理処理方法（0: 税込経理、1: 旧税抜経理、2: 税抜経理） | 
**return_code** | **i32** | 不動産所得使用区分（0: 一般、3: 一般/不動産） ※個人事業主のみ設定可能 | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


