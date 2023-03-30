// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>An entitlement represents capacity in a product owned by the customer. For example, a customer might own some number of users or seats in an SaaS application or some amount of data capacity in a multi-tenant database.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct Entitlement  {
    /// <p>The product code for which the given entitlement applies. Product codes are provided by AWS Marketplace when the product listing is created.</p>
    #[doc(hidden)]
    pub product_code: std::option::Option<std::string::String>,
    /// <p>The dimension for which the given entitlement applies. Dimensions represent categories of capacity in a product and are specified when the product is listed in AWS Marketplace.</p>
    #[doc(hidden)]
    pub dimension: std::option::Option<std::string::String>,
    /// <p>The customer identifier is a handle to each unique customer in an application. Customer identifiers are obtained through the ResolveCustomer operation in AWS Marketplace Metering Service.</p>
    #[doc(hidden)]
    pub customer_identifier: std::option::Option<std::string::String>,
    /// <p>The EntitlementValue represents the amount of capacity that the customer is entitled to for the product.</p>
    #[doc(hidden)]
    pub value: std::option::Option<crate::model::EntitlementValue>,
    /// <p>The expiration date represents the minimum date through which this entitlement is expected to remain valid. For contractual products listed on AWS Marketplace, the expiration date is the date at which the customer will renew or cancel their contract. Customers who are opting to renew their contract will still have entitlements with an expiration date.</p>
    #[doc(hidden)]
    pub expiration_date: std::option::Option<aws_smithy_types::DateTime>,
}
impl Entitlement {
    /// <p>The product code for which the given entitlement applies. Product codes are provided by AWS Marketplace when the product listing is created.</p>
    pub fn product_code(&self) -> std::option::Option<& str> {
        self.product_code.as_deref()
    }
    /// <p>The dimension for which the given entitlement applies. Dimensions represent categories of capacity in a product and are specified when the product is listed in AWS Marketplace.</p>
    pub fn dimension(&self) -> std::option::Option<& str> {
        self.dimension.as_deref()
    }
    /// <p>The customer identifier is a handle to each unique customer in an application. Customer identifiers are obtained through the ResolveCustomer operation in AWS Marketplace Metering Service.</p>
    pub fn customer_identifier(&self) -> std::option::Option<& str> {
        self.customer_identifier.as_deref()
    }
    /// <p>The EntitlementValue represents the amount of capacity that the customer is entitled to for the product.</p>
    pub fn value(&self) -> std::option::Option<& crate::model::EntitlementValue> {
        self.value.as_ref()
    }
    /// <p>The expiration date represents the minimum date through which this entitlement is expected to remain valid. For contractual products listed on AWS Marketplace, the expiration date is the date at which the customer will renew or cancel their contract. Customers who are opting to renew their contract will still have entitlements with an expiration date.</p>
    pub fn expiration_date(&self) -> std::option::Option<& aws_smithy_types::DateTime> {
        self.expiration_date.as_ref()
    }
}
impl Entitlement {
    /// Creates a new builder-style object to manufacture [`Entitlement`](crate::model::Entitlement).
    pub fn builder() -> crate::model::entitlement::Builder {
        crate::model::entitlement::Builder::default()
    }
}

/// See [`Entitlement`](crate::model::Entitlement).
pub mod entitlement {
    
