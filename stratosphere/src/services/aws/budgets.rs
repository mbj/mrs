pub mod budget {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-budgets-budget-autoadjustdata.html
    pub struct AutoAdjustData_ {
        pub auto_adjust_type: crate::value::ExpString,
        pub historical_options: Option<Box<HistoricalOptions_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_budgets_Budget_AutoAdjustData {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Budgets::Budget.AutoAdjustData"
            $($field $value)*)
        };
    }
    pub use crate::__aws_budgets_Budget_AutoAdjustData as AutoAdjustData;
    impl crate::value::ToValue for AutoAdjustData_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AutoAdjustType".to_string(),
                crate::value::ToValue::to_value(&self.auto_adjust_type),
            );
            if let Some(ref value) = self.historical_options {
                properties.insert(
                    "HistoricalOptions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-budgets-budget-budgetdata.html
    pub struct BudgetData_ {
        pub auto_adjust_data: Option<Box<AutoAdjustData_>>,
        pub billing_view_arn: Option<crate::value::ExpString>,
        pub budget_limit: Option<Box<Spend_>>,
        pub budget_name: Option<crate::value::ExpString>,
        pub budget_type: crate::value::ExpString,
        pub cost_filters: Option<serde_json::Value>,
        pub cost_types: Option<Box<CostTypes_>>,
        pub filter_expression: Option<Box<Expression_>>,
        pub metrics: Option<Vec<crate::value::ExpString>>,
        pub planned_budget_limits: Option<serde_json::Value>,
        pub time_period: Option<Box<TimePeriod_>>,
        pub time_unit: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_budgets_Budget_BudgetData {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Budgets::Budget.BudgetData"
            $($field $value)*)
        };
    }
    pub use crate::__aws_budgets_Budget_BudgetData as BudgetData;
    impl crate::value::ToValue for BudgetData_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.auto_adjust_data {
                properties.insert(
                    "AutoAdjustData".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.billing_view_arn {
                properties.insert(
                    "BillingViewArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.budget_limit {
                properties.insert(
                    "BudgetLimit".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.budget_name {
                properties.insert(
                    "BudgetName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "BudgetType".to_string(),
                crate::value::ToValue::to_value(&self.budget_type),
            );
            if let Some(ref value) = self.cost_filters {
                properties.insert(
                    "CostFilters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.cost_types {
                properties.insert(
                    "CostTypes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.filter_expression {
                properties.insert(
                    "FilterExpression".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.metrics {
                properties.insert(
                    "Metrics".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.planned_budget_limits {
                properties.insert(
                    "PlannedBudgetLimits".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.time_period {
                properties.insert(
                    "TimePeriod".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "TimeUnit".to_string(),
                crate::value::ToValue::to_value(&self.time_unit),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-budgets-budget-costcategoryvalues.html
    pub struct CostCategoryValues_ {
        pub key: Option<crate::value::ExpString>,
        pub match_options: Option<Vec<crate::value::ExpString>>,
        pub values: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_budgets_Budget_CostCategoryValues {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Budgets::Budget.CostCategoryValues"
            $($field $value)*)
        };
    }
    pub use crate::__aws_budgets_Budget_CostCategoryValues as CostCategoryValues;
    impl crate::value::ToValue for CostCategoryValues_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.key {
                properties.insert("Key".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.match_options {
                properties.insert(
                    "MatchOptions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.values {
                properties.insert("Values".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-budgets-budget-costtypes.html
    pub struct CostTypes_ {
        pub include_credit: Option<crate::value::ExpBool>,
        pub include_discount: Option<crate::value::ExpBool>,
        pub include_other_subscription: Option<crate::value::ExpBool>,
        pub include_recurring: Option<crate::value::ExpBool>,
        pub include_refund: Option<crate::value::ExpBool>,
        pub include_subscription: Option<crate::value::ExpBool>,
        pub include_support: Option<crate::value::ExpBool>,
        pub include_tax: Option<crate::value::ExpBool>,
        pub include_upfront: Option<crate::value::ExpBool>,
        pub use_amortized: Option<crate::value::ExpBool>,
        pub use_blended: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_budgets_Budget_CostTypes {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Budgets::Budget.CostTypes"
            $($field $value)*)
        };
    }
    pub use crate::__aws_budgets_Budget_CostTypes as CostTypes;
    impl crate::value::ToValue for CostTypes_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.include_credit {
                properties.insert(
                    "IncludeCredit".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.include_discount {
                properties.insert(
                    "IncludeDiscount".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.include_other_subscription {
                properties.insert(
                    "IncludeOtherSubscription".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.include_recurring {
                properties.insert(
                    "IncludeRecurring".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.include_refund {
                properties.insert(
                    "IncludeRefund".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.include_subscription {
                properties.insert(
                    "IncludeSubscription".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.include_support {
                properties.insert(
                    "IncludeSupport".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.include_tax {
                properties.insert(
                    "IncludeTax".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.include_upfront {
                properties.insert(
                    "IncludeUpfront".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.use_amortized {
                properties.insert(
                    "UseAmortized".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.use_blended {
                properties.insert(
                    "UseBlended".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-budgets-budget-expression.html
    pub struct Expression_ {
        pub and: Option<Vec<Expression_>>,
        pub cost_categories: Option<Box<CostCategoryValues_>>,
        pub dimensions: Option<Box<ExpressionDimensionValues_>>,
        pub not: Option<Box<Expression_>>,
        pub or: Option<Vec<Expression_>>,
        pub tags: Option<Box<TagValues_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_budgets_Budget_Expression {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Budgets::Budget.Expression"
            $($field $value)*)
        };
    }
    pub use crate::__aws_budgets_Budget_Expression as Expression;
    impl crate::value::ToValue for Expression_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.and {
                properties.insert("And".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.cost_categories {
                properties.insert(
                    "CostCategories".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.dimensions {
                properties.insert(
                    "Dimensions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.not {
                properties.insert("Not".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.or {
                properties.insert("Or".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.tags {
                properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-budgets-budget-expressiondimensionvalues.html
    pub struct ExpressionDimensionValues_ {
        pub key: Option<crate::value::ExpString>,
        pub match_options: Option<Vec<crate::value::ExpString>>,
        pub values: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_budgets_Budget_ExpressionDimensionValues {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Budgets::Budget.ExpressionDimensionValues"
            $($field $value)*)
        };
    }
    pub use crate::__aws_budgets_Budget_ExpressionDimensionValues as ExpressionDimensionValues;
    impl crate::value::ToValue for ExpressionDimensionValues_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.key {
                properties.insert("Key".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.match_options {
                properties.insert(
                    "MatchOptions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.values {
                properties.insert("Values".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-budgets-budget-historicaloptions.html
    pub struct HistoricalOptions_ {
        pub budget_adjustment_period: i32,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_budgets_Budget_HistoricalOptions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Budgets::Budget.HistoricalOptions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_budgets_Budget_HistoricalOptions as HistoricalOptions;
    impl crate::value::ToValue for HistoricalOptions_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "BudgetAdjustmentPeriod".to_string(),
                crate::value::ToValue::to_value(&self.budget_adjustment_period),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-budgets-budget-notification.html
    pub struct Notification_ {
        pub comparison_operator: crate::value::ExpString,
        pub notification_type: crate::value::ExpString,
        pub threshold: f64,
        pub threshold_type: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_budgets_Budget_Notification {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Budgets::Budget.Notification"
            $($field $value)*)
        };
    }
    pub use crate::__aws_budgets_Budget_Notification as Notification;
    impl crate::value::ToValue for Notification_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ComparisonOperator".to_string(),
                crate::value::ToValue::to_value(&self.comparison_operator),
            );
            properties.insert(
                "NotificationType".to_string(),
                crate::value::ToValue::to_value(&self.notification_type),
            );
            properties.insert(
                "Threshold".to_string(),
                crate::value::ToValue::to_value(&self.threshold),
            );
            if let Some(ref value) = self.threshold_type {
                properties.insert(
                    "ThresholdType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-budgets-budget-notificationwithsubscribers.html
    pub struct NotificationWithSubscribers_ {
        pub notification: Box<Notification_>,
        pub subscribers: Vec<Subscriber_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_budgets_Budget_NotificationWithSubscribers {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Budgets::Budget.NotificationWithSubscribers"
            $($field $value)*)
        };
    }
    pub use crate::__aws_budgets_Budget_NotificationWithSubscribers as NotificationWithSubscribers;
    impl crate::value::ToValue for NotificationWithSubscribers_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Notification".to_string(),
                crate::value::ToValue::to_value(&self.notification),
            );
            properties.insert(
                "Subscribers".to_string(),
                crate::value::ToValue::to_value(&self.subscribers),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-budgets-budget-resourcetag.html
    pub struct ResourceTag_ {
        pub key: crate::value::ExpString,
        pub value: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_budgets_Budget_ResourceTag {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Budgets::Budget.ResourceTag"
            $($field $value)*)
        };
    }
    pub use crate::__aws_budgets_Budget_ResourceTag as ResourceTag;
    impl crate::value::ToValue for ResourceTag_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Key".to_string(),
                crate::value::ToValue::to_value(&self.key),
            );
            if let Some(ref value) = self.value {
                properties.insert("Value".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-budgets-budget-spend.html
    pub struct Spend_ {
        pub amount: f64,
        pub unit: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_budgets_Budget_Spend {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Budgets::Budget.Spend"
            $($field $value)*)
        };
    }
    pub use crate::__aws_budgets_Budget_Spend as Spend;
    impl crate::value::ToValue for Spend_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Amount".to_string(),
                crate::value::ToValue::to_value(&self.amount),
            );
            properties.insert(
                "Unit".to_string(),
                crate::value::ToValue::to_value(&self.unit),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-budgets-budget-subscriber.html
    pub struct Subscriber_ {
        pub address: crate::value::ExpString,
        pub subscription_type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_budgets_Budget_Subscriber {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Budgets::Budget.Subscriber"
            $($field $value)*)
        };
    }
    pub use crate::__aws_budgets_Budget_Subscriber as Subscriber;
    impl crate::value::ToValue for Subscriber_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Address".to_string(),
                crate::value::ToValue::to_value(&self.address),
            );
            properties.insert(
                "SubscriptionType".to_string(),
                crate::value::ToValue::to_value(&self.subscription_type),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-budgets-budget-tagvalues.html
    pub struct TagValues_ {
        pub key: Option<crate::value::ExpString>,
        pub match_options: Option<Vec<crate::value::ExpString>>,
        pub values: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_budgets_Budget_TagValues {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Budgets::Budget.TagValues"
            $($field $value)*)
        };
    }
    pub use crate::__aws_budgets_Budget_TagValues as TagValues;
    impl crate::value::ToValue for TagValues_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.key {
                properties.insert("Key".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.match_options {
                properties.insert(
                    "MatchOptions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.values {
                properties.insert("Values".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-budgets-budget-timeperiod.html
    pub struct TimePeriod_ {
        pub end: Option<crate::value::ExpString>,
        pub start: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_budgets_Budget_TimePeriod {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Budgets::Budget.TimePeriod"
            $($field $value)*)
        };
    }
    pub use crate::__aws_budgets_Budget_TimePeriod as TimePeriod;
    impl crate::value::ToValue for TimePeriod_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.end {
                properties.insert("End".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.start {
                properties.insert("Start".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
}
pub mod budgetsaction {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-budgets-budgetsaction-actionthreshold.html
    pub struct ActionThreshold_ {
        pub r#type: crate::value::ExpString,
        pub value: f64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_budgets_BudgetsAction_ActionThreshold {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Budgets::BudgetsAction.ActionThreshold"
            $($field $value)*)
        };
    }
    pub use crate::__aws_budgets_BudgetsAction_ActionThreshold as ActionThreshold;
    impl crate::value::ToValue for ActionThreshold_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Type".to_string(),
                crate::value::ToValue::to_value(&self.r#type),
            );
            properties.insert(
                "Value".to_string(),
                crate::value::ToValue::to_value(&self.value),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-budgets-budgetsaction-definition.html
    pub struct Definition_ {
        pub iam_action_definition: Option<Box<IamActionDefinition_>>,
        pub scp_action_definition: Option<Box<ScpActionDefinition_>>,
        pub ssm_action_definition: Option<Box<SsmActionDefinition_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_budgets_BudgetsAction_Definition {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Budgets::BudgetsAction.Definition"
            $($field $value)*)
        };
    }
    pub use crate::__aws_budgets_BudgetsAction_Definition as Definition;
    impl crate::value::ToValue for Definition_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.iam_action_definition {
                properties.insert(
                    "IamActionDefinition".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.scp_action_definition {
                properties.insert(
                    "ScpActionDefinition".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ssm_action_definition {
                properties.insert(
                    "SsmActionDefinition".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-budgets-budgetsaction-iamactiondefinition.html
    pub struct IamActionDefinition_ {
        pub groups: Option<Vec<crate::value::ExpString>>,
        pub policy_arn: crate::value::ExpString,
        pub roles: Option<Vec<crate::value::ExpString>>,
        pub users: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_budgets_BudgetsAction_IamActionDefinition {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Budgets::BudgetsAction.IamActionDefinition"
            $($field $value)*)
        };
    }
    pub use crate::__aws_budgets_BudgetsAction_IamActionDefinition as IamActionDefinition;
    impl crate::value::ToValue for IamActionDefinition_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.groups {
                properties.insert("Groups".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "PolicyArn".to_string(),
                crate::value::ToValue::to_value(&self.policy_arn),
            );
            if let Some(ref value) = self.roles {
                properties.insert("Roles".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.users {
                properties.insert("Users".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-budgets-budgetsaction-resourcetag.html
    pub struct ResourceTag_ {
        pub key: crate::value::ExpString,
        pub value: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_budgets_BudgetsAction_ResourceTag {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Budgets::BudgetsAction.ResourceTag"
            $($field $value)*)
        };
    }
    pub use crate::__aws_budgets_BudgetsAction_ResourceTag as ResourceTag;
    impl crate::value::ToValue for ResourceTag_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Key".to_string(),
                crate::value::ToValue::to_value(&self.key),
            );
            properties.insert(
                "Value".to_string(),
                crate::value::ToValue::to_value(&self.value),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-budgets-budgetsaction-scpactiondefinition.html
    pub struct ScpActionDefinition_ {
        pub policy_id: crate::value::ExpString,
        pub target_ids: Vec<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_budgets_BudgetsAction_ScpActionDefinition {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Budgets::BudgetsAction.ScpActionDefinition"
            $($field $value)*)
        };
    }
    pub use crate::__aws_budgets_BudgetsAction_ScpActionDefinition as ScpActionDefinition;
    impl crate::value::ToValue for ScpActionDefinition_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "PolicyId".to_string(),
                crate::value::ToValue::to_value(&self.policy_id),
            );
            properties.insert(
                "TargetIds".to_string(),
                crate::value::ToValue::to_value(&self.target_ids),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-budgets-budgetsaction-ssmactiondefinition.html
    pub struct SsmActionDefinition_ {
        pub instance_ids: Vec<crate::value::ExpString>,
        pub region: crate::value::ExpString,
        pub subtype: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_budgets_BudgetsAction_SsmActionDefinition {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Budgets::BudgetsAction.SsmActionDefinition"
            $($field $value)*)
        };
    }
    pub use crate::__aws_budgets_BudgetsAction_SsmActionDefinition as SsmActionDefinition;
    impl crate::value::ToValue for SsmActionDefinition_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "InstanceIds".to_string(),
                crate::value::ToValue::to_value(&self.instance_ids),
            );
            properties.insert(
                "Region".to_string(),
                crate::value::ToValue::to_value(&self.region),
            );
            properties.insert(
                "Subtype".to_string(),
                crate::value::ToValue::to_value(&self.subtype),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-budgets-budgetsaction-subscriber.html
    pub struct Subscriber_ {
        pub address: crate::value::ExpString,
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_budgets_BudgetsAction_Subscriber {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Budgets::BudgetsAction.Subscriber"
            $($field $value)*)
        };
    }
    pub use crate::__aws_budgets_BudgetsAction_Subscriber as Subscriber;
    impl crate::value::ToValue for Subscriber_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Address".to_string(),
                crate::value::ToValue::to_value(&self.address),
            );
            properties.insert(
                "Type".to_string(),
                crate::value::ToValue::to_value(&self.r#type),
            );
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-budgets-budget.html
pub struct Budget_ {
    pub budget: super::budgets::budget::BudgetData_,
    pub notifications_with_subscribers:
        Option<Vec<super::budgets::budget::NotificationWithSubscribers_>>,
    pub resource_tags: Option<Vec<super::budgets::budget::ResourceTag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_budgets_Budget {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Budgets::Budget" $($field
        $value)*)
    };
}
pub use crate::__aws_budgets_Budget as Budget;
impl crate::template::ToResource for Budget_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Budgets"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Budget"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "Budget".to_string(),
            crate::value::ToValue::to_value(&self.budget),
        );
        if let Some(ref value) = self.notifications_with_subscribers {
            properties.insert(
                "NotificationsWithSubscribers".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.resource_tags {
            properties.insert(
                "ResourceTags".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-budgets-budgetsaction.html
pub struct BudgetsAction_ {
    pub action_threshold: super::budgets::budgetsaction::ActionThreshold_,
    pub action_type: crate::value::ExpString,
    pub approval_model: Option<crate::value::ExpString>,
    pub budget_name: crate::value::ExpString,
    pub definition: super::budgets::budgetsaction::Definition_,
    pub execution_role_arn: crate::value::ExpString,
    pub notification_type: crate::value::ExpString,
    pub resource_tags: Option<Vec<super::budgets::budgetsaction::ResourceTag_>>,
    pub subscribers: Vec<super::budgets::budgetsaction::Subscriber_>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_budgets_BudgetsAction {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Budgets::BudgetsAction"
        $($field $value)*)
    };
}
pub use crate::__aws_budgets_BudgetsAction as BudgetsAction;
impl crate::template::ToResource for BudgetsAction_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Budgets"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("BudgetsAction"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "ActionThreshold".to_string(),
            crate::value::ToValue::to_value(&self.action_threshold),
        );
        properties.insert(
            "ActionType".to_string(),
            crate::value::ToValue::to_value(&self.action_type),
        );
        if let Some(ref value) = self.approval_model {
            properties.insert(
                "ApprovalModel".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "BudgetName".to_string(),
            crate::value::ToValue::to_value(&self.budget_name),
        );
        properties.insert(
            "Definition".to_string(),
            crate::value::ToValue::to_value(&self.definition),
        );
        properties.insert(
            "ExecutionRoleArn".to_string(),
            crate::value::ToValue::to_value(&self.execution_role_arn),
        );
        properties.insert(
            "NotificationType".to_string(),
            crate::value::ToValue::to_value(&self.notification_type),
        );
        if let Some(ref value) = self.resource_tags {
            properties.insert(
                "ResourceTags".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Subscribers".to_string(),
            crate::value::ToValue::to_value(&self.subscribers),
        );
        properties
    }
}
