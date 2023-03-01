# GetUsersCapabilities200Response

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**wallet_txns** | [**crate::models::UserCapabilityWithConfirm**](userCapabilityWithConfirm.md) |  | 
**deals** | [**crate::models::UserCapabilityWithSelfOnly**](userCapabilityWithSelfOnly.md) |  | 
**transfers** | [**crate::models::UserCapabilityWithSelfOnly**](userCapabilityWithSelfOnly.md) |  | 
**docs** | [**crate::models::UserCapabilityWithSelfOnly**](userCapabilityWithSelfOnly.md) |  | 
**doc_postings** | [**crate::models::UserCapabilityJustCreate**](userCapabilityJustCreate.md) |  | 
**receipts** | [**crate::models::UserCapabilityWithSelfOnly**](userCapabilityWithSelfOnly.md) |  | 
**receipt_stream_editor** | [**crate::models::UserCapabilityJustRead**](userCapabilityJustRead.md) |  | 
**spreadsheets** | [**crate::models::UserCapabilityJustCreateRead**](userCapabilityJustCreateRead.md) |  | 
**expense_applications** | [**crate::models::UserCapabilityWithSelfOnly**](userCapabilityWithSelfOnly.md) |  | 
**expense_application_sync_payroll** | [**crate::models::UserCapabilityJustCreate**](userCapabilityJustCreate.md) |  | 
**payment_requests** | [**crate::models::UserCapabilityWithSelfOnly**](userCapabilityWithSelfOnly.md) |  | 
**approval_requests** | [**crate::models::UserCapabilityWithSelfOnly**](userCapabilityWithSelfOnly.md) |  | 
**reports** | [**crate::models::UserCapabilityJustRead**](userCapabilityJustRead.md) |  | 
**reports_income_expense** | [**crate::models::UserCapabilityJustRead**](userCapabilityJustRead.md) |  | 
**reports_receivables** | [**crate::models::UserCapabilityJustRead**](userCapabilityJustRead.md) |  | 
**reports_payables** | [**crate::models::UserCapabilityJustReadWrite**](userCapabilityJustReadWrite.md) |  | 
**reports_cash_balance** | [**crate::models::UserCapabilityJustRead**](userCapabilityJustRead.md) |  | 
**reports_managements_planning** | [**crate::models::UserCapabilityJustReadWrite**](userCapabilityJustReadWrite.md) |  | 
**reports_managements_navigation** | [**crate::models::UserCapabilityJustReadWrite**](userCapabilityJustReadWrite.md) |  | 
**reports_crosstabs** | [**crate::models::UserCapabilityJustRead**](userCapabilityJustRead.md) |  | 
**reports_custom_reports_aggregate** | [**crate::models::UserCapabilityJustRead**](userCapabilityJustRead.md) |  | 
**reports_pl** | [**crate::models::UserCapabilityJustRead**](userCapabilityJustRead.md) |  | 
**reports_bs** | [**crate::models::UserCapabilityJustRead**](userCapabilityJustRead.md) |  | 
**reports_general_ledgers** | [**crate::models::UserCapabilityJustRead**](userCapabilityJustRead.md) |  | 
**reports_journals** | [**crate::models::UserCapabilityJustRead**](userCapabilityJustRead.md) |  | 
**manual_journals** | [**crate::models::UserCapabilityWithSelfOnly**](userCapabilityWithSelfOnly.md) |  | 
**fixed_assets** | [**crate::models::UserCapability**](userCapability.md) |  | 
**inventory_refreshes** | [**crate::models::UserCapability**](userCapability.md) |  | 
**biz_allocations** | [**crate::models::UserCapability**](userCapability.md) |  | 
**payment_records** | [**crate::models::UserCapability**](userCapability.md) |  | 
**annual_reports** | [**crate::models::UserCapabilityJustRead**](userCapabilityJustRead.md) |  | 
**tax_reports** | [**crate::models::UserCapabilityJustRead**](userCapabilityJustRead.md) |  | 
**consumption_entries** | [**crate::models::UserCapabilityJustRead**](userCapabilityJustRead.md) |  | 
**tax_return** | [**crate::models::UserCapabilityJustRead**](userCapabilityJustRead.md) |  | 
**account_item_statements** | [**crate::models::UserCapabilityJustRead**](userCapabilityJustRead.md) |  | 
**month_end** | [**crate::models::UserCapabilityJustRead**](userCapabilityJustRead.md) |  | 
**year_end** | [**crate::models::UserCapabilityJustReadUpdate**](userCapabilityJustReadUpdate.md) |  | 
**walletables** | [**crate::models::UserCapabilityWithSync**](userCapabilityWithSync.md) |  | 
**companies** | [**crate::models::UserCapabilityJustReadUpdate**](userCapabilityJustReadUpdate.md) |  | 
**invitations** | [**crate::models::UserCapability**](userCapability.md) |  | 
**access_controls** | [**crate::models::UserCapabilityWithWrite**](userCapabilityWithWrite.md) |  | 
**sign_in_logs** | [**crate::models::UserCapabilityJustRead**](userCapabilityJustRead.md) |  | 
**user_attribute_logs** | [**crate::models::UserCapabilityJustRead**](userCapabilityJustRead.md) |  | 
**app_role_logs** | [**crate::models::UserCapabilityJustRead**](userCapabilityJustRead.md) |  | 
**txn_relationship_logs** | [**crate::models::UserCapabilityJustRead**](userCapabilityJustRead.md) |  | 
**backups** | [**crate::models::UserCapabilityJustRead**](userCapabilityJustRead.md) |  | 
**opening_balances** | [**crate::models::UserCapabilityJustReadUpdate**](userCapabilityJustReadUpdate.md) |  | 
**system_conversion** | [**crate::models::UserCapabilityJustRead**](userCapabilityJustRead.md) |  | 
**resets** | [**crate::models::UserCapabilityJustRead**](userCapabilityJustRead.md) |  | 
**partners** | [**crate::models::UserCapability**](userCapability.md) |  | 
**items** | [**crate::models::UserCapability**](userCapability.md) |  | 
**sections** | [**crate::models::UserCapability**](userCapability.md) |  | 
**tags** | [**crate::models::UserCapability**](userCapability.md) |  | 
**account_items** | [**crate::models::UserCapability**](userCapability.md) |  | 
**taxes** | [**crate::models::UserCapabilityJustReadUpdate**](userCapabilityJustReadUpdate.md) |  | 
**payroll_item_sets** | [**crate::models::UserCapability**](userCapability.md) |  | 
**user_matchers** | [**crate::models::UserCapability**](userCapability.md) |  | 
**deal_templates** | [**crate::models::UserCapability**](userCapability.md) |  | 
**manual_journal_templates** | [**crate::models::UserCapability**](userCapability.md) |  | 
**cost_allocations** | [**crate::models::UserCapabilityJustReadUpdate**](userCapabilityJustReadUpdate.md) |  | 
**approval_flow_routes** | [**crate::models::UserCapability**](userCapability.md) |  | 
**expense_application_templates** | [**crate::models::UserCapability**](userCapability.md) |  | 
**request_forms** | [**crate::models::UserCapability**](userCapability.md) |  | 
**system_messages_for_admin** | [**crate::models::UserCapabilityJustRead**](userCapabilityJustRead.md) |  | 
**company_internal_announcements** | [**crate::models::UserCapabilityJustUpdate**](userCapabilityJustUpdate.md) |  | 
**doc_change_logs** | [**crate::models::UserCapabilityJustRead**](userCapabilityJustRead.md) |  | 
**workflows** | [**crate::models::UserCapabilityJustReadUpdateDestroy**](userCapabilityJustReadUpdateDestroy.md) |  | 
**oauth_applications** | [**crate::models::UserCapability**](userCapability.md) |  | 
**oauth_authorizations** | [**crate::models::UserCapability**](userCapability.md) |  | 
**bank_accountant_staff_users** | [**crate::models::UserCapability**](userCapability.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