    /// A builder for [`Entitlement`](crate::model::Entitlement).
    #[non_exhaustive]
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) product_code: std::option::Option<std::string::String>,
        pub(crate) dimension: std::option::Option<std::string::String>,
        pub(crate) customer_identifier: std::option::Option<std::string::String>,
        pub(crate) value: std::option::Option<crate::model::EntitlementValue>,
        pub(crate) expiration_date: std::option::Option<aws_smithy_types::DateTime>,
    }
    impl Builder {
        /// <p>The product code for which the given entitlement applies. Product codes are provided by AWS Marketplace when the product listing is created.</p>
        pub fn product_code(mut self, input: impl Into<std::string::String>) -> Self {
            self.product_code = Some(input.into());
            self
        }
        /// <p>The product code for which the given entitlement applies. Product codes are provided by AWS Marketplace when the product listing is created.</p>
        pub fn set_product_code(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.product_code = input; self
        }
        /// <p>The dimension for which the given entitlement applies. Dimensions represent categories of capacity in a product and are specified when the product is listed in AWS Marketplace.</p>
        pub fn dimension(mut self, input: impl Into<std::string::String>) -> Self {
            self.dimension = Some(input.into());
            self
        }
        /// <p>The dimension for which the given entitlement applies. Dimensions represent categories of capacity in a product and are specified when the product is listed in AWS Marketplace.</p>
        pub fn set_dimension(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.dimension = input; self
        }
        /// <p>The customer identifier is a handle to each unique customer in an application. Customer identifiers are obtained through the ResolveCustomer operation in AWS Marketplace Metering Service.</p>
        pub fn customer_identifier(mut self, input: impl Into<std::string::String>) -> Self {
            self.customer_identifier = Some(input.into());
            self
        }
        /// <p>The customer identifier is a handle to each unique customer in an application. Customer identifiers are obtained through the ResolveCustomer operation in AWS Marketplace Metering Service.</p>
        pub fn set_customer_identifier(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.customer_identifier = input; self
        }
        /// <p>The EntitlementValue represents the amount of capacity that the customer is entitled to for the product.</p>
        pub fn value(mut self, input: crate::model::EntitlementValue) -> Self {
            self.value = Some(input);
            self
        }
        /// <p>The EntitlementValue represents the amount of capacity that the customer is entitled to for the product.</p>
        pub fn set_value(mut self, input: std::option::Option<crate::model::EntitlementValue>) -> Self {
            self.value = input; self
        }
        /// <p>The expiration date represents the minimum date through which this entitlement is expected to remain valid. For contractual products listed on AWS Marketplace, the expiration date is the date at which the customer will renew or cancel their contract. Customers who are opting to renew their contract will still have entitlements with an expiration date.</p>
        pub fn expiration_date(mut self, input: aws_smithy_types::DateTime) -> Self {
            self.expiration_date = Some(input);
            self
        }
        /// <p>The expiration date represents the minimum date through which this entitlement is expected to remain valid. For contractual products listed on AWS Marketplace, the expiration date is the date at which the customer will renew or cancel their contract. Customers who are opting to renew their contract will still have entitlements with an expiration date.</p>
        pub fn set_expiration_date(mut self, input: std::option::Option<aws_smithy_types::DateTime>) -> Self {
            self.expiration_date = input; self
        }
        /// Consumes the builder and constructs a [`Entitlement`](crate::model::Entitlement).
        pub fn build(self) -> crate::model::Entitlement {
            crate::model::Entitlement {
                product_code: self.product_code
                ,
                dimension: self.dimension
                ,
                customer_identifier: self.customer_identifier
                ,
                value: self.value
                ,
                expiration_date: self.expiration_date
                ,
            }
        }
    }
    
    
}

