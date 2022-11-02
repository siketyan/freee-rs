# ApiV1EmployeesHealthInsuranceRuleSerializer

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | 健康保険ルールID | [optional]
**company_id** | Option<**i32**> | 事業所ID | [optional]
**employee_id** | Option<**i32**> | 従業員ID | [optional]
**entried** | Option<**bool**> | 健康保険に加入しているかどうか | [optional]
**health_insurance_salary_calc_type** | Option<**String**> | 給与計算時の健康保険料の計算方法 | [optional]
**health_insurance_bonus_calc_type** | Option<**String**> | 賞与計算時の健康保険料の計算方法 | [optional]
**manual_health_insurance_amount_of_employee_salary** | Option<**i32**> | 給与計算時の健康保険料の直接指定金額（従業員負担分） | [optional]
**manual_health_insurance_amount_of_employee_bonus** | Option<**i32**> | 賞与計算時の健康保険料の直接指定金額（従業員負担分） | [optional]
**manual_health_insurance_amount_of_company_salary** | Option<**f32**> | 給与計算時の健康保険料の直接指定金額（会社負担分） | [optional]
**manual_health_insurance_amount_of_company_bonus** | Option<**f32**> | 賞与計算時の健康保険料の直接指定金額（会社負担分） | [optional]
**care_insurance_salary_calc_type** | Option<**String**> | 給与計算時の介護保険料の計算方法 | [optional]
**care_insurance_bonus_calc_type** | Option<**String**> | 賞与計算時の介護保険料の計算方法 | [optional]
**manual_care_insurance_amount_of_employee_salary** | Option<**i32**> | 給与計算時の介護保険料の直接指定金額（従業員負担分） | [optional]
**manual_care_insurance_amount_of_employee_bonus** | Option<**i32**> | 賞与計算時の介護保険料の直接指定金額（従業員負担分） | [optional]
**manual_care_insurance_amount_of_company_salary** | Option<**f32**> | 給与計算時の介護保険料の直接指定金額（会社負担分） | [optional]
**manual_care_insurance_amount_of_company_bonus** | Option<**f32**> | 賞与計算時の介護保険料の直接指定金額（会社負担分） | [optional]
**reference_num** | Option<**String**> | 健康保険の被保険者整理番号 | [optional]
**standard_monthly_remuneration** | Option<**i32**> | 標準報酬月額 | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