/// <p>The EntitlementValue represents the amount of capacity that the customer is entitled to for the product.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct EntitlementValue  {
    /// <p>The IntegerValue field will be populated with an integer value when the entitlement is an integer type. Otherwise, the field will not be set.</p>
    #[doc(hidden)]
    pub integer_value: std::option::Option<i32>,
    /// <p>The DoubleValue field will be populated with a double value when the entitlement is a double type. Otherwise, the field will not be set.</p>
    #[doc(hidden)]
    pub double_value: std::option::Option<f64>,
    /// <p>The BooleanValue field will be populated with a boolean value when the entitlement is a boolean type. Otherwise, the field will not be set.</p>
    #[doc(hidden)]
    pub boolean_value: std::option::Option<bool>,
    /// <p>The StringValue field will be populated with a string value when the entitlement is a string type. Otherwise, the field will not be set.</p>
    #[doc(hidden)]
    pub string_value: std::option::Option<std::string::String>,
}
impl EntitlementValue {
    /// <p>The IntegerValue field will be populated with an integer value when the entitlement is an integer type. Otherwise, the field will not be set.</p>
    pub fn integer_value(&self) -> std::option::Option<i32> {
        self.integer_value
    }
    /// <p>The DoubleValue field will be populated with a double value when the entitlement is a double type. Otherwise, the field will not be set.</p>
    pub fn double_value(&self) -> std::option::Option<f64> {
        self.double_value
    }
    /// <p>The BooleanValue field will be populated with a boolean value when the entitlement is a boolean type. Otherwise, the field will not be set.</p>
    pub fn boolean_value(&self) -> std::option::Option<bool> {
        self.boolean_value
    }
    /// <p>The StringValue field will be populated with a string value when the entitlement is a string type. Otherwise, the field will not be set.</p>
    pub fn string_value(&self) -> std::option::Option<& str> {
        self.string_value.as_deref()
    }
}
impl EntitlementValue {
    /// Creates a new builder-style object to manufacture [`EntitlementValue`](crate::model::EntitlementValue).
    pub fn builder() -> crate::model::entitlement_value::Builder {
        crate::model::entitlement_value::Builder::default()
    }
}

/// See [`EntitlementValue`](crate::model::EntitlementValue).
pub mod entitlement_value {
    
    /// A builder for [`EntitlementValue`](crate::model::EntitlementValue).
    #[non_exhaustive]
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) integer_value: std::option::Option<i32>,
        pub(crate) double_value: std::option::Option<f64>,
        pub(crate) boolean_value: std::option::Option<bool>,
        pub(crate) string_value: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The IntegerValue field will be populated with an integer value when the entitlement is an integer type. Otherwise, the field will not be set.</p>
        pub fn integer_value(mut self, input: i32) -> Self {
            self.integer_value = Some(input);
            self
        }
        /// <p>The IntegerValue field will be populated with an integer value when the entitlement is an integer type. Otherwise, the field will not be set.</p>
        pub fn set_integer_value(mut self, input: std::option::Option<i32>) -> Self {
            self.integer_value = input; self
        }
        /// <p>The DoubleValue field will be populated with a double value when the entitlement is a double type. Otherwise, the field will not be set.</p>
        pub fn double_value(mut self, input: f64) -> Self {
            self.double_value = Some(input);
            self
        }
        /// <p>The DoubleValue field will be populated with a double value when the entitlement is a double type. Otherwise, the field will not be set.</p>
        pub fn set_double_value(mut self, input: std::option::Option<f64>) -> Self {
            self.double_value = input; self
        }
        /// <p>The BooleanValue field will be populated with a boolean value when the entitlement is a boolean type. Otherwise, the field will not be set.</p>
        pub fn boolean_value(mut self, input: bool) -> Self {
            self.boolean_value = Some(input);
            self
        }
        /// <p>The BooleanValue field will be populated with a boolean value when the entitlement is a boolean type. Otherwise, the field will not be set.</p>
        pub fn set_boolean_value(mut self, input: std::option::Option<bool>) -> Self {
            self.boolean_value = input; self
        }
        /// <p>The StringValue field will be populated with a string value when the entitlement is a string type. Otherwise, the field will not be set.</p>
        pub fn string_value(mut self, input: impl Into<std::string::String>) -> Self {
            self.string_value = Some(input.into());
            self
        }
        /// <p>The StringValue field will be populated with a string value when the entitlement is a string type. Otherwise, the field will not be set.</p>
        pub fn set_string_value(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.string_value = input; self
        }
        /// Consumes the builder and constructs a [`EntitlementValue`](crate::model::EntitlementValue).
        pub fn build(self) -> crate::model::EntitlementValue {
            crate::model::EntitlementValue {
                integer_value: self.integer_value
                ,
                double_value: self.double_value
                ,
                boolean_value: self.boolean_value
                ,
                string_value: self.string_value
                ,
            }
        }
    }
    
    
}

/// When writing a match expression against `GetEntitlementFilterName`, it is important to ensure
/// your code is forward-compatible. That is, if a match arm handles a case for a
/// feature that is supported by the service but has not been represented as an enum
/// variant in a current version of SDK, your code should continue to work when you
/// upgrade SDK to a future version in which the enum does include a variant for that
/// feature.
/// 
/// Here is an example of how you can make a match expression forward-compatible:
/// 
/// ```text
/// # let getentitlementfiltername = unimplemented!();
/// match getentitlementfiltername {
///     GetEntitlementFilterName::CustomerIdentifier => { /* ... */ },
///     GetEntitlementFilterName::Dimension => { /* ... */ },
///     other @ _ if other.as_str() == "NewFeature" => { /* handles a case for `NewFeature` */ },
///     _ => { /* ... */ },
/// }
/// ```
/// The above code demonstrates that when `getentitlementfiltername` represents
/// `NewFeature`, the execution path will lead to the second last match arm,
/// even though the enum does not contain a variant `GetEntitlementFilterName::NewFeature`
/// in the current version of SDK. The reason is that the variable `other`,
/// created by the `@` operator, is bound to
/// `GetEntitlementFilterName::Unknown(UnknownVariantValue("NewFeature".to_owned()))`
/// and calling `as_str` on it yields `"NewFeature"`.
/// This match expression is forward-compatible when executed with a newer
/// version of SDK where the variant `GetEntitlementFilterName::NewFeature` is defined.
/// Specifically, when `getentitlementfiltername` represents `NewFeature`,
/// the execution path will hit the second last match arm as before by virtue of
/// calling `as_str` on `GetEntitlementFilterName::NewFeature` also yielding `"NewFeature"`.
/// 
/// Explicitly matching on the `Unknown` variant should
/// be avoided for two reasons:
/// - The inner data `UnknownVariantValue` is opaque, and no further information can be extracted.
/// - It might inadvertently shadow other intended match arms.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::Eq, std::cmp::Ord, std::cmp::PartialEq, std::cmp::PartialOrd, std::fmt::Debug, std::hash::Hash)]
pub enum GetEntitlementFilterName {
    #[allow(missing_docs)] // documentation missing in model
    CustomerIdentifier,
    #[allow(missing_docs)] // documentation missing in model
    Dimension,
    /// `Unknown` contains new variants that have been added since this code was generated.
    Unknown(crate::types::UnknownVariantValue)
}
impl std::convert::From<&str> for GetEntitlementFilterName {
                fn from(s: &str) -> Self {
                    match s {
                        "CUSTOMER_IDENTIFIER" => GetEntitlementFilterName::CustomerIdentifier,
"DIMENSION" => GetEntitlementFilterName::Dimension,
other => GetEntitlementFilterName::Unknown(crate::types::UnknownVariantValue(other.to_owned()))
                    }
                }
            }
impl std::str::FromStr for GetEntitlementFilterName {
                type Err = std::convert::Infallible;

                fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
                    Ok(GetEntitlementFilterName::from(s))
                }
            }
impl GetEntitlementFilterName {
                /// Returns the `&str` value of the enum member.
                pub fn as_str(&self) -> &str {
                    match self {
    GetEntitlementFilterName::CustomerIdentifier => "CUSTOMER_IDENTIFIER",
    GetEntitlementFilterName::Dimension => "DIMENSION",
    GetEntitlementFilterName::Unknown(value) => value.as_str()
}
                }
                /// Returns all the `&str` representations of the enum members.
                pub const fn values() -> &'static [&'static str] {
                    &["CUSTOMER_IDENTIFIER", "DIMENSION"]
                }
            }
impl AsRef<str> for GetEntitlementFilterName {
                fn as_ref(&self) -> &str {
                    self.as_str()
                }
            }

